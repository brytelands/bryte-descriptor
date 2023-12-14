use std::io::{Write};
use borsh::{BorshDeserialize, BorshSerialize, BorshSchema};
use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint, entrypoint::ProgramResult, msg, program::invoke_signed, program_error::ProgramError, pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar};
use bryte_descriptor_attribute::{state_descriptor};
use bryte_descriptor_state::states::{DescriptorDeserialize, DescriptorSerialize, Discriminator, SchemaEvent};
use bryte_descriptor_state::states::SchemaEventAnchor;
use bryte_descriptor_state::states::Descriptor;

entrypoint!(initialize);

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

mod Test {
    use std::cell::RefCell;
    use std::rc::Rc;
    use borsh::BorshDeserialize;
    use borsh::schema::BorshSchemaContainer;
    use borsh_serde_adapter::deserialize_adapter::deserialize_from_schema;
    use solana_program::account_info::AccountInfo;
    use bryte_descriptor_state::states::DescriptorSerialize;
    use crate::{PersonState, PersonStateDescriptor};
    use bytes::BufMut;

    #[test]
    fn test() {
        let mut binding = 1000u64;

        let mut pda_account = AccountInfo {
            key: &Default::default(),
            lamports: Rc::new(RefCell::new(&mut binding)),
            data: Rc::new(RefCell::new(&mut [])),
            owner: &Default::default(),
            rent_epoch: 0,
            is_signer: false,
            is_writable: false,
            executable: false,
        };
        let mut pda_account_state = PersonState {
            is_initialized: true,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };
        let mut writer = vec![].writer();
        pda_account_state.try_serialize(&mut &mut writer).expect("TODO: panic message");
        let schema_bytes = PersonStateDescriptor::default().schema;
        println!("schema_bytes {:?}", &schema_bytes);

        let schema = BorshSchemaContainer::deserialize_reader(&mut schema_bytes.as_slice()).expect("Deserializing BorshSchemaContainer failed.");

        // let mut x = &pda_account.data.borrow_mut()[..];
        println!("{:?}", &writer);
        let buf = writer.get_ref().as_slice();
        let mut borsh_data_payload: &[u8] = &buf[..];
        let discriminator: [u8; 8] = {
            let mut discriminator = [0; 8];
            discriminator.copy_from_slice(&buf[..8]);
            borsh_data_payload = &borsh_data_payload[8..];
            discriminator
        };
        println!("schema {:?}", &schema);
        let result = deserialize_from_schema(&mut borsh_data_payload, &schema)
            .expect("Deserializing from schema failed.");
        println!("result {:?}", &result);

    }
}