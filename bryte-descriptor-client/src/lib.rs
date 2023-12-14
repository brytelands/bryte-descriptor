use std::str::FromStr;
use base64::Engine;
use base64::engine::general_purpose;
use borsh::BorshDeserialize;
use borsh::schema::BorshSchemaContainer;
use borsh_serde_adapter::deserialize_adapter::deserialize_from_schema;
use log::trace;
use serde_json::Value;
use solana_account_decoder::UiAccountEncoding;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_rpc_client_api::config::RpcAccountInfoConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::hash::{Hash, hash};
use solana_sdk::pubkey::Pubkey;
use anyhow::Result as Result;

pub async fn get_account_data(account_key: String, program_id: String, rpc_url: String) -> Result<Value> {
    let commitment_config = CommitmentConfig::finalized();
    let client = RpcClient::new_with_commitment(rpc_url, commitment_config);

    let account_config = RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::Base64),
        data_slice: None,
        commitment: None,
        min_context_slot: None,
    };

    let account = Pubkey::from_str(&account_key).unwrap();
    let account_data = client.get_account_with_config(&account, account_config.clone()).await.unwrap();

    trace!("{:?}", account_data);
    let data_bytes = account_data.value.unwrap().data;//general_purpose::STANDARD.decode(account_data.value.unwrap().data.as_slice()).unwrap();
    trace!("{:?}", &data_bytes);

    let mut borsh_data_payload: &[u8] = &data_bytes[..];
    let discriminator: [u8; 8] = {
        let mut discriminator = [0; 8];
        discriminator.copy_from_slice(&data_bytes[..8]);
        borsh_data_payload = &borsh_data_payload[8..];
        discriminator
    };

    trace!("{:?}", &borsh_data_payload);

    let program_key = Pubkey::from_str(&program_id).unwrap();
    let (descriptor_key, _) = Pubkey::find_program_address(&[&discriminator], &program_key);
    let mut account_data_descriptor = client.get_account_data(&descriptor_key).await.unwrap();
    trace!("descriptor: {:?}", &account_data_descriptor);
    let mut to_split = &account_data_descriptor[..];//base64::decode(account_data_descriptor.value.unwrap().data.as_slice()).unwrap();
   //TODO CHANGE SCHEMA FROM VEC to ['u8'] todo 12 for anchor 4.. for non anchor....
    let mut d_schema = &to_split[12..];
    trace!("d_schema: {:?}", &d_schema);

    let schema = BorshSchemaContainer::deserialize_reader(&mut d_schema).expect("Deserializing BorshSchemaContainer failed.");

    trace!("schema {:?}", &schema);
    let result = deserialize_from_schema(&mut borsh_data_payload, &schema)
        .expect("Deserializing from schema failed.");
    trace!("result {:?}", result);
    Ok(result)
}

pub async fn get_discriminator(account_key: String, rpc_url: String) -> Result<String> {
    let commitment_config = CommitmentConfig::finalized();
    let client = RpcClient::new_with_commitment(rpc_url, commitment_config);

    let account_config = RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::Base64),
        data_slice: None,
        commitment: None,
        min_context_slot: None,
    };

    let account = Pubkey::from_str(&account_key).unwrap();
    let account_data = client.get_account_with_config(&account, account_config.clone()).await.unwrap();

    trace!("{:?}", account_data);
    let data_bytes = account_data.value.unwrap().data;//general_purpose::STANDARD.decode(account_data.value.unwrap().data.as_slice()).unwrap();
    trace!("{:?}", &data_bytes);

    let mut borsh_data_payload: &[u8] = &data_bytes[..];
    let discriminator: [u8; 8] = {
        let mut discriminator = [0; 8];
        discriminator.copy_from_slice(&data_bytes[..8]);
        borsh_data_payload = &borsh_data_payload[8..];
        discriminator
    };

    let disc: String = format!("{discriminator:?}").parse().unwrap();
    Ok(disc)
}

pub async fn get_discriminator_offline(account_name: String, account_type: String) -> Result<String> {
    let discriminator_preimage = format!("{account_type}:{account_name}");
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(
        &bryte_descriptor_state::bryte_desciptor_hash::hash(discriminator_preimage.as_bytes()).to_bytes()[..8],
    );
    let disc: String = format!("{discriminator:?}").parse().unwrap();
    Ok(disc)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[tokio::test]
    // async fn it_works() {
    //     get_account_data("C5rgunkkV3fScayNVthBVyTNMiUeBwR8nNnTJZCw7L49".to_string(), "FSGyoSWjXobTDZo1bdrsnUixhk2S3V6bpRahip9SCBJi".to_string(), "http://localhost:8899".to_string()).await;
    // }

    #[tokio::test]
    async fn test_get_discriminator_offline() {
        let disc = get_discriminator_offline("ExampleAccount".to_string(), "account".to_string()).await.unwrap();
        println!("{}", disc);
        assert_eq!(disc, "[188, 13, 109, 204, 38, 99, 69, 246]".to_string());
    }
}
