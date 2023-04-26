use clap::{Parser, Subcommand};
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
mod data_proof;
use data_proof::{DataProof, DataProofJson};
use std::fs;

#[derive(Subcommand, Debug)]
enum SubCommand {
    EncodeDataProof {
        #[clap(short, long)]
        proof: String,
    },
    DecodeDataProof {
        #[clap(short, long)]
        proof: String,
    }
}

#[derive(Parser, Debug)]
#[clap(version)]
struct Arguments {
    #[command(subcommand)]
    cmd: SubCommand,
}

// #[derive(Debug, Deserialize)]
//     #[serde(crate = "near_sdk::serde")]
//     pub struct JsonDataProof {
//         #[serde(with = "hex::serde")]
//         pub header_data: Vec<u8>,
//         pub account_proof: Vec<String>,
//         #[serde(with = "hex::serde")]
//         pub account_state: Vec<u8>, 
//         pub storage_proof: Vec<String>, 
//         #[serde(with = "hex::serde")]
//         pub storage_key_hash: Vec<u8>,
//         #[serde(with = "hex::serde")]
//         pub value: Vec<u8>,
//         eth_height: u64
//     }



    // pub fn get_json_data_proof(filename: String) -> JsonDataProof{
    //     let contents = std::fs::read_to_string(&filename).expect("Unable to read file");
    //     serde_json::from_str(&contents).expect("Unable to deserialize")
    // }

    // pub fn get_data_proof(file_path: String) -> DataProof{
    //     let json_proof = get_json_data_proof(file_path);
    //     let header_data = json_proof.header_data;
    //     let account_proof = json_proof.account_proof.into_iter().map(|x| hex::decode(x).unwrap()).collect();
    //     let account_state = json_proof.account_state;
    //     let storage_proof = json_proof.storage_proof.into_iter().map(|x| hex::decode(x).unwrap()).collect();
    //     let storage_key_hash = json_proof.storage_key_hash;
        
    //     DataProof{
    //         header_data,
    //         account_proof,
    //         account_state,
    //         storage_proof,
    //         storage_key_hash,
    //         value: json_proof.value,
    //         eth_height: json_proof.eth_height

    //     }
    // }



fn main() {
    let args = Arguments::parse();

    // let proof = get_data_proof(String::from("./src/data/data.json"));
    // let proof_json = get_json_data_proof(String::from("./src/data/data.json"));
    // println!("\n\n\n{:?}", proof_json);
    // let encoded_data_proof = near_sdk::base64::encode(proof_json.try_to_vec().unwrap());

    // println!("^^^^^^^^^^^^^^^^^Encoded Proof is:  \n\n{:?}", serde_json::to_string(&encoded_data_proof).unwrap());

    // let data_proof = DataProof::try_from_slice(&encoded_data_proof.0).unwrap_or_else(|_| env::panic_str("Invalid borsh format of the `DataProof`"));
    // let decoded_base64 = near_sdk::base64::decode(encoded_data_proof).expect("Invalid base64 proof");
    // let actual_proof = DataProof::try_from_slice(&decoded_base64).expect("Invalid borsh format of the `Data Proof`");
    // println!("^^^^^^^^^^^^^^^^Decoded Proof is: \n\n{:?}", serde_json::to_string(&actual_proof).unwrap());
    match args.cmd {
        
        SubCommand::EncodeDataProof { proof: _ } => {
            let file_path = "/Users/9to5mac/Desktop/code/aurora/rainbow-bridge-oracle-connector/FEE-DATA-PROOF.json";
            println!("In file {}", file_path);

            let proof = fs::read_to_string(file_path)
                .expect("Should have been able to read the file");
            let json_proof: DataProofJson =
                serde_json::from_str(&proof).expect("Invalid json format of the `JsonProof`");
            let data_proof = DataProof {
                header_data: json_proof.header_data,
                account_proof: json_proof.account_proof,
                account_state: json_proof.account_state,
                storage_proof: json_proof.storage_proof,
                storage_key_hash: json_proof.storage_key_hash,
                value: json_proof.value,
                eth_height: json_proof.eth_height
            };
            
            let encoded_data_proof = near_sdk::base64::encode(data_proof.try_to_vec().unwrap());

            println!(
                "Encoded data proof:\n{}",
                serde_json::to_string(&encoded_data_proof).unwrap()
            );
        }
        SubCommand::DecodeDataProof { proof } => {
            let decoded_base64 = near_sdk::base64::decode(proof).expect("Invalid base64 proof");
            let data_proof = DataProof::try_from_slice(&decoded_base64)
                .expect("Invalid borsh format of the `data_proof`");

            println!(
                "Decoded proof:\n{}",
                serde_json::to_string(&data_proof).unwrap()
            );
        }
    }
}
