extern crate proc_macro;

use proc_macro::TokenStream;

use all_args_constructor::all_args_constructor;
use builder::builder;
use getter::getter;
use getter_mut::getter_mut;
use no_args_constructor::no_args_constructor;
use setter::setter;

mod getter;
mod getter_mut;
mod setter;
mod no_args_constructor;
mod all_args_constructor;
mod builder;

mod utils;

#[proc_macro_derive(Getter)]
pub fn derive_getter(input: TokenStream) -> TokenStream {
    getter(input)
}

#[proc_macro_derive(GetterMut)]
pub fn derive_getter_mut(input: TokenStream) -> TokenStream {
    getter_mut(input)
}

#[proc_macro_derive(Setter)]
pub fn derive_setter(input: TokenStream) -> TokenStream {
    setter(input)
}

#[proc_macro_derive(AllArgsConstructor)]
pub fn derive_no_args_constructor(input: TokenStream) -> TokenStream {
    no_args_constructor(input)
}

#[proc_macro_derive(NoArgsConstructor)]
pub fn derive_all_args_constructor(input: TokenStream) -> TokenStream {
    all_args_constructor(input)
}

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    builder(input)
}