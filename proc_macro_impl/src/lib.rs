// POD Struct Deserializer/Serializer
use {
    proc_macro::TokenStream,
    quote::quote,
    std::ops::Mul
};

fn helper(
    offset: &mut usize,
    fields_range: &mut Vec<(syn::Ident, std::ops::Range<usize>)>,
    field: &syn::Field,
    ident: &str,
    array_len: Option<u16>,
) {
    let mul_by = if let Some(len) = array_len {
        len
    } else {
        1
    } as usize;

    let old_offset = *offset;

    match ident {
        "u8" => {
            *offset += std::mem::size_of::<u8>().mul(mul_by);
            fields_range.push(
                (field.ident.clone().unwrap(), old_offset..*offset)
            )
        },
        "u16" => {
            *offset += std::mem::size_of::<u16>().mul(mul_by);
            fields_range.push(
                (field.ident.clone().unwrap(), old_offset..*offset)
            )
        },
        "u32" => {
            *offset += std::mem::size_of::<u32>().mul(mul_by);
            fields_range.push(
                (field.ident.clone().unwrap(), old_offset..*offset)
            )
        },
        "u64" => {
            *offset += std::mem::size_of::<u64>().mul(mul_by);
            fields_range.push(
                (field.ident.clone().unwrap(), old_offset..*offset)
            )
        },
        "u128" => {
            *offset += std::mem::size_of::<u128>().mul(mul_by);
            fields_range.push(
                (field.ident.clone().unwrap(), old_offset..*offset)
            )
        },
        "i8" => {
            *offset += std::mem::size_of::<i8>().mul(mul_by);
            fields_range.push(
                (field.ident.clone().unwrap(), old_offset..*offset)
            )
        },
        "i16" => {
            *offset += std::mem::size_of::<i16>().mul(mul_by);
            fields_range.push(
                (field.ident.clone().unwrap(), old_offset..*offset)
            )
        },
        "i32" => {
            *offset += std::mem::size_of::<i32>().mul(mul_by);
            fields_range.push(
                (field.ident.clone().unwrap(), old_offset..*offset)
            )
        },
        "i64" => {
            *offset += std::mem::size_of::<i64>().mul(mul_by);
            fields_range.push(
                (field.ident.clone().unwrap(), old_offset..*offset)
            )
        },
        "i128" => {
            *offset += std::mem::size_of::<i128>().mul(mul_by);
            fields_range.push(
                (field.ident.clone().unwrap(), old_offset..*offset)
            )
        },
        "bool" => {
            *offset += std::mem::size_of::<bool>().mul(mul_by);
            fields_range.push(
                (field.ident.clone().unwrap(), old_offset..*offset)
            )
        },
        "usize" => {
            *offset += std::mem::size_of::<usize>().mul(mul_by);
            fields_range.push(
                (field.ident.clone().unwrap(), old_offset..*offset)
            )
        },
        "isize" => {
            *offset += std::mem::size_of::<isize>().mul(mul_by);
            fields_range.push(
                (field.ident.clone().unwrap(), old_offset..*offset)
            )
        },
        _ => panic!("All struct field's types must be POD .")
    };
}

/// <b>PodDeSe</b> macro is designed to serialize and deserialize the on-chain data-accounts without needing for `borsh`.
/// It is much lighter than the `borsh` macros and also much cheeper in terms of CU usage. It is recommended for on-chain
/// programs that have no complex state-accounts.
/// 1. The crate which is using this must be an on-chain program (proc_macro is using solana_program crate)
/// 2. The read & write methods that are generated via using this proc_macro all have `unsafe` keyword because
///    it is up to the `user` to validate and check the method's input parameters.
/// 3. This proc_macro is only for structs with `POD` data types fields like `u8`, `i8`, `[T: n]`, ... .
#[proc_macro_derive(PodDeSe)]
pub fn pod_struct_de_se(input: TokenStream) -> TokenStream {
    let syn::ItemStruct {
        ident: struct_ident,
        fields,
        ..
    } = syn::parse_macro_input!(input as syn::ItemStruct);

    match fields {
        syn::Fields::Named(named_fields) => {
            let named_fields: Vec<syn::Field> = named_fields
                .named
                .into_iter()
                .collect::<Vec<_>>();

            let mut offset = 0_usize;
            let mut fields_range: Vec<(syn::Ident, std::ops::Range<usize>)> = Vec::with_capacity(named_fields.len());

            named_fields.iter().for_each(|field| {
                let filed_type = &field.ty;

                match filed_type {
                    syn::Type::Array(array_info) => {
                        let syn::TypeArray {
                            elem,
                            len: array_length,
                            ..
                        } = array_info;

                        match array_length {
                            syn::Expr::Lit(literal_info) => {
                                match &literal_info.lit {
                                    syn::Lit::Int(int_literal) => {
                                        let length = int_literal
                                            .base10_parse::<u16>()
                                            .unwrap();
                                        let ty = elem.as_ref();

                                        if let syn::Type::Path(path) = ty {
                                            let ident_str = path
                                                .path
                                                .get_ident()
                                                .unwrap()
                                                .to_string();

                                            helper(
                                                &mut offset,
                                                &mut fields_range,
                                                field,
                                                &ident_str,
                                                Some(length)
                                            );
                                        } else {
                                            panic!("Invalid state!");
                                        };
                                    },
                                    _ => panic!("Only integer literal is allowed.")
                                };
                            },
                            _ => panic!("Invalid length expr! only literal length is allowed.")
                        };
                    },
                    _ => {
                        if let syn::Type::Path(type_path) = filed_type {
                            let ty = type_path
                                .path
                                .get_ident()
                                .expect("Only simple types with single identifier!")
                                .to_string();

                            helper(
                                &mut offset,
                                &mut fields_range,
                                field,
                                &ty,
                                None
                            );
                        } else {
                            panic!("Invalid state!");
                        };
                    }
                };
            });

            let field_methods = fields_range.iter().map(|(ident, range)| {
                let fn_write_ident = quote::format_ident!("write_{}", ident);
                let fn_read_ident = quote::format_ident!("read_{}", ident);

                let visibility = syn::Ident::new("pub", proc_macro::Span::call_site().into());
                let fn_reads_ret_ty: syn::Type = syn::parse_str("Vec<u8>").unwrap();

                let start = range.start;
                let end = range.end;
                let field_data_len = end - start;
            
                quote! {
                    #visibility unsafe fn #fn_write_ident(account_data: &mut [u8], expected_input: &[u8]) {
                        ::solana_program::program_memory::sol_memcpy(
                            account_data
                                .get_mut(#start..#end)
                                .unwrap(),
                            expected_input,
                            #field_data_len
                        );
                    }
            
                    #visibility unsafe fn #fn_read_ident(account_data: &[u8]) -> #fn_reads_ret_ty {
                        account_data
                            .get(#start..#end)
                            .unwrap()
                            .to_vec()
                    }
                }
            }).collect::<Vec<_>>();

            let implementation = quote! {
                #[automatically_derived]
                impl #struct_ident {
                    #(#field_methods)*
                }
            };

            return implementation.into();
        },
        _ => return syn::Error::new(
            struct_ident.span(),
            "Only named structs are supported!"
        ).to_compile_error().into()
    };
}
// POD Struct Deserializer/Serializer