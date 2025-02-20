extern crate proc_macro;

// コンパイラのapi
use proc_macro::TokenStream;

// synのデータ構造を取り、Rustコードに変換
use quote::quote;

// 文字列からRustコードを構文解析し、処理を行えるデータ構造にする
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    // トレイトの実装内容を構築
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
