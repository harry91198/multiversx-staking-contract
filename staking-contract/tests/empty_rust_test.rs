use multiversx_sc::{codec::multi_types::OptionalValue, types::Address};
use multiversx_sc_scenario::{
    managed_address, managed_biguint, rust_biguint, whitebox::*, DebugApi,
};
use staking_contract::*;

const WASM_PATH: &'static str = "output/staking-contract.wasm";
const OWNER_BALANCE: u128 = 1_000_000_000_000_000_000_000_000;
const CONTRACT_BALANCE: u64 = 1_000_000_000_000;
const USERA_BALANCE: u64 = 1_000_000_000_000;
const USERB_BALANCE: u64 = 2_000_000_000_000;
const USERC_BALANCE: u64 = 3_000_000_000_000;

struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> staking_contract::ContractObj<DebugApi>,
{
    pub b_mock: BlockchainStateWrapper,
    pub owner_address: Address,
    pub usera_address: Address,
    pub userb_address: Address,
    pub userc_address: Address,
    pub contract_wrapper:
        ContractObjWrapper<staking_contract::ContractObj<DebugApi>, ContractObjBuilder>,
}

impl<ContractObjBuilder> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> staking_contract::ContractObj<DebugApi>,
{
    pub fn new(sc_builder: ContractObjBuilder) -> Self {
        let rust_zero = rust_biguint!(0u64);
        let mut b_mock = BlockchainStateWrapper::new();
        let owner_address = b_mock.create_user_account(&rust_biguint!(OWNER_BALANCE));
        let usera_address = b_mock.create_user_account(&rust_biguint!(USERA_BALANCE));
        let userb_address = b_mock.create_user_account(&rust_biguint!(USERB_BALANCE));
        let userc_address = b_mock.create_user_account(&rust_biguint!(USERC_BALANCE));
        let sc_wrapper =
            b_mock.create_sc_account(&rust_zero, Some(&owner_address), sc_builder, WASM_PATH);

        // simulate deploy
        b_mock
            .execute_tx(&owner_address, &sc_wrapper, &rust_zero, |sc| {
                sc.init();
            })
            .assert_ok();

        ContractSetup {
            b_mock,
            owner_address,
            usera_address,
            userb_address,
            userc_address,
            contract_wrapper: sc_wrapper,
        }
    }
}

