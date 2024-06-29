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

extern crate self as airone;

use crate::prelude::{save_mode::ManualSave, *};
use paste::paste;
use std::fs::remove_file;

#[derive(Debug, AironeDbDerive)]
pub struct Animal {
    a: i32,
    n: f64,
    testo: String,
}

#[derive(PartialEq, Debug, AironeDbDerive)]
pub struct Foo {
    a: i32,
}

#[test]
fn save_increment_delete() {
    let mut db: AironeDb<Animal, save_mode::AutoSave> =
        AironeDb::new_with_custom_name("animal_increment_delete").unwrap();
    db.push(Animal {
        a: 0,
        n: 5.0,
        testo: "Abc".to_string(),
    })
    .unwrap();
    db.push(Animal {
        a: 3,
        n: std::f64::consts::SQRT_2,
        testo: "Se\nco\nndo".to_string(),
    })
    .unwrap();
    assert_eq!(db.len(), 2);

    let a = *db.get(0).unwrap().get_a();
    db.get_mut(0).unwrap().set_a(a + 1).unwrap();

    assert_eq!(*db.get(0).unwrap().get_a(), 1);

    db.get_mut(1)
        .unwrap()
        .set_testo("Abc\tdd\n".into())
        .unwrap();
    assert_eq!(db.get(1).unwrap().get_testo(), "Abc\tdd\n");

    db.remove(0).unwrap();
    db.remove(0).unwrap();

    remove_file("animal_increment_delete.csv").unwrap();
    remove_file("animal_increment_delete.changes.csv").unwrap();
}

#[test]
#[should_panic]
fn remove_empty_list() {
    remove_file("empty_list.changes.csv").unwrap();
    remove_file("empty_list.csv").unwrap();
    let mut db: AironeDb<Animal> = AironeDb::new_with_custom_name("empty_list").unwrap();

    // Test cleanup

    db.remove(0).unwrap();
    remove_file("empty_list.changes.csv").unwrap();
    remove_file("empty_list.csv").unwrap();
}

#[test]
fn serialize_deserialize() {
    #[derive(Debug, AironeDbDerive)]
    struct SerdeTest {
        testo: String,
        opt: Option<String>,
        op2: Option<i32>,
        op3: Option<f64>,
        op_bool: Option<bool>,
        op_none: Option<bool>,
    }
    const STR_1: &str = "Abc\t\n\\\n\r$";
    {
        let mut db: AironeDb<SerdeTest> = AironeDb::new_with_custom_name("serde").unwrap();
        db.push(SerdeTest {
            testo: STR_1.to_string(),
            opt: Some(STR_1.to_string()),
            op2: Some(-34354),
            op3: Some(-0.23834),
            op_bool: Some(false),
            op_none: None,
        })
        .unwrap();
    }
    {
        let db: AironeDb<SerdeTest> = AironeDb::new_with_custom_name("serde").unwrap();
        assert_eq!(db[0].get_testo(), STR_1);
        assert_eq!(*db[0].get_opt(), Some(STR_1.to_string()));
        assert_eq!(*db[0].get_op2(), Some(-34354));
        assert_eq!(*db[0].get_op3(), Some(-0.23834));
        assert!(db[0].get_op_bool().is_some());
        assert!(!db[0].get_op_bool().unwrap());
        assert!(db.get(0).unwrap().get_op_none().is_none());
    }
    remove_file("serde.changes.csv").unwrap();
    remove_file("serde.csv").unwrap();
}

