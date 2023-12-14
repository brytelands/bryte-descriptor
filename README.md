# bryte-descriptor

This is a suite of libraries that provide functionality as an alternative to IDLs for data *reads*.

### bryte-descriptor-state

This library provides traits to define data that is used for creating and managing schemas and serialization for Solana accounts and events.

### bryte-descriptor-attribute

This library provides macros for account and event structs that generate "descriptor" structs. These descriptor structs
hold the Borsh Schemas for your accounts or events. These schemas can be retrieved on-chain and used to deserialize
account or event data.

This library supports both Solana programs and Anchor programs.

Please see the anchor-demo-program or bryte-descriptor-solana-demo for example usage.

### bryte-descriptor-client

This client library provides functions to retrieve account data as JSON and lookup discriminators for accounts and events.

### anchor-demo-program

This is an example Anchor program demonstrating the use of the bryte-descriptor library.

### bryte-descriptor-solana-demo

This is an example vanilla Solana program demonstrating the use of the bryte-descriptor library.

Anchor Example

```rust
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
```

Solana Example

```rust
use std::io::{Write};
use borsh::{BorshDeserialize, BorshSerialize, BorshSchema};
use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint, entrypoint::ProgramResult, msg, program::invoke_signed, program_error::ProgramError, pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar};
use bryte_descriptor_attribute::{state_descriptor};
use bryte_descriptor_state::states::{DescriptorDeserialize, DescriptorSerialize, Discriminator, SchemaEvent};
use bryte_descriptor_state::states::SchemaEventAnchor;
use bryte_descriptor_state::states::Descriptor;

#[state_descriptor]
#[derive(Default, Debug)]
pub struct PersonState {
    is_initialized: bool,
    first_name: String,
    last_name: String,
}

impl PersonState {
    const SIZE: usize = 8 + 1 + 24 + 24;
}

// Accounts required
/// 1. [signer, writable] Funding account
/// 2. [writable] PDA account
/// 3. [] System Program
pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    // Getting required accounts
    let funding_account = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    let pda_account_descriptor = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // Getting PDA Bump from instruction data
    let (pda_bump, _) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    // Checking if passed PDA and expected PDA are equal
    let signers_seeds: &[&[u8]; 3] = &[
        b"customaddress",
        &funding_account.key.to_bytes(),
        &[*pda_bump],
    ];
    let pda = Pubkey::create_program_address(signers_seeds, program_id)?;
    msg!("pda {:?}", pda);

    if pda.ne(&pda_account.key) {
        return Err(ProgramError::InvalidAccountData);
    }

    // Assessing required lamports and creating transaction instruction
    let lamports_required = Rent::get()?.minimum_balance(PersonState::SIZE);
    let create_pda_account_ix = system_instruction::create_account(
        &funding_account.key,
        &pda_account.key,
        lamports_required,
        PersonState::SIZE.try_into().unwrap(),
        &program_id,
    );
    // Invoking the instruction but with PDAs as additional signer
    invoke_signed(
        &create_pda_account_ix,
        &[
            funding_account.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[signers_seeds],
    )?;

    // Setting state for PDA
    let pda_account_state = PersonState {
        is_initialized: true,
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
    };
    pda_account_state.try_serialize(&mut &mut pda_account.data.borrow_mut()[..]);

    let (pda_descriptor, pda_descriptor_bump) = Pubkey::find_program_address(
        &[&PersonState::DISCRIMINATOR],
        &program_id
    );

    if pda_descriptor.ne(&pda_account_descriptor.key) {
        return Err(ProgramError::InvalidAccountData);
    }

    // Assessing required lamports and creating transaction instruction
    let lamports_required = Rent::get()?.minimum_balance(PersonStateDescriptor::size());
    let create_pda_account_descriptor_ix = system_instruction::create_account(
        &funding_account.key,
        &pda_descriptor,
        lamports_required,
        PersonStateDescriptor::size().try_into().unwrap(),
        &program_id,
    );

    // Invoking the instruction but with PDAs as additional signer
    invoke_signed(
        &create_pda_account_descriptor_ix,
        &[
            funding_account.clone(),
            pda_account_descriptor.clone(),
            system_program.clone(),
        ],
        &[
            &[
                &PersonState::DISCRIMINATOR,
                &[pda_descriptor_bump],
            ],
        ]
    )?;

    // Setting state for PDA
    let pda_account_state_descriptor = PersonStateDescriptor::default();
    pda_account_state_descriptor.try_serialize(&mut &mut pda_account_descriptor.data.borrow_mut()[..]);

    Ok(())
}
```