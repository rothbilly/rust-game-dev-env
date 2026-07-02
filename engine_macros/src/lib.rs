use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item,ItemStruct, Ident, Fields};


// ==========================================
//  Attribute Macro for SetGet
// ==========================================


#[proc_macro_attribute]
pub fn set_get(attr: TokenStream, item: TokenStream) -> TokenStream {

    let input_struct = parse_macro_input!(item as ItemStruct);
    let target_field = parse_macro_input!(attr as Ident);

    let struct_name = &input_struct.ident;

    let (impl_generic, ty_generic,  where_clause) = input_struct.generics.split_for_impl();


    let mut field_type = None;

    if let Fields::Named(ref fields ) = input_struct.fields{
        for field in &fields.named{
            if let Some(ref ident) = field.ident{
                if ident == &target_field{
                    field_type = Some(&field.ty);
                    break;
                }
            }
        }
    } 

    let field_type = match field_type{
        Some(ty) => ty,
        None => {
            return syn::Error::new_spanned(
                &target_field,
                format!("Field '{}' not found in struct '{}'", &target_field, struct_name)
        ).to_compile_error()
        .into()
        }

    };

    let expanded = quote! {
        // Leave base struct untouched
        #input_struct

        impl #impl_generic crate::SetGet<#field_type> for  #struct_name #ty_generic #where_clause 
        where
            #field_type: std::clone::Clone + std::default::Default
        {
            fn get(&self) -> #field_type{
                self.#target_field.clone()
            }

            fn set(&mut self, val: #field_type){
                self.#target_field = val;
            }
        }
    };

    TokenStream::from(expanded)
}

// ==========================================
//  Attribute Macro for Registering a App
// ==========================================


#[proc_macro_attribute]
pub fn register_app(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the item the macro is attached to (e.g., `struct TestGameStruct`)
    let input_item = parse_macro_input!(item as Item);
    let binding = input_item.clone();
    // Extract the identifier of the item to use in the constructor
    let item_identifier = match &binding {
        Item::Struct(item_struct) => &item_struct.ident,
        Item::Fn(item_fn) => &item_fn.sig.ident,
        _ => {
            return syn::Error::new_spanned(&input_item,
            "register_app attribute can only be applied to structs or functions"
            ).to_compile_error().into();
        }
    };

    let app_name: String = if !attr.is_empty(){
        let attr_meta = parse_macro_input!(attr as syn::LitStr);
        attr_meta.value()
        }
        else {
            item_identifier.to_string()
        };
    
    let mutated_item = match input_item {
        Item::Struct(mut item_struct) => {
            if let syn::Fields::Unit = item_struct.fields {
                item_struct.fields = syn::Fields::Named(syn::parse_quote!({}));
            }

            if let syn::Fields::Named(ref mut fields) = item_struct.fields {
                fields.named.push(syn::parse_quote! {
                    pub base: crate::core::game::Game
                });
            }
            quote! { #item_struct }
        },
        other => quote! { #other }
    };
    
    let expanded = quote!{
        // Keep the original struct/function definition intact
        #mutated_item

        impl std::ops::Deref for #item_identifier {
            type Target = crate::core::game::Game;
            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }

        impl std::ops::DerefMut for #item_identifier {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.base
            }
        }

        // Submit to inventory
        inventory::submit! {
            AppRegistration {
                name: #app_name,
                constructor: || std::rc::Rc::new(std::cell::RefCell::new(#item_identifier::default())),
            }
        }
    };

    TokenStream::from(expanded)

}



// ==========================================
//  Derive Macro for auto get the Struct Name
// ==========================================


use syn::{DeriveInput};

//warn
#[proc_macro_derive(IName)]
pub fn i_name_derive(input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let struct_name_str = struct_name.to_string();

    let expanded = quote! {
        impl #impl_generics crate::editor::inspector::IName for #struct_name #ty_generics #where_clause {
            fn get_struct_name(&self) -> String {
                #struct_name_str.to_string()
            }
        }
    };

    TokenStream::from(expanded)
}

// ==========================================
//  Derive Macro for auto implement IVisibility trait
// ==========================================


#[proc_macro_derive(IVisibility)]
pub fn ivisibility_derive(input: TokenStream) -> TokenStream {
    // Parse the representation of the struct
    let input = parse_macro_input!(input as DeriveInput);
    
    // Get the identifier (name) of the struct
    let name = input.ident;

    // Generate the implementation
    let expanded = quote! {
        impl crate::editor::inspector::IVisibility for #name {
            fn is_visible(&self) -> bool {
                self.is_visible
            }

            fn set_visible(&mut self, value: bool) {
                self.is_visible = value;
            }
        }
    };


    TokenStream::from(expanded)
}