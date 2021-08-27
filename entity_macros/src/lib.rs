extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

// https://doc.rust-lang.org/book/ch19-06-macros.html#procedural-macros-for-generating-code-from-attributes
#[proc_macro_derive(Physics)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_entity_macro(&ast)
}

fn impl_entity_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl crate::entities::world::HasWorldInfo for #name {
            fn get_world_info(&self) -> &WorldInfo{
                return &self.world;
            }
            // add_getter!(get_world_info, world, WorldInfo);
            fn get_mut_world(&mut self) -> &mut WorldInfo{
                return &mut self.world;
            }
        }
    };
    gen.into()
}

// https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros
// #[proc_macro_attribute]
// pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
//     println!("attr: \"{}\"", attr.to_string());
//     println!("item: \"{}\"", item.to_string());
//     item
// }

// https://users.rust-lang.org/t/solved-derive-and-proc-macro-add-field-to-an-existing-struct/52307/5
// #[proc_macro_attribute]
// pub fn add_physics(args: TokenStream, input: TokenStream) -> TokenStream {
//     let mut item_struct = parse_macro_input!(input as ItemStruct);
//     let _ = parse_macro_input!(args as parse::Nothing);

//     if let syn::Fields::Named(ref mut fields) = item_struct.fields {
//         fields.named.push(
//             syn::Field::parse_named
//                 .parse2(quote! { pub __world: WorldInfo })
//                 .unwrap(),
//         );
//     }

//     return quote! {
//         #item_struct
//     }
//     .into();
// }

// #[proc_macro_attribute]
// pub fn add_field(args: TokenStream, input: TokenStream) -> TokenStream {
//     let mut item_struct = parse_macro_input!(input as ItemStruct);
//     let _ = parse_macro_input!(args as parse::Nothing);

//     if let syn::Fields::Named(ref mut fields) = item_struct.fields {
//         fields.named.push(
//             syn::Field::parse_named
//                 .parse2(quote! { pub a: String })
//                 .unwrap(),
//         );
//     }

//     return quote! {
//         #item_struct
//     }
//     .into();
// }