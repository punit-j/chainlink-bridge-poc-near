// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// You can also run a script with `npx hardhat run <script>`. If you do that, Hardhat
// will compile your contracts, add the Hardhat Runtime Environment's members to the
// global scope, and execute the script.
const hre = require("hardhat");

async function main() {
  const [deployer] = await hre.ethers.getSigners();
  console.log("contracts deploying using address: ", deployer.address)
  const CLPF = await hre.ethers.getContractFactory("ChainLinkPriceFetcher");
  // const fetcher = await CLPF.deploy([1,2] , ["0x1b44F3514812d835EB1BDB0acB33d3fA3351Ee43", "0x694AA1769357215DE4FAC081bf1f309aDC325306"], 2);
  const fetcher = await CLPF.attach("0x0eF5e9df7f6db314a170C672D438bc3E8592C128");
  await fetcher.deployed();

  console.log("fetcher deployed at: ", fetcher.address)

  const tx = await fetcher.fetchPrice(1);
  console.log(await tx.wait(), "yess")
  console.log(await fetcher.latestPrice(1), "checking")
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