#[test]
fn bulk_delete() {
    {
        let mut db: AironeDb<Animal> = AironeDb::new_with_custom_name("bulk_delete").unwrap();
        db.push(Animal {
            a: 0,
            n: 5.0,
            testo: "Abc".to_string(),
        })
        .unwrap();
        db.push(Animal {
            a: 1,
            n: std::f64::consts::SQRT_2,
            testo: "Se\nco\nndo".to_string(),
        })
        .unwrap();
        db.push(Animal {
            a: 2,
            n: std::f64::consts::SQRT_2,
            testo: "Se\nco\nndo".to_string(),
        })
        .unwrap();

        //db.drain(0..2).unwrap();
        let _: Vec<_> = db.drain(0..2).unwrap().collect();

        assert_eq!(db[0].get_a().clone(), 2);
        db.remove(0).unwrap();
    }
    {
        let mut db: AironeDb<Animal> = AironeDb::new_with_custom_name("bulk_delete").unwrap();
        db.push(Animal {
            a: 0,
            n: 5.0,
            testo: "Abc".to_string(),
        })
        .unwrap();
        db.push(Animal {
            a: 1,
            n: std::f64::consts::SQRT_2,
            testo: "Se\nco\nndo".to_string(),
        })
        .unwrap();
        db.push(Animal {
            a: 2,
            n: std::f64::consts::SQRT_2,
            testo: "Se\nco\nndo".to_string(),
        })
        .unwrap();

        //db.drain(1..2).unwrap();
        let _: Vec<_> = db.drain(1..2).unwrap().collect();

        assert_eq!(db[0].get_a().clone(), 0);
        assert_eq!(db[1].get_a().clone(), 2);
        db.clear().unwrap();
    }
    {
        let mut db: AironeDb<Animal> = AironeDb::new_with_custom_name("bulk_delete").unwrap();
        db.push(Animal {
            a: 0,
            n: 5.0,
            testo: "Abc".to_string(),
        })
        .unwrap();
        db.push(Animal {
            a: 1,
            n: std::f64::consts::SQRT_2,
            testo: "Se\nco\nndo".to_string(),
        })
        .unwrap();
        db.push(Animal {
            a: 2,
            n: std::f64::consts::SQRT_2,
            testo: "Se\nco\nndo".to_string(),
        })
        .unwrap();

        //db.drain(1..1).unwrap();
        let _: Vec<_> = db.drain(1..1).unwrap().collect();

        assert_eq!(db.len(), 3);
        db.clear().unwrap();
    }
    {
        let mut db: AironeDb<Animal> = AironeDb::new_with_custom_name("bulk_delete").unwrap();
        db.push(Animal {
            a: 0,
            n: 5.0,
            testo: "Abc".to_string(),
        })
        .unwrap();
        db.push(Animal {
            a: 1,
            n: std::f64::consts::SQRT_2,
            testo: "Se\nco\nndo".to_string(),
        })
        .unwrap();
        db.push(Animal {
            a: 2,
            n: std::f64::consts::SQRT_2,
            testo: "Se\nco\nndo".to_string(),
        })
        .unwrap();

        //db.drain(0..=2).unwrap();
        let _: Vec<_> = db.drain(0..=2).unwrap().collect();

        assert_eq!(db.len(), 0);
        db.clear().unwrap();
    }

    remove_file("bulk_delete.csv").unwrap();
    remove_file("bulk_delete.changes.csv").unwrap();
}

#[test]
fn run_types_tests() {
    let _ = remove_file("MyStruct.csv");
    let _ = remove_file("MyStruct.changes.csv");
    macro_rules! type_test {
        ( $( ($my_type:ty, $value:expr) ),* ) => {
            $(
                paste!{
                    #[allow(non_snake_case)]
                    fn [<types_testing_ $my_type>]()
                    {
                        println!("Running for type {}", stringify!($my_type));
                        #[derive(Debug, AironeDbDerive)]
                        struct MyStruct{
                            field: $my_type
                        }

                        {
                            let mut db = AironeDb::<MyStruct>::new().unwrap();
                            db.push(MyStruct{field: $value}).unwrap();
                        }
                        {
                            let db: AironeDb<MyStruct> = AironeDb::<MyStruct>::new().unwrap();
                            assert_eq!(
                                *db[0].get_field(),
                                $value
                            );
                        }

                        remove_file("MyStruct.csv").unwrap();
                        remove_file("MyStruct.changes.csv").unwrap();
                    }

                    // Run built function
                    [<types_testing_ $my_type>]();
                }
            )*

        };
    }

    type_test!(
        (i32, -2317833),
        (u32, 3834733),
        (bool, false),
        (f32, -34879234789234.33),
        (f64, -2378932389.3434),
        (usize, 789234),
        (String, "Asad\ned\th\r".to_string())
    );
}

