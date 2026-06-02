use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Ident, Fields};


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