use hex::FromHex;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};


pub fn keccak256(bytes: &[u8]) -> [u8; 32] {
    use tiny_keccak::{Hasher, Keccak};
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(bytes);
    hasher.finalize(&mut output);
    output
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Hex(pub Vec<u8>);

impl<'de> Deserialize<'de> for Hex {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut s = <String as Deserialize>::deserialize(deserializer)?;
        if s.starts_with("0x") {
            s = s[2..].to_string();
        }
        let result = Vec::from_hex(&s).map_err(|err| serde::de::Error::custom(err.to_string()))?;
        Ok(Hex(result))
    }
}


impl Serialize for Hex {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&hex::encode(&self.0))
    }
}

#[derive(Serialize, Deserialize)]
pub struct DataProofJson {
    pub header_data: Hex,
    pub account_proof: Vec<Hex>,
    pub account_state: Hex,
    pub storage_proof: Vec<Hex>,
    pub storage_key_hash: Hex,
    pub value: Hex,
    pub eth_height: u64
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct DataProof{
   pub header_data: Hex,
   pub account_proof: Vec<Hex>,
   pub account_state: Hex,
   pub storage_proof: Vec<Hex>,
   pub storage_key_hash: Hex,
   pub value: Hex,
   pub eth_height: u64
}

// #[derive(
//     BorshDeserialize, BorshSerialize, Debug, Clone, Serialize, Deserialize, PartialEq,
// )]
// pub struct DataProof{
//     pub header_data: Vec<u8>,
//     pub account_proof: Vec<Vec<u8>>, // account proof
//     pub account_state: Vec<u8>,      // rlp encoded account state
//     pub storage_proof: Vec<Vec<u8>>, // storage proof
//     pub storage_key_hash: Vec<u8>,   // keccak256 of storage key
//     pub value: Vec<u8>,      // storage value
//     pub eth_height: u64,
// }
