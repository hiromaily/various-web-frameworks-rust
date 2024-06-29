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

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use super::operations;
use super::InnerStruct;
use crate::error::Error;
use crate::serde::Deserialize;
use crate::serde::SerializedStruct;

/// Returns the changes file and the collected elements
pub fn full_load<T: InnerStruct>(filename: &str) -> Result<(BufWriter<File>, Vec<T>), Error> {
    os_check();

    let basefile = PathBuf::from(&format!("{}{}", filename, ".csv"));
    let changes_file = PathBuf::from(&format!("{}{}", filename, ".changes.csv"));

    let mut elements: Vec<T> = Vec::new();

    load_base_file(&mut elements, &basefile)?;
    load_changes_file(&mut elements, &changes_file)?;
    dump_compacted_file(&elements, &basefile)?;

    // Delete changes file
    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(changes_file)
        .unwrap();
    let filewriter = BufWriter::new(file);

    Ok((filewriter, elements))
}

fn load_base_file<T: InnerStruct>(elements: &mut Vec<T>, basefile: &Path) -> Result<(), Error> {
    if let Ok(file) = std::fs::OpenOptions::new().read(true).open(basefile) {
        let buf = BufReader::new(file);
        let mut lines = buf.lines();
        {
            let header_line = lines.next().unwrap().unwrap();
            check_field_consistency(&header_line, T::COLUMNS);
        }
        for line in lines {
            let ser = SerializedStruct::from_escaped_line(&line?);
            elements.push(Deserialize::deserialize(&ser)?);
        }
    }
    Ok(())
}

/// Apply modifications contained
/// in the transaction log file
fn load_changes_file<T: InnerStruct>(
    elements: &mut Vec<T>,
    changes_file: &Path,
) -> Result<(), Error> {
    let file = match std::fs::OpenOptions::new().read(true).open(changes_file) {
        Ok(e) => e,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            File::create(changes_file)?;
            return Ok(());
        }
        Err(e) => return Err(e.into()),
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let op = operations::Operation::from_line(&line?)?;
        op.apply(elements)?;
    }
    Ok(())
}

fn dump_compacted_file<T: InnerStruct>(elements: &[T], basefile: &Path) -> Result<(), Error> {
    let f = File::create(basefile)?;
    let mut writer = BufWriter::new(f);

    writeln!(writer, "{}", T::COLUMNS.join("\t")).unwrap();

    for element in elements.iter() {
        let serialized_object: SerializedStruct = element.serialize();
        writeln!(writer, "{}", serialized_object.to_escaped_line())?;
    }
    writer.flush()?;
    Ok(())
}

/// Maps struct_field → column order in the file
fn check_field_consistency(header_line: &str, struct_fields: &[&str]) {
    let detected_columns: Vec<&str> = header_line.split('\t').collect();
    assert_eq!(
        detected_columns.len(),
        struct_fields.len(),
        "Column mismatch. Did you add or delete some fields? "
    );
    for i in 0..detected_columns.len() {
        assert_eq!(
            struct_fields[i],
            detected_columns[i],
            "Column mismatch. Did you reorder or change some field name? Collapse the CSV, fix column or field ordering and naming, then re-run"
        );
    }
}

fn os_check() {
    if cfg!(target_os = "windows") {
        eprintln!("You're running a Windows operating system, but Airone is not supported on, has never been tested on and does not endorse proprietary operating systems.\n\
            I encourage you to switch to an actually tested and privacy-and-freedom respecting operating system such as Gnu/Linux.\n\
            Head to https://www.fsf.org/windows for more information.");
    } else if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
        eprintln!("You're running an Apple operating system, but Airone is not supported on, has never been tested on and does not endorse proprietary operating systems.\n\
            I encourage you to switch to an actually tested and privacy-and-freedom respecting operating system such as Gnu/Linux.\n\
            Learn more at https://www.fsf.org/campaigns/apple");
    }
    {
        // WSL
        let output = std::process::Command::new("uname")
            .arg("-a")
            .output()
            .expect("Failed to execute command");
        let st = String::from_utf8(output.stdout).unwrap();
        if st.to_ascii_lowercase().contains("microsoft") {
            eprintln!("You're running airone inside of WSL, which is a compatibility layer and may unexpectedly break or introduce bugs. Airone has never been tested on WSL. \n\
                Anyway, to get the best from this library I highly encourage you to switch to a traditionally installed operating system such as a full Gnu/Linux installation instead.
                ");
        }
    }
}
