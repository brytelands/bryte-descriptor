use anchor_lang::AnchorSerialize;
use anchor_lang::prelude::*;

declare_id!("48dKJJqhS5B1oEGBzgDVBmz3KCRYeniD3CabZ5QehbTC");

#[program]
pub mod bryte_descriptor_demo {
    use anchor_lang::{Discriminator, Event};
    use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
    use bryte_descriptor_attribute::{init_descriptor, SchemaGeneratorAnchor};
    use bryte_descriptor_state::states::SchemaEvent;
    use bryte_descriptor_state::states::SchemaEventAnchor;
    use bryte_descriptor_state::states::Descriptor;
    use bryte_descriptor_state::states::DescriptorDeserialize;

    use super::*;

    /// Initialize the program.
    /// Generate and publish the schemas for:
    /// - ExampleAccount
    /// - CloudEvent
    /// - NonAnchorExample
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let example_account = &mut ctx.accounts.example_account;
        let example_account_descriptor = &mut ctx.accounts.example_account_descriptor;
        init_descriptor!(example_account_descriptor);

        Ok(())
    }

    /// Update the account.
    /// Log (emit! or publish!) the following events:
    /// - CloudEvent
    /// - NonAnchorExample
    pub fn update_account(ctx: Context<UpdateAccount>, new_name: String) -> Result<()> {
        let example_account = &mut ctx.accounts.example_account;
        msg!("Updating account");

        let clock = Clock::get()?;
        let ts = clock.unix_timestamp.to_string();

        let cloud_event_id: String = ["cloud_event", &ts.clone()].join(":");
        let source = ["/solana-program/", "48dKJJqhS5B1oEGBzgDVBmz3KCRYeniD3CabZ5QehbTC"].join("");
        let subject = bs58::encode(&example_account.key()).into_string();

        let ts = clock.unix_timestamp.to_string();

        let cloud_event = CloudEvent {
                data: Datadef::String {value: "{\"key\":\"value\"}".to_string()},
                data_base_64: Option::from("".to_string()),
                datacontenttype: Option::from("application/json".to_string()),
                dataschema: Option::from("".to_string()),
                id: cloud_event_id,
                source: source.to_string(),
                specversion: String::from("1.0"),
                subject: Option::from(subject.clone()),
                time: Option::from(ts),
                type_: "solana.program.instruction.UpdateAccount".to_string(),
        };
        emit!(cloud_event);

        let example_account_counter = &example_account.data.counter + 1;

        example_account.counter = example_account_counter;
        example_account.program_id = "48dKJJqhS5B1oEGBzgDVBmz3KCRYeniD3CabZ5QehbTC".to_string();
        example_account.data.counter = example_account_counter;
        example_account.data.name = new_name;

        Ok(())
    }

    #[derive(Accounts)]
    pub struct Initialize<'info> {
        #[account(init, payer = signer, space = 1000)]
        pub example_account: Account<'info, ExampleAccount>,
        #[account(init, payer = signer, space = ExampleAccountDescriptor::size(), seeds = [&ExampleAccount::DISCRIMINATOR], bump)]
        pub example_account_descriptor: Account<'info, ExampleAccountDescriptor>,
        #[account(mut)]
        pub signer: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct UpdateAccount<'info> {
        #[account(mut)]
        pub example_account: Account<'info, ExampleAccount>,
        pub signer: Signer<'info>
    }

    /// For Anchor accounts we simply need to derive the following traits:
    /// - BorshSchema
    /// - SchemaGeneratorAnchor
    #[account]
    #[derive(Default, BorshSchema, SchemaGeneratorAnchor)]
    pub struct ExampleAccount {
        pub counter: i32,
        pub program_id: String,
        pub data: BasicStruct,
    }

    #[derive(Default, Clone, AnchorSerialize, AnchorDeserialize, BorshSchema)]
    pub struct BasicStruct {
        pub counter: i32,
        pub name: String,
    }

    /// For Anchor events we simply need to derive the following traits:
    /// - BorshSchema
    /// - SchemaGeneratorAnchor
    #[event]
    #[derive( Debug, Default, PartialEq, Eq, BorshSchema, SchemaGeneratorAnchor)]
    pub struct CloudEvent {
        pub data: Datadef,
        pub data_base_64: Option<String>,
        pub datacontenttype: Option<String>,
        pub dataschema: Option<String>,
        pub id: String,
        pub source: String,
        pub specversion: String,
        pub subject: Option<String>,
        pub time: Option<String>,
        pub type_: String,
    }

    impl Clone for CloudEvent {
        fn clone(&self) -> CloudEvent {
            CloudEvent {
                data: self.data.clone(),
                data_base_64: self.data_base_64.clone(),
                datacontenttype: self.datacontenttype.clone(),
                dataschema: self.dataschema.clone(),
                id: self.id.clone(),
                source: self.source.clone(),
                specversion: self.specversion.clone(),
                subject: self.subject.clone(),
                time: self.time.clone(),
                type_: self.type_.clone(),
            }
        }

    }

    #[derive(Debug, Clone, Default, PartialEq, Eq, AnchorSerialize, AnchorDeserialize, BorshSchema)]
    pub enum Datadef {
        #[default]
        Null,
        Boolean { value: bool },
        Array { value: Vec<u8> },
        Number { value: u64 },
        String { value: String },
    }

    impl From<&Datadef> for Datadef {
        fn from(value: &Datadef) -> Self {
            value.clone()
        }
    }

    impl From<bool> for Datadef {
        fn from(value: bool) -> Self {
            Self::Boolean { value }
        }
    }

    impl From<Vec<u8>> for Datadef {
        fn from(value: Vec<u8>) -> Self {
            Self::Array { value }
        }
    }

    impl From<u64> for Datadef {
        fn from(value: u64) -> Self {
            Self::Number { value }
        }
    }

}
