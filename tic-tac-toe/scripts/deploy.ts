import { ethers } from "hardhat";

async function main() {
  const ONE_GWEI = 1_000_000_000;
  const lockedAmount = ONE_GWEI;
  const [x_account, o_account] = await ethers.getSigners();
  const game = await ethers.deployContract("TicTacToe", [0, 0, o_account.address], {
    value: lockedAmount,
  });

  await game.waitForDeployment();

  console.log(
    `Game deployed with ${ethers.formatEther(
      lockedAmount
    )}ETH deployed to ${game.target}`
  );
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
