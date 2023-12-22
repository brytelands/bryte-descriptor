use crate::bryte_desciptor_hash;

pub fn custom_discriminator(state_type:String, state_name: String) -> [u8; 8] {
    let discriminator_preimage = format!("{state_type}:{state_name}");
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(
        &bryte_desciptor_hash::hash(discriminator_preimage.as_bytes()).to_bytes()[..8],
    );
    discriminator
}

pub fn custom_discriminator_as_string(state_type:String, state_name: String) -> String {
    let discriminator_preimage = format!("{state_type}:{state_name}");
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(
        &bryte_desciptor_hash::hash(discriminator_preimage.as_bytes()).to_bytes()[..8],
    );
    format!("{discriminator:?}").parse().unwrap()
}
