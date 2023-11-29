# Tic Tac Toe

This project is a HardHat TicTacToe game smart contract. Each instance of the contract represents one game between a challenger and a challengee. The contract is deployed by the challenger, along with an initial move. The challengee then matches the bet fronted by the challenger. The players then alternate taking turns until the game is complete or one player takes too long to play. After which, player(s) may withdraw funds according to the outcome of the game.

Note: This project is a POC, so there are a number of core features that haven't been completed yet.

Try running some of the following tasks:

```shell
npx hardhat help
npx hardhat test
REPORT_GAS=true npx hardhat test
npx hardhat node
npx hardhat run scripts/deploy.ts
```
