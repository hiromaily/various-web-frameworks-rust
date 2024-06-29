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

pub mod save_mode {
    /// A trait that represents all available save modes
    pub trait SaveModeExt {}

    /// Always save and flush to file after each mutable operation
    pub struct AutoSave;
    impl SaveModeExt for AutoSave {}

    /// Manually save
    ///
    /// Changes won't be automatically saved, which is useful to undo an operation in memory.
    /// When you manually save, data is persisted and flushed to file.
    pub struct ManualSave;
    impl SaveModeExt for ManualSave {}
}

#[derive(PartialEq, Debug)]
pub enum BufferMode {
    /// When saving, uses a buffered reader to optimize writing speed
    ///
    /// This is useful for bulk modifications.
    Buffered,
    /// When saving, always flushes the write buffer to disk
    ///
    /// This is useful to ensure small saves have been written to disk.
    /// Good for small changes, but degrades performance when
    /// writing a lot of data modifications in a very short time
    AlwaysFlush,
}
