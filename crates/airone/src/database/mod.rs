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

use std::io::Write;
use std::ops::{Bound, Index, RangeBounds};
use std::{fs::File, io::BufWriter, marker::PhantomData};

use crate::error::Error;
pub mod settings;

use crate::serde::{InnerStruct, SerializableField, SerializedFieldValue, SerializedStruct};

mod operations;
mod write_proxy;
use settings::save_mode::{AutoSave, SaveModeExt};
pub use write_proxy::WriteProxy;

use self::settings::save_mode::ManualSave;
use self::settings::BufferMode;
mod loading;

pub struct AironeDb<T, SaveMode = AutoSave>
where
    T: InnerStruct,
    SaveMode: SaveModeExt,
{
    buf_writer: BufWriter<File>,
    elements: Vec<T>,
    pending_changes: Vec<operations::RevertableChange>,
    mode: PhantomData<SaveMode>,
    write_mode: BufferMode,
}
// Common public methods
impl<T: InnerStruct, SaveMode: SaveModeExt> AironeDb<T, SaveMode> {
    pub fn new() -> Result<Self, Error> {
        Self::new_with_custom_name(T::STRUCT_NAME)
    }
    pub fn new_with_custom_name(custom_name: &str) -> Result<Self, Error> {
        let (filewriter, data) = loading::full_load(custom_name)?;
        Ok(Self {
            buf_writer: filewriter,
            elements: data,
            pending_changes: Vec::new(),
            mode: PhantomData,
            write_mode: BufferMode::Buffered,
        })
    }

    pub fn get_all(&self) -> &[T] {
        &self.elements
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.elements.get(index)
    }

    /// Mutably access the element
    /// at the specified index
    pub fn get_mut(&mut self, index: usize) -> Option<WriteProxy<T, SaveMode>> {
        if index < self.len() {
            Some(WriteProxy::new(self, index))
        } else {
            None
        }
    }

    /// Returns the number of elements in the list
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Checks if the inner array is empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn set_buffer_mode(&mut self, mode: BufferMode) -> Result<(), Error> {
        if self.write_mode != mode {
            self.write_mode = mode;
            if self.write_mode == BufferMode::AlwaysFlush {
                self.buf_writer.flush()?;
            }
        }
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.elements.iter()
    }

    /// Change the SaveMode to the provided value
    ///
    /// For example, you can convert a db to [ManualSave] or [AutoSave] mode.
    /// While changing mode, any pending change is written to disk and you won't be able to rollback.
    pub fn set_save_mode<NewSaveMode: SaveModeExt>(
        mut self,
    ) -> Result<AironeDb<T, NewSaveMode>, Error> {
        self._save()?;

        Ok(AironeDb::<T, NewSaveMode> {
            buf_writer: self.buf_writer,
            elements: self.elements,
            pending_changes: self.pending_changes,
            mode: PhantomData,
            write_mode: self.write_mode,
        })
    }
}

// Common private methods:
// insert()
// push()
// pop()
// extend()
// append()
// dedup()
// remove()
// drain()
// clear()
// _set_field()
// _save()
impl<T: InnerStruct, SaveMode: SaveModeExt> AironeDb<T, SaveMode> {
    fn _insert(&mut self, index: usize, element: T) {
        let change = operations::RevertableChange::new_add(index, element.serialize());
        change.apply_forward(&mut self.elements).unwrap();
        self.pending_changes.push(change);
    }
    /// Adds a new element to the end of the list
    fn _push(&mut self, element: T) {
        self._insert(self.len(), element)
    }
    fn _pop(&mut self) -> Option<T> {
        if !self.elements.is_empty() {
            Some(self._remove(self.elements.len() - 1))
        } else {
            None
        }
    }
    fn _extend(&mut self, iter: impl Iterator<Item = T>) {
        for el in iter {
            self._push(el);
        }
    }
    fn _append(&mut self, other: &mut Vec<T>) {
        for el in other.drain(..) {
            self._push(el);
        }
    }

