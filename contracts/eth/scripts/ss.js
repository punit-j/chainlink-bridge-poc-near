const hre = require("hardhat");

async function main() {
  console.log(await
  hre.storageLayout.export());
}
main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
  });
  