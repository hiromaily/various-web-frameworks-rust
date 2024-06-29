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

use airone::{database::settings::save_mode::AutoSave, prelude::*};
use airone_derive::AironeDbDerive;

// First define the struct you want to persist
// Derive the aironeDbDerive trait on it
#[derive(AironeDbDerive, Debug)]
struct MyTestStruct {
    first_field: u32,
    name: String,
    an_id: usize,
}

fn main() {
    // Instantiate an aironedb, containing a vec of MyTestStruct structs
    let mut db: AironeDb<MyTestStruct, AutoSave> = AironeDb::new().expect("");
    // Add a new element. The change is persisted automatically
    db.push(MyTestStruct {
        first_field: 0,
        name: String::from("test_string"),
        an_id: 57,
    })
    .unwrap();
    db.get_mut(0).unwrap().set_an_id(57).unwrap();
    db.get_mut(0).unwrap().set_name(String::from("aa")).unwrap();

    // // Remove the element
    db.remove(0).unwrap();
}
