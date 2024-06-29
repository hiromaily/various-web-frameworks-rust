//  ------------------------------------------------------------------
//  Airone
//  is a Rust library which provides a simple in-memory,
//  write-on-update database that is persisted
//  to an append-only transaction file.
//
//  Copyright © 2022,2023,2024 Massimo Gismondi
//
//  This file is part of Airone.
//  Airone is free software: you can redistribute it and/or
//  modify it under the terms of the GNU Affero General Public License
//  as published by the Free Software Foundation, either version 3
//  of the License, or (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//  GNU General Public License for more details.

//  You should have received a copy of the GNU Affero General Public License
//  along with this program. If not, see <https://www.gnu.org/licenses/>.
//  ------------------------------------------------------------------

// Bulk operations can be heavy to write, as each time
// you make a change, the output file is flushed to ensure it's persisted to disk.
//
// If you're making many changes together, for example adding 500 elements to the db in one shot,
// it's better to flush data to disk only once, at the end of the operation.
// This is called a `bulk operation`.
//
// Some operations like clearing a vector handle it internally automatically.

use airone::prelude::*;

// First define the struct.
// We will persist a list of MyTestStruct objects.
//
// Derive the aironeDbDerive trait on it
#[derive(AironeDbDerive)]
struct MyTestStruct {
    first_field: u32,
    name: String,
    an_id: usize,
}

fn main() {
    // Instantiate an aironedb, containing a vec of MyTestStruct structs
    let mut db: AironeDb<MyTestStruct, save_mode::AutoSave> = AironeDb::new().unwrap();

    // --------------------
    //     SLOW VERSION
    // --------------------
    // Add many elements in one shot.
    for i in 0..500 {
        // This causes the changes file to be flushed once per push operation
        db.push(MyTestStruct {
            first_field: i,
            name: String::from("test_string"),
            an_id: 57,
        })
        .unwrap();
    }
    // Let's remove the elements, one by one.
    // This is NOT optimal
    for _ in 0..500 {
        // This causes the changes file to be flushed once per push operation
        db.pop().unwrap();
    }

    // --------------------
    //     FAST VERSION
    // --------------------
    //
    // Add and clear the db as previously, but flush the changes only once

    // Push in bulk elements from a Vec
    let mut elements = vec![
        MyTestStruct {
            first_field: 0,
            name: String::from("test_string"),
            an_id: 9,
        },
        MyTestStruct {
            first_field: 1,
            name: String::from("test_string"),
            an_id: 5,
        },
        MyTestStruct {
            first_field: 2,
            name: String::from("test_string"),
            an_id: 7,
        },
    ];
    db.append(&mut elements).unwrap();

    // A single call that removes every element
    // in an faster way
    db.clear().unwrap();

    // Keep the example clean
    // and delete what we have done,
    // just to keep it tidy
    db.clear().unwrap();
}
