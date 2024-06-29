# Airone

Airone is a library inspired from Aral Balkan's [JSDB](https://github.com/small-tech/jsdb) principles applied to a Rust library.

The name has nothing to do with "air" or "one", it simply comes from the Italian word "Airone", which means "Heron".
Hence, it has to be read `[aiˈrone]`. The idea behind is to be "elegantly lightweight".

This library persists a list of structs from memory to disk. Every change in the data in memory is saved to disk automatically in a fast way, avoiding the full dump of the whole list.

As saving a long list of objects to a file each time a change happens may be cumbersome, `airone` writes each incremental change to an append-only file. This incremental transaction log is applied at the next restart to compact the base dump file; in this way, the heavy operation of writing the full dump of data from memory is executed only once at startup. After that, each change is written in a fast way to the append-only log.

# Architecture

## Usage scenario

Airone is developed to be used in situations where all of these prerequisites apply:

- no big-data: the whole dataset should fit in memory
- the dataset has to be persisted to disk
- after some data is modified, the change needs to be written to disk in a _fast_ way
- we can slow down the program start time without any relevant consequence
- no nested objects

These limits can be a problem for some usage scenarios. Existing databases add all sort of optimizations, caching mechanisms and are very complex, in order to be able to deal with such big amount of data.  
However, when these prerequisites apply, we can leverage these limits to simplify the source code, making it more maintanable and getting rid of all the bloatware.
Moreover, limiting object nesting makes it compatible with CSV style files, ensuring you can manipulate data with standard UNIX tools such as `grep` and `awk`.

Here are two examples of good usage scenarios:

