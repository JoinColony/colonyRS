pub use colony_network::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod colony_network {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "ColonyNetwork was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"auction\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"quantity\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AuctionCreated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"colonyId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"colonyAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ColonyAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"colony\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"label\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ColonyLabelRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"resolver\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ColonyNetworkInitialised\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"version\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"resolver\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ColonyVersionAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"extensionId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"version\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExtensionAddedToNetwork\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"extensionId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"colony\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"deprecated\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExtensionDeprecated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"extensionId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"colony\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"version\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExtensionInstalled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"extensionId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"colony\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ExtensionUninstalled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"extensionId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"colony\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"version\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExtensionUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"authority\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"LogSetAuthority\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"LogSetOwner\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"metaColony\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"rootSkillId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MetaColonyCreated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address payable\",\"name\":\"relayerAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"functionSignature\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MetaTransactionExecuted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"miningCycleResolver\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MiningCycleResolverSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"feeInverse\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NetworkFeeInverseSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"ens\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"rootNode\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RegistrarInitialised\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"miner\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tokensLost\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReputationMinerPenalised\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"hash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"nLeaves\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReputationMiningCycleComplete\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"inactiveReputationMiningCycle\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReputationMiningInitialised\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReputationMiningRewardSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"newHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newNLeaves\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address[]\",\"name\":\"stakers\",\"type\":\"address[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"reward\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReputationRootHashSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"skillId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"parentSkillId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SkillAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAuthorityAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenAuthorityDeployed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenDeployed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenLocking\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenLockingAddressSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"status\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"label\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"UserLabelRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"authority\",\"outputs\":[{\"internalType\":\"contract DSAuthority\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_payload\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_sigR\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_sigS\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_sigV\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"executeMetaTransaction\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChainId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes[]\",\"name\":\"data\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"multicall\",\"outputs\":[{\"internalType\":\"bytes[]\",\"name\":\"results\",\"type\":\"bytes[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract DSAuthority\",\"name\":\"authority_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAuthority\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setOwner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_chainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_payload\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_sigR\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_sigS\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_sigV\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"verify\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_colony\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isColony\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentColonyVersion\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMetaColony\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getColonyCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSkillCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReputationMiningSkillId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_version\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getColonyVersionResolver\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSkill\",\"outputs\":[{\"internalType\":\"struct ColonyNetworkDataTypes.Skill\",\"name\":\"skill\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint128\",\"name\":\"nParents\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"nChildren\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"parents\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"children\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"globalSkill\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"deprecated\",\"type\":\"bool\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReputationRootHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReputationRootHashNLeaves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReputationRootHashNNodes\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenLocking\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTokenLocking\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTokenLocking\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_version\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_resolver\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addColonyVersion\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_resolver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_version\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialise\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getColony\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_parentSkillId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addSkill\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_parentSkillIndex\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getParentSkillId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChildSkillId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deprecateSkill\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_deprecated\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deprecateSkill\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialiseRootLocalSkill\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_amount\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"appendReputationUpdateLog\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_slot\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"checkNotAdditionalProtectedVariable\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFeeInverse\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"_feeInverse\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_feeInverse\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFeeInverse\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMetatransactionNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_status\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPayoutWhitelist\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPayoutWhitelist\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static COLONYNETWORK_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct ColonyNetwork<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ColonyNetwork<M> {
        fn clone(&self) -> Self {
            ColonyNetwork(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ColonyNetwork<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for ColonyNetwork<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ColonyNetwork))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ColonyNetwork<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), COLONYNETWORK_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `addColonyVersion` (0xf07eb921) function"]
        pub fn add_colony_version(
            &self,
            version: ethers::core::types::U256,
            resolver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 126, 185, 33], (version, resolver))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addSkill` (0x162419cc) function"]
        pub fn add_skill(
            &self,
            parent_skill_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([22, 36, 25, 204], parent_skill_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `appendReputationUpdateLog` (0x5a8adafa) function"]
        pub fn append_reputation_update_log(
            &self,
            user: ethers::core::types::Address,
            amount: I256,
            skill_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([90, 138, 218, 250], (user, amount, skill_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `authority` (0xbf7e214f) function"]
        pub fn authority(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([191, 126, 33, 79], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `checkNotAdditionalProtectedVariable` (0x8831fa34) function"]
        pub fn check_not_additional_protected_variable(
            &self,
            slot: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([136, 49, 250, 52], slot)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deprecateSkill` (0xbb47bf18) function"]
        pub fn deprecate_skill(
            &self,
            skill_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([187, 71, 191, 24], skill_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deprecateSkill` (0xd20b156b) function"]
        pub fn deprecate_skill_with_deprecated(
            &self,
            skill_id: ethers::core::types::U256,
            deprecated: bool,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([210, 11, 21, 107], (skill_id, deprecated))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeMetaTransaction` (0x0c53c51c) function"]
        pub fn execute_meta_transaction(
            &self,
            user: ethers::core::types::Address,
            payload: ethers::core::types::Bytes,
            sig_r: [u8; 32],
            sig_s: [u8; 32],
            sig_v: u8,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash([12, 83, 197, 28], (user, payload, sig_r, sig_s, sig_v))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getChainId` (0x3408e470) function"]
        pub fn get_chain_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([52, 8, 228, 112], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getChildSkillId` (0x09d10a5e) function"]
        pub fn get_child_skill_id(
            &self,
            skill_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([9, 209, 10, 94], (skill_id, child_skill_index))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getColony` (0x6f92650d) function"]
        pub fn get_colony(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([111, 146, 101, 13], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getColonyCount` (0x711a39be) function"]
        pub fn get_colony_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([113, 26, 57, 190], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getColonyVersionResolver` (0x8f14c886) function"]
        pub fn get_colony_version_resolver(
            &self,
            version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([143, 20, 200, 134], version)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentColonyVersion` (0xbca1e4df) function"]
        pub fn get_current_colony_version(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([188, 161, 228, 223], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getFeeInverse` (0xef63c380) function"]
        pub fn get_fee_inverse(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([239, 99, 195, 128], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMetaColony` (0x731bc22f) function"]
        pub fn get_meta_colony(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([115, 27, 194, 47], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMetatransactionNonce` (0xb3eac1d8) function"]
        pub fn get_metatransaction_nonce(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([179, 234, 193, 216], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getParentSkillId` (0xd987fc16) function"]
        pub fn get_parent_skill_id(
            &self,
            skill_id: ethers::core::types::U256,
            parent_skill_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([217, 135, 252, 22], (skill_id, parent_skill_index))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPayoutWhitelist` (0x60539ef2) function"]
        pub fn get_payout_whitelist(
            &self,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([96, 83, 158, 242], token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReputationMiningSkillId` (0x1c46bc7f) function"]
        pub fn get_reputation_mining_skill_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([28, 70, 188, 127], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReputationRootHash` (0x97824c3f) function"]
        pub fn get_reputation_root_hash(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([151, 130, 76, 63], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReputationRootHashNLeaves` (0x1262bd19) function"]
        pub fn get_reputation_root_hash_n_leaves(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([18, 98, 189, 25], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReputationRootHashNNodes` (0x7680150d) function"]
        pub fn get_reputation_root_hash_n_nodes(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([118, 128, 21, 13], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSkill` (0xbd880fae) function"]
        pub fn get_skill(
            &self,
            skill_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, Skill> {
            self.0
                .method_hash([189, 136, 15, 174], skill_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getSkillCount` (0x05f53b29) function"]
        pub fn get_skill_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([5, 245, 59, 41], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTokenLocking` (0xb319902a) function"]
        pub fn get_token_locking(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([179, 25, 144, 42], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialise` (0x3b4d206b) function"]
        pub fn initialise(
            &self,
            resolver: ethers::core::types::Address,
            version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 77, 32, 107], (resolver, version))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialiseRootLocalSkill` (0x2501ab2c) function"]
        pub fn initialise_root_local_skill(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([37, 1, 171, 44], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isColony` (0xdb0dd7ff) function"]
        pub fn is_colony(
            &self,
            colony: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([219, 13, 215, 255], colony)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `multicall` (0xac9650d8) function"]
        pub fn multicall(
            &self,
            data: ::std::vec::Vec<ethers::core::types::Bytes>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::Bytes>>
        {
            self.0
                .method_hash([172, 150, 80, 216], data)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAuthority` (0x7a9e5e4b) function"]
        pub fn set_authority(
            &self,
            authority: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 158, 94, 75], authority)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFeeInverse` (0x3812b259) function"]
        pub fn set_fee_inverse(
            &self,
            fee_inverse: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 18, 178, 89], fee_inverse)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setOwner` (0x13af4035) function"]
        pub fn set_owner(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 175, 64, 53], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPayoutWhitelist` (0xe7875ea8) function"]
        pub fn set_payout_whitelist(
            &self,
            token: ethers::core::types::Address,
            status: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 135, 94, 168], (token, status))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTokenLocking` (0x8fb6839a) function"]
        pub fn set_token_locking(
            &self,
            token_locking: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([143, 182, 131, 154], token_locking)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verify` (0x6281133d) function"]
        pub fn verify(
            &self,
            owner: ethers::core::types::Address,
            nonce: ethers::core::types::U256,
            chain_id: ethers::core::types::U256,
            payload: ethers::core::types::Bytes,
            sig_r: [u8; 32],
            sig_s: [u8; 32],
            sig_v: u8,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [98, 129, 19, 61],
                    (owner, nonce, chain_id, payload, sig_r, sig_s, sig_v),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AuctionCreated` event"]
        pub fn auction_created_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AuctionCreatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ColonyAdded` event"]
        pub fn colony_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ColonyAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ColonyLabelRegistered` event"]
        pub fn colony_label_registered_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ColonyLabelRegisteredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ColonyNetworkInitialised` event"]
        pub fn colony_network_initialised_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ColonyNetworkInitialisedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ColonyVersionAdded` event"]
        pub fn colony_version_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ColonyVersionAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExtensionAddedToNetwork` event"]
        pub fn extension_added_to_network_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExtensionAddedToNetworkFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExtensionDeprecated` event"]
        pub fn extension_deprecated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExtensionDeprecatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExtensionInstalled` event"]
        pub fn extension_installed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExtensionInstalledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExtensionUninstalled` event"]
        pub fn extension_uninstalled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExtensionUninstalledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExtensionUpgraded` event"]
        pub fn extension_upgraded_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExtensionUpgradedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogSetAuthority` event"]
        pub fn log_set_authority_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogSetAuthorityFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LogSetOwner` event"]
        pub fn log_set_owner_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogSetOwnerFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MetaColonyCreated` event"]
        pub fn meta_colony_created_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MetaColonyCreatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MetaTransactionExecuted` event"]
        pub fn meta_transaction_executed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MetaTransactionExecutedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MiningCycleResolverSet` event"]
        pub fn mining_cycle_resolver_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MiningCycleResolverSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NetworkFeeInverseSet` event"]
        pub fn network_fee_inverse_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, NetworkFeeInverseSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RegistrarInitialised` event"]
        pub fn registrar_initialised_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RegistrarInitialisedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReputationMinerPenalised` event"]
        pub fn reputation_miner_penalised_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReputationMinerPenalisedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReputationMiningCycleComplete` event"]
        pub fn reputation_mining_cycle_complete_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReputationMiningCycleCompleteFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReputationMiningInitialised` event"]
        pub fn reputation_mining_initialised_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReputationMiningInitialisedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReputationMiningRewardSet` event"]
        pub fn reputation_mining_reward_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReputationMiningRewardSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReputationRootHashSet` event"]
        pub fn reputation_root_hash_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReputationRootHashSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SkillAdded` event"]
        pub fn skill_added_filter(&self) -> ethers::contract::builders::Event<M, SkillAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenAuthorityDeployed` event"]
        pub fn token_authority_deployed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenAuthorityDeployedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenDeployed` event"]
        pub fn token_deployed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenDeployedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenLockingAddressSet` event"]
        pub fn token_locking_address_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenLockingAddressSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenWhitelisted` event"]
        pub fn token_whitelisted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenWhitelistedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `UserLabelRegistered` event"]
        pub fn user_label_registered_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, UserLabelRegisteredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ColonyNetworkEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ColonyNetwork<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "AuctionCreated",
        abi = "AuctionCreated(address,address,uint256)"
    )]
    pub struct AuctionCreatedFilter {
        pub auction: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub quantity: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "ColonyAdded", abi = "ColonyAdded(uint256,address,address)")]
    pub struct ColonyAddedFilter {
        #[ethevent(indexed)]
        pub colony_id: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub colony_address: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ColonyLabelRegistered",
        abi = "ColonyLabelRegistered(address,bytes32)"
    )]
    pub struct ColonyLabelRegisteredFilter {
        #[ethevent(indexed)]
        pub colony: ethers::core::types::Address,
        pub label: [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ColonyNetworkInitialised",
        abi = "ColonyNetworkInitialised(address)"
    )]
    pub struct ColonyNetworkInitialisedFilter {
        pub resolver: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ColonyVersionAdded",
        abi = "ColonyVersionAdded(uint256,address)"
    )]
    pub struct ColonyVersionAddedFilter {
        pub version: ethers::core::types::U256,
        pub resolver: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ExtensionAddedToNetwork",
        abi = "ExtensionAddedToNetwork(bytes32,uint256)"
    )]
    pub struct ExtensionAddedToNetworkFilter {
        #[ethevent(indexed)]
        pub extension_id: [u8; 32],
        pub version: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ExtensionDeprecated",
        abi = "ExtensionDeprecated(bytes32,address,bool)"
    )]
    pub struct ExtensionDeprecatedFilter {
        #[ethevent(indexed)]
        pub extension_id: [u8; 32],
        #[ethevent(indexed)]
        pub colony: ethers::core::types::Address,
        pub deprecated: bool,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ExtensionInstalled",
        abi = "ExtensionInstalled(bytes32,address,uint256)"
    )]
    pub struct ExtensionInstalledFilter {
        #[ethevent(indexed)]
        pub extension_id: [u8; 32],
        #[ethevent(indexed)]
        pub colony: ethers::core::types::Address,
        pub version: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ExtensionUninstalled",
        abi = "ExtensionUninstalled(bytes32,address)"
    )]
    pub struct ExtensionUninstalledFilter {
        #[ethevent(indexed)]
        pub extension_id: [u8; 32],
        #[ethevent(indexed)]
        pub colony: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ExtensionUpgraded",
        abi = "ExtensionUpgraded(bytes32,address,uint256)"
    )]
    pub struct ExtensionUpgradedFilter {
        #[ethevent(indexed)]
        pub extension_id: [u8; 32],
        #[ethevent(indexed)]
        pub colony: ethers::core::types::Address,
        pub version: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "LogSetAuthority", abi = "LogSetAuthority(address)")]
    pub struct LogSetAuthorityFilter {
        #[ethevent(indexed)]
        pub authority: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "LogSetOwner", abi = "LogSetOwner(address)")]
    pub struct LogSetOwnerFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "MetaColonyCreated",
        abi = "MetaColonyCreated(address,address,uint256)"
    )]
    pub struct MetaColonyCreatedFilter {
        pub meta_colony: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub root_skill_id: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "MetaTransactionExecuted",
        abi = "MetaTransactionExecuted(address,address,bytes)"
    )]
    pub struct MetaTransactionExecutedFilter {
        pub user: ethers::core::types::Address,
        pub relayer_address: ethers::core::types::Address,
        pub function_signature: ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "MiningCycleResolverSet",
        abi = "MiningCycleResolverSet(address)"
    )]
    pub struct MiningCycleResolverSetFilter {
        pub mining_cycle_resolver: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "NetworkFeeInverseSet", abi = "NetworkFeeInverseSet(uint256)")]
    pub struct NetworkFeeInverseSetFilter {
        pub fee_inverse: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "RegistrarInitialised",
        abi = "RegistrarInitialised(address,bytes32)"
    )]
    pub struct RegistrarInitialisedFilter {
        pub ens: ethers::core::types::Address,
        pub root_node: [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ReputationMinerPenalised",
        abi = "ReputationMinerPenalised(address,uint256)"
    )]
    pub struct ReputationMinerPenalisedFilter {
        pub miner: ethers::core::types::Address,
        pub tokens_lost: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ReputationMiningCycleComplete",
        abi = "ReputationMiningCycleComplete(bytes32,uint256)"
    )]
    pub struct ReputationMiningCycleCompleteFilter {
        pub hash: [u8; 32],
        pub n_leaves: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ReputationMiningInitialised",
        abi = "ReputationMiningInitialised(address)"
    )]
    pub struct ReputationMiningInitialisedFilter {
        pub inactive_reputation_mining_cycle: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ReputationMiningRewardSet",
        abi = "ReputationMiningRewardSet(uint256)"
    )]
    pub struct ReputationMiningRewardSetFilter {
        pub amount: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ReputationRootHashSet",
        abi = "ReputationRootHashSet(bytes32,uint256,address[],uint256)"
    )]
    pub struct ReputationRootHashSetFilter {
        pub new_hash: [u8; 32],
        pub new_n_leaves: ethers::core::types::U256,
        pub stakers: Vec<ethers::core::types::Address>,
        pub reward: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SkillAdded", abi = "SkillAdded(uint256,uint256)")]
    pub struct SkillAddedFilter {
        pub skill_id: ethers::core::types::U256,
        pub parent_skill_id: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TokenAuthorityDeployed",
        abi = "TokenAuthorityDeployed(address)"
    )]
    pub struct TokenAuthorityDeployedFilter {
        pub token_authority_address: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "TokenDeployed", abi = "TokenDeployed(address)")]
    pub struct TokenDeployedFilter {
        pub token_address: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TokenLockingAddressSet",
        abi = "TokenLockingAddressSet(address)"
    )]
    pub struct TokenLockingAddressSetFilter {
        pub token_locking: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "TokenWhitelisted", abi = "TokenWhitelisted(address,bool)")]
    pub struct TokenWhitelistedFilter {
        pub token: ethers::core::types::Address,
        pub status: bool,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "UserLabelRegistered",
        abi = "UserLabelRegistered(address,bytes32)"
    )]
    pub struct UserLabelRegisteredFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        pub label: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ColonyNetworkEvents {
        AuctionCreatedFilter(AuctionCreatedFilter),
        ColonyAddedFilter(ColonyAddedFilter),
        ColonyLabelRegisteredFilter(ColonyLabelRegisteredFilter),
        ColonyNetworkInitialisedFilter(ColonyNetworkInitialisedFilter),
        ColonyVersionAddedFilter(ColonyVersionAddedFilter),
        ExtensionAddedToNetworkFilter(ExtensionAddedToNetworkFilter),
        ExtensionDeprecatedFilter(ExtensionDeprecatedFilter),
        ExtensionInstalledFilter(ExtensionInstalledFilter),
        ExtensionUninstalledFilter(ExtensionUninstalledFilter),
        ExtensionUpgradedFilter(ExtensionUpgradedFilter),
        LogSetAuthorityFilter(LogSetAuthorityFilter),
        LogSetOwnerFilter(LogSetOwnerFilter),
        MetaColonyCreatedFilter(MetaColonyCreatedFilter),
        MetaTransactionExecutedFilter(MetaTransactionExecutedFilter),
        MiningCycleResolverSetFilter(MiningCycleResolverSetFilter),
        NetworkFeeInverseSetFilter(NetworkFeeInverseSetFilter),
        RegistrarInitialisedFilter(RegistrarInitialisedFilter),
        ReputationMinerPenalisedFilter(ReputationMinerPenalisedFilter),
        ReputationMiningCycleCompleteFilter(ReputationMiningCycleCompleteFilter),
        ReputationMiningInitialisedFilter(ReputationMiningInitialisedFilter),
        ReputationMiningRewardSetFilter(ReputationMiningRewardSetFilter),
        ReputationRootHashSetFilter(ReputationRootHashSetFilter),
        SkillAddedFilter(SkillAddedFilter),
        TokenAuthorityDeployedFilter(TokenAuthorityDeployedFilter),
        TokenDeployedFilter(TokenDeployedFilter),
        TokenLockingAddressSetFilter(TokenLockingAddressSetFilter),
        TokenWhitelistedFilter(TokenWhitelistedFilter),
        UserLabelRegisteredFilter(UserLabelRegisteredFilter),
    }
    impl ethers::contract::EthLogDecode for ColonyNetworkEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AuctionCreatedFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::AuctionCreatedFilter(decoded));
            }
            if let Ok(decoded) = ColonyAddedFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::ColonyAddedFilter(decoded));
            }
            if let Ok(decoded) = ColonyLabelRegisteredFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::ColonyLabelRegisteredFilter(decoded));
            }
            if let Ok(decoded) = ColonyNetworkInitialisedFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::ColonyNetworkInitialisedFilter(decoded));
            }
            if let Ok(decoded) = ColonyVersionAddedFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::ColonyVersionAddedFilter(decoded));
            }
            if let Ok(decoded) = ExtensionAddedToNetworkFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::ExtensionAddedToNetworkFilter(decoded));
            }
            if let Ok(decoded) = ExtensionDeprecatedFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::ExtensionDeprecatedFilter(decoded));
            }
            if let Ok(decoded) = ExtensionInstalledFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::ExtensionInstalledFilter(decoded));
            }
            if let Ok(decoded) = ExtensionUninstalledFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::ExtensionUninstalledFilter(decoded));
            }
            if let Ok(decoded) = ExtensionUpgradedFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::ExtensionUpgradedFilter(decoded));
            }
            if let Ok(decoded) = LogSetAuthorityFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::LogSetAuthorityFilter(decoded));
            }
            if let Ok(decoded) = LogSetOwnerFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::LogSetOwnerFilter(decoded));
            }
            if let Ok(decoded) = MetaColonyCreatedFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::MetaColonyCreatedFilter(decoded));
            }
            if let Ok(decoded) = MetaTransactionExecutedFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::MetaTransactionExecutedFilter(decoded));
            }
            if let Ok(decoded) = MiningCycleResolverSetFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::MiningCycleResolverSetFilter(decoded));
            }
            if let Ok(decoded) = NetworkFeeInverseSetFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::NetworkFeeInverseSetFilter(decoded));
            }
            if let Ok(decoded) = RegistrarInitialisedFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::RegistrarInitialisedFilter(decoded));
            }
            if let Ok(decoded) = ReputationMinerPenalisedFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::ReputationMinerPenalisedFilter(decoded));
            }
            if let Ok(decoded) = ReputationMiningCycleCompleteFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::ReputationMiningCycleCompleteFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ReputationMiningInitialisedFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::ReputationMiningInitialisedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ReputationMiningRewardSetFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::ReputationMiningRewardSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ReputationRootHashSetFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::ReputationRootHashSetFilter(decoded));
            }
            if let Ok(decoded) = SkillAddedFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::SkillAddedFilter(decoded));
            }
            if let Ok(decoded) = TokenAuthorityDeployedFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::TokenAuthorityDeployedFilter(decoded));
            }
            if let Ok(decoded) = TokenDeployedFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::TokenDeployedFilter(decoded));
            }
            if let Ok(decoded) = TokenLockingAddressSetFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::TokenLockingAddressSetFilter(decoded));
            }
            if let Ok(decoded) = TokenWhitelistedFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::TokenWhitelistedFilter(decoded));
            }
            if let Ok(decoded) = UserLabelRegisteredFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::UserLabelRegisteredFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ColonyNetworkEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ColonyNetworkEvents::AuctionCreatedFilter(element) => element.fmt(f),
                ColonyNetworkEvents::ColonyAddedFilter(element) => element.fmt(f),
                ColonyNetworkEvents::ColonyLabelRegisteredFilter(element) => element.fmt(f),
                ColonyNetworkEvents::ColonyNetworkInitialisedFilter(element) => element.fmt(f),
                ColonyNetworkEvents::ColonyVersionAddedFilter(element) => element.fmt(f),
                ColonyNetworkEvents::ExtensionAddedToNetworkFilter(element) => element.fmt(f),
                ColonyNetworkEvents::ExtensionDeprecatedFilter(element) => element.fmt(f),
                ColonyNetworkEvents::ExtensionInstalledFilter(element) => element.fmt(f),
                ColonyNetworkEvents::ExtensionUninstalledFilter(element) => element.fmt(f),
                ColonyNetworkEvents::ExtensionUpgradedFilter(element) => element.fmt(f),
                ColonyNetworkEvents::LogSetAuthorityFilter(element) => element.fmt(f),
                ColonyNetworkEvents::LogSetOwnerFilter(element) => element.fmt(f),
                ColonyNetworkEvents::MetaColonyCreatedFilter(element) => element.fmt(f),
                ColonyNetworkEvents::MetaTransactionExecutedFilter(element) => element.fmt(f),
                ColonyNetworkEvents::MiningCycleResolverSetFilter(element) => element.fmt(f),
                ColonyNetworkEvents::NetworkFeeInverseSetFilter(element) => element.fmt(f),
                ColonyNetworkEvents::RegistrarInitialisedFilter(element) => element.fmt(f),
                ColonyNetworkEvents::ReputationMinerPenalisedFilter(element) => element.fmt(f),
                ColonyNetworkEvents::ReputationMiningCycleCompleteFilter(element) => element.fmt(f),
                ColonyNetworkEvents::ReputationMiningInitialisedFilter(element) => element.fmt(f),
                ColonyNetworkEvents::ReputationMiningRewardSetFilter(element) => element.fmt(f),
                ColonyNetworkEvents::ReputationRootHashSetFilter(element) => element.fmt(f),
                ColonyNetworkEvents::SkillAddedFilter(element) => element.fmt(f),
                ColonyNetworkEvents::TokenAuthorityDeployedFilter(element) => element.fmt(f),
                ColonyNetworkEvents::TokenDeployedFilter(element) => element.fmt(f),
                ColonyNetworkEvents::TokenLockingAddressSetFilter(element) => element.fmt(f),
                ColonyNetworkEvents::TokenWhitelistedFilter(element) => element.fmt(f),
                ColonyNetworkEvents::UserLabelRegisteredFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addColonyVersion` function with signature `addColonyVersion(uint256,address)` and selector `[240, 126, 185, 33]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "addColonyVersion", abi = "addColonyVersion(uint256,address)")]
    pub struct AddColonyVersionCall {
        pub version: ethers::core::types::U256,
        pub resolver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `addSkill` function with signature `addSkill(uint256)` and selector `[22, 36, 25, 204]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "addSkill", abi = "addSkill(uint256)")]
    pub struct AddSkillCall {
        pub parent_skill_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `appendReputationUpdateLog` function with signature `appendReputationUpdateLog(address,int256,uint256)` and selector `[90, 138, 218, 250]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "appendReputationUpdateLog",
        abi = "appendReputationUpdateLog(address,int256,uint256)"
    )]
    pub struct AppendReputationUpdateLogCall {
        pub user: ethers::core::types::Address,
        pub amount: I256,
        pub skill_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `authority` function with signature `authority()` and selector `[191, 126, 33, 79]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "authority", abi = "authority()")]
    pub struct AuthorityCall;
    #[doc = "Container type for all input parameters for the `checkNotAdditionalProtectedVariable` function with signature `checkNotAdditionalProtectedVariable(uint256)` and selector `[136, 49, 250, 52]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "checkNotAdditionalProtectedVariable",
        abi = "checkNotAdditionalProtectedVariable(uint256)"
    )]
    pub struct CheckNotAdditionalProtectedVariableCall {
        pub slot: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `deprecateSkill` function with signature `deprecateSkill(uint256)` and selector `[187, 71, 191, 24]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "deprecateSkill", abi = "deprecateSkill(uint256)")]
    pub struct DeprecateSkillCall {
        pub skill_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `deprecateSkill` function with signature `deprecateSkill(uint256,bool)` and selector `[210, 11, 21, 107]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "deprecateSkill", abi = "deprecateSkill(uint256,bool)")]
    pub struct DeprecateSkillWithDeprecatedCall {
        pub skill_id: ethers::core::types::U256,
        pub deprecated: bool,
    }
    #[doc = "Container type for all input parameters for the `executeMetaTransaction` function with signature `executeMetaTransaction(address,bytes,bytes32,bytes32,uint8)` and selector `[12, 83, 197, 28]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "executeMetaTransaction",
        abi = "executeMetaTransaction(address,bytes,bytes32,bytes32,uint8)"
    )]
    pub struct ExecuteMetaTransactionCall {
        pub user: ethers::core::types::Address,
        pub payload: ethers::core::types::Bytes,
        pub sig_r: [u8; 32],
        pub sig_s: [u8; 32],
        pub sig_v: u8,
    }
    #[doc = "Container type for all input parameters for the `getChainId` function with signature `getChainId()` and selector `[52, 8, 228, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getChainId", abi = "getChainId()")]
    pub struct GetChainIdCall;
    #[doc = "Container type for all input parameters for the `getChildSkillId` function with signature `getChildSkillId(uint256,uint256)` and selector `[9, 209, 10, 94]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getChildSkillId", abi = "getChildSkillId(uint256,uint256)")]
    pub struct GetChildSkillIdCall {
        pub skill_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getColony` function with signature `getColony(uint256)` and selector `[111, 146, 101, 13]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getColony", abi = "getColony(uint256)")]
    pub struct GetColonyCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getColonyCount` function with signature `getColonyCount()` and selector `[113, 26, 57, 190]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getColonyCount", abi = "getColonyCount()")]
    pub struct GetColonyCountCall;
    #[doc = "Container type for all input parameters for the `getColonyVersionResolver` function with signature `getColonyVersionResolver(uint256)` and selector `[143, 20, 200, 134]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "getColonyVersionResolver",
        abi = "getColonyVersionResolver(uint256)"
    )]
    pub struct GetColonyVersionResolverCall {
        pub version: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getCurrentColonyVersion` function with signature `getCurrentColonyVersion()` and selector `[188, 161, 228, 223]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getCurrentColonyVersion", abi = "getCurrentColonyVersion()")]
    pub struct GetCurrentColonyVersionCall;
    #[doc = "Container type for all input parameters for the `getFeeInverse` function with signature `getFeeInverse()` and selector `[239, 99, 195, 128]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getFeeInverse", abi = "getFeeInverse()")]
    pub struct GetFeeInverseCall;
    #[doc = "Container type for all input parameters for the `getMetaColony` function with signature `getMetaColony()` and selector `[115, 27, 194, 47]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getMetaColony", abi = "getMetaColony()")]
    pub struct GetMetaColonyCall;
    #[doc = "Container type for all input parameters for the `getMetatransactionNonce` function with signature `getMetatransactionNonce(address)` and selector `[179, 234, 193, 216]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "getMetatransactionNonce",
        abi = "getMetatransactionNonce(address)"
    )]
    pub struct GetMetatransactionNonceCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getParentSkillId` function with signature `getParentSkillId(uint256,uint256)` and selector `[217, 135, 252, 22]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getParentSkillId", abi = "getParentSkillId(uint256,uint256)")]
    pub struct GetParentSkillIdCall {
        pub skill_id: ethers::core::types::U256,
        pub parent_skill_index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getPayoutWhitelist` function with signature `getPayoutWhitelist(address)` and selector `[96, 83, 158, 242]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPayoutWhitelist", abi = "getPayoutWhitelist(address)")]
    pub struct GetPayoutWhitelistCall {
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getReputationMiningSkillId` function with signature `getReputationMiningSkillId()` and selector `[28, 70, 188, 127]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "getReputationMiningSkillId",
        abi = "getReputationMiningSkillId()"
    )]
    pub struct GetReputationMiningSkillIdCall;
    #[doc = "Container type for all input parameters for the `getReputationRootHash` function with signature `getReputationRootHash()` and selector `[151, 130, 76, 63]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getReputationRootHash", abi = "getReputationRootHash()")]
    pub struct GetReputationRootHashCall;
    #[doc = "Container type for all input parameters for the `getReputationRootHashNLeaves` function with signature `getReputationRootHashNLeaves()` and selector `[18, 98, 189, 25]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "getReputationRootHashNLeaves",
        abi = "getReputationRootHashNLeaves()"
    )]
    pub struct GetReputationRootHashNLeavesCall;
    #[doc = "Container type for all input parameters for the `getReputationRootHashNNodes` function with signature `getReputationRootHashNNodes()` and selector `[118, 128, 21, 13]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "getReputationRootHashNNodes",
        abi = "getReputationRootHashNNodes()"
    )]
    pub struct GetReputationRootHashNNodesCall;
    #[doc = "Container type for all input parameters for the `getSkill` function with signature `getSkill(uint256)` and selector `[189, 136, 15, 174]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getSkill", abi = "getSkill(uint256)")]
    pub struct GetSkillCall {
        pub skill_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getSkillCount` function with signature `getSkillCount()` and selector `[5, 245, 59, 41]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getSkillCount", abi = "getSkillCount()")]
    pub struct GetSkillCountCall;
    #[doc = "Container type for all input parameters for the `getTokenLocking` function with signature `getTokenLocking()` and selector `[179, 25, 144, 42]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getTokenLocking", abi = "getTokenLocking()")]
    pub struct GetTokenLockingCall;
    #[doc = "Container type for all input parameters for the `initialise` function with signature `initialise(address,uint256)` and selector `[59, 77, 32, 107]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initialise", abi = "initialise(address,uint256)")]
    pub struct InitialiseCall {
        pub resolver: ethers::core::types::Address,
        pub version: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `initialiseRootLocalSkill` function with signature `initialiseRootLocalSkill()` and selector `[37, 1, 171, 44]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initialiseRootLocalSkill", abi = "initialiseRootLocalSkill()")]
    pub struct InitialiseRootLocalSkillCall;
    #[doc = "Container type for all input parameters for the `isColony` function with signature `isColony(address)` and selector `[219, 13, 215, 255]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isColony", abi = "isColony(address)")]
    pub struct IsColonyCall {
        pub colony: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `multicall` function with signature `multicall(bytes[])` and selector `[172, 150, 80, 216]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "multicall", abi = "multicall(bytes[])")]
    pub struct MulticallCall {
        pub data: ::std::vec::Vec<ethers::core::types::Bytes>,
    }
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `setAuthority` function with signature `setAuthority(address)` and selector `[122, 158, 94, 75]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setAuthority", abi = "setAuthority(address)")]
    pub struct SetAuthorityCall {
        pub authority: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setFeeInverse` function with signature `setFeeInverse(uint256)` and selector `[56, 18, 178, 89]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setFeeInverse", abi = "setFeeInverse(uint256)")]
    pub struct SetFeeInverseCall {
        pub fee_inverse: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setOwner` function with signature `setOwner(address)` and selector `[19, 175, 64, 53]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setOwner", abi = "setOwner(address)")]
    pub struct SetOwnerCall {
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setPayoutWhitelist` function with signature `setPayoutWhitelist(address,bool)` and selector `[231, 135, 94, 168]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setPayoutWhitelist", abi = "setPayoutWhitelist(address,bool)")]
    pub struct SetPayoutWhitelistCall {
        pub token: ethers::core::types::Address,
        pub status: bool,
    }
    #[doc = "Container type for all input parameters for the `setTokenLocking` function with signature `setTokenLocking(address)` and selector `[143, 182, 131, 154]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setTokenLocking", abi = "setTokenLocking(address)")]
    pub struct SetTokenLockingCall {
        pub token_locking: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `verify` function with signature `verify(address,uint256,uint256,bytes,bytes32,bytes32,uint8)` and selector `[98, 129, 19, 61]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "verify",
        abi = "verify(address,uint256,uint256,bytes,bytes32,bytes32,uint8)"
    )]
    pub struct VerifyCall {
        pub owner: ethers::core::types::Address,
        pub nonce: ethers::core::types::U256,
        pub chain_id: ethers::core::types::U256,
        pub payload: ethers::core::types::Bytes,
        pub sig_r: [u8; 32],
        pub sig_s: [u8; 32],
        pub sig_v: u8,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ColonyNetworkCalls {
        AddColonyVersion(AddColonyVersionCall),
        AddSkill(AddSkillCall),
        AppendReputationUpdateLog(AppendReputationUpdateLogCall),
        Authority(AuthorityCall),
        CheckNotAdditionalProtectedVariable(CheckNotAdditionalProtectedVariableCall),
        DeprecateSkill(DeprecateSkillCall),
        DeprecateSkillWithDeprecated(DeprecateSkillWithDeprecatedCall),
        ExecuteMetaTransaction(ExecuteMetaTransactionCall),
        GetChainId(GetChainIdCall),
        GetChildSkillId(GetChildSkillIdCall),
        GetColony(GetColonyCall),
        GetColonyCount(GetColonyCountCall),
        GetColonyVersionResolver(GetColonyVersionResolverCall),
        GetCurrentColonyVersion(GetCurrentColonyVersionCall),
        GetFeeInverse(GetFeeInverseCall),
        GetMetaColony(GetMetaColonyCall),
        GetMetatransactionNonce(GetMetatransactionNonceCall),
        GetParentSkillId(GetParentSkillIdCall),
        GetPayoutWhitelist(GetPayoutWhitelistCall),
        GetReputationMiningSkillId(GetReputationMiningSkillIdCall),
        GetReputationRootHash(GetReputationRootHashCall),
        GetReputationRootHashNLeaves(GetReputationRootHashNLeavesCall),
        GetReputationRootHashNNodes(GetReputationRootHashNNodesCall),
        GetSkill(GetSkillCall),
        GetSkillCount(GetSkillCountCall),
        GetTokenLocking(GetTokenLockingCall),
        Initialise(InitialiseCall),
        InitialiseRootLocalSkill(InitialiseRootLocalSkillCall),
        IsColony(IsColonyCall),
        Multicall(MulticallCall),
        Owner(OwnerCall),
        SetAuthority(SetAuthorityCall),
        SetFeeInverse(SetFeeInverseCall),
        SetOwner(SetOwnerCall),
        SetPayoutWhitelist(SetPayoutWhitelistCall),
        SetTokenLocking(SetTokenLockingCall),
        Verify(VerifyCall),
    }
    impl ethers::core::abi::AbiDecode for ColonyNetworkCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddColonyVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::AddColonyVersion(decoded));
            }
            if let Ok(decoded) =
                <AddSkillCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::AddSkill(decoded));
            }
            if let Ok(decoded) =
                <AppendReputationUpdateLogCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyNetworkCalls::AppendReputationUpdateLog(decoded));
            }
            if let Ok(decoded) =
                <AuthorityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::Authority(decoded));
            }
            if let Ok(decoded) =
                <CheckNotAdditionalProtectedVariableCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyNetworkCalls::CheckNotAdditionalProtectedVariable(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <DeprecateSkillCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::DeprecateSkill(decoded));
            }
            if let Ok(decoded) =
                <DeprecateSkillWithDeprecatedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyNetworkCalls::DeprecateSkillWithDeprecated(decoded));
            }
            if let Ok(decoded) =
                <ExecuteMetaTransactionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::ExecuteMetaTransaction(decoded));
            }
            if let Ok(decoded) =
                <GetChainIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::GetChainId(decoded));
            }
            if let Ok(decoded) =
                <GetChildSkillIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::GetChildSkillId(decoded));
            }
            if let Ok(decoded) =
                <GetColonyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::GetColony(decoded));
            }
            if let Ok(decoded) =
                <GetColonyCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::GetColonyCount(decoded));
            }
            if let Ok(decoded) =
                <GetColonyVersionResolverCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyNetworkCalls::GetColonyVersionResolver(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentColonyVersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::GetCurrentColonyVersion(decoded));
            }
            if let Ok(decoded) =
                <GetFeeInverseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::GetFeeInverse(decoded));
            }
            if let Ok(decoded) =
                <GetMetaColonyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::GetMetaColony(decoded));
            }
            if let Ok(decoded) =
                <GetMetatransactionNonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::GetMetatransactionNonce(decoded));
            }
            if let Ok(decoded) =
                <GetParentSkillIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::GetParentSkillId(decoded));
            }
            if let Ok(decoded) =
                <GetPayoutWhitelistCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::GetPayoutWhitelist(decoded));
            }
            if let Ok(decoded) =
                <GetReputationMiningSkillIdCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyNetworkCalls::GetReputationMiningSkillId(decoded));
            }
            if let Ok(decoded) =
                <GetReputationRootHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::GetReputationRootHash(decoded));
            }
            if let Ok(decoded) =
                <GetReputationRootHashNLeavesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyNetworkCalls::GetReputationRootHashNLeaves(decoded));
            }
            if let Ok(decoded) =
                <GetReputationRootHashNNodesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyNetworkCalls::GetReputationRootHashNNodes(decoded));
            }
            if let Ok(decoded) =
                <GetSkillCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::GetSkill(decoded));
            }
            if let Ok(decoded) =
                <GetSkillCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::GetSkillCount(decoded));
            }
            if let Ok(decoded) =
                <GetTokenLockingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::GetTokenLocking(decoded));
            }
            if let Ok(decoded) =
                <InitialiseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::Initialise(decoded));
            }
            if let Ok(decoded) =
                <InitialiseRootLocalSkillCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyNetworkCalls::InitialiseRootLocalSkill(decoded));
            }
            if let Ok(decoded) =
                <IsColonyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::IsColony(decoded));
            }
            if let Ok(decoded) =
                <MulticallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::Multicall(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <SetAuthorityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::SetAuthority(decoded));
            }
            if let Ok(decoded) =
                <SetFeeInverseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::SetFeeInverse(decoded));
            }
            if let Ok(decoded) =
                <SetOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::SetOwner(decoded));
            }
            if let Ok(decoded) =
                <SetPayoutWhitelistCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::SetPayoutWhitelist(decoded));
            }
            if let Ok(decoded) =
                <SetTokenLockingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::SetTokenLocking(decoded));
            }
            if let Ok(decoded) = <VerifyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::Verify(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ColonyNetworkCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ColonyNetworkCalls::AddColonyVersion(element) => element.encode(),
                ColonyNetworkCalls::AddSkill(element) => element.encode(),
                ColonyNetworkCalls::AppendReputationUpdateLog(element) => element.encode(),
                ColonyNetworkCalls::Authority(element) => element.encode(),
                ColonyNetworkCalls::CheckNotAdditionalProtectedVariable(element) => {
                    element.encode()
                }
                ColonyNetworkCalls::DeprecateSkill(element) => element.encode(),
                ColonyNetworkCalls::DeprecateSkillWithDeprecated(element) => element.encode(),
                ColonyNetworkCalls::ExecuteMetaTransaction(element) => element.encode(),
                ColonyNetworkCalls::GetChainId(element) => element.encode(),
                ColonyNetworkCalls::GetChildSkillId(element) => element.encode(),
                ColonyNetworkCalls::GetColony(element) => element.encode(),
                ColonyNetworkCalls::GetColonyCount(element) => element.encode(),
                ColonyNetworkCalls::GetColonyVersionResolver(element) => element.encode(),
                ColonyNetworkCalls::GetCurrentColonyVersion(element) => element.encode(),
                ColonyNetworkCalls::GetFeeInverse(element) => element.encode(),
                ColonyNetworkCalls::GetMetaColony(element) => element.encode(),
                ColonyNetworkCalls::GetMetatransactionNonce(element) => element.encode(),
                ColonyNetworkCalls::GetParentSkillId(element) => element.encode(),
                ColonyNetworkCalls::GetPayoutWhitelist(element) => element.encode(),
                ColonyNetworkCalls::GetReputationMiningSkillId(element) => element.encode(),
                ColonyNetworkCalls::GetReputationRootHash(element) => element.encode(),
                ColonyNetworkCalls::GetReputationRootHashNLeaves(element) => element.encode(),
                ColonyNetworkCalls::GetReputationRootHashNNodes(element) => element.encode(),
                ColonyNetworkCalls::GetSkill(element) => element.encode(),
                ColonyNetworkCalls::GetSkillCount(element) => element.encode(),
                ColonyNetworkCalls::GetTokenLocking(element) => element.encode(),
                ColonyNetworkCalls::Initialise(element) => element.encode(),
                ColonyNetworkCalls::InitialiseRootLocalSkill(element) => element.encode(),
                ColonyNetworkCalls::IsColony(element) => element.encode(),
                ColonyNetworkCalls::Multicall(element) => element.encode(),
                ColonyNetworkCalls::Owner(element) => element.encode(),
                ColonyNetworkCalls::SetAuthority(element) => element.encode(),
                ColonyNetworkCalls::SetFeeInverse(element) => element.encode(),
                ColonyNetworkCalls::SetOwner(element) => element.encode(),
                ColonyNetworkCalls::SetPayoutWhitelist(element) => element.encode(),
                ColonyNetworkCalls::SetTokenLocking(element) => element.encode(),
                ColonyNetworkCalls::Verify(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ColonyNetworkCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ColonyNetworkCalls::AddColonyVersion(element) => element.fmt(f),
                ColonyNetworkCalls::AddSkill(element) => element.fmt(f),
                ColonyNetworkCalls::AppendReputationUpdateLog(element) => element.fmt(f),
                ColonyNetworkCalls::Authority(element) => element.fmt(f),
                ColonyNetworkCalls::CheckNotAdditionalProtectedVariable(element) => element.fmt(f),
                ColonyNetworkCalls::DeprecateSkill(element) => element.fmt(f),
                ColonyNetworkCalls::DeprecateSkillWithDeprecated(element) => element.fmt(f),
                ColonyNetworkCalls::ExecuteMetaTransaction(element) => element.fmt(f),
                ColonyNetworkCalls::GetChainId(element) => element.fmt(f),
                ColonyNetworkCalls::GetChildSkillId(element) => element.fmt(f),
                ColonyNetworkCalls::GetColony(element) => element.fmt(f),
                ColonyNetworkCalls::GetColonyCount(element) => element.fmt(f),
                ColonyNetworkCalls::GetColonyVersionResolver(element) => element.fmt(f),
                ColonyNetworkCalls::GetCurrentColonyVersion(element) => element.fmt(f),
                ColonyNetworkCalls::GetFeeInverse(element) => element.fmt(f),
                ColonyNetworkCalls::GetMetaColony(element) => element.fmt(f),
                ColonyNetworkCalls::GetMetatransactionNonce(element) => element.fmt(f),
                ColonyNetworkCalls::GetParentSkillId(element) => element.fmt(f),
                ColonyNetworkCalls::GetPayoutWhitelist(element) => element.fmt(f),
                ColonyNetworkCalls::GetReputationMiningSkillId(element) => element.fmt(f),
                ColonyNetworkCalls::GetReputationRootHash(element) => element.fmt(f),
                ColonyNetworkCalls::GetReputationRootHashNLeaves(element) => element.fmt(f),
                ColonyNetworkCalls::GetReputationRootHashNNodes(element) => element.fmt(f),
                ColonyNetworkCalls::GetSkill(element) => element.fmt(f),
                ColonyNetworkCalls::GetSkillCount(element) => element.fmt(f),
                ColonyNetworkCalls::GetTokenLocking(element) => element.fmt(f),
                ColonyNetworkCalls::Initialise(element) => element.fmt(f),
                ColonyNetworkCalls::InitialiseRootLocalSkill(element) => element.fmt(f),
                ColonyNetworkCalls::IsColony(element) => element.fmt(f),
                ColonyNetworkCalls::Multicall(element) => element.fmt(f),
                ColonyNetworkCalls::Owner(element) => element.fmt(f),
                ColonyNetworkCalls::SetAuthority(element) => element.fmt(f),
                ColonyNetworkCalls::SetFeeInverse(element) => element.fmt(f),
                ColonyNetworkCalls::SetOwner(element) => element.fmt(f),
                ColonyNetworkCalls::SetPayoutWhitelist(element) => element.fmt(f),
                ColonyNetworkCalls::SetTokenLocking(element) => element.fmt(f),
                ColonyNetworkCalls::Verify(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddColonyVersionCall> for ColonyNetworkCalls {
        fn from(var: AddColonyVersionCall) -> Self {
            ColonyNetworkCalls::AddColonyVersion(var)
        }
    }
    impl ::std::convert::From<AddSkillCall> for ColonyNetworkCalls {
        fn from(var: AddSkillCall) -> Self {
            ColonyNetworkCalls::AddSkill(var)
        }
    }
    impl ::std::convert::From<AppendReputationUpdateLogCall> for ColonyNetworkCalls {
        fn from(var: AppendReputationUpdateLogCall) -> Self {
            ColonyNetworkCalls::AppendReputationUpdateLog(var)
        }
    }
    impl ::std::convert::From<AuthorityCall> for ColonyNetworkCalls {
        fn from(var: AuthorityCall) -> Self {
            ColonyNetworkCalls::Authority(var)
        }
    }
    impl ::std::convert::From<CheckNotAdditionalProtectedVariableCall> for ColonyNetworkCalls {
        fn from(var: CheckNotAdditionalProtectedVariableCall) -> Self {
            ColonyNetworkCalls::CheckNotAdditionalProtectedVariable(var)
        }
    }
    impl ::std::convert::From<DeprecateSkillCall> for ColonyNetworkCalls {
        fn from(var: DeprecateSkillCall) -> Self {
            ColonyNetworkCalls::DeprecateSkill(var)
        }
    }
    impl ::std::convert::From<DeprecateSkillWithDeprecatedCall> for ColonyNetworkCalls {
        fn from(var: DeprecateSkillWithDeprecatedCall) -> Self {
            ColonyNetworkCalls::DeprecateSkillWithDeprecated(var)
        }
    }
    impl ::std::convert::From<ExecuteMetaTransactionCall> for ColonyNetworkCalls {
        fn from(var: ExecuteMetaTransactionCall) -> Self {
            ColonyNetworkCalls::ExecuteMetaTransaction(var)
        }
    }
    impl ::std::convert::From<GetChainIdCall> for ColonyNetworkCalls {
        fn from(var: GetChainIdCall) -> Self {
            ColonyNetworkCalls::GetChainId(var)
        }
    }
    impl ::std::convert::From<GetChildSkillIdCall> for ColonyNetworkCalls {
        fn from(var: GetChildSkillIdCall) -> Self {
            ColonyNetworkCalls::GetChildSkillId(var)
        }
    }
    impl ::std::convert::From<GetColonyCall> for ColonyNetworkCalls {
        fn from(var: GetColonyCall) -> Self {
            ColonyNetworkCalls::GetColony(var)
        }
    }
    impl ::std::convert::From<GetColonyCountCall> for ColonyNetworkCalls {
        fn from(var: GetColonyCountCall) -> Self {
            ColonyNetworkCalls::GetColonyCount(var)
        }
    }
    impl ::std::convert::From<GetColonyVersionResolverCall> for ColonyNetworkCalls {
        fn from(var: GetColonyVersionResolverCall) -> Self {
            ColonyNetworkCalls::GetColonyVersionResolver(var)
        }
    }
    impl ::std::convert::From<GetCurrentColonyVersionCall> for ColonyNetworkCalls {
        fn from(var: GetCurrentColonyVersionCall) -> Self {
            ColonyNetworkCalls::GetCurrentColonyVersion(var)
        }
    }
    impl ::std::convert::From<GetFeeInverseCall> for ColonyNetworkCalls {
        fn from(var: GetFeeInverseCall) -> Self {
            ColonyNetworkCalls::GetFeeInverse(var)
        }
    }
    impl ::std::convert::From<GetMetaColonyCall> for ColonyNetworkCalls {
        fn from(var: GetMetaColonyCall) -> Self {
            ColonyNetworkCalls::GetMetaColony(var)
        }
    }
    impl ::std::convert::From<GetMetatransactionNonceCall> for ColonyNetworkCalls {
        fn from(var: GetMetatransactionNonceCall) -> Self {
            ColonyNetworkCalls::GetMetatransactionNonce(var)
        }
    }
    impl ::std::convert::From<GetParentSkillIdCall> for ColonyNetworkCalls {
        fn from(var: GetParentSkillIdCall) -> Self {
            ColonyNetworkCalls::GetParentSkillId(var)
        }
    }
    impl ::std::convert::From<GetPayoutWhitelistCall> for ColonyNetworkCalls {
        fn from(var: GetPayoutWhitelistCall) -> Self {
            ColonyNetworkCalls::GetPayoutWhitelist(var)
        }
    }
    impl ::std::convert::From<GetReputationMiningSkillIdCall> for ColonyNetworkCalls {
        fn from(var: GetReputationMiningSkillIdCall) -> Self {
            ColonyNetworkCalls::GetReputationMiningSkillId(var)
        }
    }
    impl ::std::convert::From<GetReputationRootHashCall> for ColonyNetworkCalls {
        fn from(var: GetReputationRootHashCall) -> Self {
            ColonyNetworkCalls::GetReputationRootHash(var)
        }
    }
    impl ::std::convert::From<GetReputationRootHashNLeavesCall> for ColonyNetworkCalls {
        fn from(var: GetReputationRootHashNLeavesCall) -> Self {
            ColonyNetworkCalls::GetReputationRootHashNLeaves(var)
        }
    }
    impl ::std::convert::From<GetReputationRootHashNNodesCall> for ColonyNetworkCalls {
        fn from(var: GetReputationRootHashNNodesCall) -> Self {
            ColonyNetworkCalls::GetReputationRootHashNNodes(var)
        }
    }
    impl ::std::convert::From<GetSkillCall> for ColonyNetworkCalls {
        fn from(var: GetSkillCall) -> Self {
            ColonyNetworkCalls::GetSkill(var)
        }
    }
    impl ::std::convert::From<GetSkillCountCall> for ColonyNetworkCalls {
        fn from(var: GetSkillCountCall) -> Self {
            ColonyNetworkCalls::GetSkillCount(var)
        }
    }
    impl ::std::convert::From<GetTokenLockingCall> for ColonyNetworkCalls {
        fn from(var: GetTokenLockingCall) -> Self {
            ColonyNetworkCalls::GetTokenLocking(var)
        }
    }
    impl ::std::convert::From<InitialiseCall> for ColonyNetworkCalls {
        fn from(var: InitialiseCall) -> Self {
            ColonyNetworkCalls::Initialise(var)
        }
    }
    impl ::std::convert::From<InitialiseRootLocalSkillCall> for ColonyNetworkCalls {
        fn from(var: InitialiseRootLocalSkillCall) -> Self {
            ColonyNetworkCalls::InitialiseRootLocalSkill(var)
        }
    }
    impl ::std::convert::From<IsColonyCall> for ColonyNetworkCalls {
        fn from(var: IsColonyCall) -> Self {
            ColonyNetworkCalls::IsColony(var)
        }
    }
    impl ::std::convert::From<MulticallCall> for ColonyNetworkCalls {
        fn from(var: MulticallCall) -> Self {
            ColonyNetworkCalls::Multicall(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for ColonyNetworkCalls {
        fn from(var: OwnerCall) -> Self {
            ColonyNetworkCalls::Owner(var)
        }
    }
    impl ::std::convert::From<SetAuthorityCall> for ColonyNetworkCalls {
        fn from(var: SetAuthorityCall) -> Self {
            ColonyNetworkCalls::SetAuthority(var)
        }
    }
    impl ::std::convert::From<SetFeeInverseCall> for ColonyNetworkCalls {
        fn from(var: SetFeeInverseCall) -> Self {
            ColonyNetworkCalls::SetFeeInverse(var)
        }
    }
    impl ::std::convert::From<SetOwnerCall> for ColonyNetworkCalls {
        fn from(var: SetOwnerCall) -> Self {
            ColonyNetworkCalls::SetOwner(var)
        }
    }
    impl ::std::convert::From<SetPayoutWhitelistCall> for ColonyNetworkCalls {
        fn from(var: SetPayoutWhitelistCall) -> Self {
            ColonyNetworkCalls::SetPayoutWhitelist(var)
        }
    }
    impl ::std::convert::From<SetTokenLockingCall> for ColonyNetworkCalls {
        fn from(var: SetTokenLockingCall) -> Self {
            ColonyNetworkCalls::SetTokenLocking(var)
        }
    }
    impl ::std::convert::From<VerifyCall> for ColonyNetworkCalls {
        fn from(var: VerifyCall) -> Self {
            ColonyNetworkCalls::Verify(var)
        }
    }
    #[doc = "Container type for all return fields from the `addSkill` function with signature `addSkill(uint256)` and selector `[22, 36, 25, 204]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AddSkillReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `authority` function with signature `authority()` and selector `[191, 126, 33, 79]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AuthorityReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `deprecateSkill` function with signature `deprecateSkill(uint256,bool)` and selector `[210, 11, 21, 107]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DeprecateSkillWithDeprecatedReturn(pub bool);
    #[doc = "Container type for all return fields from the `executeMetaTransaction` function with signature `executeMetaTransaction(address,bytes,bytes32,bytes32,uint8)` and selector `[12, 83, 197, 28]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ExecuteMetaTransactionReturn(pub ethers::core::types::Bytes);
    #[doc = "Container type for all return fields from the `getChainId` function with signature `getChainId()` and selector `[52, 8, 228, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetChainIdReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getChildSkillId` function with signature `getChildSkillId(uint256,uint256)` and selector `[9, 209, 10, 94]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetChildSkillIdReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getColony` function with signature `getColony(uint256)` and selector `[111, 146, 101, 13]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetColonyReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getColonyCount` function with signature `getColonyCount()` and selector `[113, 26, 57, 190]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetColonyCountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getColonyVersionResolver` function with signature `getColonyVersionResolver(uint256)` and selector `[143, 20, 200, 134]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetColonyVersionResolverReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getCurrentColonyVersion` function with signature `getCurrentColonyVersion()` and selector `[188, 161, 228, 223]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCurrentColonyVersionReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getFeeInverse` function with signature `getFeeInverse()` and selector `[239, 99, 195, 128]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetFeeInverseReturn {
        pub fee_inverse: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getMetaColony` function with signature `getMetaColony()` and selector `[115, 27, 194, 47]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetMetaColonyReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getMetatransactionNonce` function with signature `getMetatransactionNonce(address)` and selector `[179, 234, 193, 216]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetMetatransactionNonceReturn {
        pub nonce: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getParentSkillId` function with signature `getParentSkillId(uint256,uint256)` and selector `[217, 135, 252, 22]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetParentSkillIdReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getPayoutWhitelist` function with signature `getPayoutWhitelist(address)` and selector `[96, 83, 158, 242]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetPayoutWhitelistReturn(pub bool);
    #[doc = "Container type for all return fields from the `getReputationMiningSkillId` function with signature `getReputationMiningSkillId()` and selector `[28, 70, 188, 127]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetReputationMiningSkillIdReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getReputationRootHash` function with signature `getReputationRootHash()` and selector `[151, 130, 76, 63]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetReputationRootHashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `getReputationRootHashNLeaves` function with signature `getReputationRootHashNLeaves()` and selector `[18, 98, 189, 25]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetReputationRootHashNLeavesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getReputationRootHashNNodes` function with signature `getReputationRootHashNNodes()` and selector `[118, 128, 21, 13]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetReputationRootHashNNodesReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getSkill` function with signature `getSkill(uint256)` and selector `[189, 136, 15, 174]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetSkillReturn {
        pub skill: Skill,
    }
    #[doc = "Container type for all return fields from the `getSkillCount` function with signature `getSkillCount()` and selector `[5, 245, 59, 41]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetSkillCountReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getTokenLocking` function with signature `getTokenLocking()` and selector `[179, 25, 144, 42]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetTokenLockingReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `initialiseRootLocalSkill` function with signature `initialiseRootLocalSkill()` and selector `[37, 1, 171, 44]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct InitialiseRootLocalSkillReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `isColony` function with signature `isColony(address)` and selector `[219, 13, 215, 255]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsColonyReturn(pub bool);
    #[doc = "Container type for all return fields from the `multicall` function with signature `multicall(bytes[])` and selector `[172, 150, 80, 216]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MulticallReturn {
        pub results: ::std::vec::Vec<ethers::core::types::Bytes>,
    }
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `verify` function with signature `verify(address,uint256,uint256,bytes,bytes32,bytes32,uint8)` and selector `[98, 129, 19, 61]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VerifyReturn(pub bool);
    #[doc = "`Skill(uint128,uint128,uint256[],uint256[],bool,bool)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Skill {
        pub n_parents: u128,
        pub n_children: u128,
        pub parents: Vec<ethers::core::types::U256>,
        pub children: Vec<ethers::core::types::U256>,
        pub global_skill: bool,
        pub deprecated: bool,
    }
}
