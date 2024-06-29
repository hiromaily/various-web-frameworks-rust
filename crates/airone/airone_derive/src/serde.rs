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

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;
use syn::{punctuated::Punctuated, token::Comma, Data, DataStruct, DeriveInput, Field, Fields};

pub fn build_serde(input: &DeriveInput, _cur_crate: TokenStream) -> TokenStream {
    let struct_name = &input.ident;
    let (_impl_generics, _ty_generics, _where_clause) = input.generics.split_for_impl();

    let fields = match input.data.clone() {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => {
            panic!("this derive macro only works on structs with named fields")
        }
    };

    // #[automatically_derived]
    //     impl #impl_generics AironeDbConstructor<#struct_name>
    //         for #cur_crate::database::AironeDb<#struct_name> #ty_generics #where_clause

    let impl_inner_struct = impl_innerstruct_trait(struct_name, &fields);
    let impl_from_into = impl_serialize_deserialize(struct_name, &fields);
    quote! {
        #[automatically_derived]
        impl airone::serde::InnerStruct for #struct_name
        {
            #impl_inner_struct
        }
        #impl_from_into
    }
}

fn impl_innerstruct_trait(struct_name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream {
    let column_names_str = fields.iter().map(|f| f.ident.as_ref().unwrap().to_string());
    let struct_name_str = struct_name.to_string();

    let match_for_get = fields.iter().map(|field| {
        let field_name = &field.ident.as_ref().unwrap();
        let field_str = field_name.to_string();
        let field_type = &field.ty;
        quote! {
            #field_str => {
                let v = &self.#field_name as &dyn Any;
                return v.downcast_ref::<V>().expect(
                    &format!(
                        "Can't convert provided generic type for field `{}` to `{}`",
                        stringify!(#field_name),
                        stringify!(#field_type)
                    )
                );
            }
        }
    });

    let match_for_set = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_str = field_name.as_ref().unwrap().to_string();
        let field_type: &syn::Type = &field.ty;
        // String type must be handled separately
        quote! {
            #field_str => {
                self.#field_name = (
                    *(&value as &dyn Any).downcast_ref::<#field_type>()
                        .expect(
                            &format!(
                                "Can't convert provided generic type for field `{}` to `{}`",
                                stringify!(#field_name),
                                stringify!(#field_type)
                            )
                        )
                    ).clone()
            }
        }
    });

    let match_for_setstr = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_str = field_name.as_ref().unwrap().to_string();
        quote! {
            #field_str => {
                self.#field_name = SerializableField::deserialize_field(value)?;
            }
        }
    });

    let setting_unknown_field_panic_handler = quote! {
        panic!(
            "Error trying to set field `{}`, but it doesn't exist in type \"{}\". \nAvailable fields are:\n{}",
            key,
            stringify!(#struct_name),
            Self::COLUMNS.iter().map(|c|
            {
                format!(" -`{}`", c)
            }).collect::<Vec<_>>().join("\n")
        )
    };

    quote! {
        const COLUMNS: &'static [&'static str] = &[
            #(#column_names_str),*
        ];
        const STRUCT_NAME: &'static str = #struct_name_str;

        // fn get<V: SerializableField>(&self, key: &str) -> &V
        // {
        //     use std::any::Any;
        //     match key
        //     {
        //         "a"=>{let v = &self.a as &dyn Any;
        //             return v.downcast_ref::<V>().unwrap();},
        //         "b"=>{let v = &self.a as &dyn Any;
        //             return v.downcast_ref::<V>().unwrap();},
        //         _ => panic!("")
        //     }
        // }
        fn get<V: airone::serde::SerializableField>(&self, key: &str) -> &V {
            use std::any::Any;
            match key
            {
                #(#match_for_get),*
                _ => {
                    panic!(
                        "Error trying to get field `{}`, but it doesn't exist in type \"{}\". \nAvailable fields are:\n{}",
                        key,
                        stringify!(#struct_name),
                        Self::COLUMNS.iter().map(|c|
                        {
                            format!(" -`{}`", c)
                        }).collect::<Vec<_>>().join("\n")
                    )
                }
            }
        }

        // fn set<V: SerializableField>(&mut self, key: &str, value: V)
        // {
        //     use std::any::Any;
        //     match key
        //     {
        //         "a" => {
        //             self.a = (*(&value as &dyn Any).downcast_ref::<i32>().unwrap()).clone()
        //         },
        //         "b" => {
        //             self.b = (*(&value as &dyn Any).downcast_ref::<String>().unwrap()).clone()
        //         },
        //         _ => {unreachable!()}
        //     }
        // }
        fn set<V: airone::serde::SerializableField>(&mut self, key: &str, value: V)
        {
            use std::any::Any;
            match key
            {
                #(#match_for_set),*
                _ => {#setting_unknown_field_panic_handler}
            }
        }


        // fn set_str(&mut self, key: &str, value: SerializedFieldValue) -> Result<(), Error>
        // {
        //     match key
        //     {
        //         "a" => {
        //             self.a = value.clone().try_into()?;
        //         },
        //         "b" => {
        //             self.b = value.clone().try_into()?;
        //         },
        //         _ => {unreachable!()}
        //     }
        //     Ok(())
        // }
        fn set_str(&mut self, key: &str, value: airone::serde::SerializedFieldValue) -> Result<(), airone::error::Error>
        {
            use airone::serde::SerializableField;
            match key
            {
                #(#match_for_setstr),*
                _ => {#setting_unknown_field_panic_handler}
            }
            Ok(())
        }
    }
}

fn impl_serialize_deserialize(
    struct_name: &Ident,
    fields: &Punctuated<Field, Comma>,
) -> TokenStream {
    let iter_deserialize = fields.iter().map(|f| {
        let fieldname = f.ident.as_ref().unwrap();
        quote! {
            #fieldname: airone::serde::SerializableField::deserialize_field(values.next().unwrap())?
        }
    });

    let iter_serialize = fields.iter().map(|f| {
        let fieldname = f.ident.as_ref().unwrap();
        quote! {
            self.#fieldname.serialize_field()
        }
    });

    quote! {
        // impl Deserialize for Abc
        // {
        //     fn deserialize(value: &SerializedStruct) -> Result<Self, Error> {
        //         let mut values = value.get_values().iter().cloned();
        //         Ok(
        //             Self {
        //              a: SerializableField::deserialize_field(values.next().unwrap())?,
        //              b: SerializableField::deserialize_field(values.next().unwrap())? }
        //         )
        //     }
        // }
        #[automatically_derived]
        impl airone::serde::Deserialize for #struct_name
        {
            fn deserialize(value: &airone::serde::SerializedStruct) -> Result<Self, airone::error::Error>
            {
                use airone::serde::InnerStruct;
                let values = value.get_values();
                assert!(values.len()==Self::COLUMNS.len());
                let mut values = values.iter().cloned();
                Ok(
                    Self {
                        #(#iter_deserialize),*
                    }
                )
            }
        }


        // impl Serialize for Abc
        // {
        //     fn serialize(&self) -> SerializedStruct {
        //         SerializedStruct::new(
        //             vec![
                    //     self.a.serialize_field(),
                    //     self.b.serialize_field()
                    // ]
        //         )
        //     }
        // }
        #[automatically_derived]
        impl airone::serde::Serialize for #struct_name
        {
            fn serialize(&self) -> airone::serde::SerializedStruct
            {
                use airone::serde::SerializableField;
                airone::serde::SerializedStruct::new(
                    vec![
                        #(#iter_serialize),*
                    ]
                )
            }
        }
    }
}
