const Web3 = require("web3");
const { ethers } = require("ethers");
const {Header, Account} = require('eth-object');
const fs = require('fs');

async function fetchAggregatorAddress(){
    const web3 = new Web3("https://eth-mainnet.g.alchemy.com/v2/6c9sdilJ_6JXc2ipSKoD_6gJZ7ZafdWF"); 
    const x = await web3.eth.getStorageAt("0xF4030086522a5bEEa4988F8cA5B36dbC97BeE88c", "2", console.log);
    console.log(x)
}

async function findPriceFeedProof() {
    const web3 = new Web3("https://eth-mainnet.g.alchemy.com/v2/6c9sdilJ_6JXc2ipSKoD_6gJZ7ZafdWF"); 
    const hotVars = await web3.eth.getStorageAt("0x37bC7498f4FF12C19678ee8fE19d713b87F6a9e6", "42");
    const roundID = parseInt(hotVars.substring(2,22), 16)
    console.log(roundID, "currentRoundID")

    const paddedSlot = ethers.utils.hexZeroPad(43, 32);
    const paddedKey = ethers.utils.hexZeroPad(roundID, 32);
    const itemSlot = ethers.utils.keccak256(paddedKey + paddedSlot.slice(2));  

    const transmission = await web3.eth.getStorageAt("0x37bC7498f4FF12C19678ee8fE19d713b87F6a9e6", itemSlot);
    const currentAnswer = parseInt(transmission.substring(50,66), 16)
    console.log(currentAnswer, "currentAnswer")

    const latestBlock = await web3.eth.getBlock("latest")
    const latestBlockNumber = latestBlock.number
    console.log(latestBlockNumber, "latestBlock")

    const web3Proof = await web3.eth.getProof("0x37bC7498f4FF12C19678ee8fE19d713b87F6a9e6", [itemSlot], "0x"+ latestBlockNumber.toString(16))
    web3Proof.nonce = web3.utils.toHex(web3Proof.nonce)
    web3Proof.balance = web3.utils.toHex(web3Proof.balance)
    let account_state = Account.fromRpc(web3Proof).serialize()
    const header_rlp = Header.fromRpc(latestBlock).serialize();

    const proof = {
        header_data : header_rlp.toString('hex'),
        account_proof : web3Proof.accountProof,
        account_state: account_state.toString('hex'),
        storage_proof: web3Proof.storageProof[0].proof,
        storage_key_hash: ethers.utils.keccak256(itemSlot),
        value: web3Proof.storageProof[0].value,
        eth_height: latestBlockNumber
    }
    fs.writeFile('../FEE-DATA-PROOF.json',JSON.stringify(proof, null, null), (err) => {
        if (err) throw err;
}   )
    
    console.log(proof, "proof")
}
findPriceFeedProof()
/*
AGGREGATOR ADDRESSES ETH MAINNET:
ETH/USD : 0x37bC7498f4FF12C19678ee8fE19d713b87F6a9e6
BTC/USD : 0xAe74faA92cB67A95ebCAB07358bC222e33A34dA7
*/