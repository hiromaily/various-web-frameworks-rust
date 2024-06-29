# Airone

This crate is forked from [airone](https://gitlab.com/MassiminoilTrace/airone).

## Overview

Airone is a Rust library which provides a simple in-memory, write-on-update database that is persisted to an append-only transaction file, inspired from Aral Balkan's [JSDB](https://codeberg.org/small-tech/jsdb).

In short, it acts as a `Vec<T>`, but any change is automatically saved to file rapidly.

The name has nothing to do with "air" or "one", it simply comes from the Italian word "Airone", which means "Heron".
Hence, it has to be read `[aiˈrone]`.

- save process is fast
- keeps all data in memory for rapid read access

This library persists a list of structs from memory to disk. Every change in the data in memory is saved to disk automatically in a fast way, avoiding the full dump of the whole list.

AironeDb objects resemble simple arrays of structs, but they automatically save any change to disk under the hood. Here's an example:

```rust
let mut my_airone_array = AironeDb::<MyStruct>::new().unwrap();

my_airone_array.push(    //  <-- This command
    MyStruct{field1: 5}  //      saves the new data to disk
);                       //      automatically

my_airone_array
    .get_mut(0).unwrap() //  <-- Get a mutable proxy
                         //      to the element at index 0
    .set_field1(10);     //  <-- A setter method is generated automatically.
                         //      The change is persisted to file.

println!("{}",
    my_airone_array[0].field1 // you can access it by index
);
```

The Rust-to-C library bridge has been removed, in favour of a pure C library version, which is smaller, it simplifies the procmacro maintainability and is easier for you to include in any C-compatible project. You can find it at <https://gitlab.com/MassiminoilTrace/airone_c>.

## Usage scenario

Airone is developed to be used in situations where all of these prerequisites apply:

- no big-data: the whole dataset should fit in memory
- the dataset has to be persisted to disk
- after some data is modified, the change needs to be written to disk in a _fast_ way
- we can slow down the program start time without any relevant consequence
- no nested objects

Here are two examples of good usage scenarios:

- small web server: data may change fast and can entirely fit into memory. The startup can be slow, as long as the server is performant _while_ it's running. I provide an [external template](https://gitlab.com/MassiminoilTrace/rocket_plus_airone_template) using airone and a Rocket server
- an offline GUI/TUI program: you can store data modifications in a very fast way thanks to airone's append-only file architecture, so the interface freeze during the saving process is barely noticeable. Here's a very barebone [template](https://gitlab.com/MassiminoilTrace/gtk3_plus_airone_template) combining airone with Gtk3

These limits can be a problem for some usage scenarios. Existing databases add all sort of optimizations, caching mechanisms and are very complex, in order to be able to deal with such big amount of data.  
However, when the above-mentioned prerequisites apply, we can leverage these limits to simplify the source code, making it more maintanable and getting rid of all the bloatware.
Moreover, limiting object nesting makes it compatible with CSV style files, ensuring you can manipulate data with standard UNIX tools such as `grep` and `awk`.

## Context

Airone is aimed at helping _small-tech_ to be produced, which is a way of building technology antithetical to the Silicon Valley model. Small-tech features close-to-zero data collection, a free and open source structure to achieve good transparency, no lust to scale exponentially, no desire in tracking people's behaviour while crunching tons of data and, overall, it has a goal to keep things simple and human.

## Documentation

The most basic usage example looks like this:

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

From this point, you can interact with a Vec of data of type Foo and automatically persist any change to file automatically.

```rust
let mut db: AironeDb<Foo> = AironeDb::new()?;
// Add an element
db.push(
    struct Foo{field1: 0.0, field2: "Abc".to_string(), field3: None}
    );
db.get(0); // <-- returns a read only Option<&Foo>

let m = db.get_mut(0).unwrap()  // <-- returns a convenient
    .set_field1(0.5);           //     writable wrapper of Foo
                                //     in an Option<T>

// Remove element at index 0
db.remove(0);
```

Head to the official [Documentation](https://massiminoiltrace.gitlab.io/airone/airone/index.html) for the full usage instructions and implementation details.

# Copyright

This is **NOT** public domain, make sure to respect the license terms.
You can find the license text in the [COPYING](https://gitlab.com/MassiminoilTrace/airone/-/blob/master/COPYING) file.

Copyright © 2022,2023,2024 Massimo Gismondi

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program. If not, see https://www.gnu.org/licenses/.

## External libraries used in this project

This project uses some external libraries as described in the `Cargo.toml` file.

- AGPL-3.0-or-later: airone_derive
- MIT / Apache-2.0: proc-macro2, quote, syn
- (MIT / Apache-2.0) AND Unicode-DFS-2016: unicode-ident

This project is licensed using the license you can find in the COPYING file.
