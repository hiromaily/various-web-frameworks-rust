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

use std::any::Any;

use crate::error::Error;

/// This has methods to get and set the value
/// by a string key
pub trait InnerStruct: Serialize + Deserialize {
    const COLUMNS: &'static [&'static str];
    const STRUCT_NAME: &'static str;
    fn set_str(&mut self, key: &str, value: SerializedFieldValue) -> Result<(), Error>;
    fn set<V: SerializableField>(&mut self, key: &str, value: V);
    fn get<V: SerializableField>(&self, key: &str) -> &V;
}

pub trait Serialize {
    fn serialize(&self) -> SerializedStruct;
}
pub trait Deserialize
where
    Self: Sized,
{
    fn deserialize(s: &SerializedStruct) -> Result<Self, Error>;
}

pub trait SerializableField: Any + Clone
where
    Self: Sized,
{
    fn serialize_field(&self) -> SerializedFieldValue;
    fn deserialize_field(v: SerializedFieldValue) -> Result<Self, Error>;
}

#[derive(Clone, Debug)]
pub struct SerializedStruct(Vec<SerializedFieldValue>);
impl SerializedStruct {
    pub fn new(v: Vec<SerializedFieldValue>) -> Self {
        Self(v)
    }
    pub fn get_values(&self) -> &[SerializedFieldValue] {
        &self.0
    }
    pub(crate) fn to_escaped_line(&self) -> String {
        self.0
            .iter()
            .map(|s| s.to_escaped_string())
            .collect::<Vec<String>>()
            .join("\t")
    }
    pub(crate) fn from_escaped_line(l: &str) -> Self {
        Self(
            l.split('\t')
                .map(SerializedFieldValue::from_escaped_string)
                .collect(),
        )
    }
}

#[derive(Clone, Debug)]
pub struct SerializedFieldValue(String);
impl SerializedFieldValue {
    pub fn new(s: String) -> Self {
        Self(s)
    }
    pub fn get(&self) -> &str {
        &self.0
    }
    pub(crate) fn to_escaped_string(&self) -> String {
        self.0
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
    }
    pub(crate) fn from_escaped_string(s: &str) -> Self {
        SerializedFieldValue(
            s.replace("\\n", "\n")
                .replace("\\r", "\r")
                .replace("\\t", "\t"),
        )
    }
}

macro_rules! impl_serde_value {
    ($($t:ty),*) => {
        $(
            impl SerializableField for $t
            {
                fn serialize_field(&self) -> SerializedFieldValue
                {
                    SerializedFieldValue::new(self.to_string())
                }
                fn deserialize_field(v: SerializedFieldValue) -> Result<Self, Error> {
                    Ok(
                        v.get().parse()?
                    )
                }
            }
        )*
    };
}

impl_serde_value!(bool, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

impl<T: SerializableField> SerializableField for Option<T> {
    fn serialize_field(&self) -> SerializedFieldValue {
        if let Some(e) = self {
            e.serialize_field()
        } else {
            SerializedFieldValue::new(String::new())
        }
    }
    fn deserialize_field(v: SerializedFieldValue) -> Result<Self, Error> {
        let s = v.get();
        if !s.is_empty() {
            let t: T = SerializableField::deserialize_field(v)?;
            Ok(Some(t))
        } else {
            Ok(None)
        }
    }
}

// Impl strings
impl SerializableField for String {
    fn serialize_field(&self) -> SerializedFieldValue {
        SerializedFieldValue::new(self.to_string())
    }
    fn deserialize_field(v: SerializedFieldValue) -> Result<Self, Error> {
        Ok(v.get().to_string())
    }
}

#[cfg(test)]
mod example {
    use crate::{database::settings::save_mode::AutoSave, prelude::AironeDb};

    use super::*;

    #[derive(Clone)]
    struct Abc {
        a: i32,
        b: String,
    }
    impl InnerStruct for Abc {
        const COLUMNS: &'static [&'static str] = &["a", "b"];
        const STRUCT_NAME: &'static str = "Abc";
        fn get<V: SerializableField>(&self, key: &str) -> &V {
            use std::any::Any;
            match key {
                "a" => {
                    let v = &self.a as &dyn Any;
                    return v.downcast_ref::<V>().unwrap();
                }
                "b" => {
                    let v = &self.a as &dyn Any;
                    return v.downcast_ref::<V>().unwrap();
                }
                _ => panic!(""),
            }
        }
        fn set<V: SerializableField>(&mut self, key: &str, value: V) {
            use std::any::Any;
            match key {
                "a" => self.a = *(&value as &dyn Any).downcast_ref().unwrap(),
                "b" => self
                    .b
                    .clone_from((&value as &dyn Any).downcast_ref::<String>().unwrap()),
                _ => {
                    unreachable!()
                }
            }
        }
        fn set_str(&mut self, key: &str, value: SerializedFieldValue) -> Result<(), Error> {
            match key {
                "a" => {
                    self.a = SerializableField::deserialize_field(value)?;
                }
                "b" => {
                    self.b = SerializableField::deserialize_field(value)?;
                }
                _ => {
                    unreachable!()
                }
            }
            Ok(())
        }
    }
    impl Deserialize for Abc {
        fn deserialize(value: &SerializedStruct) -> Result<Self, Error> {
            let mut values = value.get_values().iter().cloned();
            Ok(Self {
                a: SerializableField::deserialize_field(values.next().unwrap())?,
                b: SerializableField::deserialize_field(values.next().unwrap())?,
            })
        }
    }
    impl Serialize for Abc {
        fn serialize(&self) -> SerializedStruct {
            SerializedStruct::new(vec![self.a.serialize_field(), self.b.serialize_field()])
        }
    }

    impl AironeDb<Abc, AutoSave> {
        #[allow(unused)]
        fn get_a(&self, index: usize) -> &i32 {
            self.get_all()[index].get("a")
        }
    }
}
