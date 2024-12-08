
# setup
1. install node
2. npm i -g ganache-cli


# stard
1. cargo run --bin block
> run mock chain in http://127.0.0.1:8545
2. cargo run --bin t-derc20
> deploy coin on chain
3. cargo run --bin t-depositMOO
> deposit eth to moo coin for mint

## Note
- [x] I finish deploy
- [x] created coin
- [ ] swap and pair coin
- [ ] process on auto test


## Chain
| Hardfork | Id |
| ----------- | ----------- | 
| shanghai | 1337 |
     

use git bash find path => where ganache-cli


## something broken
if eip1599 not actived 
```
 npm uninstall -g ganache-cli
 npm install -g ganache
```

## build contract
1. npm install -g solc
2. npx solc --bin --abi --optimize -p -o src/contract_build ./src/contract/SwapPool.sol