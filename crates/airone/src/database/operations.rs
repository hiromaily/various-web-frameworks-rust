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

use super::InnerStruct;
use super::{SerializedFieldValue, SerializedStruct};
use crate::error::Error;
use crate::serde::Deserialize;
use std::io::{self, Write};

#[derive(Debug)]
pub struct RevertableChange {
    forward_op: Operation,
    backward_op: Operation,
}
impl RevertableChange {
    pub fn new_add(index: usize, element: SerializedStruct) -> Self {
        RevertableChange {
            forward_op: Operation::new_insert(index, element),
            backward_op: Operation::new_delete(index),
        }
    }
    pub fn new_edit(
        index: usize,
        fieldname: &str,
        old_value: SerializedFieldValue,
        new_value: SerializedFieldValue,
    ) -> Self {
        RevertableChange {
            forward_op: Operation::new_edit(index, fieldname.to_string(), new_value),
            backward_op: Operation::new_edit(index, fieldname.to_string(), old_value),
        }
    }
    pub fn new_delete(index: usize, old_element: SerializedStruct) -> Self {
        RevertableChange {
            forward_op: Operation::new_delete(index),
            backward_op: Operation::new_insert(index, old_element),
        }
    }

    pub fn apply_forward<T: InnerStruct>(&self, data: &mut Vec<T>) -> Result<(), Error> {
        self.forward_op.apply(data)
    }
    pub fn apply_backward<T: InnerStruct>(self, data: &mut Vec<T>) -> Result<(), Error> {
        self.backward_op.apply(data)
    }
    pub fn persist(self, w: &mut impl Write) -> Result<(), io::Error> {
        self.forward_op.persist(w)
    }
}

#[derive(Clone, Debug)]
pub enum Operation {
    Add {
        index: usize,
        serialized_object: SerializedStruct,
    },
    Edit {
        index: usize,
        fieldname: String,
        serialized_new_value: SerializedFieldValue,
    },
    Delete {
        index: usize,
    },
}
impl Operation {
    pub fn new_insert(index: usize, object: SerializedStruct) -> Self {
        Self::Add {
            index,
            serialized_object: object,
        }
    }
    pub fn new_edit(index: usize, fieldname: String, new_value: SerializedFieldValue) -> Self {
        Self::Edit {
            index,
            fieldname,
            serialized_new_value: new_value,
        }
    }
    pub fn new_delete(index: usize) -> Self {
        Self::Delete { index }
    }

    fn to_line(&self) -> String {
        match self {
            Self::Add {
                index,
                serialized_object,
            } => {
                format!("A\t{index}\t{}", serialized_object.to_escaped_line())
            }
            Self::Edit {
                index,
                fieldname,
                serialized_new_value,
            } => {
                format!(
                    "E\t{index}\t{fieldname}\t{}",
                    serialized_new_value.to_escaped_string()
                )
            }
            Self::Delete { index } => {
                format!("D\t{index}")
            }
        }
    }

    pub fn from_line(line: &str) -> Result<Self, crate::error::Error> {
        let mut parts_iter = line.split('\t');
        let tipo_op = parts_iter.next().unwrap();
        if tipo_op == "A" {
            // Aggiungi un elemento
            let index: usize = parts_iter.next().unwrap().parse().unwrap();
            let obj_string = parts_iter.collect::<Vec<&str>>().join("\t");
            Ok(Operation::Add {
                index,
                serialized_object: SerializedStruct::from_escaped_line(&obj_string),
            })
        } else if tipo_op == "D" {
            let index: usize = parts_iter.next().unwrap().parse().unwrap();
            Ok(Operation::Delete { index })
        } else if tipo_op == "E" {
            // formato:
            // E indice campo valore
            let index: usize = parts_iter.next().unwrap().parse().unwrap();
            let field: String = parts_iter.next().unwrap().to_string();
            let value_str: String = parts_iter.next().unwrap().to_string();

            Ok(Operation::Edit {
                index,
                fieldname: field,
                serialized_new_value: SerializedFieldValue::from_escaped_string(&value_str),
            })
        } else {
            panic!("Parse line invalid, wrong operation {}", tipo_op)
        }
    }

    pub fn apply<T: InnerStruct>(&self, data: &mut Vec<T>) -> Result<(), Error> {
        match self.clone() {
            Self::Add {
                index,
                serialized_object,
            } => data.insert(index, Deserialize::deserialize(&serialized_object)?),
            Self::Edit {
                index,
                fieldname,
                serialized_new_value,
            } => data[index].set_str(&fieldname, serialized_new_value)?,
            Self::Delete { index } => {
                data.remove(index);
            }
        };
        Ok(())
    }

    pub fn persist<W: Write>(self, w: &mut W) -> Result<(), io::Error> {
        writeln!(w, "{}", self.to_line())
    }
}
