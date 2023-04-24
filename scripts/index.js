const Web3 = require("web3");

async function fetchAggregatorAddress(){
    const web3 = new Web3("https://eth-mainnet.g.alchemy.com/v2/6c9sdilJ_6JXc2ipSKoD_6gJZ7ZafdWF"); 
    const x = await web3.eth.getStorageAt("0xF4030086522a5bEEa4988F8cA5B36dbC97BeE88c", "2", console.log);
    console.log(x)
}

async function findPriceFeedProof() {
    const web3 = new Web3("https://eth-mainnet.g.alchemy.com/v2/6c9sdilJ_6JXc2ipSKoD_6gJZ7ZafdWF"); 
    const web3Proof = await web3.eth.getProof("0x37bC7498f4FF12C19678ee8fE19d713b87F6a9e6", "", "17095182")
}

/*
AGGREGATOR ADDRESSES ETH MAINNET:
ETH/USD : 0x37bC7498f4FF12C19678ee8fE19d713b87F6a9e6
BTC/USD : 0xAe74faA92cB67A95ebCAB07358bC222e33A34dA7
*/