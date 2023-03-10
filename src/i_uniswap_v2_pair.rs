pub use i_uniswap_v2_pair::*;
#[allow(
    clippy::too_many_arguments,
    non_camel_case_types,
    clippy::upper_case_acronyms
)]
pub mod i_uniswap_v2_pair {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    //!IUniswapV2Pair was auto-generated with ethers-rs Abigen. More information at: <https://github.com/gakonst/ethers-rs>
    use ::ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ::ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ::ethers::providers::Middleware;
    use std::sync::Arc;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount0\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount1\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Burn\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount0\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount1\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Mint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount0In\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount1In\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount0Out\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount1Out\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Swap\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint112\",\"name\":\"reserve0\",\"type\":\"uint112\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint112\",\"name\":\"reserve1\",\"type\":\"uint112\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Sync\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DOMAIN_SEPARATOR\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"MINIMUM_LIQUIDITY\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"PERMIT_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burn\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amount0\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount1\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReserves\",\"outputs\":[{\"internalType\":\"uint112\",\"name\":\"reserve0\",\"type\":\"uint112\",\"components\":[]},{\"internalType\":\"uint112\",\"name\":\"reserve1\",\"type\":\"uint112\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"blockTimestampLast\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"kLast\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"liquidity\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonces\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"permit\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"price0CumulativeLast\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"price1CumulativeLast\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"skim\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount0Out\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount1Out\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swap\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"sync\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token0\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"token1\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IUNISWAPV2PAIR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IUniswapV2Pair<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for IUniswapV2Pair<M> {
        fn clone(&self) -> Self {
            IUniswapV2Pair(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IUniswapV2Pair<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IUniswapV2Pair<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IUniswapV2Pair))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IUniswapV2Pair<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IUNISWAPV2PAIR_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MINIMUM_LIQUIDITY` (0xba9a7a56) function
        pub fn minimum_liquidity(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([186, 154, 122, 86], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PERMIT_TYPEHASH` (0x30adf81f) function
        pub fn permit_typehash(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([48, 173, 248, 31], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0x89afcb44) function
        pub fn burn(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([137, 175, 203, 68], to)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factory` (0xc45a0155) function
        pub fn factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReserves` (0x0902f1ac) function
        pub fn get_reserves(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128, u32)> {
            self.0
                .method_hash([9, 2, 241, 172], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x485cc955) function
        pub fn initialize(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 92, 201, 85], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `kLast` (0x7464fc3d) function
        pub fn k_last(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([116, 100, 252, 61], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x6a627842) function
        pub fn mint(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([106, 98, 120, 66], to)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(&self) -> ::ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit` (0xd505accf) function
        pub fn permit(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `price0CumulativeLast` (0x5909c0d5) function
        pub fn price_0_cumulative_last(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([89, 9, 192, 213], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `price1CumulativeLast` (0x5a3d5493) function
        pub fn price_1_cumulative_last(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([90, 61, 84, 147], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `skim` (0xbc25cf77) function
        pub fn skim(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 37, 207, 119], to)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0x022c0d9f) function
        pub fn swap(
            &self,
            amount_0_out: ::ethers::core::types::U256,
            amount_1_out: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 44, 13, 159], (amount_0_out, amount_1_out, to, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(&self) -> ::ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sync` (0xfff6cae9) function
        pub fn sync(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 246, 202, 233], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token0` (0x0dfe1681) function
        pub fn token_0(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([13, 254, 22, 129], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token1` (0xd21220a7) function
        pub fn token_1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([210, 18, 32, 167], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, value))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(&self) -> ::ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        ///Gets the contract's `Burn` event
        pub fn burn_filter(&self) -> ::ethers::contract::builders::Event<M, BurnFilter> {
            self.0.event()
        }
        ///Gets the contract's `Mint` event
        pub fn mint_filter(&self) -> ::ethers::contract::builders::Event<M, MintFilter> {
            self.0.event()
        }
        ///Gets the contract's `Swap` event
        pub fn swap_filter(&self) -> ::ethers::contract::builders::Event<M, SwapFilter> {
            self.0.event()
        }
        ///Gets the contract's `Sync` event
        pub fn sync_filter(&self) -> ::ethers::contract::builders::Event<M, SyncFilter> {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(&self) -> ::ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        /// Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract
        pub fn events(&self) -> ::ethers::contract::builders::Event<M, IUniswapV2PairEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IUniswapV2Pair<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "Burn", abi = "Burn(address,uint256,uint256,address)")]
    pub struct BurnFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "Mint", abi = "Mint(address,uint256,uint256)")]
    pub struct MintFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "Swap",
        abi = "Swap(address,uint256,uint256,uint256,uint256,address)"
    )]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub amount_0_in: ::ethers::core::types::U256,
        pub amount_1_in: ::ethers::core::types::U256,
        pub amount_0_out: ::ethers::core::types::U256,
        pub amount_1_out: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "Sync", abi = "Sync(uint112,uint112)")]
    pub struct SyncFilter {
        pub reserve_0: u128,
        pub reserve_1: u128,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum IUniswapV2PairEvents {
        ApprovalFilter(ApprovalFilter),
        BurnFilter(BurnFilter),
        MintFilter(MintFilter),
        SwapFilter(SwapFilter),
        SyncFilter(SyncFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for IUniswapV2PairEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(IUniswapV2PairEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(IUniswapV2PairEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(IUniswapV2PairEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(IUniswapV2PairEvents::SwapFilter(decoded));
            }
            if let Ok(decoded) = SyncFilter::decode_log(log) {
                return Ok(IUniswapV2PairEvents::SyncFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(IUniswapV2PairEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IUniswapV2PairEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IUniswapV2PairEvents::ApprovalFilter(element) => element.fmt(f),
                IUniswapV2PairEvents::BurnFilter(element) => element.fmt(f),
                IUniswapV2PairEvents::MintFilter(element) => element.fmt(f),
                IUniswapV2PairEvents::SwapFilter(element) => element.fmt(f),
                IUniswapV2PairEvents::SyncFilter(element) => element.fmt(f),
                IUniswapV2PairEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `MINIMUM_LIQUIDITY` function with signature `MINIMUM_LIQUIDITY()` and selector `0xba9a7a56`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "MINIMUM_LIQUIDITY", abi = "MINIMUM_LIQUIDITY()")]
    pub struct MinimumLiquidityCall;
    ///Container type for all input parameters for the `PERMIT_TYPEHASH` function with signature `PERMIT_TYPEHASH()` and selector `0x30adf81f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "PERMIT_TYPEHASH", abi = "PERMIT_TYPEHASH()")]
    pub struct PermitTypehashCall;
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `burn` function with signature `burn(address)` and selector `0x89afcb44`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "burn", abi = "burn(address)")]
    pub struct BurnCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `factory` function with signature `factory()` and selector `0xc45a0155`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    ///Container type for all input parameters for the `getReserves` function with signature `getReserves()` and selector `0x0902f1ac`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "getReserves", abi = "getReserves()")]
    pub struct GetReservesCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address)` and selector `0x485cc955`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address,address)")]
    pub struct InitializeCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `kLast` function with signature `kLast()` and selector `0x7464fc3d`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "kLast", abi = "kLast()")]
    pub struct KlastCall;
    ///Container type for all input parameters for the `mint` function with signature `mint(address)` and selector `0x6a627842`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "mint", abi = "mint(address)")]
    pub struct MintCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `price0CumulativeLast` function with signature `price0CumulativeLast()` and selector `0x5909c0d5`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "price0CumulativeLast", abi = "price0CumulativeLast()")]
    pub struct Price0CumulativeLastCall;
    ///Container type for all input parameters for the `price1CumulativeLast` function with signature `price1CumulativeLast()` and selector `0x5a3d5493`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "price1CumulativeLast", abi = "price1CumulativeLast()")]
    pub struct Price1CumulativeLastCall;
    ///Container type for all input parameters for the `skim` function with signature `skim(address)` and selector `0xbc25cf77`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "skim", abi = "skim(address)")]
    pub struct SkimCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `swap` function with signature `swap(uint256,uint256,address,bytes)` and selector `0x022c0d9f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "swap", abi = "swap(uint256,uint256,address,bytes)")]
    pub struct SwapCall {
        pub amount_0_out: ::ethers::core::types::U256,
        pub amount_1_out: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `sync` function with signature `sync()` and selector `0xfff6cae9`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "sync", abi = "sync()")]
    pub struct SyncCall;
    ///Container type for all input parameters for the `token0` function with signature `token0()` and selector `0x0dfe1681`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "token0", abi = "token0()")]
    pub struct Token0Call;
    ///Container type for all input parameters for the `token1` function with signature `token1()` and selector `0xd21220a7`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "token1", abi = "token1()")]
    pub struct Token1Call;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum IUniswapV2PairCalls {
        DomainSeparator(DomainSeparatorCall),
        MinimumLiquidity(MinimumLiquidityCall),
        PermitTypehash(PermitTypehashCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        Decimals(DecimalsCall),
        Factory(FactoryCall),
        GetReserves(GetReservesCall),
        Initialize(InitializeCall),
        Klast(KlastCall),
        Mint(MintCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Permit(PermitCall),
        Price0CumulativeLast(Price0CumulativeLastCall),
        Price1CumulativeLast(Price1CumulativeLastCall),
        Skim(SkimCall),
        Swap(SwapCall),
        Symbol(SymbolCall),
        Sync(SyncCall),
        Token0(Token0Call),
        Token1(Token1Call),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for IUniswapV2PairCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <MinimumLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::MinimumLiquidity(decoded));
            }
            if let Ok(decoded) =
                <PermitTypehashCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::PermitTypehash(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BurnCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::Burn(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::Factory(decoded));
            }
            if let Ok(decoded) =
                <GetReservesCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::GetReserves(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <KlastCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::Klast(decoded));
            }
            if let Ok(decoded) = <MintCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::Nonces(decoded));
            }
            if let Ok(decoded) =
                <PermitCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::Permit(decoded));
            }
            if let Ok(decoded) =
                <Price0CumulativeLastCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::Price0CumulativeLast(decoded));
            }
            if let Ok(decoded) =
                <Price1CumulativeLastCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::Price1CumulativeLast(decoded));
            }
            if let Ok(decoded) = <SkimCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::Skim(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::Swap(decoded));
            }
            if let Ok(decoded) =
                <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::Symbol(decoded));
            }
            if let Ok(decoded) = <SyncCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::Sync(decoded));
            }
            if let Ok(decoded) =
                <Token0Call as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::Token0(decoded));
            }
            if let Ok(decoded) =
                <Token1Call as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::Token1(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IUniswapV2PairCalls::TransferFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IUniswapV2PairCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IUniswapV2PairCalls::DomainSeparator(element) => element.encode(),
                IUniswapV2PairCalls::MinimumLiquidity(element) => element.encode(),
                IUniswapV2PairCalls::PermitTypehash(element) => element.encode(),
                IUniswapV2PairCalls::Allowance(element) => element.encode(),
                IUniswapV2PairCalls::Approve(element) => element.encode(),
                IUniswapV2PairCalls::BalanceOf(element) => element.encode(),
                IUniswapV2PairCalls::Burn(element) => element.encode(),
                IUniswapV2PairCalls::Decimals(element) => element.encode(),
                IUniswapV2PairCalls::Factory(element) => element.encode(),
                IUniswapV2PairCalls::GetReserves(element) => element.encode(),
                IUniswapV2PairCalls::Initialize(element) => element.encode(),
                IUniswapV2PairCalls::Klast(element) => element.encode(),
                IUniswapV2PairCalls::Mint(element) => element.encode(),
                IUniswapV2PairCalls::Name(element) => element.encode(),
                IUniswapV2PairCalls::Nonces(element) => element.encode(),
                IUniswapV2PairCalls::Permit(element) => element.encode(),
                IUniswapV2PairCalls::Price0CumulativeLast(element) => element.encode(),
                IUniswapV2PairCalls::Price1CumulativeLast(element) => element.encode(),
                IUniswapV2PairCalls::Skim(element) => element.encode(),
                IUniswapV2PairCalls::Swap(element) => element.encode(),
                IUniswapV2PairCalls::Symbol(element) => element.encode(),
                IUniswapV2PairCalls::Sync(element) => element.encode(),
                IUniswapV2PairCalls::Token0(element) => element.encode(),
                IUniswapV2PairCalls::Token1(element) => element.encode(),
                IUniswapV2PairCalls::TotalSupply(element) => element.encode(),
                IUniswapV2PairCalls::Transfer(element) => element.encode(),
                IUniswapV2PairCalls::TransferFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IUniswapV2PairCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IUniswapV2PairCalls::DomainSeparator(element) => element.fmt(f),
                IUniswapV2PairCalls::MinimumLiquidity(element) => element.fmt(f),
                IUniswapV2PairCalls::PermitTypehash(element) => element.fmt(f),
                IUniswapV2PairCalls::Allowance(element) => element.fmt(f),
                IUniswapV2PairCalls::Approve(element) => element.fmt(f),
                IUniswapV2PairCalls::BalanceOf(element) => element.fmt(f),
                IUniswapV2PairCalls::Burn(element) => element.fmt(f),
                IUniswapV2PairCalls::Decimals(element) => element.fmt(f),
                IUniswapV2PairCalls::Factory(element) => element.fmt(f),
                IUniswapV2PairCalls::GetReserves(element) => element.fmt(f),
                IUniswapV2PairCalls::Initialize(element) => element.fmt(f),
                IUniswapV2PairCalls::Klast(element) => element.fmt(f),
                IUniswapV2PairCalls::Mint(element) => element.fmt(f),
                IUniswapV2PairCalls::Name(element) => element.fmt(f),
                IUniswapV2PairCalls::Nonces(element) => element.fmt(f),
                IUniswapV2PairCalls::Permit(element) => element.fmt(f),
                IUniswapV2PairCalls::Price0CumulativeLast(element) => element.fmt(f),
                IUniswapV2PairCalls::Price1CumulativeLast(element) => element.fmt(f),
                IUniswapV2PairCalls::Skim(element) => element.fmt(f),
                IUniswapV2PairCalls::Swap(element) => element.fmt(f),
                IUniswapV2PairCalls::Symbol(element) => element.fmt(f),
                IUniswapV2PairCalls::Sync(element) => element.fmt(f),
                IUniswapV2PairCalls::Token0(element) => element.fmt(f),
                IUniswapV2PairCalls::Token1(element) => element.fmt(f),
                IUniswapV2PairCalls::TotalSupply(element) => element.fmt(f),
                IUniswapV2PairCalls::Transfer(element) => element.fmt(f),
                IUniswapV2PairCalls::TransferFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for IUniswapV2PairCalls {
        fn from(var: DomainSeparatorCall) -> Self {
            IUniswapV2PairCalls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<MinimumLiquidityCall> for IUniswapV2PairCalls {
        fn from(var: MinimumLiquidityCall) -> Self {
            IUniswapV2PairCalls::MinimumLiquidity(var)
        }
    }
    impl ::std::convert::From<PermitTypehashCall> for IUniswapV2PairCalls {
        fn from(var: PermitTypehashCall) -> Self {
            IUniswapV2PairCalls::PermitTypehash(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for IUniswapV2PairCalls {
        fn from(var: AllowanceCall) -> Self {
            IUniswapV2PairCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for IUniswapV2PairCalls {
        fn from(var: ApproveCall) -> Self {
            IUniswapV2PairCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for IUniswapV2PairCalls {
        fn from(var: BalanceOfCall) -> Self {
            IUniswapV2PairCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BurnCall> for IUniswapV2PairCalls {
        fn from(var: BurnCall) -> Self {
            IUniswapV2PairCalls::Burn(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for IUniswapV2PairCalls {
        fn from(var: DecimalsCall) -> Self {
            IUniswapV2PairCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for IUniswapV2PairCalls {
        fn from(var: FactoryCall) -> Self {
            IUniswapV2PairCalls::Factory(var)
        }
    }
    impl ::std::convert::From<GetReservesCall> for IUniswapV2PairCalls {
        fn from(var: GetReservesCall) -> Self {
            IUniswapV2PairCalls::GetReserves(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for IUniswapV2PairCalls {
        fn from(var: InitializeCall) -> Self {
            IUniswapV2PairCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<KlastCall> for IUniswapV2PairCalls {
        fn from(var: KlastCall) -> Self {
            IUniswapV2PairCalls::Klast(var)
        }
    }
    impl ::std::convert::From<MintCall> for IUniswapV2PairCalls {
        fn from(var: MintCall) -> Self {
            IUniswapV2PairCalls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for IUniswapV2PairCalls {
        fn from(var: NameCall) -> Self {
            IUniswapV2PairCalls::Name(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for IUniswapV2PairCalls {
        fn from(var: NoncesCall) -> Self {
            IUniswapV2PairCalls::Nonces(var)
        }
    }
    impl ::std::convert::From<PermitCall> for IUniswapV2PairCalls {
        fn from(var: PermitCall) -> Self {
            IUniswapV2PairCalls::Permit(var)
        }
    }
    impl ::std::convert::From<Price0CumulativeLastCall> for IUniswapV2PairCalls {
        fn from(var: Price0CumulativeLastCall) -> Self {
            IUniswapV2PairCalls::Price0CumulativeLast(var)
        }
    }
    impl ::std::convert::From<Price1CumulativeLastCall> for IUniswapV2PairCalls {
        fn from(var: Price1CumulativeLastCall) -> Self {
            IUniswapV2PairCalls::Price1CumulativeLast(var)
        }
    }
    impl ::std::convert::From<SkimCall> for IUniswapV2PairCalls {
        fn from(var: SkimCall) -> Self {
            IUniswapV2PairCalls::Skim(var)
        }
    }
    impl ::std::convert::From<SwapCall> for IUniswapV2PairCalls {
        fn from(var: SwapCall) -> Self {
            IUniswapV2PairCalls::Swap(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for IUniswapV2PairCalls {
        fn from(var: SymbolCall) -> Self {
            IUniswapV2PairCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<SyncCall> for IUniswapV2PairCalls {
        fn from(var: SyncCall) -> Self {
            IUniswapV2PairCalls::Sync(var)
        }
    }
    impl ::std::convert::From<Token0Call> for IUniswapV2PairCalls {
        fn from(var: Token0Call) -> Self {
            IUniswapV2PairCalls::Token0(var)
        }
    }
    impl ::std::convert::From<Token1Call> for IUniswapV2PairCalls {
        fn from(var: Token1Call) -> Self {
            IUniswapV2PairCalls::Token1(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for IUniswapV2PairCalls {
        fn from(var: TotalSupplyCall) -> Self {
            IUniswapV2PairCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for IUniswapV2PairCalls {
        fn from(var: TransferCall) -> Self {
            IUniswapV2PairCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for IUniswapV2PairCalls {
        fn from(var: TransferFromCall) -> Self {
            IUniswapV2PairCalls::TransferFrom(var)
        }
    }
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `MINIMUM_LIQUIDITY` function with signature `MINIMUM_LIQUIDITY()` and selector `0xba9a7a56`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct MinimumLiquidityReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PERMIT_TYPEHASH` function with signature `PERMIT_TYPEHASH()` and selector `0x30adf81f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct PermitTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `burn` function with signature `burn(address)` and selector `0x89afcb44`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct BurnReturn {
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `factory` function with signature `factory()` and selector `0xc45a0155`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct FactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getReserves` function with signature `getReserves()` and selector `0x0902f1ac`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetReservesReturn {
        pub reserve_0: u128,
        pub reserve_1: u128,
        pub block_timestamp_last: u32,
    }
    ///Container type for all return fields from the `kLast` function with signature `kLast()` and selector `0x7464fc3d`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct KlastReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mint` function with signature `mint(address)` and selector `0x6a627842`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct MintReturn {
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct NameReturn(pub String);
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `price0CumulativeLast` function with signature `price0CumulativeLast()` and selector `0x5909c0d5`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct Price0CumulativeLastReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `price1CumulativeLast` function with signature `price1CumulativeLast()` and selector `0x5a3d5493`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct Price1CumulativeLastReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct SymbolReturn(pub String);
    ///Container type for all return fields from the `token0` function with signature `token0()` and selector `0x0dfe1681`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct Token0Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `token1` function with signature `token1()` and selector `0xd21220a7`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct Token1Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct TransferFromReturn(pub bool);
}
