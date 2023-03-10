pub use i_uniswap_v2_callee::*;
#[allow(
    clippy::too_many_arguments,
    non_camel_case_types,
    clippy::upper_case_acronyms
)]
pub mod i_uniswap_v2_callee {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    //!IUniswapV2Callee was auto-generated with ethers-rs Abigen. More information at: <https://github.com/gakonst/ethers-rs>
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
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount0\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount1\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"uniswapV2Call\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IUNISWAPV2CALLEE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IUniswapV2Callee<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for IUniswapV2Callee<M> {
        fn clone(&self) -> Self {
            IUniswapV2Callee(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IUniswapV2Callee<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IUniswapV2Callee<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IUniswapV2Callee))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IUniswapV2Callee<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IUNISWAPV2CALLEE_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `uniswapV2Call` (0x10d1e85c) function
        pub fn uniswap_v2_call(
            &self,
            sender: ::ethers::core::types::Address,
            amount_0: ::ethers::core::types::U256,
            amount_1: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 209, 232, 92], (sender, amount_0, amount_1, data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IUniswapV2Callee<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `uniswapV2Call` function with signature `uniswapV2Call(address,uint256,uint256,bytes)` and selector `0x10d1e85c`
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
        name = "uniswapV2Call",
        abi = "uniswapV2Call(address,uint256,uint256,bytes)"
    )]
    pub struct UniswapV2CallCall {
        pub sender: ::ethers::core::types::Address,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
}
