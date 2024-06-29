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
use syn::{Data, DataStruct, DeriveInput, Fields};

pub fn build_getset(input: DeriveInput) -> TokenStream {
    let struct_name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => {
            panic!("this derive macro only works on structs with named fields")
        }
    };

    // -------------------------------------------------
    // Implemento quelle sul singolo elemento T
    // e sul writeproxy

    let trait_name_single_object_getters = syn::Ident::new(
        &format!("AironeInterfaceGetter{}", struct_name),
        proc_macro2::Span::call_site(),
    );
    let trait_name_single_object_setters_fallible = syn::Ident::new(
        &format!("AironeInterfaceSettersFallible{}", struct_name),
        proc_macro2::Span::call_site(),
    );
    let trait_name_single_object_setters_infallible = syn::Ident::new(
        &format!("AironeInterfaceSettersInfallible{}", struct_name),
        proc_macro2::Span::call_site(),
    );

    let gettersetter_singleelement_trait = {
        let trait_functions_getter = fields.iter().map(|f| {
            let field_name = f.ident.as_ref().unwrap();
            let get_function_name = syn::Ident::new(
                &format!("get_{}", field_name),
                proc_macro2::Span::call_site(),
            );
            let field_type = &f.ty;
            quote! {
                fn #get_function_name(&self) -> &#field_type;
            }
        });

        let trait_functions_setter_fallible = fields.iter().map(|f|
            {
                let field_name = f.ident.as_ref().unwrap();
                let set_function_name = syn::Ident::new(
                    &format!("set_{}", field_name),
                    proc_macro2::Span::call_site()
                );
                let field_type = &f.ty;
                quote!{
                    fn #set_function_name(&mut self, value: #field_type) -> Result<(), airone::error::Error>;
                }
            }
        );
        let trait_functions_setter_infallible = fields.iter().map(|f| {
            let field_name = f.ident.as_ref().unwrap();
            let set_function_name = syn::Ident::new(
                &format!("set_{}", field_name),
                proc_macro2::Span::call_site(),
            );
            let field_type = &f.ty;
            quote! {
                fn #set_function_name(&mut self, value: #field_type);
            }
        });

        quote! {
            pub trait #impl_generics #trait_name_single_object_getters
            #ty_generics #where_clause
            {
                #(#trait_functions_getter)*
            }
            pub trait #impl_generics #trait_name_single_object_setters_fallible
            #ty_generics #where_clause
            {
                #(#trait_functions_setter_fallible)*
            }
            pub trait #impl_generics #trait_name_single_object_setters_infallible
            #ty_generics #where_clause
            {
                #(#trait_functions_setter_infallible)*
            }
        }
    };

    // Solo le get sul &T
    let getter_impl_on_t = {
        let functions = fields.clone().into_iter().map(|f| {
            let field_name = f.ident;
            let get_function_name = syn::Ident::new(
                &format!("get_{}", field_name.as_ref().unwrap()),
                proc_macro2::Span::call_site(),
            );
            let field_type = f.ty;
            quote! {
                fn #get_function_name(&self) -> &#field_type
                {
                    return &self.#field_name;
                }
            }
        });

        quote! {
            #[automatically_derived]
            impl #impl_generics #trait_name_single_object_getters for #struct_name
            #ty_generics #where_clause
            {
                #(#functions)*
            }
        }
    };

    // sia le get, sia le set sul writeproxy
    let gettersetter_impl_on_writeproxy = {
        let functions_get = fields.clone().into_iter().map(|f| {
            let field_name = f.ident;
            let get_function_name = syn::Ident::new(
                &format!("get_{}", field_name.as_ref().unwrap()),
                proc_macro2::Span::call_site(),
            );
            let field_type = f.ty;

            quote! {
                fn #get_function_name(&self) -> &#field_type
                {
                    return &self.get(stringify!(#field_name));
                }
            }
        });

        let functions_set_infallible = fields.clone().into_iter().map(|f| {
            let field_name = f.ident.unwrap();
            let set_function_name = syn::Ident::new(
                &format!("set_{}", field_name),
                proc_macro2::Span::call_site(),
            );
            let field_type = f.ty;

            quote! {
                fn #set_function_name(&mut self, new_value: #field_type)
                {
                    self.set(stringify!(#field_name), new_value);
                }
            }
        });

        let functions_set_fallible = fields.clone().into_iter().map(|f|
            {
                let field_name = f.ident.unwrap();
                let set_function_name = syn::Ident::new(
                    &format!("set_{}", field_name),
                    proc_macro2::Span::call_site()
                );
                let field_type = f.ty;

                quote!{
                    fn #set_function_name(&mut self, new_value: #field_type) -> Result<(), airone::error::Error>
                    {
                        self.set(stringify!(#field_name), new_value)
                    }
                }
            }
        );

        //

        quote! {
            #[automatically_derived]
            impl <'prox, SaveMode> #trait_name_single_object_getters for airone::database::WriteProxy<'prox, #struct_name, SaveMode>
                where
                SaveMode: airone::database::settings::save_mode::SaveModeExt
            {
                #(#functions_get)*
            }

            #[automatically_derived]
            impl <'prox> #trait_name_single_object_setters_fallible  for airone::database::WriteProxy<'prox, #struct_name, airone::database::settings::save_mode::AutoSave>
            {
                #(#functions_set_fallible)*
            }
            #[automatically_derived]
            impl <'prox> #trait_name_single_object_setters_infallible  for airone::database::WriteProxy<'prox, #struct_name, airone::database::settings::save_mode::ManualSave>
            {
                #(#functions_set_infallible)*
            }
        }
    };

    quote! {
    #gettersetter_singleelement_trait
    #getter_impl_on_t
    #gettersetter_impl_on_writeproxy

    // #high_level_ops
    }
}
