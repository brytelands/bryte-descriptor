extern crate proc_macro;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::parse_macro_input;
use syn::{parse::Parse, Expr, Token};

struct MacroInput2 {
    arg_1: Expr,
    comma: Token![,],
    arg_2: Expr,
}

impl Parse for MacroInput2 {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            arg_1: input.parse()?,
            comma: input.parse()?,
            arg_2: input.parse()?,
        })
    }
}

#[proc_macro_derive(SchemaGeneratorAnchor)]
pub fn schema_generator_anchor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let account_strct = parse_macro_input!(input as syn::ItemStruct);

    let account_name = &account_strct.ident;
    let account_name_descriptor = format!("{account_name}Descriptor");
    let account_name_desc_ident = Ident::new(&account_name_descriptor, Span::call_site());

    proc_macro::TokenStream::from(quote! {
        impl bryte_descriptor_state::states::SchemaEventAnchor for #account_name {
             fn generate_schema(self) -> Vec<u8> {
                let mut defs = Default::default();
                Self::add_definitions_recursively(&mut defs);
                let container: borsh::schema::BorshSchemaContainer = Self::schema_container();
                let mut data = container
                    .try_to_vec()
                    .expect("Failed to serialize BorshSchemaContainer for account");
                data
            }
        }

        #[account]
        #[derive(BorshSchema)]
        pub struct #account_name_desc_ident {
            pub schema: Vec<u8>
        }

        impl #account_name_desc_ident {
            fn size() -> usize {
                Self::default().schema.len() + 12 //8 for discriminator and 4 for size
            }
        }

        impl bryte_descriptor_state::states::Descriptor for #account_name_desc_ident {
            fn schema(&self) -> Vec<u8> {
               let schema_gen = #account_name::default().generate_schema();
                schema_gen
            }
        }

        impl Default for #account_name_desc_ident {
           fn default() -> Self {
               let schema_gen = #account_name::default().generate_schema();
                #account_name_desc_ident {
                 schema: schema_gen
                }
            }
        }
    })
}