- small web server: data may change fast and can entirely fit into memory. The startup can be slow, as long as the server is performant _while_ it's running. I provide an [external template](https://gitlab.com/MassiminoilTrace/rocket_plus_airone_template) using airone and a Rocket server
- an offline GUI/TUI program: you can store data modifications in a very fast way thanks to airone's append-only file architecture, so the interface freeze during the saving process is barely noticeable. Here's a very barebone [template](https://gitlab.com/MassiminoilTrace/gtk3_plus_airone_template) combining airone with Gtk3

## Context

Airone is aimed at helping _small-tech_ to be produced, which is a way of building technology antithetical to the Silicon Valley model. While the sylicon valley architecture is optimized for big data, Small-tech is optimized for close-to-zero data collection. It also has a free and open source structure to achieve good transparency, no lust to scale exponentially, no desire in tracking people's behaviour while crunching tons of data and, overall, it has a goal to keep things simple and human.

## Implementation limitations

The derive macro currently does _not_ support fields with reference and lifetime like `&'a str`; use owned types like `String` instead.

### State of the project

The project has been used in some offline programs, let me know if you used it successfully in new production projects and how it can be improved.

### Operating system

`airone` has not been tested on Windows and Mac. Beware that WSL is a compatibility layer and may unexpectedly break too. I encourage you to switch to an actually freedom-and-privacy respecting operating system such as GNU/Linux, where this library has been tested on more. Head to <https://www.youtube.com/watch?v=Ag1AKIl_2GM> for more information.

### Licensing

This project is licensed under an AGPL style license. Head to the COPYING file and Copyright section for detailed information.
Make sure to respect the license terms to use this library.

If you want to integrate `airone` into a program with closed source or other non-compatible license, I can sell you a copy with a custom license or provide consultation and support service. The only caveat is that development efforts, bug fixes and new features developed in this way will also be merged back into this FOSS version.

# Usage

## Installation

Add this line to your dependencies section of the `Cargo.toml` file.

```toml
airone = "0.8.0"
```

## Basic operations

The crate exposes a generic struct `AironeDb<T>` and a convenient macro to derive custom types to be used as T.

The core lies in the [AironeDbDerive](airone_derive::AironeDbDerive) derive macro. Apply it to a struct named as you wish to it
will define implementations for `AironeDb<Foo>`.
The newly creates struct acts as a proxy between you and the underlying list of data, automatically persisting
changes when they happen and providing methods to interact with them.

The most basic setup you can achieve looks like this. First, we import the macros and the needed traits. Then, we generate the implementations for the desired type by using [AironeDbDerive](airone_derive::AironeDbDerive).

```rust
use airone::prelude::*;

#[derive(AironeDbDerive)]
struct Foo
{
    pub field1: f64,
    pub field2: String,
    field3: Option<i32>
}
```

And here is how you can interact with your data, mainly using methods provided in [AironeDb](database::AironeDb).

You can also transparently access the data in readmode using the index operator.

You can access and edit data by using common Vec methods, like [push()](<database::AironeDb::push()>), [get()](<database::AironeDb::get()>) or [get_mut()](<database::AironeDb::get_mut()>) methods.

```rust
# use airone::prelude::*;
#
# // This generates implementations for type AironeDb<Foo>
# // to interact with the data while saving any change to disk.
# #[derive(AironeDbDerive)]
# struct Foo
# {
#     pub field1: f64,
#     pub field2: String,
#     field3: Option<i32>
# }
#
{
    // Open the database
    let mut db: AironeDb<Foo> = AironeDb::new().unwrap();
    // Add one element using a public method
    db.push(
        Foo{
            field1: 0.5,
            field2: "Hello world".to_string(),
            field3: Some(-45)
        }
    );
    // Change a field using the generated setter method
    db.get_mut(0).unwrap().set_field3(None);
    // The database is closed automatically here
}
{
    // Open again, check the modified data
    // has been correctly persisted.
    let db: AironeDb<Foo> = AironeDb::new().unwrap();
    assert_eq!(
        *db[0].get_field3(),
        None
    );

    // Access using index directly in read-mode
    db[0].get_field3();
}
# use std::fs::{remove_file};
# remove_file("Foo.csv").unwrap();
# remove_file("Foo.changes.csv").unwrap();
```

In addition to the methods from the [AironeDb](database::AironeDb) struct,
some getter and setters are generated for each variable to change the element
at the specified index in the form of:

```rust,ignore
fn set_$field_name(&mut self, new_value: $field_type)
fn get_$field_name(&self)
```

## Querying data

An AironeDb object can be turned into a readable iterator or can be filtered using standard Vec methods like [drain](Vec::drain) and [retain](<Vec::retain()>)

```rust
# use airone::prelude::*;
# #[derive(AironeDbDerive)]
# struct QueryExample
# {
#     internal_id: i32,
#     my_text: String
# }
#
let mut db: AironeDb<QueryExample> = AironeDb::new().unwrap();
// Fill in data how you want here
// …
//
# db.push(
#     QueryExample{
#         internal_id: 56,
#         my_text: "Hello world".to_string()
#     }
# );
# db.push(
#     QueryExample{
#         internal_id: 57,
#         my_text: "Test string".to_string()
#     }
# );
# db.push(
#     QueryExample{
#         internal_id: 57,
#         my_text: "Test string".to_string()
#     }
# );
// Can use dot notation chaining operations
db.retain(
    |e|
    {
        e.get_my_text() == "Test string"
    }
);

// Do something else with `db` object


# use std::fs::{remove_file};
# remove_file("QueryExample.csv").unwrap();
# remove_file("QueryExample.changes.csv").unwrap();
```

## Configuration

### Save mode

Airone can operate in two ways, depending on the provided generic unit struct.

- [AutoSave](database::settings::save_mode::AutoSave): data is saved to file at _every_ modification, each time a mutable method edits some value
  - benefit: you're sure data is saved, no need to remember saving data
  - cons: all mutable methods return a Result, with a possible ioError. Also you can't rollback changes in memory, because each change is instantly persisted.
- [ManualSave](database::settings::save_mode::ManualSave): data is saved to file _only_ when you manually do so
  - benefit: cleaner to read, as mutable methods only change data in-memory and don't return IoErrors. You'll need to handle a Result only when saving. Also, you can rollback data in memory to the last known written state
  - cons: easier to forget to save the data to disk

```rust
# use airone::prelude::*;
# use save_mode::{AutoSave, ManualSave};
# #[derive(AironeDbDerive)]
# struct MyStruct{f: i32}
/// AutoSave mode
let mut db: AironeDb<MyStruct, AutoSave> = AironeDb::new().unwrap();
db.push(MyStruct{f:0})
    .unwrap(); // A Result is returned
               //because we're writing to disk

/// Set Airone to ManualSave mode
let mut db = db.set_save_mode::<ManualSave>().unwrap();
db.push(MyStruct{f:0}); // No need to unwrap, change happens in memory only
db.save().unwrap(); // we're writing to disk only here

```

### Buffering write

Writing to file can happen either in sync or in buffered mode.

[BufferMode](database::settings::BufferMode::Buffered) will reduce the number of syscall by writing to file only when enough changes have been collected.
[AlwaysFlush](database::settings::BufferMode::AlwaysFlush) mode will flush data to file after each mutable operation.

```rust
# use airone::prelude::*;
# #[derive(AironeDbDerive)]
# struct MyStruct{f: i32}
let mut db: AironeDb<MyStruct> = AironeDb::new().unwrap();
db.set_buffer_mode(BufferMode::AlwaysFlush);
```

# Internal Data Format

Data is written into two files, depending of the phase of execution.

The base dump file contains the full dump of data in memory. This is recreated whenever the program starts by using the old dump data file as a base point and applying each incremental change to it. Afterwards, the data is saved to the a new dump file and the old transaction log is deleted.

From this point, the program continues its execution saving changes to a new transaction log.

Both files follow this character convention:

- the `\n` character is used as a newline (no carriage return)
- the `\t` character is used as a field separator.

## Base Dump File

The base dump file is saved using a standard UNIX-style CSV text file.

Let's use this struct as an example:

```rust
struct ExampleStruct
{
    field1: String,
    field2: f64,
    field3: i32
}
```

Given a list of two `ExampleStruct` elements, the base dump file could look like this. Notice the first line used as a column header:

```plain
field1	field2	field3
abc	3.15	57
text2	47.89	-227
```

If columns are reordered or renamed in the struct or in the csv, airone panics at startup to avoid corrupting data, manually fix the csv or the struct field and try to re-run

## Append-only transaction log

When the program is running, changes are written to the append-only transaction log.
Each line of this file is formatted as it follows, depending on the applied operation.

### Adding an element

The first letter `A` sets the operation to `Add`. The new object fields are serialized as in the base dump file, by writing each field's value in the proper order.

The index sets the position in the list where the element will be added.

Structure:

```plain
A	index	field1	field2	field3
```

Example:

```plain
A	3	abc	3.15	57
```

### Deleting an element

The first letter `D` sets the operation to `Delete`. After that, it expects the index of the element to remove.

Structure:

```plain
D	index_of_element_to_remove
```

Example:

```plain
D	2
```

### Setting a field value

The first letter `E` sets the operation to `Edit`. After that field, adhere to the following structure.

Structure:

```plain
E	index_of_element	field_to_change	new_value
```

Example

```plain
E	0	field2	-57.5
```

## Extending and supporting custom types

You can serialize and deserialize your custom types by implementing [SerializableField](serde::SerializableField) trait on each field type.

# Copyright

This is **NOT** public domain, make sure to respect the license terms.
You can find the license text in the [COPYING](https://gitlab.com/MassiminoilTrace/airone/-/blob/master/COPYING) file.

Copyright © 2022,2023,2024 Massimo Gismondi

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
