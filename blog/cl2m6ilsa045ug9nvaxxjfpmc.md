# How to Safely convert BigNumbers with ethers

## The Problem?
Big numbers are an everyday use for web3 developers on EVM based blockchains. Consider a smart contract like the below:


```sol
pragma solidity ^0.8.13;

contract Sample {
    uint256 public constant number = 5000 ether;
}
``` 

If we were to interact with the contract above via ethers, we would write something like:

```js
import { ethers } from "ethers";

const address =  "...";
const abi = [...];

const someAction = (bigNumber) => {
    console.log(bigNumber.toNumber()); // throws an error
}

const errorAction = () => {}

const contract = new ethers.Contract(address, abi);
contract.number().then(someAction).catch(errorAction);
``` 

From ethersjs docs, BigNumbers have a 'toNumber' property which ought to convert the BigNumber to a normal number. However, this has it's limits. It works for big number values less than 1 ether but for anything more, problems arise due to [JavaScript Safe integer](https://docs.ethers.io/v5/api/utils/bignumber/#BigNumber--notes-safenumbers).

## The fix?
I created a gist [right here](https://gist.github.com/elcharitas/c593f3ba4bc6553c3adbfdf54d0dde62) Feel free to drop comments on it
```js
import { utils } from "ethers";

export function convert(value, unit = "ether") {
    if (value instanceof ethers.BigNumber) {
        return utils.formatUnits(value.toString(), unit);
    } else {
        return ethers.BigNumber.from(value);
    }
}
``` 

The code above simply allows us to convert from normal numbers to big numbers and viz-a-viz.

I hope this helps someone. Cheers!

