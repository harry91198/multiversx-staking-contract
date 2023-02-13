# multiversx-staking-contract
This is a Staking smart contract developed using RUST language and configured for [MultiversX chain(ELROND)](https://docs.multiversx.com/welcome/welcome-to-multiversx). [mxpy](https://docs.multiversx.com/sdk-and-tools/sdk-py/) is used for installation, compilation, building, testing, deploying.

## Getting Started

Mxpy and Rust should already be installed.
VSCode is used with extensions
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [MultiversX VSCode ext](https://marketplace.visualstudio.com/items?itemName=MultiversX.vscode-elrond-ide)

add wallet pem file in ./staking-contract

## Smart Contract
Staking smart contract is deployed here [erd1qqqqqqqqqqqqqpgqtz5me5t0tqze8ckpre6u34y0nc7dyg7ahfwsqpyuh0](https://devnet-explorer.multiversx.com/accounts/erd1qqqqqqqqqqqqqpgqtz5me5t0tqze8ckpre6u34y0nc7dyg7ahfwsqpyuh0)


Requirement: 
```bash
- Users can deposit their EGLD tokens & become a staker.
- Rewards are distributed ona global speed. (100,000EGLD(1e-12)/sec in our contract)
- User earn rewards in proportion to their stake.
- Users can withdraw and claim their rewards at any time.
```

Approach: 
```bash
- Users each staked amount is stored
- Total rewards and userRewardsPaid is calculated
- The difference of the two will give total rewards for the user
- reward amount is updatded every time some one stakes/unstakes/claimsReward
```

Documentation: 
```bash
- fundrewards: Owner or any other address will transfer funds in smart contract using this that will act as reward and will be kept in smart contract.
- stake: accepts EGLD and maps amount of EGLD received with the function caller address(user).
- unstake: returns back the staked EGLD amount by user, It has optional parameter, ie if we pass `unstake_amount` it will unstake only that much amount, otherwise in case of no amount passed, will unstake all of the staked amount by the caller address.
- claim_rewards: transfers the calculated EGLD reward for the users who have staked.

-staked_addresses: returns all addresses participating in staking
-staking_position: returns staked amount by the user, whose contract address is passed as input
-contract_reward_balance: tracks & returns the contracts reward balance(extra fund which was added, which will be used when rewards gets distributed)
-updated_at: returns the last block timestamp, used for rewards calculation
-reward_stored: returns total rewards stored till now
-user_reward_paid: returns total rewards paid for a user
-rewards: tracks the current rewards amount for a user
-total_supply: returns total staked amount in the contract by all users
-update_reward: function the calculate and update the rewards. Called everytime a new stake/unstake happens
-reward: calculates and returns rewards stored till last timestamp and the new awards added after then
-earned: calculates and returns rewards earned by user from the last time
```


## Function calls

We can interact with deployed smart contracts using snippets.sh file (./staking-contract/interactions/snippets.sh)
It contains all functions to interact with the contract, whether it be the write endpoints or view functions.
- deploy() - deploys conntract on devnet multiversx chain
- upgrade() - upgrades the contract if some chnges are made after deploymen
- stake() - stakes the amount by calling stake function and transfer EGLD
- unstake() - unstakes the already staked amount by user
- claimRewards() - claims the rewards generated for the user
- fundrewards() - transfers some EGLD into contract so that contract can distribute rewards out of them





## Staking Flowchart & Testing

This, exactly, has been implemented in testing,
Mandos can be used for testing, this project uses Rust testing framework, more details provided in [multiversx docs](https://docs.multiversx.com/developers/developer-reference/rust-testing-framework)
Testing file can be found in ./staking-contract/tests/empty_rust_test.rs


Steps in test file are as follows:
- creates mock chain, user accounts and owner accounts
- deploys smart contracts and calls init()
- checks init info, like balance of all user addresses and contract addresses
- funds contract before hand to give out rewards from contract in future, using owners address
- checks contract's updated balance
- set the block timestamp for P0 (we are making 5 periods in total to check stake/unstake/rewards functionality)
- stakes for UserA
- compares userA's updated balance and contracts updated balance
- set the block timestamp for P1
- stakes for UserB
- compares userB's updated balance and contracts updated balance
- set the block timestamp for P2
- stakes for UserC
- compares userC's updated balance and contracts updated balance
- set the block timestamp for P3
- userC unstakes after at P3 (after 10 seconds from its staking)
- checks contract and userC address balance just after unstake
- claiming rewards for userC
- checking contract's & userC's balance after claim_rewards
- set the block timestamp for P4
- userB unstakes after at P4 (after 30 seconds from its staking)
- checks contract and userB address balance just after unstake
- claiming rewards for userB
- checking contract's & userB's balance after claim_rewards
- set the block timestamp for P5
- userA unstakes after at P5 (after 50 seconds from its staking)
- checks contract and userB address balance just after unstake
- claiming rewards for userA
- checking contract's & userA's balance after claim_rewards



![Untitled drawio](https://user-images.githubusercontent.com/35892549/218341985-4c8b1676-7942-4038-8c91-57155fae5729.png)




Click on this link for full staking diagram

Image Draw.io - https://viewer.diagrams.net/?tags=%7B%7D&highlight=0000ff&edit=_blank&layers=1&nav=1&title=Untitled.drawio#R7V1bk5s4Fv4t%2B9BVM6lyFxIXw2PanWQekp3eyaZ2Zl%2B2sJFtNhi8QMfd8zC%2FfSWQMEgC4zY3Y1KVahDi4nP5dL6j25262L18Cu399kvgIO8OKs7Lnfp4ByEwAcB%2FSMlrWgIVTUlLNqHr0FrHgq%2Fun4gWsmrProOiQsU4CLzY3RcLV4Hvo1VcKLPDMDgUq60Dr%2FjWvb1BQsHXle2Jpf9ynXiblppwfiz%2FBbmbLXszMKz0ys5mlekviba2ExxyReqHO3URBkGcHu1eFsgj0mNySe%2F7WHI1%2B7AQ%2BXGdGz5%2B%2FmOzPfzjydsDUwHLHfj3759mmpE%2B5oftPdNfvH72nSiVaBzaWKTQ8PALHpYhPtqQI%2FzX3u3xgb%2BMyJ%2BDi38qVEJ0sENya%2FJr41cmQuRgidLTIIy3wSbwbe%2FDsfQhDPA7EflOBZ8d63wOAvz8R4AL%2F4vi%2BJWah%2F0cB7hoG%2B88ehWLIHz9nd6fnPyRP3kk9gjA8ZxYo6Gz0ycUujsUo5Dek%2F4C8tmloqZFUfAcrlCFfJnJ2uEGxRX1oKJmJoGdCQX4e8LXRKieHbs%2Fil9iU6PeZPWyW58CF38jVKgHqvN7Pb3nlRmoXnxG%2Bmn0tqP5vA9D%2BzVXbU8qROUvAmbxPfT0Y1l15hVvqw7UQn18kH4vdzf7%2BGC9jlDM3ZET87Eo8Zlz%2FGfer7nf6%2BUGXzR35QasHRStRFc6MfUZ4IC3%2BqvOrD4v1C5aemNWbAqtgIP2XvBK2gER%2F6OdHcaFFkLiAp%2FtJY4FCmZre%2B7Gx8crbFjE%2FB5%2BoDB2cVv7nl7YuY6TegiK3D%2FtZfI8YqNUJfjh%2BsOd%2FphZLXkAeilYCw0E6M3H5jdvzxWuLNokfbxyr4C5lj7rrWbaPh4BQZG%2FHnwsa15DOBjZk8Pnnfd%2BFQd5ZSSKewoiN3YDopRlEMfBTqKtmKBVXr%2FBc%2By5PlpkoZhSpSgBYEpFD6gPKIdjBKbSom0u%2BDKUck0UBH2uVA0xSgLvAALww6fPj9fUADSI9WpNrNdL3Kq2D12kOVVQ3LcIhe%2Bv2x9Uo9hEGH27Ry9BUIO2rNW0ZQP2acuG2ErDahS6ujbYONUGA5O9gyUUht4ia1IEerhuBNLMgSGQdeUIBM26EHQpc7pIziwQy1mzOjYIsiotf4Z5gGUVjB8MHYKg2HAQDFpcNwYZ6rAwCPQTBpWkPo%2B8oCwX9OLGv%2BeOc3fhs%2BNN5KQF%2BqDDmoCn9wp4YC4yCJKgUSK0GgfegdMxl8WirGsBPGZcOaWxLBZ%2BtE3EtbXDiNqd8b9n0gfz8ByvZ%2BbxNNfbkdwZ7W2%2FoG9WcRV4BCrfE5Fslj9hweOvVtifn5NHKOvAj2dre%2Bd6r2nVX5D3AxEzyF2PEicnVwHcv%2BQvpC8lV%2Fwg3Nle7toPO3Rt%2FBcbkx0%2Fh6SzrLLeyt6XVTlQnCQXNSX1eMXD8IPCGf75K9ffiHcG4X5r%2B%2FSRMC3D1hnPqN2T4sz02TUXg55P36Swn5pciUP8sDV%2BPnuTj9KrGNeT3rzcaw5B6BQ%2FLHsW%2Fi3L7y5%2BHHlmFIfBdzSjLUOh3tJefd8kIDzj9Ah1kKowf%2FBz7ksdtApCm7SHs3jrrr77KKKf5%2Fq4mWTy4evmdFlZL%2Fc5hXprL7BjXjiOG%2B09%2B5VVJw0uPvibu9vjJsX2qR1zlv2U5Xaxl6XmXTR5kvKVGPwSix1bRO4DsW39NJvly6mgJt%2BYfKM73yiDbsf9cakhTzbYjw32ibS80QNIbT138PMpV1Ep2B3%2FlqHeF%2BR7QVrpC6YAK3qcjbagRrvAobVLunSUv6MDfzG9ZRf4ARE71QxpDWbbnO0Ak4qXdxgcDBpkkI6SbxkS72EOxYe8nufuI%2FKew9aN0df0pY%2BH0OZYYAOETxgEYImML6uTp3x8P3CDlK%2BXvNO1Ur7aXUZav5RPTNmOjfKdznHprM9zwCxPpOYTVxtzLHB18WiTXA1MXG3yjRH5xsTVRmeDE1frg6uBa%2BBqcJ4NRO%2BPrbHe64mt1WFrdQdF6Xq%2FbE0fO1ubK3IBH9kaUPkh6YMfFqWLJHsib2MODq4uQG2SvMGJvE2%2BMSLfmMjb6GxwIm99kDd4DeRNlcy%2F6pq6AVEiE3WrZGR1qJvRL3WTTKsbGXWrdjTS0aZBa%2FBcTaTYE1cbcyxwdfFok1xNnbja5Bsj8o2Jq43OBieu1gdXU6%2BBq2mSaXBdczU4cbX6XM2oydWMngdFzkfP1eBJrqazNES2lNPgqZtIsSfqNubQ4OrC0yapmzZRt8k3RuQbE3UbnQ1O1K0P6qZdA3XTe6duuhjkf43t70gQ05GtgU5EpaqwICpgyYaTmoYoK6i3JCsDDlRWml6UFTT7l5U4U2sYsjKUoqxU6TDlbmU1DYy8rYjg6qLSJhmbPjG2yTdG5BsTYxudDU6MrQ%2FGpl8DYzN6Z2w99bXRPjN2XKfPLOufyzrr0rtU7t%2Fdie66lrcwMeqOnUwXVb2gPy659dy9R%2Bbc7iBacXuQk%2FXNDnYTkSyZObZFgk%2F0FSr3isoGtzY0I48lPjQOSdrrOjTEUZ%2Ff%2FGHQ9rnCeYGu9k3b2TTUCYgbAuJ53fnHl46LeBMO63M%2Bd2tU4jBfvxMcFvv%2Bx7ZfxLxk7nmGw0AH3JCNC3GYJQ4LDxVgpT1UlixONhRU1rltaXS2QFdvmNzP6nwjxuS6u1Q0sr3fuZis8RsC0M8tw2S%2BfheYzPZNEXYSw2UjguXq9QWVexywNYPDM47fZAthdIDEkl01BoLEGjCLnmDofcfH5pSoaBaLzdqbrQ4jUWENL1FhiomKvzSyuc%2FCsuShw9UhsVmdqJgp9%2BacjWRoaFAze7SmFh%2BrW10hM3ONnF4Xnu3u%2FvNbyQ7lvecv5r3nL0yRV%2BRl1rvIBHLB2G1vApvIRcMN2nWRC2t45MISyQU%2BXZhYtXBBFDyOJu3k4uUW3V%2B%2B6aRPthACeyy%2Fl1yLTZpINgYFzwLjMHtuz4BkfGbW%2FDc0tkjou1YSgZAlWUlXu8JaeckOi1v0Ym%2FIRooP%2Bxyi0tIMZOFp3a3dF%2BTQ5iI997xF%2Bun4fep6vYarFS5PBzjkrjjG0tCNZrQvrPaq1VzrVVVaU78Y20%2Fq70j9qgrrLvbbogGIAe1kAB0ZgAatARiA2H89GUBrBlCMjHSm6h7VL%2FZ%2BTurvyP%2BN%2FtVvit6Pf8NH%2FF95d5cumk8PMlUdrYIONUysgisErLZUvUV%2Bf0J12Bac4EDrCnpbOdCU6m1uLZOBt5vQdlysoMJdtq4ozehUVYoRPZxLEB1IdKq1FtKbkvUC8ZPUbAiqv4z2ya%2Bnqn13rnbVUqo8Ks1qxXBtLg3XOtatOPUQVur2jf5rkHZhYbDWYZwa1jRleBqWZKeI9xrNeS%2B4Bd3y3quxSbs9albsb4HNavYmcFnTIKdZ2LtmxSSammr2rZrUbyF20i2F06Ssf69rXYoZsSl6ujwu1nVtALoVk13V0dPZur2NuElXBqjbU0z2%2FDjpVjmsfBxc1%2FoUmU4cxLaXDB10TmqrQnMsueQHMTqtuuOE0V%2Bf42S6flru2OH3X8mM2Tjp%2Bb9XdEHPgR%2BzlBNsRlMzjrEA2cxKmZ4Ak0MLihL7XCsVpd6iouAQFGWdpyjjFhWl6tIdOLtVFWRPnnyqQlWaLu0%2Bk6nKUu7baqhwaDE1VCd1pes18c9sU1MiTa%2FU1Kg1YqjKEIBOZNu3qxLZ%2FmFd66N8WTUigDtZTzC34o5VsmQJG4KnrDGHgcpzhEJSPXL9FVkC6ymjZfjL03eVLGVCVoXh6FWBOdGVbvI0ixbVH7EpM6Mi42vBHoS%2BRGnQCCUG0R5klu9p1bY9gMke%2BB7Ikti0W4soXyq%2FBYt4kKTmJgvJDZ83B2ghYuamRQtZ5C0C3rxFCH0qqjkAixBTRF21IpNFqNwsD00D%2FVsEEBMc7bYik0XkWw2LwwjZ2Ihu7UFMonSFEOrN24PKWmyWrFHlibVuLULMDAgaucGZpMh33odhkpXIVnQlUmlmbmndxcRgaguiRV26XpRWXDFKN00%2BHZj%2BCHrf0cjEpcTkKyVkz0l%2Fo%2FCccye8il%2BsVH%2BWAarqtzPjFQIxrQPYOAXpaIWrnPJKMaM836bcmxa%2F4GRDyzjMixCa7SjX%2FpxXyHC5h1hqajs1rRhLDaPt7C9rp928RQijaaRdX93agyxnx%2BkjOrg7z04EHOG2kUmFCKmsH8SqRnHPs%2FcRct4qRq79nkPp4lwSOfJr%2F0rQ%2BzfSvvgbD537vqxnvjA7n%2B%2BPsT1smL4dowdibFE7qH95li3bG0GYoVfslhZGy8H8GFbJ1dwMg%2FPvrX73X2IRLC7eUQ9%2BzhqwV9PkawQoRrXNc%2Fl%2BCXBoktUhAL%2FARoPIIY4Z%2Bifpn8VFJUtqiLjeZNBICFp0nMZZt8l4E8y%2FSYWgqMKZDEQsCYa0NsUTMqy%2FJvDX9GKUbgDRFZqD%2FjpvyxZKKDger7UugB%2FKEiMNAb9xAXBX3Zt8i2QclHDPMGEblmQ76sM2kMC21Z7Li3T%2FxlH7hAYHiNoyCjdw1Da4xdx0pc2A3VBOvw2YEqVZHM%2FqBLQvHzVRCtr6WdFybuXWQUNu9ZYFNSBXlUBue4EyFOnYrUNutQaHB7mqbJTC5ZBbLcdLMRdyY0izMaWn1pR5G%2BjWeZ0uUxvf%2FdIF6GpipzLzyVBc4yiFv9jdoUd3vUZYAav0h7PaTyj8ilaiSbS9VlGlYbW0kFGVh5S7NJdCA%2Ba9bDUcmXlosL1h%2F2y9gJwZhBSSee1HybDzh9dvEdm38WNhKHpiCUnJ6fWxz54SekqrdIpo1UTQ8umj7WmcT5oCScZUk6i7tY0es7fllP0ZbZA%2F5vb3vET6TBWVZDTT0OLTMCCx6RHGsd1vvwQO6Uj%2B8H8%3D