#[test]
fn chaining() {
    let mut db: AironeDb<Animal> = AironeDb::new_with_custom_name("chaining").unwrap();
    db.push(Animal {
        a: 0,
        n: 5.0,
        testo: "Abc".to_string(),
    })
    .unwrap();
    db.push(Animal {
        a: 1,
        n: std::f64::consts::SQRT_2,
        testo: "Se\nco\nndo".to_string(),
    })
    .unwrap();
    db.push(Animal {
        a: 2,
        n: std::f64::consts::SQRT_2,
        testo: "Se\nco\nndo".to_string(),
    })
    .unwrap();
    db.push(Animal {
        a: 3,
        n: std::f64::consts::SQRT_2,
        testo: "Se\nco\nndo".to_string(),
    })
    .unwrap();

    let _ = db.retain(|e| *e.get_a() != 1);

    assert_eq!(db.len(), 3);
    assert_eq!(*db[0].get_a(), 0);
    assert_eq!(*db[1].get_a(), 2);
    assert_eq!(*db[2].get_a(), 3);

    {
        let mut res = db.iter().filter(|e| e.get_testo() == "Se\nco\nndo");
        assert_eq!(res.next().unwrap().get_a(), &2);
        assert_eq!(res.next().unwrap().get_a(), &3);
    }

    db.remove(0).unwrap();

    remove_file("chaining.csv").unwrap();
    remove_file("chaining.changes.csv").unwrap();
}

#[test]
fn rollback_test() {
    let _ = remove_file("rollback_test.csv");
    let _ = remove_file("rollback_test.changes.csv");
    let mut db: AironeDb<Animal> = AironeDb::new_with_custom_name("rollback_test").unwrap();
    db.push(Animal {
        a: 0,
        n: 5.0,
        testo: "Abc".to_string(),
    })
    .unwrap();
    db.push(Animal {
        a: 1,
        n: std::f64::consts::SQRT_2,
        testo: "Se\nco\nndo".to_string(),
    })
    .unwrap();
    db.push(Animal {
        a: 2,
        n: std::f64::consts::SQRT_2,
        testo: "Se\nco\nndo".to_string(),
    })
    .unwrap();

    // Add and rollback
    let mut db = db.set_save_mode::<ManualSave>().unwrap();
    db.push(Animal {
        a: 156,
        n: std::f64::consts::SQRT_2,
        testo: "test".to_string(),
    });
    db.push(Animal {
        a: 157,
        n: std::f64::consts::SQRT_2,
        testo: "test2".to_string(),
    });
    assert_eq!(db.len(), 5);
    db.rollback();
    assert_eq!(*db[0].get_a(), 0);
    assert_eq!(*db[1].get_a(), 1);
    assert_eq!(*db[2].get_a(), 2);
    assert_eq!(db.len(), 3);

    // Add and rollback
    db.push(Animal {
        a: 156,
        n: std::f64::consts::SQRT_2,
        testo: "test".to_string(),
    });
    db.push(Animal {
        a: 157,
        n: std::f64::consts::SQRT_2,
        testo: "test2".to_string(),
    });
    assert_eq!(db.len(), 5);
    db.save().unwrap();
    db.rollback();
    assert_eq!(db.len(), 5);
    //db.drain(3..=4);
    //let _: Vec<_> = db.drain(3..=4).unwrap().collect();
    db.save().unwrap();

    // Edit and rollback
    db.get_mut(2).unwrap().set_a(10);
    assert_eq!(*db[2].get_a(), 10);

    db.rollback();
    assert_eq!(*db[2].get_a(), 2);

    // Delete and rollback
    db.remove(1);
    db.remove(0);
    assert_eq!(db.len(), 1);
    db.rollback();
    assert_eq!(db.len(), 3);

    remove_file("rollback_test.csv").unwrap();
    remove_file("rollback_test.changes.csv").unwrap();
}

#[test]
fn add_remove_rollback() {
    let mut db: AironeDb<Animal, ManualSave> =
        AironeDb::new_with_custom_name("add_remove_rollback").unwrap();
    db.push(Animal {
        a: 0,
        n: 5.0,
        testo: "Abc".to_string(),
    });
    db.push(Animal {
        a: 1,
        n: std::f64::consts::SQRT_2,
        testo: "Se\nco\nndo".to_string(),
    });
    db.push(Animal {
        a: 2,
        n: std::f64::consts::SQRT_2,
        testo: "Se\nco\nndo".to_string(),
    });
    db.save().unwrap();

    db.remove(1);
    db.rollback();

    assert_eq!(db.len(), 3);
    assert_eq!(*db[0].get_a(), 0);
    assert_eq!(*db[1].get_a(), 1);
    assert_eq!(*db[2].get_a(), 2);

    // Insert at position 1,
    // then rollback
    db.insert(
        1,
        Animal {
            a: 56,
            n: std::f64::consts::SQRT_2,
            testo: "Se\nco\nndo".to_string(),
        },
    );
    assert_eq!(db.len(), 4);
    assert_eq!(*db[0].get_a(), 0);
    assert_eq!(*db[1].get_a(), 56);
    assert_eq!(*db[2].get_a(), 1);

    db.rollback();
    assert_eq!(db.len(), 3);
    assert_eq!(*db[0].get_a(), 0);
    assert_eq!(*db[1].get_a(), 1);
    assert_eq!(*db[2].get_a(), 2);

    // Insert at given position and commit.
    // Then, let's try to remove it and
    // rollback the deletion
    db.insert(
        1,
        Animal {
            a: 56,
            n: std::f64::consts::SQRT_2,
            testo: "Se\nco\nndo".to_string(),
        },
    );
    db.save().unwrap();
    db.remove(1);
    assert_eq!(db.len(), 3);

    db.rollback();
    assert_eq!(db.len(), 4);
    assert_eq!(*db[0].get_a(), 0);
    assert_eq!(*db[1].get_a(), 56);
    assert_eq!(*db[2].get_a(), 1);

    remove_file("add_remove_rollback.csv").unwrap();
    remove_file("add_remove_rollback.changes.csv").unwrap();
}