    fn _dedup(&mut self)
    where
        T: PartialEq,
    {
        let mut i = 0;
        while self.len() > 1 && i < self.len() - 1 {
            if self.elements[i] == self.elements[i + 1] {
                self._remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    fn _remove(&mut self, index: usize) -> T {
        let change =
            operations::RevertableChange::new_delete(index, self.elements[index].serialize());
        let element = self.elements.remove(index);
        self.pending_changes.push(change);
        element
    }

    fn _drain(&mut self, range: impl RangeBounds<usize>) -> impl Iterator<Item = T> {
        let start: usize = match range.start_bound() {
            Bound::Excluded(e) => e + 1,
            Bound::Included(e) => *e,
            Bound::Unbounded => 0,
        };
        let end: usize = match range.end_bound() {
            Bound::Excluded(e) => e - 1,
            Bound::Included(e) => *e,
            Bound::Unbounded => self.len() - 1,
        };
        let mut res = Vec::new();
        for i in (start..=end).rev() {
            res.push(self._remove(i));
        }
        res.into_iter()
    }

    fn _retain<F: Fn(&T) -> bool>(&mut self, f: F) {
        let mut indices_to_remove: Vec<usize> = self
            .elements
            .iter()
            .enumerate()
            .filter(|(_, v)| !f(v))
            .map(|(i, _)| i)
            .collect();

        indices_to_remove.sort();

        for i in indices_to_remove.iter().rev() {
            self._remove(*i);
        }
    }

    fn _clear(&mut self) {
        for i in (0..self.len()).rev() {
            self._remove(i);
        }
    }

    fn _set<V: SerializableField>(&mut self, index: usize, fieldname: &'static str, value: V) {
        let change = operations::RevertableChange::new_edit(
            index,
            fieldname,
            self.elements[index].get::<V>(fieldname).serialize_field(),
            value.serialize_field(),
        );
        change.apply_forward(&mut self.elements).unwrap();
        self.pending_changes.push(change);
    }

    fn _save(&mut self) -> Result<(), Error> {
        for change in self.pending_changes.drain(..) {
            change.persist(&mut self.buf_writer)?;
        }
        if self.write_mode == BufferMode::AlwaysFlush {
            self.buf_writer.flush()?;
        }
        Ok(())
    }
}

// COMMON public mutable methods
// insert()
// push()
// extend()
// append()
// dedup()
// remove()
// drain()
// retain()
// clear()

// AutoSave only public methods:
// to_manualsave_mode()

// ManualSave only public methods:
// to_autosave_mode()
// save()
// rollback()

// Autosave methods
impl<T: InnerStruct> AironeDb<T, AutoSave> {
    /// Inserts an element at position `index` within the vector, shifting all
    /// elements after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    pub fn insert(&mut self, index: usize, element: T) -> Result<(), Error> {
        self._insert(index, element);
        self._save()?;
        Ok(())
    }

    /// Adds a new element to the end of the list
    pub fn push(&mut self, element: T) -> Result<(), Error> {
        self._push(element);
        self._save()?;
        Ok(())
    }

    /// Removes the last element of the list and returns it if existing
    pub fn pop(&mut self) -> Result<Option<T>, Error> {
        let el = self._pop();
        self._save()?;
        Ok(el)
    }

    /// Adds the elements of an iterator to the end of the list.
    pub fn extend(&mut self, iter: impl Iterator<Item = T>) -> Result<(), Error> {
        self._extend(iter);
        self._save()?;
        Ok(())
    }

    /// Adds multiple elements to the end of the list,
    /// automatically reducing the number of syscalls to write
    /// them to disk
    ///
    /// Works similar to
    /// <https://doc.rust-lang.org/std/vec/struct.Vec.html#method.append>
    pub fn append(&mut self, other: &mut Vec<T>) -> Result<(), Error> {
        self._append(other);
        self._save()?;
        Ok(())
    }

    /// Removes consecutive duplicates
    pub fn dedup(&mut self) -> Result<(), Error>
    where
        T: PartialEq,
    {
        self._dedup();
        self._save()?;
        Ok(())
    }

    /// Removes and returns the element at `index`. Panics if out of bounds.
    pub fn remove(&mut self, index: usize) -> Result<T, Error> {
        let el = self._remove(index);
        self._save()?;
        Ok(el)
    }

    /// Removes the specified range in bulk and returns an iterator
    /// over the removed elements
    pub fn drain(
        &mut self,
        range: impl RangeBounds<usize>,
    ) -> Result<impl Iterator<Item = T>, Error> {
        let res = self._drain(range);
        self._save()?;
        Ok(res)
    }

    /// Keeps only the elements that match the predicate
    ///
    /// Removes all the elements that do not satisfy the condition
    pub fn retain<F: Fn(&T) -> bool>(&mut self, f: F) -> Result<(), Error> {
        self._retain(f);
        self._save()?;
        Ok(())
    }

    /// Deletes all elements from the list
    pub fn clear(&mut self) -> Result<(), Error> {
        self._clear();
        self._save()?;
        Ok(())
    }
    pub fn set<V: SerializableField>(
        &mut self,
        index: usize,
        fieldname: &'static str,
        value: V,
    ) -> Result<(), Error> {
        self._set(index, fieldname, value);
        self._save()?;
        Ok(())
    }
}

impl<T: InnerStruct> AironeDb<T, ManualSave> {
    /// Inserts an element at position `index` within the vector, shifting all
    /// elements after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    pub fn insert(&mut self, index: usize, element: T) {
        self._insert(index, element);
    }

    /// Adds a new element to the end of the list
    pub fn push(&mut self, element: T) {
        self._push(element);
    }

    /// Removes the last element of the list and returns it if existing
    pub fn pop(&mut self) -> Option<T> {
        self._pop()
    }

    /// Adds the elements of an iterator to the end of the list.
    pub fn extend(&mut self, iter: impl Iterator<Item = T>) {
        self._extend(iter);
    }

    /// Adds multiple elements to the end of the list,
    /// automatically reducing the number of syscalls to write
    /// them to disk
    ///
    /// Works similar to
    /// <https://doc.rust-lang.org/std/vec/struct.Vec.html#method.append>
    pub fn append(&mut self, other: &mut Vec<T>) {
        self._append(other);
    }

    /// Removes consecutive duplicates
    pub fn dedup(&mut self)
    where
        T: PartialEq,
    {
        self._dedup();
    }

    /// Removes and returns the element at `index`. Panics if out of bounds.
    pub fn remove(&mut self, index: usize) -> T {
        self._remove(index)
    }

    /// Removes the specified range in bulk and returns an iterator
    /// over the removed elements
    pub fn drain(&mut self, range: impl RangeBounds<usize>) -> impl Iterator<Item = T> {
        self._drain(range)
    }

    /// Keeps only the elements that match the predicate
    ///
    /// Removes all the elements that do not satisfy the condition
    pub fn retain<F: Fn(&T) -> bool>(&mut self, f: F) {
        self._retain(f);
    }

    /// Deletes all elements from the list
    pub fn clear(&mut self) {
        self._clear();
    }
    pub fn set<V: SerializableField>(&mut self, index: usize, fieldname: &'static str, value: V) {
        self._set(index, fieldname, value);
    }

    /// Saves to file all in-memory transactions, to persist the current db state.
    pub fn save(&mut self) -> Result<(), Error> {
        self._save()
    }

    /// Rollbacks the data in memory to the last
    /// known state which was persisted to disk
    pub fn rollback(&mut self) {
        for change in self.pending_changes.drain(..).rev() {
            change.apply_backward(&mut self.elements).expect(
                "We're deserializing data that was just serialized in-memory only,
                this should not crash",
            );
        }
    }
}

/// External traits
impl<T, SaveMode> Index<usize> for AironeDb<T, SaveMode>
where
    T: InnerStruct,
    SaveMode: SaveModeExt,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}
