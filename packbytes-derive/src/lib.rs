//! Derive macros for the `packbytes` crate.
#![warn(missing_docs)]
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Fields, Ident, Item, ItemEnum, ItemStruct, LitInt, Meta, Type};

type UnitFields = Punctuated<syn::Field, Comma>;

/// Derive the `FromBytes` trait for structs where each field implements it.
///
/// Const generics in stable don't allow implementing `FromBytes` for arrays `[T; N]` where
/// `T: FromBytes`. This macro circumvents that by deriving a different implementation for fields
/// whose types are arrays, allowing the trait to be derived even for structs with such fields.
///
/// # Endianness
/// By default, the `FromBytes` and `ToBytes` derive macros assume that the data is prefered to be stored
/// in the little endian order.
/// You can change this by setting the attribute `#[packbytes(be)]` for big endian or `#[packbytes(ne)]`
/// for the platform native endian.
#[proc_macro_derive(FromBytes, attributes(packbytes))]
pub fn frombytes_derive(input: TokenStream) -> TokenStream {
    let item: ItemStruct =
        syn::parse(input).expect("#[derive(FromBytes)] can be only applied to structs");

    let name = &item.ident;
    let generics = &item.generics;
    let unit_fields = UnitFields::new();

    let mut prefers_le = quote!(true);
    for attr in item.attrs.iter() {
        if let Meta::List(ref list) = attr.meta {
            if list.path.is_ident("packbytes") {
                get_endianness(&list.tokens, &mut prefers_le);
            }
        }
    }

    let fields = match item.fields {
        Fields::Named(fields) => fields.named.into_iter(),
        Fields::Unnamed(fields) => fields.unnamed.into_iter(),
        _ => unit_fields.into_iter(),
    };

    let fields = fields.enumerate().map(|(i, field)| {
        let name = field.ident.map_or_else(
            || {
                let i = i.to_string();
                let lit = LitInt::new(&i, Span::call_site());
                quote!(#lit)
            },
            |n| quote!(#n),
        );
        (name, field.ty)
    });

    let field_sizes = fields.clone().map(|(_, ty)| {
        if let Type::Array(arr) = ty {
            let len = arr.len;
            let aty = arr.elem;
            quote! { (#len) * <<#aty as ::packbytes::FromBytes>::Bytes as ::packbytes::ByteArray>::SIZE }
        } else {
            quote! { <<#ty as ::packbytes::FromBytes>::Bytes as ::packbytes::ByteArray>::SIZE }
        }
    });

    let from_fields = |method| {
        fields.clone().map(move |(name, ty)| {
        if let Type::Array(arr) = ty {
            let len = arr.len;
            let aty = arr.elem;
            quote! {
                #name: {
                    let size = <<#aty as ::packbytes::FromBytes>::Bytes as ::packbytes::ByteArray>::SIZE;
                    let val = ::core::array::from_fn(|j| {
                        <#aty as ::packbytes::FromBytes>::#method(bytes[i+j*size..i+(j+1)*size].try_into().unwrap())
                    });
                    i += (#len) * size;
                    val
                }
            }
        } else {
            quote! {
                #name: {
                    let size = <<#ty as ::packbytes::FromBytes>::Bytes as ::packbytes::ByteArray>::SIZE;
                    let val = <#ty as ::packbytes::FromBytes>::#method(bytes[i..i+size].try_into().unwrap());
                    i += size;
                    val
                }
            }
        }
    })
    };
    let from_le_fields = from_fields(quote!(from_le_bytes));
    let from_be_fields = from_fields(quote!(from_be_bytes));

    let tokens = quote! {
        impl #generics ::packbytes::FromBytes for #name #generics {
            type Bytes = [u8; #( #field_sizes + )* 0];

            const PREFERS_LE: bool = #prefers_le;

            #[inline]
            fn from_le_bytes(bytes: Self::Bytes) -> Self {
                let mut i = 0;
                Self { #( #from_le_fields , )* }
            }

            #[inline]
            fn from_be_bytes(bytes: Self::Bytes) -> Self {
                let mut i = 0;
                Self { #( #from_be_fields , )* }
            }
        }
    };
    TokenStream::from(tokens)
}

/// Derive the `ToBytes` trait for structs where each field implements it and fieldless enums.
///
/// Const generics in stable don't allow implementing `ToBytes` for arrays `[T; N]` where
/// `T: ToBytes`. This macro circumvents that by deriving a different implementation for fields
/// whose types are arrays, allowing the trait to be derived even for structs with such fields.
///
/// # Endianness
/// By default, the `FromBytes` and `ToBytes` derive macros assume that the data is prefered to be stored
/// in the little endian order.
/// You can change this by setting the attribute `#[packbytes(be)]` for big endian or `#[packbytes(ne)]`
/// for the platform native endian.
///
/// # Fieldless enums
/// The trait is implemented for fieldless enums by converting the numerical value (of the type
/// set by the `repr` attribute on the enum) to bytes.
#[proc_macro_derive(ToBytes, attributes(packbytes))]
pub fn tobytes_derive(input: TokenStream) -> TokenStream {
    match syn::parse::<Item>(input) {
        Ok(Item::Struct(item)) => tobytes_struct_derive(item),
        Ok(Item::Enum(item)) => tobytes_enum_derive(item),
        _ => panic!("#[derive(ToBytes)] can be only applied to structs or enums"),
    }
}

fn tobytes_struct_derive(item: ItemStruct) -> TokenStream {
    let name = &item.ident;
    let generics = &item.generics;
    let unit_fields = UnitFields::new();

    let mut prefers_le = quote!(true);
    for attr in item.attrs.iter() {
        if let Meta::List(ref list) = attr.meta {
            if list.path.is_ident("packbytes") {
                get_endianness(&list.tokens, &mut prefers_le);
            }
        }
    }

    let fields = match item.fields {
        Fields::Named(fields) => fields.named.into_iter(),
        Fields::Unnamed(fields) => fields.unnamed.into_iter(),
        _ => unit_fields.into_iter(),
    };

    let fields = fields.enumerate().map(|(i, field)| {
        let name = field.ident.map_or_else(
            || {
                let i = i.to_string();
                let lit = LitInt::new(&i, Span::call_site());
                quote!(#lit)
            },
            |n| quote!(#n),
        );
        (name, field.ty)
    });

    let field_sizes = fields.clone().map(|(_, ty)| {
        if let Type::Array(arr) = ty {
            let len = arr.len;
            let aty = arr.elem;
            quote! { (#len) * <<#aty as ::packbytes::ToBytes>::Bytes as ::packbytes::ByteArray>::SIZE }
        } else {
            quote! { <<#ty as ::packbytes::ToBytes>::Bytes as ::packbytes::ByteArray>::SIZE }
        }
    });

    let to_fields = |method| {
        fields.clone().map(move |(name, ty)| {
        if let Type::Array(arr) = ty {
            let len = arr.len;
            let aty = arr.elem;
            quote! {
                let size = <<#aty as ::packbytes::ToBytes>::Bytes as ::packbytes::ByteArray>::SIZE;
                for j in 0..(#len) {
                    bytes[i+j*size..i+(j+1)*size].copy_from_slice(&<#aty as ::packbytes::ToBytes>::#method(self.#name[j]));
                }
                i += (#len)*size;
            }
        } else {
            quote! {
                let size = <<#ty as ::packbytes::ToBytes>::Bytes as ::packbytes::ByteArray>::SIZE;
                bytes[i..i+size].copy_from_slice(&<#ty as ::packbytes::ToBytes>::#method(self.#name));
                i += size;
            }
        }
    })
    };
    let to_le_fields = to_fields(quote!(to_le_bytes));
    let to_be_fields = to_fields(quote!(to_be_bytes));

    let tokens = quote! {
        impl #generics ::packbytes::ToBytes for #name #generics {
            type Bytes = [u8; #( #field_sizes + )* 0];

            const PREFERS_LE: bool = #prefers_le;

            #[inline]
            fn to_le_bytes(self) -> Self::Bytes {
                let mut bytes = <Self::Bytes as ::packbytes::ByteArray>::zeroed();
                let mut i = 0;
                #( #to_le_fields )*
                bytes
            }

            #[inline]
            fn to_be_bytes(self) -> Self::Bytes {
                let mut bytes = <Self::Bytes as ::packbytes::ByteArray>::zeroed();
                let mut i = 0;
                #( #to_be_fields )*
                bytes
            }
        }
    };
    TokenStream::from(tokens)
}

fn tobytes_enum_derive(item: ItemEnum) -> TokenStream {
    for variant in item.variants.iter() {
        let Fields::Unit = variant.fields else {
            panic!("#[derive(ToBytes)] can be only applied to fieldless enums");
        };
    }

    let mut repr = quote!(u8);
    let mut prefers_le = quote!(true);
    for attr in item.attrs.iter() {
        if let Meta::List(ref list) = attr.meta {
            if list.path.is_ident("packbytes") {
                get_endianness(&list.tokens, &mut prefers_le);
            } else if list.path.is_ident("repr") {
                get_numeric_type(&list.tokens, &mut repr);
            }
        }
    }

    let name = &item.ident;
    let generics = &item.generics;

    let tokens = quote! {
        impl #generics ::packbytes::ToBytes for #name #generics {
            type Bytes = [u8; #repr::BITS as usize / 8];

            #[inline]
            fn to_le_bytes(self) -> Self::Bytes {
                (self as #repr).to_le_bytes()
            }

            #[inline]
            fn to_be_bytes(self) -> Self::Bytes {
                (self as #repr).to_be_bytes()
            }
        }
    };
    TokenStream::from(tokens)
}

/// Derive the `TryFromBytes` trait for structs where each field implements it and fieldless enums.
///
/// Const generics in stable don't allow implementing `TryFromBytes` for arrays `[T; N]` where
/// `T: TryFromBytes`. This macro circumvents that by deriving a different implementation for fields
/// whose types are arrays, allowing the trait to be derived even for structs with such fields.
/// Note that before `core::array::try_from_fn` is stabilised, this is provided only for `T: FromBytes`.
///
/// # Endianness
/// By default, the `FromBytes` and `ToBytes` derive macros assume that the data is prefered to be stored
/// in the little endian order.
/// You can change this by setting the attribute `#[packbytes(be)]` for big endian or `#[packbytes(ne)]`
/// for the platform native endian.
///
/// # Fieldless enums
/// The trait is implementing for fieldless enums by first converting bytes to a numerical value
/// (of the type set by the `repr` attribute on the enum) and then comparing it to the values of all variants.
///
/// # Errors
/// By default, the error type is `packbytes::errors::InvalidData`. You can provide a custom error
/// type with the `packbytes_error` attribute.
///
/// For enums, in case the bytes don't represent a valid enum variant, a value of the erro provided by
/// the `Default` trait (if implemented) is returned. You can overrride it by setting the
/// `packbytes_error_exp` attribute for a custom error expression.
///
/// For structs, you need to make sure that your error implements `From<<T as TryFromBytes>::Error>` for every `T`
/// which is a type of a field of the struct. In particular, the types that implement `FromBytes`
/// have error type `std::convert::Infallible` (the conversion can never fail).
/// Thus your error type should implement `From<std::convert::Infallible>`.
/// (When the `!` type is stabilised, this will automatically be true for every type.
///
/// ```
/// # use packbytes_derive::TryFromBytes;
/// enum MyError {
///     InvalidFoo,
///     SomethingElse
/// }
///
/// impl From<std::convert::Infallible> for MyError {
///     fn from(impossible: std::convert::Infallible) -> Self {
///         unreachable!()
///     }
/// }
///
/// #[derive(TryFromBytes)]
/// #[packbytes_error(MyError)]
/// #[packbytes_error_exp(MyError::InvalidFoo)]
/// enum Foo {
///     Bar,
///     Baz
/// }
///
/// #[derive(TryFromBytes)]
/// #[packbytes_error(MyError)]
/// struct MyStruct {
///     val: u16,
///     foo: Foo
/// }
/// ```
#[proc_macro_derive(TryFromBytes, attributes(packbytes, packbytes_error, packbytes_error_exp))]
pub fn tryfrombytes_derive(input: TokenStream) -> TokenStream {
    match syn::parse::<Item>(input) {
        Ok(Item::Struct(item)) => tryfrombytes_struct_derive(item),
        Ok(Item::Enum(item)) => tryfrombytes_enum_derive(item),
        _ => panic!("#[derive(TryFromBytes)] can be only applied to structs or enums"),
    }
}

fn tryfrombytes_struct_derive(item: ItemStruct) -> TokenStream {
    let name = &item.ident;
    let generics = &item.generics;
    let unit_fields = UnitFields::new();

    let mut error = quote!(::packbytes::error::InvalidData);
    let mut prefers_le = quote!(true);
    for attr in item.attrs.iter() {
        if let Meta::List(ref list) = attr.meta {
            if list.path.is_ident("packbytes") {
                get_endianness(&list.tokens, &mut prefers_le);
            } else if list.path.is_ident("packbytes_error") {
                error = list.tokens.clone();
            }
        }
    }

    let fields = match item.fields {
        Fields::Named(fields) => fields.named.into_iter(),
        Fields::Unnamed(fields) => fields.unnamed.into_iter(),
        _ => unit_fields.into_iter(),
    };

    let fields = fields.enumerate().map(|(i, field)| {
        let name = field.ident.map_or_else(
            || {
                let i = i.to_string();
                let lit = LitInt::new(&i, Span::call_site());
                quote!(#lit)
            },
            |n| quote!(#n),
        );
        (name, field.ty)
    });

    let field_sizes = fields.clone().map(|(_, ty)| {
        if let Type::Array(arr) = ty {
            let len = arr.len;
            let aty = arr.elem;
            quote! { (#len) * <<#aty as ::packbytes::FromBytes>::Bytes as ::packbytes::ByteArray>::SIZE }
        } else {
            quote! { <<#ty as ::packbytes::TryFromBytes>::Bytes as ::packbytes::ByteArray>::SIZE }
        }
    });

    // TODO: switch to `core::array::try_from_fn` once stabilised
    let from_fields = |method, regular_method| {
        fields.clone().map(move |(name, ty)| {
        if let Type::Array(arr) = ty {
            let len = arr.len;
            let aty = arr.elem;
            quote! {
                #name: {
                    let size = <<#aty as ::packbytes::FromBytes>::Bytes as ::packbytes::ByteArray>::SIZE;
                    let val = ::core::array::from_fn(|j| {
                        <#aty as ::packbytes::FromBytes>::#regular_method(bytes[i+j*size..i+(j+1)*size].try_into().unwrap())
                    });
                    i += (#len) * size;
                    val
                }
            }
        } else {
            quote! {
                #name: {
                    let size = <<#ty as ::packbytes::TryFromBytes>::Bytes as ::packbytes::ByteArray>::SIZE;
                    let val = <#ty as ::packbytes::TryFromBytes>::#method(bytes[i..i+size].try_into().unwrap())?;
                    i += size;
                    val
                }
            }
        }
    })
    };
    let from_le_fields = from_fields(quote!(try_from_le_bytes), quote!(from_le_bytes));
    let from_be_fields = from_fields(quote!(try_from_be_bytes), quote!(from_be_bytes));

    let tokens = quote! {
        impl #generics ::packbytes::TryFromBytes for #name #generics {
            type Bytes = [u8; #( #field_sizes + )* 0];
            type Error = #error;

            const PREFERS_LE: bool = #prefers_le;

            #[inline]
            fn try_from_le_bytes(bytes: Self::Bytes) -> Result<Self, Self::Error> {
                let mut i = 0;
                Ok(Self { #( #from_le_fields , )* })
            }

            #[inline]
            fn try_from_be_bytes(bytes: Self::Bytes) -> Result<Self, Self::Error> {
                let mut i = 0;
                Ok(Self { #( #from_be_fields , )* })
            }
        }
    };
    TokenStream::from(tokens)
}

fn tryfrombytes_enum_derive(item: ItemEnum) -> TokenStream {
    for variant in item.variants.iter() {
        let Fields::Unit = variant.fields else {
            panic!("#[derive(TryFromBytes)] can be only applied to fieldless enums");
        };
    }

    let mut repr = quote!(u8);
    let mut error = quote!(::packbytes::error::InvalidData);
    let mut error_exp = quote!(Default::default());
    let mut prefers_le = quote!(true);
    for attr in item.attrs.iter() {
        if let Meta::List(ref list) = attr.meta {
            if list.path.is_ident("packbytes") {
                get_endianness(&list.tokens, &mut prefers_le);
            } else if list.path.is_ident("repr") {
                get_numeric_type(&list.tokens, &mut repr);
            } else if list.path.is_ident("packbytes_error") {
                error = list.tokens.clone();
            } else if list.path.is_ident("packbytes_error_exp") {
                error_exp = list.tokens.clone();
            }
        }
    }

    let name = &item.ident;
    let generics = &item.generics;

    let branches_le = item
        .variants
        .iter()
        .map(|v| v.ident.clone())
        .map(|variant| quote!(a if a == (Self :: #variant as #repr) => Ok(Self :: #variant) ,));
    let branches_be = branches_le.clone();

    let tokens = quote! {
        impl #generics ::packbytes::TryFromBytes for #name #generics {
            type Bytes = [u8; #repr::BITS as usize / 8];
            type Error = #error;

            const PREFERS_LE: bool = #prefers_le;

            #[inline]
            fn try_from_le_bytes(bytes: Self::Bytes) -> Result<Self, Self::Error> {
                match #repr::from_le_bytes(bytes) {
                    #( #branches_le )*
                    _ => Err(#error_exp)
                }
            }

            #[inline]
            fn try_from_be_bytes(bytes: Self::Bytes) -> Result<Self, Self::Error> {
                match #repr::from_be_bytes(bytes) {
                    #( #branches_be )*
                    _ => Err(#error_exp)
                }
            }
        }
    };
    TokenStream::from(tokens)
}

fn get_endianness(ts: &proc_macro2::TokenStream, end: &mut proc_macro2::TokenStream) {
    let ident = syn::parse2::<Ident>(ts.clone()).unwrap().to_string();
    match ident.as_str() {
        "be" => { *end = quote!(false); },
        "ne" => { *end = quote!(cfg!(target_endian = "little")); }
        "le" => {},
        _ => { panic!("the valid values of the `packbytes` attribute are \"le\", \"be\" and \"ne\""); }
    }
}

fn get_numeric_type(ts: &proc_macro2::TokenStream, repr: &mut proc_macro2::TokenStream) {
    if let Ok(ident) = syn::parse2::<Ident>(ts.clone()) {
        let ident = ident.to_string();
        if matches!(ident.as_str(),
            "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
        ) {
            *repr = ts.clone();
        }
    }
}
