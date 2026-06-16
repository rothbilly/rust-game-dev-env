use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item,ItemStruct, Ident, Fields};


// SetGetter Attribute macro
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


#[proc_macro_attribute]
pub fn register_app(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the item the macro is attached to (e.g., `struct TestGameStruct`)
    let input_item = parse_macro_input!(item as Item);

    // Extract the identifier of the item to use in the constructor
    let item_identifier = match &input_item {
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
    
    let expanded = quote!{
        // Keep the original struct/function definition intact
        #input_item

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