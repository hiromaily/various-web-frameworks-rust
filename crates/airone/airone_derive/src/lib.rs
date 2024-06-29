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
#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod gettersetter;
use gettersetter::build_getset;

mod serde;
use serde::build_serde;

/// A macro needed to generate methods to use your
/// struct with [AironeDb](https://gitlab.com/MassiminoilTrace/airone)
#[proc_macro_derive(AironeDbDerive)]
pub fn getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let crate_name = quote! {airone};

    let serde_data = build_serde(&input, crate_name.clone());
    let access_functions = build_getset(input.clone());

    quote! {
        #serde_data
        #access_functions
    }
    .into()
}
