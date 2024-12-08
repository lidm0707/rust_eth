
# start mock block chain

1. npm install -g solc
2. npx solc --bin --abi --optimize -p -o src/contract_build ./src/contract/SwapPool.sol
3. npm i -g ganache-cli

> if eip1599 not actived 
>  npm uninstall -g ganache-cli
> npm install -g ganache

## Note

- [x] I finish deploy
- [x] created coin
- [ ] swap and pair coin
- [ ] process on auto test




Chain
==================
Hardfork: shanghai
Id:       1337

use git bash find path => where ganache-cli