#[test]
fn stake_unstake_test() {
    let mut setup = ContractSetup::new(staking_contract::contract_obj);
    let owner_addr = setup.owner_address.clone();
    let usera_addr = setup.usera_address.clone();
    let userb_addr = setup.userb_address.clone();
    let userc_addr = setup.userc_address.clone();




    // checking init info, like balance of all user addresses and contract addresses
    setup
        .b_mock
        .check_egld_balance(&usera_addr, &rust_biguint!(USERA_BALANCE));
    setup
        .b_mock
        .check_egld_balance(&userb_addr, &rust_biguint!(USERB_BALANCE));
    setup
        .b_mock
        .check_egld_balance(&userc_addr, &rust_biguint!(USERC_BALANCE));
    setup
        .b_mock
        .check_egld_balance(setup.contract_wrapper.address_ref(), &rust_biguint!(0));

    







    // funding contract before hand to give out rewards from contract in future, using owners address
    setup
        .b_mock
        .execute_tx(
            &owner_addr,
            &setup.contract_wrapper,
            &rust_biguint!(CONTRACT_BALANCE),
            |sc| {
                sc.fundrewards();

                assert_eq!(
                    sc.contract_reward_balance().get(),
                    managed_biguint!(CONTRACT_BALANCE)
                );
            },
        )
        .assert_ok();


    //
    // checking contract's updated balance
    //
    setup
        .b_mock
        .check_egld_balance(setup.contract_wrapper.address_ref(), &rust_biguint!(CONTRACT_BALANCE));


    //
    // we'll make 5 periods in total to check stake/unstake/rewards functionality
    // setting block timestamp for P0
    //
    setup.b_mock.set_block_timestamp(1676222000);


    //
    // stake for UserA
    //
    setup
        .b_mock
        .execute_tx(
            &usera_addr,
            &setup.contract_wrapper,
            &rust_biguint!(USERA_BALANCE),
            |sc| {
                sc.stake();

                assert_eq!(
                    sc.staking_position(&managed_address!(&usera_addr)).get(),
                    managed_biguint!(USERA_BALANCE)
                );
            },
        )
        .assert_ok();


    //
    //comparing userA's updated balance and contracts updated balance
    //
    setup
        .b_mock
        .check_egld_balance(&usera_addr, &rust_biguint!(0));
    setup.b_mock.check_egld_balance(
        setup.contract_wrapper.address_ref(),
        &rust_biguint!(USERA_BALANCE+CONTRACT_BALANCE),
    );


    //
    // setting block timestamp for P1
    // 
    setup.b_mock.set_block_timestamp(1676222010);



    //
    // stake for UserB
    //
    setup
        .b_mock
        .execute_tx(
            &userb_addr,
            &setup.contract_wrapper,
            &rust_biguint!(USERB_BALANCE),
            |sc| {
                sc.stake();

                assert_eq!(
                    sc.staking_position(&managed_address!(&userb_addr)).get(),
                    managed_biguint!(USERB_BALANCE)
                );
            },
        )
        .assert_ok();


    //
    //  comparing userB's updated balance and contracts updated balance
    //
    setup
        .b_mock
        .check_egld_balance(&userb_addr, &rust_biguint!(0));
    setup.b_mock.check_egld_balance(
        setup.contract_wrapper.address_ref(),
        &rust_biguint!(USERA_BALANCE+USERB_BALANCE+CONTRACT_BALANCE),
    );




    //
    // setting block timestamp for P2
    // 
    setup.b_mock.set_block_timestamp(1676222020);



    //
    // stake for UserC
    //
    setup
        .b_mock
        .execute_tx(
            &userc_addr,
            &setup.contract_wrapper,
            &rust_biguint!(USERC_BALANCE),
            |sc| {
                sc.stake();

                assert_eq!(
                    sc.staking_position(&managed_address!(&userc_addr)).get(),
                    managed_biguint!(USERC_BALANCE)
                );
            },
        )
        .assert_ok();


    //
    //  comparing userC's updated balance and contracts updated balance
    //
    setup
        .b_mock
        .check_egld_balance(&userc_addr, &rust_biguint!(0));
    setup.b_mock.check_egld_balance(
        setup.contract_wrapper.address_ref(),
        &rust_biguint!(USERA_BALANCE+USERB_BALANCE+USERC_BALANCE+CONTRACT_BALANCE),
    );


    //
    // setting block timestamp for P3
    // 
    setup.b_mock.set_block_timestamp(1676222030);


    //
    //  userC will unstake after at P3 (after 10 seconds from its staking)
    //
    setup
        .b_mock
        .execute_tx(
            &userc_addr,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.unstake(OptionalValue::None);

                assert_eq!(
                    sc.staking_position(&managed_address!(&userc_addr)).get(),
                    managed_biguint!(0)
                );
            },
        )
        .assert_ok();


    //
    // checking contract and userC address balance just after unstake
    //
    setup
        .b_mock
        .check_egld_balance(&userc_addr, &rust_biguint!(USERC_BALANCE));
    setup
        .b_mock
        .check_egld_balance(setup.contract_wrapper.address_ref(), &rust_biguint!(USERA_BALANCE + USERB_BALANCE + CONTRACT_BALANCE));





    //
    //claiming rewards for userC
    //
    setup
        .b_mock
        .execute_tx(
            &userc_addr,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.claim_rewards();

                assert_eq!(
                    sc.contract_reward_balance().get(),//999999500001
                    managed_biguint!(CONTRACT_BALANCE-499999) //1000000000000
                );
            },
        )
        .assert_ok();


    //
    // checking contract's & userC's balance after claim_rewards
    //
    setup
        .b_mock
        .check_egld_balance(&userc_addr, &rust_biguint!(USERC_BALANCE+499999));
    setup
        .b_mock
        .check_egld_balance(setup.contract_wrapper.address_ref(), &rust_biguint!( USERA_BALANCE+ USERB_BALANCE+ CONTRACT_BALANCE-499999));







    //
    // setting block timestamp for P4
    // 
    setup.b_mock.set_block_timestamp(1676222040);




    //
    //  userB will unstake after at P4 (after 30 seconds from its staking)
    //
    setup
        .b_mock
        .execute_tx(
            &userb_addr,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.unstake(OptionalValue::None);

                assert_eq!(
                    sc.staking_position(&managed_address!(&userb_addr)).get(),
                    managed_biguint!(0)
                );
            },
        )
        .assert_ok();

    //
    // checking contract and userB address balance just after unstake
    //
    setup
        .b_mock
        .check_egld_balance(&userb_addr, &rust_biguint!(USERB_BALANCE));
    setup
        .b_mock
        .check_egld_balance(setup.contract_wrapper.address_ref(), &rust_biguint!(USERA_BALANCE + CONTRACT_BALANCE - 499999));





    
    //
    //claiming rewards for userB
    //
    setup
        .b_mock
        .execute_tx(
            &userb_addr,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.claim_rewards();

                assert_eq!(
                    sc.contract_reward_balance().get(),//999997833335
                    managed_biguint!(CONTRACT_BALANCE-499999-1666666) //999999500001
                );
            },
        )
        .assert_ok();


    //
    // checking contract's & userB's balance after claim_rewards
    //
    setup
        .b_mock
        .check_egld_balance(&userb_addr, &rust_biguint!(USERB_BALANCE+1666666));
    setup
        .b_mock
        .check_egld_balance(setup.contract_wrapper.address_ref(), &rust_biguint!( USERA_BALANCE + CONTRACT_BALANCE-499999-1666666));





    



    //
    // setting block timestamp for P5
    // 
    setup.b_mock.set_block_timestamp(1676222050);




    //
    //  userA will unstake after at P5 (after 50 seconds from its staking)
    //
    setup
        .b_mock
        .execute_tx(
            &usera_addr,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.unstake(OptionalValue::None);

                assert_eq!(
                    sc.staking_position(&managed_address!(&usera_addr)).get(),
                    managed_biguint!(0)
                );
            },
        )
        .assert_ok();

    //
    // checking contract and userB address balance just after unstake
    //
    setup
        .b_mock
        .check_egld_balance(&usera_addr, &rust_biguint!(USERA_BALANCE));
    setup
        .b_mock
        .check_egld_balance(setup.contract_wrapper.address_ref(), &rust_biguint!(CONTRACT_BALANCE - 499999 - 1666666));





    
    //
    //claiming rewards for userA
    //
    setup
        .b_mock
        .execute_tx(
            &usera_addr,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.claim_rewards();

                assert_eq!(
                    sc.contract_reward_balance().get(),//999995000002
                    managed_biguint!(CONTRACT_BALANCE-499999-1666666-2833333) //999997833335
                );
            },
        )
        .assert_ok();


    //
    // checking contract's & userA's balance after claim_rewards
    //
    setup
        .b_mock
        .check_egld_balance(&usera_addr, &rust_biguint!(USERA_BALANCE+2833333));
    setup
        .b_mock
        .check_egld_balance(setup.contract_wrapper.address_ref(), &rust_biguint!( CONTRACT_BALANCE-499999-1666666-2833333));


}
