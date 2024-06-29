extern crate self as airone;

use crate::prelude::*;

use self::save_mode::ManualSave;

#[derive(AironeDbDerive)]
struct Abc {
    field1: i32,
}

#[test]
fn manualsave() {
    {
        let mut d: AironeDb<Abc, ManualSave> =
            AironeDb::new_with_custom_name("manualsave").unwrap();

        d.push(Abc { field1: 0 });
        d.save().unwrap();

        d.pop();
    }

    let d: AironeDb<Abc, ManualSave> = AironeDb::new_with_custom_name("manualsave").unwrap();
    assert_eq!(d.len(), 1);

    let _ = std::fs::remove_file("manualsave.changes.csv");
    let _ = std::fs::remove_file("manualsave.csv");
}
