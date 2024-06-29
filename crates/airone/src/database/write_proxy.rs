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

use crate::error::Error;

use super::settings;
use super::settings::save_mode::*;
use super::AironeDb;
use super::InnerStruct;
use super::SerializableField;

// WriteProxy
/// A write proxy
/// to ensure that calls to `set`
/// will properly persist changes
pub struct WriteProxy<'a, T, SaveMode>
where
    T: InnerStruct,
    SaveMode: settings::save_mode::SaveModeExt,
{
    index: usize,
    db: &'a mut AironeDb<T, SaveMode>,
}
impl<'a, T, SaveMode> WriteProxy<'a, T, SaveMode>
where
    T: InnerStruct,
    SaveMode: settings::save_mode::SaveModeExt,
{
    pub(super) fn new(db: &'a mut AironeDb<T, SaveMode>, index: usize) -> Self {
        WriteProxy { db, index }
    }

    #[doc(hidden)]
    pub fn get<V: SerializableField>(&self, fieldname: &'static str) -> &V {
        // use crate::serde::InnerStruct;
        self.db[self.index].get(fieldname)
    }
}

impl<'a, T> WriteProxy<'a, T, AutoSave>
where
    T: InnerStruct,
{
    #[doc(hidden)]
    pub fn set<V: SerializableField>(
        &mut self,
        fieldname: &'static str,
        value: V,
    ) -> Result<(), Error> {
        self.db.set(self.index, fieldname, value)?;
        Ok(())
    }
}

impl<'a, T> WriteProxy<'a, T, ManualSave>
where
    T: InnerStruct,
{
    #[doc(hidden)]
    pub fn set<V: SerializableField>(&mut self, fieldname: &'static str, value: V) {
        self.db.set(self.index, fieldname, value);
    }
}
