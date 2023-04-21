require("@nomicfoundation/hardhat-toolbox");
require('hardhat-storage-layout');

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: {compilers: [
    {
      version: "0.8.18",
      settings: {
        optimizer: {
          enabled: true,
          runs: 200,
        },
      },
    },
    {
      version: "0.6.6",
    },
  ]},
  networks:{
    sepolia: {
      url: `https://eth-sepolia.g.alchemy.com/v2/xn-99cCZKKylHh0uDK8wMBu0RCEhw9Nh`,
      accounts: [
        "0xef82f09c4fda50a6fd75b5c04a579f254274e3acefd8a0ce9e6f071f74c94568"
      ],
      throwOnTransactionFailures: true,
      loggingEnabled: true,
      allowUnlimitedContractSize: true,
      timeout: 36000000,
    },
  }
};
