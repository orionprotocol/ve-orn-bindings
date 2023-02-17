pub use i_orion_pool_v2_factory::*;
#[allow(
    clippy::too_many_arguments,
    non_camel_case_types,
    clippy::upper_case_acronyms
)]
pub mod i_orion_pool_v2_factory {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    //!IOrionPoolV2Factory was auto-generated with ethers-rs Abigen. More information at: <https://github.com/gakonst/ethers-rs>
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
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token0\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"token1\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"pair\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PairCreated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allPairs\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"pair\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allPairsLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenA\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenB\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createPair\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"pair\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"feeTo\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"feeToSetter\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenA\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenB\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPair\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"pair\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFeeTo\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFeeToSetter\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IORIONPOOLV2FACTORY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IOrionPoolV2Factory<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for IOrionPoolV2Factory<M> {
        fn clone(&self) -> Self {
            IOrionPoolV2Factory(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IOrionPoolV2Factory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IOrionPoolV2Factory<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IOrionPoolV2Factory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IOrionPoolV2Factory<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IORIONPOOLV2FACTORY_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `allPairs` (0x1e3dd18b) function
        pub fn all_pairs(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([30, 61, 209, 139], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allPairsLength` (0x574f2ba3) function
        pub fn all_pairs_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([87, 79, 43, 163], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createPair` (0xc9c65396) function
        pub fn create_pair(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([201, 198, 83, 150], (token_a, token_b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeTo` (0x017e7e58) function
        pub fn fee_to(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([1, 126, 126, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeToSetter` (0x094b7415) function
        pub fn fee_to_setter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([9, 75, 116, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPair` (0xe6a43905) function
        pub fn get_pair(
            &self,
            token_a: ::ethers::core::types::Address,
            token_b: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([230, 164, 57, 5], (token_a, token_b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeTo` (0xf46901ed) function
        pub fn set_fee_to(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([244, 105, 1, 237], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeToSetter` (0xa2e74af6) function
        pub fn set_fee_to_setter(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 231, 74, 246], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `PairCreated` event
        pub fn pair_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, PairCreatedFilter> {
            self.0.event()
        }
        /// Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract
        pub fn events(&self) -> ::ethers::contract::builders::Event<M, PairCreatedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IOrionPoolV2Factory<M>
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
    #[ethevent(
        name = "PairCreated",
        abi = "PairCreated(address,address,address,uint256)"
    )]
    pub struct PairCreatedFilter {
        #[ethevent(indexed)]
        pub token_0: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_1: ::ethers::core::types::Address,
        pub pair: ::ethers::core::types::Address,
        pub p3: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `allPairs` function with signature `allPairs(uint256)` and selector `0x1e3dd18b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "allPairs", abi = "allPairs(uint256)")]
    pub struct AllPairsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `allPairsLength` function with signature `allPairsLength()` and selector `0x574f2ba3`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "allPairsLength", abi = "allPairsLength()")]
    pub struct AllPairsLengthCall;
    ///Container type for all input parameters for the `createPair` function with signature `createPair(address,address)` and selector `0xc9c65396`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "createPair", abi = "createPair(address,address)")]
    pub struct CreatePairCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `feeTo` function with signature `feeTo()` and selector `0x017e7e58`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "feeTo", abi = "feeTo()")]
    pub struct FeeToCall;
    ///Container type for all input parameters for the `feeToSetter` function with signature `feeToSetter()` and selector `0x094b7415`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "feeToSetter", abi = "feeToSetter()")]
    pub struct FeeToSetterCall;
    ///Container type for all input parameters for the `getPair` function with signature `getPair(address,address)` and selector `0xe6a43905`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPair", abi = "getPair(address,address)")]
    pub struct GetPairCall {
        pub token_a: ::ethers::core::types::Address,
        pub token_b: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setFeeTo` function with signature `setFeeTo(address)` and selector `0xf46901ed`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "setFeeTo", abi = "setFeeTo(address)")]
    pub struct SetFeeToCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `setFeeToSetter` function with signature `setFeeToSetter(address)` and selector `0xa2e74af6`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
    )]
    #[ethcall(name = "setFeeToSetter", abi = "setFeeToSetter(address)")]
    pub struct SetFeeToSetterCall(pub ::ethers::core::types::Address);
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum IOrionPoolV2FactoryCalls {
        AllPairs(AllPairsCall),
        AllPairsLength(AllPairsLengthCall),
        CreatePair(CreatePairCall),
        FeeTo(FeeToCall),
        FeeToSetter(FeeToSetterCall),
        GetPair(GetPairCall),
        SetFeeTo(SetFeeToCall),
        SetFeeToSetter(SetFeeToSetterCall),
    }
    impl ::ethers::core::abi::AbiDecode for IOrionPoolV2FactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AllPairsCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionPoolV2FactoryCalls::AllPairs(decoded));
            }
            if let Ok(decoded) =
                <AllPairsLengthCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionPoolV2FactoryCalls::AllPairsLength(decoded));
            }
            if let Ok(decoded) =
                <CreatePairCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionPoolV2FactoryCalls::CreatePair(decoded));
            }
            if let Ok(decoded) =
                <FeeToCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionPoolV2FactoryCalls::FeeTo(decoded));
            }
            if let Ok(decoded) =
                <FeeToSetterCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionPoolV2FactoryCalls::FeeToSetter(decoded));
            }
            if let Ok(decoded) =
                <GetPairCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionPoolV2FactoryCalls::GetPair(decoded));
            }
            if let Ok(decoded) =
                <SetFeeToCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionPoolV2FactoryCalls::SetFeeTo(decoded));
            }
            if let Ok(decoded) =
                <SetFeeToSetterCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IOrionPoolV2FactoryCalls::SetFeeToSetter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IOrionPoolV2FactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IOrionPoolV2FactoryCalls::AllPairs(element) => element.encode(),
                IOrionPoolV2FactoryCalls::AllPairsLength(element) => element.encode(),
                IOrionPoolV2FactoryCalls::CreatePair(element) => element.encode(),
                IOrionPoolV2FactoryCalls::FeeTo(element) => element.encode(),
                IOrionPoolV2FactoryCalls::FeeToSetter(element) => element.encode(),
                IOrionPoolV2FactoryCalls::GetPair(element) => element.encode(),
                IOrionPoolV2FactoryCalls::SetFeeTo(element) => element.encode(),
                IOrionPoolV2FactoryCalls::SetFeeToSetter(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IOrionPoolV2FactoryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IOrionPoolV2FactoryCalls::AllPairs(element) => element.fmt(f),
                IOrionPoolV2FactoryCalls::AllPairsLength(element) => element.fmt(f),
                IOrionPoolV2FactoryCalls::CreatePair(element) => element.fmt(f),
                IOrionPoolV2FactoryCalls::FeeTo(element) => element.fmt(f),
                IOrionPoolV2FactoryCalls::FeeToSetter(element) => element.fmt(f),
                IOrionPoolV2FactoryCalls::GetPair(element) => element.fmt(f),
                IOrionPoolV2FactoryCalls::SetFeeTo(element) => element.fmt(f),
                IOrionPoolV2FactoryCalls::SetFeeToSetter(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AllPairsCall> for IOrionPoolV2FactoryCalls {
        fn from(var: AllPairsCall) -> Self {
            IOrionPoolV2FactoryCalls::AllPairs(var)
        }
    }
    impl ::std::convert::From<AllPairsLengthCall> for IOrionPoolV2FactoryCalls {
        fn from(var: AllPairsLengthCall) -> Self {
            IOrionPoolV2FactoryCalls::AllPairsLength(var)
        }
    }
    impl ::std::convert::From<CreatePairCall> for IOrionPoolV2FactoryCalls {
        fn from(var: CreatePairCall) -> Self {
            IOrionPoolV2FactoryCalls::CreatePair(var)
        }
    }
    impl ::std::convert::From<FeeToCall> for IOrionPoolV2FactoryCalls {
        fn from(var: FeeToCall) -> Self {
            IOrionPoolV2FactoryCalls::FeeTo(var)
        }
    }
    impl ::std::convert::From<FeeToSetterCall> for IOrionPoolV2FactoryCalls {
        fn from(var: FeeToSetterCall) -> Self {
            IOrionPoolV2FactoryCalls::FeeToSetter(var)
        }
    }
    impl ::std::convert::From<GetPairCall> for IOrionPoolV2FactoryCalls {
        fn from(var: GetPairCall) -> Self {
            IOrionPoolV2FactoryCalls::GetPair(var)
        }
    }
    impl ::std::convert::From<SetFeeToCall> for IOrionPoolV2FactoryCalls {
        fn from(var: SetFeeToCall) -> Self {
            IOrionPoolV2FactoryCalls::SetFeeTo(var)
        }
    }
    impl ::std::convert::From<SetFeeToSetterCall> for IOrionPoolV2FactoryCalls {
        fn from(var: SetFeeToSetterCall) -> Self {
            IOrionPoolV2FactoryCalls::SetFeeToSetter(var)
        }
    }
    ///Container type for all return fields from the `allPairs` function with signature `allPairs(uint256)` and selector `0x1e3dd18b`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct AllPairsReturn {
        pub pair: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `allPairsLength` function with signature `allPairsLength()` and selector `0x574f2ba3`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct AllPairsLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `createPair` function with signature `createPair(address,address)` and selector `0xc9c65396`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct CreatePairReturn {
        pub pair: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `feeTo` function with signature `feeTo()` and selector `0x017e7e58`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct FeeToReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `feeToSetter` function with signature `feeToSetter()` and selector `0x094b7415`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct FeeToSetterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPair` function with signature `getPair(address,address)` and selector `0xe6a43905`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
    )]
    pub struct GetPairReturn {
        pub pair: ::ethers::core::types::Address,
    }
}
