use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Error, Fields, Ident, Item, ItemFn, ItemStruct, LitInt, Member, parse_macro_input, Result};


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
    let og_input = parse_macro_input!(item as ItemStruct);
    // Extract the identifier of the item to use in the constructor
    let item_identifier =  &og_input.ident;
        //Item::Fn(item_fn) => &item_fn.sig.ident

    //get the name from the attribute or from the struct itself
    let app_name: String = if !attr.is_empty(){
        let attr_meta = parse_macro_input!(attr as syn::LitStr);
        attr_meta.value()
        }
        else {
            item_identifier.to_string()
        };
        
    
    //generate 
    let expanded = quote!{
        // Keep the original struct/function definition intact
        #og_input
        
        // Submit to inventory
        inventory::submit! {
            AppRegistration {
                name: #app_name,
                constructor: ||{
                    println!("{} register: ",  stringify!(#app_name));
                    std::rc::Rc::new(std::cell::RefCell::new(#item_identifier::default()))
                } 
            }
        }
    };

    TokenStream::from(expanded)

}

// ==========================================
//  Attribute Macro for auto implement deref coercion
// ==========================================


#[proc_macro_attribute]
pub fn auto_deref(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    match expand(attr.into(), input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn expand(attr: proc_macro2::TokenStream, input: ItemStruct) -> Result<proc_macro2::TokenStream> {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Figure out which field to target, and its type.
    let (member, field_ty) = resolve_target_field(&attr, &input)?;

    let expanded = quote! {
        #input

        impl #impl_generics ::std::ops::Deref for #struct_name #ty_generics #where_clause {
            type Target = #field_ty;

            fn deref(&self) -> &Self::Target {
                &self.#member
            }
        }

        impl #impl_generics ::std::ops::DerefMut for #struct_name #ty_generics #where_clause {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.#member
            }
        }
    };

    Ok(expanded)
}

/// Determine the target field:
/// - `#[deref(field_name)]` picks a named field.
/// - `#[deref(0)]` picks a tuple-struct field by index.
/// - `#[deref]` (no args) picks the field itself if there's exactly one.
fn resolve_target_field(
    attr: &proc_macro2::TokenStream,
    input: &ItemStruct,
) -> Result<(Member, syn::Type)> {
    let span = Span::call_site();

    if attr.is_empty() {
        // No argument given: only valid if the struct has exactly one field.
        return match &input.fields {
            Fields::Named(fields) if fields.named.len() == 1 => {
                let f = fields.named.first().unwrap();
                let ident = f.ident.clone().unwrap();
                Ok((Member::Named(ident), f.ty.clone()))
            }
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let f = fields.unnamed.first().unwrap();
                Ok((Member::Unnamed(syn::Index::from(0)), f.ty.clone()))
            }
            _ => Err(Error::new(
                span,
                "#[deref] requires a field name/index when the struct has more than one field, \
                 e.g. #[deref(inner)] or #[deref(0)]",
            )),
        };
    }

    // Try parsing the attribute argument as an identifier (named field) first.
    if let Ok(ident) = syn::parse2::<Ident>(attr.clone()) {
        return find_named_field(input, &ident).ok_or_else(|| {
            Error::new_spanned(&ident, format!("no field named `{ident}` found on this struct"))
        });
    }

    // Otherwise try as an integer literal (tuple struct index).
    if let Ok(lit) = syn::parse2::<LitInt>(attr.clone()) {
        let index: u32 = lit.base10_parse()?;
        return find_indexed_field(input, index)
            .ok_or_else(|| Error::new_spanned(&lit, format!("no field at index {index}")));
    }

    Err(Error::new_spanned(
        attr,
        "expected a field name or index, e.g. #[deref(inner)] or #[deref(0)]",
    ))
}

fn find_named_field(input: &ItemStruct, ident: &Ident) -> Option<(Member, syn::Type)> {
    if let Fields::Named(fields) = &input.fields {
        for f in &fields.named {
            if f.ident.as_ref() == Some(ident) {
                return Some((Member::Named(ident.clone()), f.ty.clone()));
            }
        }
    }
    None
}

fn find_indexed_field(input: &ItemStruct, index: u32) -> Option<(Member, syn::Type)> {
    if let Fields::Unnamed(fields) = &input.fields {
        let f = fields.unnamed.iter().nth(index as usize)?;
        return Some((Member::Unnamed(syn::Index::from(index as usize)), f.ty.clone()));
    }
    None
}


// ==========================================
//  Derive Macro for auto get the Struct Name
// ==========================================


use syn::{DeriveInput};

//warn
#[proc_macro_derive(IName)]
pub fn i_name_derive(input: TokenStream) -> TokenStream {

    //parse the content of the struct to be derived
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