#[test]
fn clear() {
    let mut db: AironeDb<Animal> = AironeDb::new_with_custom_name("clear_elements").unwrap();

    db.push(Animal {
        a: 3,
        n: std::f64::consts::SQRT_2,
        testo: "Se\nco\nndo".to_string(),
    })
    .unwrap();
    db.push(Animal {
        a: 3,
        n: std::f64::consts::SQRT_2,
        testo: "Se\nco\nndo".to_string(),
    })
    .unwrap();
    db.push(Animal {
        a: 3,
        n: std::f64::consts::SQRT_2,
        testo: "Se\nco\nndo".to_string(),
    })
    .unwrap();
    assert!(!db.is_empty());
    db.clear().unwrap();

    assert_eq!(db.len(), 0);
    assert!(db.is_empty());

    remove_file("clear_elements.csv").unwrap();
    remove_file("clear_elements.changes.csv").unwrap();
}

#[test]
fn get_set_access() {
    let mut db: AironeDb<Foo> = AironeDb::<Foo>::new_with_custom_name("FooGetSet").unwrap();
    db.push(Foo { a: 0 }).unwrap();

    db.get_mut(0).unwrap().set_a(5).unwrap();
    assert_eq!(*db.get(0).unwrap().get_a(), 5);

    remove_file("FooGetSet.csv").unwrap();
    remove_file("FooGetSet.changes.csv").unwrap();
}

#[test]
fn bulk_push_test() {
    let mut db = AironeDb::<Foo>::new_with_custom_name("BulkPushTest").unwrap();
    db.extend(std::iter::repeat(5).take(500).map(|n| Foo { a: n }))
        .unwrap();

    assert_eq!(db.len(), 500);

    db.clear().unwrap();

    remove_file("BulkPushTest.csv").unwrap();
    remove_file("BulkPushTest.changes.csv").unwrap();
}

#[test]
fn dedup_test() {
    // Prepare test array
    let desired_data = [0, 5, 3, 2];
    let mut db: AironeDb<Foo> = AironeDb::<Foo>::new_with_custom_name("DedupTest").unwrap();
    for e in desired_data {
        db.push(Foo { a: e }).unwrap();
    }
    // Apply dedup. Everything should remain the same
    db.dedup().unwrap();
    //for i in 0..desired_data.len() {
    for (i, data) in desired_data.iter().enumerate() {
        assert_eq!(*data, db.get(i).unwrap().a);
    }
    db.clear().unwrap();

    // Test with some duplicates
    for e in desired_data {
        db.push(Foo { a: e }).unwrap();
        db.push(Foo { a: e }).unwrap();
        db.push(Foo { a: e }).unwrap();
    }
    assert_eq!(db.len(), 12);
    db.dedup().unwrap();
    assert_eq!(db.len(), 4);
    //for i in 0..desired_data.len() {
    for (i, data) in desired_data.iter().enumerate() {
        assert_eq!(*data, db.get(i).unwrap().a);
    }
    db.clear().unwrap();

    // Test where everything is duplicated
    // The array should remain with only one, non-duplicated, item
    for _i in 0..25 {
        db.push(Foo { a: 0 }).unwrap();
    }
    assert_eq!(db.len(), 25);
    db.dedup().unwrap();
    assert_eq!(db.len(), 1);
    assert_eq!(db[0].a, 0);

    remove_file("DedupTest.csv").unwrap();
    remove_file("DedupTest.changes.csv").unwrap();
}
