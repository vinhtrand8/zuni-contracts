use anchor_lang::prelude::*;

#[account]
pub struct DidDocument {
    pub controller: Pubkey,
    pub did: String,
}

#[account]
pub struct VerificationMethod {
    pub controller: Pubkey,
    pub did: String,
    pub key_id: String,
    pub r#type: String,
    pub public_key_multibase: String,
}

#[account]
pub struct Authentication {
    pub discriminator: String,
    pub did: String,
    pub key_id: String,
}

impl Authentication {
    pub const DISCRIMINATOR: &'static str = "authentication";
}

#[account]
pub struct Assertion {
    pub discriminator: String,
    pub did: String,
    pub key_id: String,
}

impl Assertion {
    pub const DISCRIMINATOR: &'static str = "assertion";
}

#[account]
pub struct KeyAgreement {
    pub discriminator: String,
    pub did: String,
    pub key_id: String,
}

impl KeyAgreement {
    pub const DISCRIMINATOR: &'static str = "key_agreement";
}