#[proc_macro_attribute]
pub fn state_descriptor(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let event_strct = parse_macro_input!(input as syn::ItemStruct);

    let state_name = &event_strct.ident;
    let state_name_descriptor = format!("{state_name}Descriptor");
    let state_name_descriptor_ident = Ident::new(&state_name_descriptor, Span::call_site());

    let discriminator: proc_macro2::TokenStream = {
        let discriminator_preimage = format!("account:{state_name}");
        let mut discriminator = [0u8; 8];
        discriminator.copy_from_slice(
            &bryte_descriptor_state::bryte_desciptor_hash::hash(discriminator_preimage.as_bytes()).to_bytes()[..8],
        );
        format!("{discriminator:?}").parse().unwrap()
    };

    let state_descriptor_discriminator: proc_macro2::TokenStream = {
        let discriminator_preimage = format!("account:{state_name_descriptor}");
        let mut discriminator = [0u8; 8];
        discriminator.copy_from_slice(
            &bryte_descriptor_state::bryte_desciptor_hash::hash(discriminator_preimage.as_bytes()).to_bytes()[..8],
        );
        format!("{discriminator:?}").parse().unwrap()
    };

    proc_macro::TokenStream::from(quote! {
        #[derive(borsh::BorshSerialize, borsh::BorshDeserialize, borsh::BorshSchema)]
        #event_strct

        impl bryte_descriptor_state::states::SchemaEvent for #state_name {
            fn data(&self) -> Vec<u8> {
                let mut d = #discriminator.to_vec();
                d.append(&mut self.try_to_vec().unwrap());
                d
            }

            fn generate_schema(self) -> Vec<u8> {
                let mut defs = Default::default();
                Self::add_definitions_recursively(&mut defs);
                let container: borsh::schema::BorshSchemaContainer = Self::schema_container();
                let mut data = container
                    .try_to_vec()
                    .expect("Failed to serialize BorshSchemaContainer for event");
                data
            }
        }

        impl bryte_descriptor_state::states::DescriptorSerialize for #state_name {
            fn try_serialize<W: std::io::Write>(
                &self,
                writer: &mut W,
            ) {
                // TODO revisit error handling
                if writer.write_all(&#discriminator).is_err() {
                    // return false;
                    // return Err(
                    //     bryte_descriptor_state::states::DescriptorError::new(),
                    // )
                }
                if BorshSerialize::serialize(self, writer).is_err() {
                    // return false;
                    // return Err(
                    //     bryte_descriptor_state::states::DescriptorError::new(),
                    // )
                }
                // Ok(())
            }
        }

        impl DescriptorDeserialize for #state_name {
            fn try_deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
                // TODO revisit error handling
                if buf.len() < #discriminator.len() {
                    // return false;
                    // return Err(
                    //         bryte_descriptor_state::states::DescriptorError::new()
                    // );
                }
                let given_disc = &buf[..8];
                if &#discriminator != given_disc {
                    // return false;
                    // return Err(
                    //     bryte_descriptor_state::states::DescriptorError::new(),
                    // );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> std::io::Result<Self> {
                let mut data: &[u8] = &buf[8..];
                BorshDeserialize::deserialize(&mut data)
                    // .map_err(|_| {
                    //         false
                    // })
            }
        }

        impl bryte_descriptor_state::states::Discriminator for #state_name {
            const DISCRIMINATOR: [u8; 8] = #discriminator;
        }

        #[derive(borsh::BorshSerialize, borsh::BorshDeserialize, borsh::BorshSchema)]
        pub struct #state_name_descriptor_ident {
            pub schema: Vec<u8>
        }

        impl #state_name_descriptor_ident {
            fn size() -> usize {
                Self::default().schema.len() + 8usize + 4usize //8 for discriminator and 4 for size
            }
        }

        impl bryte_descriptor_state::states::Discriminator for #state_name_descriptor_ident {
            const DISCRIMINATOR: [u8; 8] = #state_descriptor_discriminator;
        }

        impl bryte_descriptor_state::states::Descriptor for #state_name_descriptor_ident {
            fn schema(&self) -> Vec<u8> {
               let schema_gen = #state_name::default().generate_schema();
                schema_gen
            }
        }

        impl Default for #state_name_descriptor_ident {
           fn default() -> Self {
               let schema_gen = #state_name::default().generate_schema();
                #state_name_descriptor_ident {
                 schema: schema_gen
                }
            }
        }

         impl bryte_descriptor_state::states::DescriptorSerialize for #state_name_descriptor_ident {
            fn try_serialize<W: std::io::Write>(
                &self,
                writer: &mut W,
            ) {
                writer.write_all(&#state_descriptor_discriminator);
                BorshSerialize::serialize(self, writer);
                // if writer.write_all(&#state_descriptor_discriminator).is_err() {
                    // return false;
                    // return Err(
                    //     bryte_descriptor_state::states::DescriptorError::new(),
                    // )
                // }
                // if BorshSerialize::serialize(self, writer).is_err() {
                    // return false;
                    // return Err(
                    //     bryte_descriptor_state::states::DescriptorError::new(),
                    // )
                // }
                // Ok(())
            }
        }

        impl DescriptorDeserialize for #state_name_descriptor_ident {
            fn try_deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
                if buf.len() < #state_descriptor_discriminator.len() {
                    // return false;
                    // return Err(
                    //         bryte_descriptor_state::states::DescriptorError::new()
                    // );
                }
                let given_disc = &buf[..8];
                if &#state_descriptor_discriminator != given_disc {
                    // return false;
                    // return Err(
                    //     bryte_descriptor_state::states::DescriptorError::new(),
                    // );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> std::io::Result<Self> {
                let mut data: &[u8] = &buf[8..];
                BorshDeserialize::deserialize(&mut data)
                    // .map_err(|_| {
                    //         false
                    // })
            }
        }
    })
}

#[proc_macro]
pub fn register_program_log_data_schema(
    token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(token_stream as MacroInput2);

    let data = &input.arg_1;
    let is_anchor = &input.arg_2;
    proc_macro::TokenStream::from(quote! {
        {
            let schema_string = base64::encode(#data.generate_schema().as_slice());
            msg!("LD:{}:{}", #is_anchor, schema_string);
        }
    })
}

#[proc_macro]
pub fn register_program_account_schema(
    token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(token_stream as MacroInput2);

    let data = &input.arg_1;
    let is_anchor = &input.arg_2;
    proc_macro::TokenStream::from(quote! {
        {
            let schema_string = base64::encode(#data.generate_schema().as_slice());
            msg!("AR:{}:{}", #is_anchor, schema_string);
        }
    })
}

#[proc_macro]
pub fn register_program_instruction_log_schema(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let data: proc_macro2::TokenStream = input.into();
    proc_macro::TokenStream::from(quote! {
        {
            solana_program::log::sol_log_data(&[&#data.generate_schema()]);
        }
    })
}

#[proc_macro]
pub fn publish(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let data: proc_macro2::TokenStream = input.into();
    proc_macro::TokenStream::from(quote! {
        {
            solana_program::log::sol_log_data(&[&bryte_descriptor_state::states::SchemaEvent::data(&#data)]);
        }
    })
}

#[proc_macro]
pub fn init_descriptor(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let data: proc_macro2::TokenStream = input.into();
    proc_macro::TokenStream::from(quote! {
        {
            #data.schema = #data.schema();
        }
    })
}