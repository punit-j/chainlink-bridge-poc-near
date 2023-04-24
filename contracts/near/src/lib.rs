use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::env::{current_account_id, block_height};
use near_sdk::serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use near_sdk::Promise;
use near_sdk::{
    env, ext_contract, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault, PromiseOrValue,
};

pub type EthAddress = [u8; 20];
pub const TGAS: near_sdk::Gas = near_sdk::Gas::ONE_TERA;
pub const NO_DEPOSIT: u128 = 0;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
   Symbol
}

#[ext_contract(ext_prover)]
pub trait Prover {
    #[result_serializer(borsh)]
    fn verify_log_entry(
        &self,
        #[serializer(borsh)] log_index: u64,
        #[serializer(borsh)] log_entry_data: Vec<u8>,
        #[serializer(borsh)] receipt_index: u64,
        #[serializer(borsh)] receipt_data: Vec<u8>,
        #[serializer(borsh)] header_data: Vec<u8>,
        #[serializer(borsh)] proof: Vec<Vec<u8>>,
        #[serializer(borsh)] skip_bridge_call: bool,
    ) -> bool;

    #[result_serializer(borsh)]
    fn verify_storage_proof(
        &self,
        #[serializer(borsh)] header_data: Vec<u8>,
        #[serializer(borsh)] account_proof: Vec<Vec<u8>>, // account proof
        #[serializer(borsh)] contract_address: Vec<u8>,   // eth address
        #[serializer(borsh)] account_state: Vec<u8>,      // rlp encoded account state
        #[serializer(borsh)] storage_key_hash: Vec<u8>,   // keccak256 of storage key
        #[serializer(borsh)] storage_proof: Vec<Vec<u8>>, // storage proof
        #[serializer(borsh)] value: Vec<u8>,              // storage value
        #[serializer(borsh)] min_header_height: Option<u64>,
        #[serializer(borsh)] max_header_height: Option<u64>,
        #[serializer(borsh)] skip_bridge_call: bool,
    ) -> PromiseOrValue<bool>;
}

#[ext_contract(ext_self)]
trait ChainLinkBridgeInterface {
    fn data_proof_callback(
        &mut self,
        #[callback]
        #[serializer(borsh)]
        verification_success: bool,
        #[serializer(borsh)] symbol: String,
        #[serializer(borsh)] proof: DataProof,
    );
}

#[derive(
    Default, BorshDeserialize, BorshSerialize, Debug, Clone, Serialize, Deserialize, PartialEq,
)]
pub struct DataProof{
    header_data: Vec<u8>,
    account_proof: Vec<Vec<u8>>, // account proof
    account_state: Vec<u8>,      // rlp encoded account state
    storage_proof: Vec<Vec<u8>>, // storage proof
    storage_key_hash: Vec<u8>,   // keccak256 of storage key
    value: eth_types::U256,      // storage value
    eth_height: u64,
}

#[derive(
    Default, BorshDeserialize, BorshSerialize, Debug, Clone, Serialize, Deserialize, PartialEq,
)]
pub struct PriceFeed{
    latest_price: eth_types::U256,
    added_at: u64,
    eth_height: u64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ChainLinkBridge{
    symbol_to_pricefeed_address: LookupMap<String, EthAddress>,
    latest_price: LookupMap<String, PriceFeed>,
    prover_account: AccountId,
    min_block_delay_near: u64,
    min_block_delay_eth: u64,
}

#[near_bindgen]
impl ChainLinkBridge {
    #[init]
    #[private]
    pub fn new(
        prover_account: AccountId,
        min_block_delay_near: u64,
        min_block_delay_eth: u64,
    ) -> Self {
        require!(!env::state_exists(), "Already initialized");
        let contract = Self {
            symbol_to_pricefeed_address: LookupMap::new(StorageKey::Symbol),
            latest_price: LookupMap::new(StorageKey::Symbol),
            prover_account,
            min_block_delay_near,
            min_block_delay_eth
        };
        contract
    }

    pub fn add_feed_data(&self, symbol: String, data_proof: DataProof) -> Promise{
        let feed_address = self.symbol_to_pricefeed_address.get(&symbol).unwrap_or_else(|| {
            panic!("Price Feed not registered for {} symbol", symbol)
        });

        let previous_data = self.latest_price.get(&symbol).unwrap_or_else(|| {
            panic!("Price not registered for {} symbol", symbol)
        });

        require!(block_height() - previous_data.added_at >= self.min_block_delay_near, "Should cross min block delay for near");
        require!(data_proof.eth_height - previous_data.eth_height >= self.min_block_delay_eth, "Should cross min block delay for eth");

        ext_prover::ext(self.prover_account.clone())
            .with_static_gas(tera_gas(50))
            .with_attached_deposit(NO_DEPOSIT)
            .verify_storage_proof(
                data_proof.header_data.clone(),
                data_proof.account_proof.clone(),
                feed_address.to_vec(),
                data_proof.account_state.clone(),
                data_proof.storage_key_hash.clone(),
                data_proof.storage_proof.clone(),
                data_proof.value.try_to_vec().unwrap(),
                Some(data_proof.eth_height),
                None,
                false,
            )
            .then(
                ext_self::ext(current_account_id())
                    .with_static_gas(tera_gas(50))
                    .with_attached_deposit(NO_DEPOSIT)
                    .data_proof_callback(
                        symbol,
                        data_proof
                    ),
            )
    }

    #[private]
    fn data_proof_callback(
        &mut self,
        #[callback]
        #[serializer(borsh)]
        verification_success: bool,
        #[serializer(borsh)] symbol: String,
        #[serializer(borsh)] proof: DataProof,
    ) {
        require!(
            verification_success,
            format!("Verification failed for data proof")
        );

        self.latest_price.insert(&symbol, &PriceFeed { latest_price: proof.value, added_at: block_height(), eth_height: proof.eth_height});
    }

    //adds new price feeds with corresponding chainlink address, eg BTC/USD
    pub fn add_price_feed(&mut self, symbol: String, pricefeed_address: String) {
        self.symbol_to_pricefeed_address.insert(&symbol, &get_eth_address(pricefeed_address));
        self.latest_price.insert(&symbol, &PriceFeed { latest_price: eth_types::U256(0.into()), added_at: 0, eth_height: 0 });
    }
}

pub fn tera_gas(gas: u64) -> near_sdk::Gas {
    TGAS * gas
}

pub fn get_eth_address(address: String) -> EthAddress {
    let data = hex::decode(address)
        .unwrap_or_else(|_| near_sdk::env::panic_str("address should be a valid hex string."));
    require!(data.len() == 20, "address should be 20 bytes long");
    data.try_into().unwrap()
}