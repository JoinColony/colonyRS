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
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"auction\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"quantity\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AuctionCreated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"colonyId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"colonyAddress\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ColonyAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"colony\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"label\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ColonyLabelRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"resolver\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ColonyNetworkInitialised\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"version\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"resolver\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ColonyVersionAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"extensionId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"version\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExtensionAddedToNetwork\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"extensionId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"colony\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"deprecated\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExtensionDeprecated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"extensionId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"colony\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"version\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExtensionInstalled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"extensionId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"colony\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ExtensionUninstalled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"extensionId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"colony\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"version\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExtensionUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"metaColony\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"rootSkillId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MetaColonyCreated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"userAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address payable\",\"name\":\"relayerAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MetaTransactionExecuted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"miningCycleResolver\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MiningCycleResolverSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"feeInverse\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"NetworkFeeInverseSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RecoveryModeEntered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RecoveryModeExitApproved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RecoveryModeExited\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"setTo\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RecoveryRoleSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"slot\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"fromValue\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"toValue\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RecoveryStorageSlotSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"ens\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"rootNode\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RegistrarInitialised\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"miner\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"tokensLost\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReputationMinerPenalised\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"hash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"nLeaves\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReputationMiningCycleComplete\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"inactiveReputationMiningCycle\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReputationMiningInitialised\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReputationMiningRewardSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"newHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newNLeaves\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address[]\",\"name\":\"stakers\",\"type\":\"address[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"reward\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReputationRootHashSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"skillId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"parentSkillId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SkillAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAuthorityAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenAuthorityDeployed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenDeployed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"tokenLocking\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenLockingAddressSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"status\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenWhitelisted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"label\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"UserLabelRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approveExitRecovery\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_slot\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"checkNotAdditionalProtectedVariable\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enterRecoveryMode\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"userAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"sigR\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"sigS\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"sigV\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"executeMetaTransaction\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exitRecoveryMode\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"userAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMetatransactionNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isInRecoveryMode\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"inRecoveryMode\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes[]\",\"name\":\"data\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"multicall\",\"outputs\":[{\"internalType\":\"bytes[]\",\"name\":\"results\",\"type\":\"bytes[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"numRecoveryRoles\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"numRoles\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeRecoveryRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRecoveryRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_slot\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_value\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setStorageSlotRecovery\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"_interfaceID\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"_status\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_reputationMiningCycle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_amount\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_colony\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"_nUpdates\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"_nPreviousUpdates\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setReplacementReputationUpdateLogEntry\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_reputationMiningCycle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReplacementReputationUpdateLogEntry\",\"outputs\":[{\"internalType\":\"struct ColonyNetworkDataTypes.ReputationLogEntry\",\"name\":\"_reputationLogEntry\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"amount\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"skillId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"colony\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"nUpdates\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"nPreviousUpdates\",\"type\":\"uint128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_reputationMiningCycle\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReplacementReputationUpdateLogsExist\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"_exists\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMetaColony\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"_colonyAddress\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getColonyCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"_count\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_colony\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isColony\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"_addressIsColony\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_parentSkillId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addSkill\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSkill\",\"outputs\":[{\"internalType\":\"struct ColonyNetworkDataTypes.Skill\",\"name\":\"_skill\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint128\",\"name\":\"nParents\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"nChildren\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"parents\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"children\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"globalSkill\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"deprecated\",\"type\":\"bool\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deprecateSkill\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_deprecated\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deprecateSkill\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"_changed\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialiseRootLocalSkill\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"_rootLocalSkillId\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_amount\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"appendReputationUpdateLog\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSkillCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"_count\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReputationMiningSkillId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenLockingAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTokenLocking\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTokenLocking\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_lockingAddress\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createMetaColony\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createColony\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_colonyAddress\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_version\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_colonyName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_metadata\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createColony\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_colonyAddress\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_version\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_colonyName\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createColony\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_colonyAddress\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_version\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_colonyName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_orbitdb\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_useExtensionManager\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createColony\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_colonyAddress\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_decimals\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_version\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_colonyName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_metadata\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createColonyForFrontend\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"colony\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_version\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_resolver\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addColonyVersion\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_resolver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_version\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialise\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getColony\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_colonyAddress\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentColonyVersion\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"_version\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_parentSkillIndex\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getParentSkillId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"_parentSkillId\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getChildSkillId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"_childSkillId\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"_active\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReputationMiningCycle\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_repMiningCycleAddress\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_timeStaked\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_submissonIndex\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"calculateMinerWeight\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"_minerWeight\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_version\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getColonyVersionResolver\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_resolverAddress\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_newHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_newNLeaves\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"_stakers\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setReputationRootHash\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startNextCycle\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialiseReputationMining\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReputationRootHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"rootHash\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReputationRootHashNLeaves\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"nLeaves\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReputationRootHashNNodes\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"nNodes\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startTokenAuction\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_ens\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_rootNode\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setupRegistrar\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_username\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_orbitdb\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerUserLabel\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_colonyName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_orbitdb\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerColonyLabel\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_orbitdb\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateColonyOrbitDB\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_orbitdb\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateUserOrbitDB\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_node\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getProfileDBAddress\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"_orbitdb\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_addr\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lookupRegisteredENSDomain\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"_domain\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_node\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"addr\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getENSRegistrar\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_miningResolverAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMiningResolver\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMiningResolver\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"miningResolverAddress\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_extensionId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_resolver\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addExtensionToNetwork\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_extensionId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_version\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"installExtension\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_extensionId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_newVersion\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgradeExtension\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_extensionId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_deprecated\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deprecateExtension\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_extensionId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"uninstallExtension\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_extensionId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_version\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getExtensionResolver\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_resolver\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_extensionId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_colony\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getExtensionInstallation\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_installation\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFeeInverse\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"_feeInverse\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_feeInverse\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFeeInverse\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPayoutWhitelist\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"_status\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_status\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPayoutWhitelist\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"_stakers\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"punishStakers\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"stakeForMining\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unstakeForMining\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMiningStake\",\"outputs\":[{\"internalType\":\"struct ColonyNetworkDataTypes.MiningStake\",\"name\":\"_info\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"reward\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burnUnneededRewards\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimMiningReward\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setReputationMiningCycleReward\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getReputationMiningCycleReward\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_symbol\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_decimals\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deployTokenViaNetwork\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_colony\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"_allowedToTransfer\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deployTokenAuthority\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_tokenAuthority\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_delegate\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_allowed\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMiningDelegate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_delegate\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMiningDelegator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_delegator\",\"type\":\"address\",\"components\":[]}]}]" ;
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
        #[doc = "Calls the contract's `addExtensionToNetwork` (0x886484f7) function"]
        pub fn add_extension_to_network(
            &self,
            extension_id: [u8; 32],
            resolver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([136, 100, 132, 247], (extension_id, resolver))
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
        #[doc = "Calls the contract's `addr` (0x3b3b57de) function"]
        pub fn addr(
            &self,
            node: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([59, 59, 87, 222], node)
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
        #[doc = "Calls the contract's `approveExitRecovery` (0x31292ba7) function"]
        pub fn approve_exit_recovery(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([49, 41, 43, 167], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burnUnneededRewards` (0xfff92e6f) function"]
        pub fn burn_unneeded_rewards(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([255, 249, 46, 111], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calculateMinerWeight` (0x8e4544d7) function"]
        pub fn calculate_miner_weight(
            &self,
            time_staked: ethers::core::types::U256,
            submisson_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([142, 69, 68, 215], (time_staked, submisson_index))
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
        #[doc = "Calls the contract's `claimMiningReward` (0x63c2a510) function"]
        pub fn claim_mining_reward(
            &self,
            recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 194, 165, 16], recipient)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createColony` (0x08eb0d2c) function"]
        pub fn create_colony_0(
            &self,
            token_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([8, 235, 13, 44], token_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createColony` (0x0e0049a0) function"]
        pub fn create_colony_2(
            &self,
            token_address: ethers::core::types::Address,
            version: ethers::core::types::U256,
            colony_name: String,
            metadata: String,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash(
                    [14, 0, 73, 160],
                    (token_address, version, colony_name, metadata),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createColony` (0x17a08518) function"]
        pub fn create_colony_1(
            &self,
            token_address: ethers::core::types::Address,
            version: ethers::core::types::U256,
            colony_name: String,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([23, 160, 133, 24], (token_address, version, colony_name))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createColony` (0x9a55bfdc) function"]
        pub fn create_colony_3(
            &self,
            token_address: ethers::core::types::Address,
            version: ethers::core::types::U256,
            colony_name: String,
            orbitdb: String,
            use_extension_manager: bool,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash(
                    [154, 85, 191, 220],
                    (
                        token_address,
                        version,
                        colony_name,
                        orbitdb,
                        use_extension_manager,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createColonyForFrontend` (0x685987df) function"]
        pub fn create_colony_for_frontend(
            &self,
            token_address: ethers::core::types::Address,
            name: String,
            symbol: String,
            decimals: u8,
            version: ethers::core::types::U256,
            colony_name: String,
            metadata: String,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::Address, ethers::core::types::Address),
        > {
            self.0
                .method_hash(
                    [104, 89, 135, 223],
                    (
                        token_address,
                        name,
                        symbol,
                        decimals,
                        version,
                        colony_name,
                        metadata,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createMetaColony` (0xa2f99b86) function"]
        pub fn create_meta_colony(
            &self,
            token_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 249, 155, 134], token_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deployTokenAuthority` (0x5ac54408) function"]
        pub fn deploy_token_authority(
            &self,
            token: ethers::core::types::Address,
            colony: ethers::core::types::Address,
            allowed_to_transfer: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([90, 197, 68, 8], (token, colony, allowed_to_transfer))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deployTokenViaNetwork` (0x4e51691a) function"]
        pub fn deploy_token_via_network(
            &self,
            name: String,
            symbol: String,
            decimals: u8,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([78, 81, 105, 26], (name, symbol, decimals))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deprecateExtension` (0x04eaa104) function"]
        pub fn deprecate_extension(
            &self,
            extension_id: [u8; 32],
            deprecated: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([4, 234, 161, 4], (extension_id, deprecated))
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
        #[doc = "Calls the contract's `enterRecoveryMode` (0xe77aaee2) function"]
        pub fn enter_recovery_mode(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 122, 174, 226], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeMetaTransaction` (0x0c53c51c) function"]
        pub fn execute_meta_transaction(
            &self,
            user_address: ethers::core::types::Address,
            payload: ethers::core::types::Bytes,
            sig_r: [u8; 32],
            sig_s: [u8; 32],
            sig_v: u8,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash(
                    [12, 83, 197, 28],
                    (user_address, payload, sig_r, sig_s, sig_v),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exitRecoveryMode` (0x38e5b7ae) function"]
        pub fn exit_recovery_mode(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 229, 183, 174], ())
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
        #[doc = "Calls the contract's `getENSRegistrar` (0xa5e42536) function"]
        pub fn get_ens_registrar(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([165, 228, 37, 54], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getExtensionInstallation` (0xdc6095ae) function"]
        pub fn get_extension_installation(
            &self,
            extension_id: [u8; 32],
            colony: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([220, 96, 149, 174], (extension_id, colony))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getExtensionResolver` (0x71be7b25) function"]
        pub fn get_extension_resolver(
            &self,
            extension_id: [u8; 32],
            version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([113, 190, 123, 37], (extension_id, version))
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
            user_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([179, 234, 193, 216], user_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMiningDelegator` (0xacb377a2) function"]
        pub fn get_mining_delegator(
            &self,
            delegate: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([172, 179, 119, 162], delegate)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMiningResolver` (0x749f15cd) function"]
        pub fn get_mining_resolver(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([116, 159, 21, 205], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMiningStake` (0x09ffb3cb) function"]
        pub fn get_mining_stake(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, MiningStake> {
            self.0
                .method_hash([9, 255, 179, 203], user)
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
        #[doc = "Calls the contract's `getProfileDBAddress` (0xcd930b63) function"]
        pub fn get_profile_db_address(
            &self,
            node: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([205, 147, 11, 99], node)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReplacementReputationUpdateLogEntry` (0xe926f61c) function"]
        pub fn get_replacement_reputation_update_log_entry(
            &self,
            reputation_mining_cycle: ethers::core::types::Address,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ReputationLogEntry> {
            self.0
                .method_hash([233, 38, 246, 28], (reputation_mining_cycle, id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReplacementReputationUpdateLogsExist` (0x1d5b14e3) function"]
        pub fn get_replacement_reputation_update_logs_exist(
            &self,
            reputation_mining_cycle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([29, 91, 20, 227], reputation_mining_cycle)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReputationMiningCycle` (0xf2c8599c) function"]
        pub fn get_reputation_mining_cycle(
            &self,
            active: bool,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([242, 200, 89, 156], active)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReputationMiningCycleReward` (0x6f51d85c) function"]
        pub fn get_reputation_mining_cycle_reward(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([111, 81, 216, 92], ())
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
        #[doc = "Calls the contract's `initialiseReputationMining` (0xee65786a) function"]
        pub fn initialise_reputation_mining(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([238, 101, 120, 106], ())
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
        #[doc = "Calls the contract's `installExtension` (0x9dec0910) function"]
        pub fn install_extension(
            &self,
            extension_id: [u8; 32],
            version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 236, 9, 16], (extension_id, version))
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
        #[doc = "Calls the contract's `isInRecoveryMode` (0x80d9fad3) function"]
        pub fn is_in_recovery_mode(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([128, 217, 250, 211], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lookupRegisteredENSDomain` (0x60c64776) function"]
        pub fn lookup_registered_ens_domain(
            &self,
            addr: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([96, 198, 71, 118], addr)
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
        #[doc = "Calls the contract's `numRecoveryRoles` (0x54584330) function"]
        pub fn num_recovery_roles(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([84, 88, 67, 48], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `punishStakers` (0x0f96995d) function"]
        pub fn punish_stakers(
            &self,
            stakers: ::std::vec::Vec<ethers::core::types::Address>,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 150, 153, 93], (stakers, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `registerColonyLabel` (0xe2db777b) function"]
        pub fn register_colony_label(
            &self,
            colony_name: String,
            orbitdb: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 219, 119, 123], (colony_name, orbitdb))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `registerUserLabel` (0xb5e34fbf) function"]
        pub fn register_user_label(
            &self,
            username: String,
            orbitdb: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 227, 79, 191], (username, orbitdb))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeRecoveryRole` (0x99cf7cc7) function"]
        pub fn remove_recovery_role(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 207, 124, 199], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `reward` (0x21670f22) function"]
        pub fn reward(
            &self,
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([33, 103, 15, 34], (recipient, amount))
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
        #[doc = "Calls the contract's `setMiningDelegate` (0xf584d17d) function"]
        pub fn set_mining_delegate(
            &self,
            delegate: ethers::core::types::Address,
            allowed: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 132, 209, 125], (delegate, allowed))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMiningResolver` (0xe1636882) function"]
        pub fn set_mining_resolver(
            &self,
            mining_resolver_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 99, 104, 130], mining_resolver_address)
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
        #[doc = "Calls the contract's `setRecoveryRole` (0xd24536a5) function"]
        pub fn set_recovery_role(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 69, 54, 165], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setReplacementReputationUpdateLogEntry` (0x23100714) function"]
        pub fn set_replacement_reputation_update_log_entry(
            &self,
            reputation_mining_cycle: ethers::core::types::Address,
            id: ethers::core::types::U256,
            user: ethers::core::types::Address,
            amount: I256,
            skill_id: ethers::core::types::U256,
            colony: ethers::core::types::Address,
            n_updates: u128,
            n_previous_updates: u128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [35, 16, 7, 20],
                    (
                        reputation_mining_cycle,
                        id,
                        user,
                        amount,
                        skill_id,
                        colony,
                        n_updates,
                        n_previous_updates,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setReputationMiningCycleReward` (0x1d9a5536) function"]
        pub fn set_reputation_mining_cycle_reward(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 154, 85, 54], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setReputationRootHash` (0x743ed431) function"]
        pub fn set_reputation_root_hash(
            &self,
            new_hash: [u8; 32],
            new_n_leaves: ethers::core::types::U256,
            stakers: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([116, 62, 212, 49], (new_hash, new_n_leaves, stakers))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setStorageSlotRecovery` (0x55028923) function"]
        pub fn set_storage_slot_recovery(
            &self,
            slot: ethers::core::types::U256,
            value: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 2, 137, 35], (slot, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTokenLocking` (0x8fb6839a) function"]
        pub fn set_token_locking(
            &self,
            token_locking_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([143, 182, 131, 154], token_locking_address)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setupRegistrar` (0x6060c4aa) function"]
        pub fn setup_registrar(
            &self,
            ens: ethers::core::types::Address,
            root_node: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([96, 96, 196, 170], (ens, root_node))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stakeForMining` (0x9dfc8e01) function"]
        pub fn stake_for_mining(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 252, 142, 1], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `startNextCycle` (0x450e2ed3) function"]
        pub fn start_next_cycle(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([69, 14, 46, 211], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `startTokenAuction` (0x55a8495a) function"]
        pub fn start_token_auction(
            &self,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 168, 73, 90], token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supportsInterface` (0x01ffc9a7) function"]
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `uninstallExtension` (0xc2308c49) function"]
        pub fn uninstall_extension(
            &self,
            extension_id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 48, 140, 73], extension_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unstakeForMining` (0x1879bde7) function"]
        pub fn unstake_for_mining(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 121, 189, 231], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateColonyOrbitDB` (0x28672cea) function"]
        pub fn update_colony_orbit_db(
            &self,
            orbitdb: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([40, 103, 44, 234], orbitdb)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateUserOrbitDB` (0xed26b557) function"]
        pub fn update_user_orbit_db(
            &self,
            orbitdb: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([237, 38, 181, 87], orbitdb)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `upgradeExtension` (0xd9bd780b) function"]
        pub fn upgrade_extension(
            &self,
            extension_id: [u8; 32],
            new_version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([217, 189, 120, 11], (extension_id, new_version))
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
        #[doc = "Gets the contract's `RecoveryModeEntered` event"]
        pub fn recovery_mode_entered_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RecoveryModeEnteredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RecoveryModeExitApproved` event"]
        pub fn recovery_mode_exit_approved_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RecoveryModeExitApprovedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RecoveryModeExited` event"]
        pub fn recovery_mode_exited_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RecoveryModeExitedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RecoveryRoleSet` event"]
        pub fn recovery_role_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RecoveryRoleSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RecoveryStorageSlotSet` event"]
        pub fn recovery_storage_slot_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RecoveryStorageSlotSetFilter> {
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
        pub user_address: ethers::core::types::Address,
        pub relayer_address: ethers::core::types::Address,
        pub payload: ethers::core::types::Bytes,
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
    #[ethevent(name = "RecoveryModeEntered", abi = "RecoveryModeEntered(address)")]
    pub struct RecoveryModeEnteredFilter {
        pub user: ethers::core::types::Address,
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
        name = "RecoveryModeExitApproved",
        abi = "RecoveryModeExitApproved(address)"
    )]
    pub struct RecoveryModeExitApprovedFilter {
        pub user: ethers::core::types::Address,
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
    #[ethevent(name = "RecoveryModeExited", abi = "RecoveryModeExited(address)")]
    pub struct RecoveryModeExitedFilter {
        pub user: ethers::core::types::Address,
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
    #[ethevent(name = "RecoveryRoleSet", abi = "RecoveryRoleSet(address,bool)")]
    pub struct RecoveryRoleSetFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        pub set_to: bool,
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
        name = "RecoveryStorageSlotSet",
        abi = "RecoveryStorageSlotSet(address,uint256,bytes32,bytes32)"
    )]
    pub struct RecoveryStorageSlotSetFilter {
        pub user: ethers::core::types::Address,
        pub slot: ethers::core::types::U256,
        pub from_value: [u8; 32],
        pub to_value: [u8; 32],
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
        MetaColonyCreatedFilter(MetaColonyCreatedFilter),
        MetaTransactionExecutedFilter(MetaTransactionExecutedFilter),
        MiningCycleResolverSetFilter(MiningCycleResolverSetFilter),
        NetworkFeeInverseSetFilter(NetworkFeeInverseSetFilter),
        RecoveryModeEnteredFilter(RecoveryModeEnteredFilter),
        RecoveryModeExitApprovedFilter(RecoveryModeExitApprovedFilter),
        RecoveryModeExitedFilter(RecoveryModeExitedFilter),
        RecoveryRoleSetFilter(RecoveryRoleSetFilter),
        RecoveryStorageSlotSetFilter(RecoveryStorageSlotSetFilter),
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
            if let Ok(decoded) = RecoveryModeEnteredFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::RecoveryModeEnteredFilter(decoded));
            }
            if let Ok(decoded) = RecoveryModeExitApprovedFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::RecoveryModeExitApprovedFilter(decoded));
            }
            if let Ok(decoded) = RecoveryModeExitedFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::RecoveryModeExitedFilter(decoded));
            }
            if let Ok(decoded) = RecoveryRoleSetFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::RecoveryRoleSetFilter(decoded));
            }
            if let Ok(decoded) = RecoveryStorageSlotSetFilter::decode_log(log) {
                return Ok(ColonyNetworkEvents::RecoveryStorageSlotSetFilter(decoded));
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
                ColonyNetworkEvents::MetaColonyCreatedFilter(element) => element.fmt(f),
                ColonyNetworkEvents::MetaTransactionExecutedFilter(element) => element.fmt(f),
                ColonyNetworkEvents::MiningCycleResolverSetFilter(element) => element.fmt(f),
                ColonyNetworkEvents::NetworkFeeInverseSetFilter(element) => element.fmt(f),
                ColonyNetworkEvents::RecoveryModeEnteredFilter(element) => element.fmt(f),
                ColonyNetworkEvents::RecoveryModeExitApprovedFilter(element) => element.fmt(f),
                ColonyNetworkEvents::RecoveryModeExitedFilter(element) => element.fmt(f),
                ColonyNetworkEvents::RecoveryRoleSetFilter(element) => element.fmt(f),
                ColonyNetworkEvents::RecoveryStorageSlotSetFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `addExtensionToNetwork` function with signature `addExtensionToNetwork(bytes32,address)` and selector `[136, 100, 132, 247]`"]
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
        name = "addExtensionToNetwork",
        abi = "addExtensionToNetwork(bytes32,address)"
    )]
    pub struct AddExtensionToNetworkCall {
        pub extension_id: [u8; 32],
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
    #[doc = "Container type for all input parameters for the `addr` function with signature `addr(bytes32)` and selector `[59, 59, 87, 222]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "addr", abi = "addr(bytes32)")]
    pub struct AddrCall {
        pub node: [u8; 32],
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
    #[doc = "Container type for all input parameters for the `approveExitRecovery` function with signature `approveExitRecovery()` and selector `[49, 41, 43, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "approveExitRecovery", abi = "approveExitRecovery()")]
    pub struct ApproveExitRecoveryCall;
    #[doc = "Container type for all input parameters for the `burnUnneededRewards` function with signature `burnUnneededRewards(uint256)` and selector `[255, 249, 46, 111]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "burnUnneededRewards", abi = "burnUnneededRewards(uint256)")]
    pub struct BurnUnneededRewardsCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `calculateMinerWeight` function with signature `calculateMinerWeight(uint256,uint256)` and selector `[142, 69, 68, 215]`"]
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
        name = "calculateMinerWeight",
        abi = "calculateMinerWeight(uint256,uint256)"
    )]
    pub struct CalculateMinerWeightCall {
        pub time_staked: ethers::core::types::U256,
        pub submisson_index: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all input parameters for the `claimMiningReward` function with signature `claimMiningReward(address)` and selector `[99, 194, 165, 16]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "claimMiningReward", abi = "claimMiningReward(address)")]
    pub struct ClaimMiningRewardCall {
        pub recipient: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `createColony` function with signature `createColony(address)` and selector `[8, 235, 13, 44]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "createColony", abi = "createColony(address)")]
    pub struct CreateColony0Call {
        pub token_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `createColony` function with signature `createColony(address,uint256,string,string)` and selector `[14, 0, 73, 160]`"]
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
        name = "createColony",
        abi = "createColony(address,uint256,string,string)"
    )]
    pub struct CreateColony2Call {
        pub token_address: ethers::core::types::Address,
        pub version: ethers::core::types::U256,
        pub colony_name: String,
        pub metadata: String,
    }
    #[doc = "Container type for all input parameters for the `createColony` function with signature `createColony(address,uint256,string)` and selector `[23, 160, 133, 24]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "createColony", abi = "createColony(address,uint256,string)")]
    pub struct CreateColony1Call {
        pub token_address: ethers::core::types::Address,
        pub version: ethers::core::types::U256,
        pub colony_name: String,
    }
    #[doc = "Container type for all input parameters for the `createColony` function with signature `createColony(address,uint256,string,string,bool)` and selector `[154, 85, 191, 220]`"]
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
        name = "createColony",
        abi = "createColony(address,uint256,string,string,bool)"
    )]
    pub struct CreateColony3Call {
        pub token_address: ethers::core::types::Address,
        pub version: ethers::core::types::U256,
        pub colony_name: String,
        pub orbitdb: String,
        pub use_extension_manager: bool,
    }
    #[doc = "Container type for all input parameters for the `createColonyForFrontend` function with signature `createColonyForFrontend(address,string,string,uint8,uint256,string,string)` and selector `[104, 89, 135, 223]`"]
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
        name = "createColonyForFrontend",
        abi = "createColonyForFrontend(address,string,string,uint8,uint256,string,string)"
    )]
    pub struct CreateColonyForFrontendCall {
        pub token_address: ethers::core::types::Address,
        pub name: String,
        pub symbol: String,
        pub decimals: u8,
        pub version: ethers::core::types::U256,
        pub colony_name: String,
        pub metadata: String,
    }
    #[doc = "Container type for all input parameters for the `createMetaColony` function with signature `createMetaColony(address)` and selector `[162, 249, 155, 134]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "createMetaColony", abi = "createMetaColony(address)")]
    pub struct CreateMetaColonyCall {
        pub token_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `deployTokenAuthority` function with signature `deployTokenAuthority(address,address,address[])` and selector `[90, 197, 68, 8]`"]
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
        name = "deployTokenAuthority",
        abi = "deployTokenAuthority(address,address,address[])"
    )]
    pub struct DeployTokenAuthorityCall {
        pub token: ethers::core::types::Address,
        pub colony: ethers::core::types::Address,
        pub allowed_to_transfer: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `deployTokenViaNetwork` function with signature `deployTokenViaNetwork(string,string,uint8)` and selector `[78, 81, 105, 26]`"]
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
        name = "deployTokenViaNetwork",
        abi = "deployTokenViaNetwork(string,string,uint8)"
    )]
    pub struct DeployTokenViaNetworkCall {
        pub name: String,
        pub symbol: String,
        pub decimals: u8,
    }
    #[doc = "Container type for all input parameters for the `deprecateExtension` function with signature `deprecateExtension(bytes32,bool)` and selector `[4, 234, 161, 4]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "deprecateExtension", abi = "deprecateExtension(bytes32,bool)")]
    pub struct DeprecateExtensionCall {
        pub extension_id: [u8; 32],
        pub deprecated: bool,
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
    #[doc = "Container type for all input parameters for the `enterRecoveryMode` function with signature `enterRecoveryMode()` and selector `[231, 122, 174, 226]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "enterRecoveryMode", abi = "enterRecoveryMode()")]
    pub struct EnterRecoveryModeCall;
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
        pub user_address: ethers::core::types::Address,
        pub payload: ethers::core::types::Bytes,
        pub sig_r: [u8; 32],
        pub sig_s: [u8; 32],
        pub sig_v: u8,
    }
    #[doc = "Container type for all input parameters for the `exitRecoveryMode` function with signature `exitRecoveryMode()` and selector `[56, 229, 183, 174]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "exitRecoveryMode", abi = "exitRecoveryMode()")]
    pub struct ExitRecoveryModeCall;
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
    #[doc = "Container type for all input parameters for the `getENSRegistrar` function with signature `getENSRegistrar()` and selector `[165, 228, 37, 54]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getENSRegistrar", abi = "getENSRegistrar()")]
    pub struct GetENSRegistrarCall;
    #[doc = "Container type for all input parameters for the `getExtensionInstallation` function with signature `getExtensionInstallation(bytes32,address)` and selector `[220, 96, 149, 174]`"]
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
        name = "getExtensionInstallation",
        abi = "getExtensionInstallation(bytes32,address)"
    )]
    pub struct GetExtensionInstallationCall {
        pub extension_id: [u8; 32],
        pub colony: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getExtensionResolver` function with signature `getExtensionResolver(bytes32,uint256)` and selector `[113, 190, 123, 37]`"]
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
        name = "getExtensionResolver",
        abi = "getExtensionResolver(bytes32,uint256)"
    )]
    pub struct GetExtensionResolverCall {
        pub extension_id: [u8; 32],
        pub version: ethers::core::types::U256,
    }
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
        pub user_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getMiningDelegator` function with signature `getMiningDelegator(address)` and selector `[172, 179, 119, 162]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getMiningDelegator", abi = "getMiningDelegator(address)")]
    pub struct GetMiningDelegatorCall {
        pub delegate: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getMiningResolver` function with signature `getMiningResolver()` and selector `[116, 159, 21, 205]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getMiningResolver", abi = "getMiningResolver()")]
    pub struct GetMiningResolverCall;
    #[doc = "Container type for all input parameters for the `getMiningStake` function with signature `getMiningStake(address)` and selector `[9, 255, 179, 203]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getMiningStake", abi = "getMiningStake(address)")]
    pub struct GetMiningStakeCall {
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
    #[doc = "Container type for all input parameters for the `getProfileDBAddress` function with signature `getProfileDBAddress(bytes32)` and selector `[205, 147, 11, 99]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getProfileDBAddress", abi = "getProfileDBAddress(bytes32)")]
    pub struct GetProfileDBAddressCall {
        pub node: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getReplacementReputationUpdateLogEntry` function with signature `getReplacementReputationUpdateLogEntry(address,uint256)` and selector `[233, 38, 246, 28]`"]
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
        name = "getReplacementReputationUpdateLogEntry",
        abi = "getReplacementReputationUpdateLogEntry(address,uint256)"
    )]
    pub struct GetReplacementReputationUpdateLogEntryCall {
        pub reputation_mining_cycle: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getReplacementReputationUpdateLogsExist` function with signature `getReplacementReputationUpdateLogsExist(address)` and selector `[29, 91, 20, 227]`"]
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
        name = "getReplacementReputationUpdateLogsExist",
        abi = "getReplacementReputationUpdateLogsExist(address)"
    )]
    pub struct GetReplacementReputationUpdateLogsExistCall {
        pub reputation_mining_cycle: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getReputationMiningCycle` function with signature `getReputationMiningCycle(bool)` and selector `[242, 200, 89, 156]`"]
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
        name = "getReputationMiningCycle",
        abi = "getReputationMiningCycle(bool)"
    )]
    pub struct GetReputationMiningCycleCall {
        pub active: bool,
    }
    #[doc = "Container type for all input parameters for the `getReputationMiningCycleReward` function with signature `getReputationMiningCycleReward()` and selector `[111, 81, 216, 92]`"]
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
        name = "getReputationMiningCycleReward",
        abi = "getReputationMiningCycleReward()"
    )]
    pub struct GetReputationMiningCycleRewardCall;
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
    #[doc = "Container type for all input parameters for the `initialiseReputationMining` function with signature `initialiseReputationMining()` and selector `[238, 101, 120, 106]`"]
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
        name = "initialiseReputationMining",
        abi = "initialiseReputationMining()"
    )]
    pub struct InitialiseReputationMiningCall;
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
    #[doc = "Container type for all input parameters for the `installExtension` function with signature `installExtension(bytes32,uint256)` and selector `[157, 236, 9, 16]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "installExtension", abi = "installExtension(bytes32,uint256)")]
    pub struct InstallExtensionCall {
        pub extension_id: [u8; 32],
        pub version: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all input parameters for the `isInRecoveryMode` function with signature `isInRecoveryMode()` and selector `[128, 217, 250, 211]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isInRecoveryMode", abi = "isInRecoveryMode()")]
    pub struct IsInRecoveryModeCall;
    #[doc = "Container type for all input parameters for the `lookupRegisteredENSDomain` function with signature `lookupRegisteredENSDomain(address)` and selector `[96, 198, 71, 118]`"]
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
        name = "lookupRegisteredENSDomain",
        abi = "lookupRegisteredENSDomain(address)"
    )]
    pub struct LookupRegisteredENSDomainCall {
        pub addr: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `numRecoveryRoles` function with signature `numRecoveryRoles()` and selector `[84, 88, 67, 48]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "numRecoveryRoles", abi = "numRecoveryRoles()")]
    pub struct NumRecoveryRolesCall;
    #[doc = "Container type for all input parameters for the `punishStakers` function with signature `punishStakers(address[],uint256)` and selector `[15, 150, 153, 93]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "punishStakers", abi = "punishStakers(address[],uint256)")]
    pub struct PunishStakersCall {
        pub stakers: ::std::vec::Vec<ethers::core::types::Address>,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `registerColonyLabel` function with signature `registerColonyLabel(string,string)` and selector `[226, 219, 119, 123]`"]
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
        name = "registerColonyLabel",
        abi = "registerColonyLabel(string,string)"
    )]
    pub struct RegisterColonyLabelCall {
        pub colony_name: String,
        pub orbitdb: String,
    }
    #[doc = "Container type for all input parameters for the `registerUserLabel` function with signature `registerUserLabel(string,string)` and selector `[181, 227, 79, 191]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "registerUserLabel", abi = "registerUserLabel(string,string)")]
    pub struct RegisterUserLabelCall {
        pub username: String,
        pub orbitdb: String,
    }
    #[doc = "Container type for all input parameters for the `removeRecoveryRole` function with signature `removeRecoveryRole(address)` and selector `[153, 207, 124, 199]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "removeRecoveryRole", abi = "removeRecoveryRole(address)")]
    pub struct RemoveRecoveryRoleCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `reward` function with signature `reward(address,uint256)` and selector `[33, 103, 15, 34]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "reward", abi = "reward(address,uint256)")]
    pub struct RewardCall {
        pub recipient: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `setMiningDelegate` function with signature `setMiningDelegate(address,bool)` and selector `[245, 132, 209, 125]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setMiningDelegate", abi = "setMiningDelegate(address,bool)")]
    pub struct SetMiningDelegateCall {
        pub delegate: ethers::core::types::Address,
        pub allowed: bool,
    }
    #[doc = "Container type for all input parameters for the `setMiningResolver` function with signature `setMiningResolver(address)` and selector `[225, 99, 104, 130]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setMiningResolver", abi = "setMiningResolver(address)")]
    pub struct SetMiningResolverCall {
        pub mining_resolver_address: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `setRecoveryRole` function with signature `setRecoveryRole(address)` and selector `[210, 69, 54, 165]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setRecoveryRole", abi = "setRecoveryRole(address)")]
    pub struct SetRecoveryRoleCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setReplacementReputationUpdateLogEntry` function with signature `setReplacementReputationUpdateLogEntry(address,uint256,address,int256,uint256,address,uint128,uint128)` and selector `[35, 16, 7, 20]`"]
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
        name = "setReplacementReputationUpdateLogEntry",
        abi = "setReplacementReputationUpdateLogEntry(address,uint256,address,int256,uint256,address,uint128,uint128)"
    )]
    pub struct SetReplacementReputationUpdateLogEntryCall {
        pub reputation_mining_cycle: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
        pub amount: I256,
        pub skill_id: ethers::core::types::U256,
        pub colony: ethers::core::types::Address,
        pub n_updates: u128,
        pub n_previous_updates: u128,
    }
    #[doc = "Container type for all input parameters for the `setReputationMiningCycleReward` function with signature `setReputationMiningCycleReward(uint256)` and selector `[29, 154, 85, 54]`"]
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
        name = "setReputationMiningCycleReward",
        abi = "setReputationMiningCycleReward(uint256)"
    )]
    pub struct SetReputationMiningCycleRewardCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setReputationRootHash` function with signature `setReputationRootHash(bytes32,uint256,address[])` and selector `[116, 62, 212, 49]`"]
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
        name = "setReputationRootHash",
        abi = "setReputationRootHash(bytes32,uint256,address[])"
    )]
    pub struct SetReputationRootHashCall {
        pub new_hash: [u8; 32],
        pub new_n_leaves: ethers::core::types::U256,
        pub stakers: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `setStorageSlotRecovery` function with signature `setStorageSlotRecovery(uint256,bytes32)` and selector `[85, 2, 137, 35]`"]
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
        name = "setStorageSlotRecovery",
        abi = "setStorageSlotRecovery(uint256,bytes32)"
    )]
    pub struct SetStorageSlotRecoveryCall {
        pub slot: ethers::core::types::U256,
        pub value: [u8; 32],
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
        pub token_locking_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setupRegistrar` function with signature `setupRegistrar(address,bytes32)` and selector `[96, 96, 196, 170]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setupRegistrar", abi = "setupRegistrar(address,bytes32)")]
    pub struct SetupRegistrarCall {
        pub ens: ethers::core::types::Address,
        pub root_node: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `stakeForMining` function with signature `stakeForMining(uint256)` and selector `[157, 252, 142, 1]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "stakeForMining", abi = "stakeForMining(uint256)")]
    pub struct StakeForMiningCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `startNextCycle` function with signature `startNextCycle()` and selector `[69, 14, 46, 211]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "startNextCycle", abi = "startNextCycle()")]
    pub struct StartNextCycleCall;
    #[doc = "Container type for all input parameters for the `startTokenAuction` function with signature `startTokenAuction(address)` and selector `[85, 168, 73, 90]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "startTokenAuction", abi = "startTokenAuction(address)")]
    pub struct StartTokenAuctionCall {
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    #[doc = "Container type for all input parameters for the `uninstallExtension` function with signature `uninstallExtension(bytes32)` and selector `[194, 48, 140, 73]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "uninstallExtension", abi = "uninstallExtension(bytes32)")]
    pub struct UninstallExtensionCall {
        pub extension_id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `unstakeForMining` function with signature `unstakeForMining(uint256)` and selector `[24, 121, 189, 231]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "unstakeForMining", abi = "unstakeForMining(uint256)")]
    pub struct UnstakeForMiningCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `updateColonyOrbitDB` function with signature `updateColonyOrbitDB(string)` and selector `[40, 103, 44, 234]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "updateColonyOrbitDB", abi = "updateColonyOrbitDB(string)")]
    pub struct UpdateColonyOrbitDBCall {
        pub orbitdb: String,
    }
    #[doc = "Container type for all input parameters for the `updateUserOrbitDB` function with signature `updateUserOrbitDB(string)` and selector `[237, 38, 181, 87]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "updateUserOrbitDB", abi = "updateUserOrbitDB(string)")]
    pub struct UpdateUserOrbitDBCall {
        pub orbitdb: String,
    }
    #[doc = "Container type for all input parameters for the `upgradeExtension` function with signature `upgradeExtension(bytes32,uint256)` and selector `[217, 189, 120, 11]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "upgradeExtension", abi = "upgradeExtension(bytes32,uint256)")]
    pub struct UpgradeExtensionCall {
        pub extension_id: [u8; 32],
        pub new_version: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ColonyNetworkCalls {
        AddColonyVersion(AddColonyVersionCall),
        AddExtensionToNetwork(AddExtensionToNetworkCall),
        AddSkill(AddSkillCall),
        Addr(AddrCall),
        AppendReputationUpdateLog(AppendReputationUpdateLogCall),
        ApproveExitRecovery(ApproveExitRecoveryCall),
        BurnUnneededRewards(BurnUnneededRewardsCall),
        CalculateMinerWeight(CalculateMinerWeightCall),
        CheckNotAdditionalProtectedVariable(CheckNotAdditionalProtectedVariableCall),
        ClaimMiningReward(ClaimMiningRewardCall),
        CreateColony0(CreateColony0Call),
        CreateColony2(CreateColony2Call),
        CreateColony1(CreateColony1Call),
        CreateColony3(CreateColony3Call),
        CreateColonyForFrontend(CreateColonyForFrontendCall),
        CreateMetaColony(CreateMetaColonyCall),
        DeployTokenAuthority(DeployTokenAuthorityCall),
        DeployTokenViaNetwork(DeployTokenViaNetworkCall),
        DeprecateExtension(DeprecateExtensionCall),
        DeprecateSkill(DeprecateSkillCall),
        DeprecateSkillWithDeprecated(DeprecateSkillWithDeprecatedCall),
        EnterRecoveryMode(EnterRecoveryModeCall),
        ExecuteMetaTransaction(ExecuteMetaTransactionCall),
        ExitRecoveryMode(ExitRecoveryModeCall),
        GetChildSkillId(GetChildSkillIdCall),
        GetColony(GetColonyCall),
        GetColonyCount(GetColonyCountCall),
        GetColonyVersionResolver(GetColonyVersionResolverCall),
        GetCurrentColonyVersion(GetCurrentColonyVersionCall),
        GetENSRegistrar(GetENSRegistrarCall),
        GetExtensionInstallation(GetExtensionInstallationCall),
        GetExtensionResolver(GetExtensionResolverCall),
        GetFeeInverse(GetFeeInverseCall),
        GetMetaColony(GetMetaColonyCall),
        GetMetatransactionNonce(GetMetatransactionNonceCall),
        GetMiningDelegator(GetMiningDelegatorCall),
        GetMiningResolver(GetMiningResolverCall),
        GetMiningStake(GetMiningStakeCall),
        GetParentSkillId(GetParentSkillIdCall),
        GetPayoutWhitelist(GetPayoutWhitelistCall),
        GetProfileDBAddress(GetProfileDBAddressCall),
        GetReplacementReputationUpdateLogEntry(GetReplacementReputationUpdateLogEntryCall),
        GetReplacementReputationUpdateLogsExist(GetReplacementReputationUpdateLogsExistCall),
        GetReputationMiningCycle(GetReputationMiningCycleCall),
        GetReputationMiningCycleReward(GetReputationMiningCycleRewardCall),
        GetReputationMiningSkillId(GetReputationMiningSkillIdCall),
        GetReputationRootHash(GetReputationRootHashCall),
        GetReputationRootHashNLeaves(GetReputationRootHashNLeavesCall),
        GetReputationRootHashNNodes(GetReputationRootHashNNodesCall),
        GetSkill(GetSkillCall),
        GetSkillCount(GetSkillCountCall),
        GetTokenLocking(GetTokenLockingCall),
        Initialise(InitialiseCall),
        InitialiseReputationMining(InitialiseReputationMiningCall),
        InitialiseRootLocalSkill(InitialiseRootLocalSkillCall),
        InstallExtension(InstallExtensionCall),
        IsColony(IsColonyCall),
        IsInRecoveryMode(IsInRecoveryModeCall),
        LookupRegisteredENSDomain(LookupRegisteredENSDomainCall),
        Multicall(MulticallCall),
        NumRecoveryRoles(NumRecoveryRolesCall),
        PunishStakers(PunishStakersCall),
        RegisterColonyLabel(RegisterColonyLabelCall),
        RegisterUserLabel(RegisterUserLabelCall),
        RemoveRecoveryRole(RemoveRecoveryRoleCall),
        Reward(RewardCall),
        SetFeeInverse(SetFeeInverseCall),
        SetMiningDelegate(SetMiningDelegateCall),
        SetMiningResolver(SetMiningResolverCall),
        SetPayoutWhitelist(SetPayoutWhitelistCall),
        SetRecoveryRole(SetRecoveryRoleCall),
        SetReplacementReputationUpdateLogEntry(SetReplacementReputationUpdateLogEntryCall),
        SetReputationMiningCycleReward(SetReputationMiningCycleRewardCall),
        SetReputationRootHash(SetReputationRootHashCall),
        SetStorageSlotRecovery(SetStorageSlotRecoveryCall),
        SetTokenLocking(SetTokenLockingCall),
        SetupRegistrar(SetupRegistrarCall),
        StakeForMining(StakeForMiningCall),
        StartNextCycle(StartNextCycleCall),
        StartTokenAuction(StartTokenAuctionCall),
        SupportsInterface(SupportsInterfaceCall),
        UninstallExtension(UninstallExtensionCall),
        UnstakeForMining(UnstakeForMiningCall),
        UpdateColonyOrbitDB(UpdateColonyOrbitDBCall),
        UpdateUserOrbitDB(UpdateUserOrbitDBCall),
        UpgradeExtension(UpgradeExtensionCall),
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
                <AddExtensionToNetworkCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::AddExtensionToNetwork(decoded));
            }
            if let Ok(decoded) =
                <AddSkillCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::AddSkill(decoded));
            }
            if let Ok(decoded) = <AddrCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ColonyNetworkCalls::Addr(decoded));
            }
            if let Ok(decoded) =
                <AppendReputationUpdateLogCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyNetworkCalls::AppendReputationUpdateLog(decoded));
            }
            if let Ok(decoded) =
                <ApproveExitRecoveryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::ApproveExitRecovery(decoded));
            }
            if let Ok(decoded) =
                <BurnUnneededRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::BurnUnneededRewards(decoded));
            }
            if let Ok(decoded) =
                <CalculateMinerWeightCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::CalculateMinerWeight(decoded));
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
                <ClaimMiningRewardCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::ClaimMiningReward(decoded));
            }
            if let Ok(decoded) =
                <CreateColony0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::CreateColony0(decoded));
            }
            if let Ok(decoded) =
                <CreateColony2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::CreateColony2(decoded));
            }
            if let Ok(decoded) =
                <CreateColony1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::CreateColony1(decoded));
            }
            if let Ok(decoded) =
                <CreateColony3Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::CreateColony3(decoded));
            }
            if let Ok(decoded) =
                <CreateColonyForFrontendCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::CreateColonyForFrontend(decoded));
            }
            if let Ok(decoded) =
                <CreateMetaColonyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::CreateMetaColony(decoded));
            }
            if let Ok(decoded) =
                <DeployTokenAuthorityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::DeployTokenAuthority(decoded));
            }
            if let Ok(decoded) =
                <DeployTokenViaNetworkCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::DeployTokenViaNetwork(decoded));
            }
            if let Ok(decoded) =
                <DeprecateExtensionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::DeprecateExtension(decoded));
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
                <EnterRecoveryModeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::EnterRecoveryMode(decoded));
            }
            if let Ok(decoded) =
                <ExecuteMetaTransactionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::ExecuteMetaTransaction(decoded));
            }
            if let Ok(decoded) =
                <ExitRecoveryModeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::ExitRecoveryMode(decoded));
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
                <GetENSRegistrarCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::GetENSRegistrar(decoded));
            }
            if let Ok(decoded) =
                <GetExtensionInstallationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyNetworkCalls::GetExtensionInstallation(decoded));
            }
            if let Ok(decoded) =
                <GetExtensionResolverCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::GetExtensionResolver(decoded));
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
                <GetMiningDelegatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::GetMiningDelegator(decoded));
            }
            if let Ok(decoded) =
                <GetMiningResolverCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::GetMiningResolver(decoded));
            }
            if let Ok(decoded) =
                <GetMiningStakeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::GetMiningStake(decoded));
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
                <GetProfileDBAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::GetProfileDBAddress(decoded));
            }
            if let Ok(decoded) =
                <GetReplacementReputationUpdateLogEntryCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyNetworkCalls::GetReplacementReputationUpdateLogEntry(
                    decoded,
                ));
            }
            if let Ok (decoded) = < GetReplacementReputationUpdateLogsExistCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (ColonyNetworkCalls :: GetReplacementReputationUpdateLogsExist (decoded)) }
            if let Ok(decoded) =
                <GetReputationMiningCycleCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyNetworkCalls::GetReputationMiningCycle(decoded));
            }
            if let Ok(decoded) =
                <GetReputationMiningCycleRewardCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyNetworkCalls::GetReputationMiningCycleReward(decoded));
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
                <InitialiseReputationMiningCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyNetworkCalls::InitialiseReputationMining(decoded));
            }
            if let Ok(decoded) =
                <InitialiseRootLocalSkillCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyNetworkCalls::InitialiseRootLocalSkill(decoded));
            }
            if let Ok(decoded) =
                <InstallExtensionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::InstallExtension(decoded));
            }
            if let Ok(decoded) =
                <IsColonyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::IsColony(decoded));
            }
            if let Ok(decoded) =
                <IsInRecoveryModeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::IsInRecoveryMode(decoded));
            }
            if let Ok(decoded) =
                <LookupRegisteredENSDomainCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyNetworkCalls::LookupRegisteredENSDomain(decoded));
            }
            if let Ok(decoded) =
                <MulticallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::Multicall(decoded));
            }
            if let Ok(decoded) =
                <NumRecoveryRolesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::NumRecoveryRoles(decoded));
            }
            if let Ok(decoded) =
                <PunishStakersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::PunishStakers(decoded));
            }
            if let Ok(decoded) =
                <RegisterColonyLabelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::RegisterColonyLabel(decoded));
            }
            if let Ok(decoded) =
                <RegisterUserLabelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::RegisterUserLabel(decoded));
            }
            if let Ok(decoded) =
                <RemoveRecoveryRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::RemoveRecoveryRole(decoded));
            }
            if let Ok(decoded) = <RewardCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::Reward(decoded));
            }
            if let Ok(decoded) =
                <SetFeeInverseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::SetFeeInverse(decoded));
            }
            if let Ok(decoded) =
                <SetMiningDelegateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::SetMiningDelegate(decoded));
            }
            if let Ok(decoded) =
                <SetMiningResolverCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::SetMiningResolver(decoded));
            }
            if let Ok(decoded) =
                <SetPayoutWhitelistCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::SetPayoutWhitelist(decoded));
            }
            if let Ok(decoded) =
                <SetRecoveryRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::SetRecoveryRole(decoded));
            }
            if let Ok(decoded) =
                <SetReplacementReputationUpdateLogEntryCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyNetworkCalls::SetReplacementReputationUpdateLogEntry(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetReputationMiningCycleRewardCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyNetworkCalls::SetReputationMiningCycleReward(decoded));
            }
            if let Ok(decoded) =
                <SetReputationRootHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::SetReputationRootHash(decoded));
            }
            if let Ok(decoded) =
                <SetStorageSlotRecoveryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::SetStorageSlotRecovery(decoded));
            }
            if let Ok(decoded) =
                <SetTokenLockingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::SetTokenLocking(decoded));
            }
            if let Ok(decoded) =
                <SetupRegistrarCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::SetupRegistrar(decoded));
            }
            if let Ok(decoded) =
                <StakeForMiningCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::StakeForMining(decoded));
            }
            if let Ok(decoded) =
                <StartNextCycleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::StartNextCycle(decoded));
            }
            if let Ok(decoded) =
                <StartTokenAuctionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::StartTokenAuction(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::SupportsInterface(decoded));
            }
            if let Ok(decoded) =
                <UninstallExtensionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::UninstallExtension(decoded));
            }
            if let Ok(decoded) =
                <UnstakeForMiningCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::UnstakeForMining(decoded));
            }
            if let Ok(decoded) =
                <UpdateColonyOrbitDBCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::UpdateColonyOrbitDB(decoded));
            }
            if let Ok(decoded) =
                <UpdateUserOrbitDBCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::UpdateUserOrbitDB(decoded));
            }
            if let Ok(decoded) =
                <UpgradeExtensionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyNetworkCalls::UpgradeExtension(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ColonyNetworkCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ColonyNetworkCalls::AddColonyVersion(element) => element.encode(),
                ColonyNetworkCalls::AddExtensionToNetwork(element) => element.encode(),
                ColonyNetworkCalls::AddSkill(element) => element.encode(),
                ColonyNetworkCalls::Addr(element) => element.encode(),
                ColonyNetworkCalls::AppendReputationUpdateLog(element) => element.encode(),
                ColonyNetworkCalls::ApproveExitRecovery(element) => element.encode(),
                ColonyNetworkCalls::BurnUnneededRewards(element) => element.encode(),
                ColonyNetworkCalls::CalculateMinerWeight(element) => element.encode(),
                ColonyNetworkCalls::CheckNotAdditionalProtectedVariable(element) => {
                    element.encode()
                }
                ColonyNetworkCalls::ClaimMiningReward(element) => element.encode(),
                ColonyNetworkCalls::CreateColony0(element) => element.encode(),
                ColonyNetworkCalls::CreateColony2(element) => element.encode(),
                ColonyNetworkCalls::CreateColony1(element) => element.encode(),
                ColonyNetworkCalls::CreateColony3(element) => element.encode(),
                ColonyNetworkCalls::CreateColonyForFrontend(element) => element.encode(),
                ColonyNetworkCalls::CreateMetaColony(element) => element.encode(),
                ColonyNetworkCalls::DeployTokenAuthority(element) => element.encode(),
                ColonyNetworkCalls::DeployTokenViaNetwork(element) => element.encode(),
                ColonyNetworkCalls::DeprecateExtension(element) => element.encode(),
                ColonyNetworkCalls::DeprecateSkill(element) => element.encode(),
                ColonyNetworkCalls::DeprecateSkillWithDeprecated(element) => element.encode(),
                ColonyNetworkCalls::EnterRecoveryMode(element) => element.encode(),
                ColonyNetworkCalls::ExecuteMetaTransaction(element) => element.encode(),
                ColonyNetworkCalls::ExitRecoveryMode(element) => element.encode(),
                ColonyNetworkCalls::GetChildSkillId(element) => element.encode(),
                ColonyNetworkCalls::GetColony(element) => element.encode(),
                ColonyNetworkCalls::GetColonyCount(element) => element.encode(),
                ColonyNetworkCalls::GetColonyVersionResolver(element) => element.encode(),
                ColonyNetworkCalls::GetCurrentColonyVersion(element) => element.encode(),
                ColonyNetworkCalls::GetENSRegistrar(element) => element.encode(),
                ColonyNetworkCalls::GetExtensionInstallation(element) => element.encode(),
                ColonyNetworkCalls::GetExtensionResolver(element) => element.encode(),
                ColonyNetworkCalls::GetFeeInverse(element) => element.encode(),
                ColonyNetworkCalls::GetMetaColony(element) => element.encode(),
                ColonyNetworkCalls::GetMetatransactionNonce(element) => element.encode(),
                ColonyNetworkCalls::GetMiningDelegator(element) => element.encode(),
                ColonyNetworkCalls::GetMiningResolver(element) => element.encode(),
                ColonyNetworkCalls::GetMiningStake(element) => element.encode(),
                ColonyNetworkCalls::GetParentSkillId(element) => element.encode(),
                ColonyNetworkCalls::GetPayoutWhitelist(element) => element.encode(),
                ColonyNetworkCalls::GetProfileDBAddress(element) => element.encode(),
                ColonyNetworkCalls::GetReplacementReputationUpdateLogEntry(element) => {
                    element.encode()
                }
                ColonyNetworkCalls::GetReplacementReputationUpdateLogsExist(element) => {
                    element.encode()
                }
                ColonyNetworkCalls::GetReputationMiningCycle(element) => element.encode(),
                ColonyNetworkCalls::GetReputationMiningCycleReward(element) => element.encode(),
                ColonyNetworkCalls::GetReputationMiningSkillId(element) => element.encode(),
                ColonyNetworkCalls::GetReputationRootHash(element) => element.encode(),
                ColonyNetworkCalls::GetReputationRootHashNLeaves(element) => element.encode(),
                ColonyNetworkCalls::GetReputationRootHashNNodes(element) => element.encode(),
                ColonyNetworkCalls::GetSkill(element) => element.encode(),
                ColonyNetworkCalls::GetSkillCount(element) => element.encode(),
                ColonyNetworkCalls::GetTokenLocking(element) => element.encode(),
                ColonyNetworkCalls::Initialise(element) => element.encode(),
                ColonyNetworkCalls::InitialiseReputationMining(element) => element.encode(),
                ColonyNetworkCalls::InitialiseRootLocalSkill(element) => element.encode(),
                ColonyNetworkCalls::InstallExtension(element) => element.encode(),
                ColonyNetworkCalls::IsColony(element) => element.encode(),
                ColonyNetworkCalls::IsInRecoveryMode(element) => element.encode(),
                ColonyNetworkCalls::LookupRegisteredENSDomain(element) => element.encode(),
                ColonyNetworkCalls::Multicall(element) => element.encode(),
                ColonyNetworkCalls::NumRecoveryRoles(element) => element.encode(),
                ColonyNetworkCalls::PunishStakers(element) => element.encode(),
                ColonyNetworkCalls::RegisterColonyLabel(element) => element.encode(),
                ColonyNetworkCalls::RegisterUserLabel(element) => element.encode(),
                ColonyNetworkCalls::RemoveRecoveryRole(element) => element.encode(),
                ColonyNetworkCalls::Reward(element) => element.encode(),
                ColonyNetworkCalls::SetFeeInverse(element) => element.encode(),
                ColonyNetworkCalls::SetMiningDelegate(element) => element.encode(),
                ColonyNetworkCalls::SetMiningResolver(element) => element.encode(),
                ColonyNetworkCalls::SetPayoutWhitelist(element) => element.encode(),
                ColonyNetworkCalls::SetRecoveryRole(element) => element.encode(),
                ColonyNetworkCalls::SetReplacementReputationUpdateLogEntry(element) => {
                    element.encode()
                }
                ColonyNetworkCalls::SetReputationMiningCycleReward(element) => element.encode(),
                ColonyNetworkCalls::SetReputationRootHash(element) => element.encode(),
                ColonyNetworkCalls::SetStorageSlotRecovery(element) => element.encode(),
                ColonyNetworkCalls::SetTokenLocking(element) => element.encode(),
                ColonyNetworkCalls::SetupRegistrar(element) => element.encode(),
                ColonyNetworkCalls::StakeForMining(element) => element.encode(),
                ColonyNetworkCalls::StartNextCycle(element) => element.encode(),
                ColonyNetworkCalls::StartTokenAuction(element) => element.encode(),
                ColonyNetworkCalls::SupportsInterface(element) => element.encode(),
                ColonyNetworkCalls::UninstallExtension(element) => element.encode(),
                ColonyNetworkCalls::UnstakeForMining(element) => element.encode(),
                ColonyNetworkCalls::UpdateColonyOrbitDB(element) => element.encode(),
                ColonyNetworkCalls::UpdateUserOrbitDB(element) => element.encode(),
                ColonyNetworkCalls::UpgradeExtension(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ColonyNetworkCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ColonyNetworkCalls::AddColonyVersion(element) => element.fmt(f),
                ColonyNetworkCalls::AddExtensionToNetwork(element) => element.fmt(f),
                ColonyNetworkCalls::AddSkill(element) => element.fmt(f),
                ColonyNetworkCalls::Addr(element) => element.fmt(f),
                ColonyNetworkCalls::AppendReputationUpdateLog(element) => element.fmt(f),
                ColonyNetworkCalls::ApproveExitRecovery(element) => element.fmt(f),
                ColonyNetworkCalls::BurnUnneededRewards(element) => element.fmt(f),
                ColonyNetworkCalls::CalculateMinerWeight(element) => element.fmt(f),
                ColonyNetworkCalls::CheckNotAdditionalProtectedVariable(element) => element.fmt(f),
                ColonyNetworkCalls::ClaimMiningReward(element) => element.fmt(f),
                ColonyNetworkCalls::CreateColony0(element) => element.fmt(f),
                ColonyNetworkCalls::CreateColony2(element) => element.fmt(f),
                ColonyNetworkCalls::CreateColony1(element) => element.fmt(f),
                ColonyNetworkCalls::CreateColony3(element) => element.fmt(f),
                ColonyNetworkCalls::CreateColonyForFrontend(element) => element.fmt(f),
                ColonyNetworkCalls::CreateMetaColony(element) => element.fmt(f),
                ColonyNetworkCalls::DeployTokenAuthority(element) => element.fmt(f),
                ColonyNetworkCalls::DeployTokenViaNetwork(element) => element.fmt(f),
                ColonyNetworkCalls::DeprecateExtension(element) => element.fmt(f),
                ColonyNetworkCalls::DeprecateSkill(element) => element.fmt(f),
                ColonyNetworkCalls::DeprecateSkillWithDeprecated(element) => element.fmt(f),
                ColonyNetworkCalls::EnterRecoveryMode(element) => element.fmt(f),
                ColonyNetworkCalls::ExecuteMetaTransaction(element) => element.fmt(f),
                ColonyNetworkCalls::ExitRecoveryMode(element) => element.fmt(f),
                ColonyNetworkCalls::GetChildSkillId(element) => element.fmt(f),
                ColonyNetworkCalls::GetColony(element) => element.fmt(f),
                ColonyNetworkCalls::GetColonyCount(element) => element.fmt(f),
                ColonyNetworkCalls::GetColonyVersionResolver(element) => element.fmt(f),
                ColonyNetworkCalls::GetCurrentColonyVersion(element) => element.fmt(f),
                ColonyNetworkCalls::GetENSRegistrar(element) => element.fmt(f),
                ColonyNetworkCalls::GetExtensionInstallation(element) => element.fmt(f),
                ColonyNetworkCalls::GetExtensionResolver(element) => element.fmt(f),
                ColonyNetworkCalls::GetFeeInverse(element) => element.fmt(f),
                ColonyNetworkCalls::GetMetaColony(element) => element.fmt(f),
                ColonyNetworkCalls::GetMetatransactionNonce(element) => element.fmt(f),
                ColonyNetworkCalls::GetMiningDelegator(element) => element.fmt(f),
                ColonyNetworkCalls::GetMiningResolver(element) => element.fmt(f),
                ColonyNetworkCalls::GetMiningStake(element) => element.fmt(f),
                ColonyNetworkCalls::GetParentSkillId(element) => element.fmt(f),
                ColonyNetworkCalls::GetPayoutWhitelist(element) => element.fmt(f),
                ColonyNetworkCalls::GetProfileDBAddress(element) => element.fmt(f),
                ColonyNetworkCalls::GetReplacementReputationUpdateLogEntry(element) => {
                    element.fmt(f)
                }
                ColonyNetworkCalls::GetReplacementReputationUpdateLogsExist(element) => {
                    element.fmt(f)
                }
                ColonyNetworkCalls::GetReputationMiningCycle(element) => element.fmt(f),
                ColonyNetworkCalls::GetReputationMiningCycleReward(element) => element.fmt(f),
                ColonyNetworkCalls::GetReputationMiningSkillId(element) => element.fmt(f),
                ColonyNetworkCalls::GetReputationRootHash(element) => element.fmt(f),
                ColonyNetworkCalls::GetReputationRootHashNLeaves(element) => element.fmt(f),
                ColonyNetworkCalls::GetReputationRootHashNNodes(element) => element.fmt(f),
                ColonyNetworkCalls::GetSkill(element) => element.fmt(f),
                ColonyNetworkCalls::GetSkillCount(element) => element.fmt(f),
                ColonyNetworkCalls::GetTokenLocking(element) => element.fmt(f),
                ColonyNetworkCalls::Initialise(element) => element.fmt(f),
                ColonyNetworkCalls::InitialiseReputationMining(element) => element.fmt(f),
                ColonyNetworkCalls::InitialiseRootLocalSkill(element) => element.fmt(f),
                ColonyNetworkCalls::InstallExtension(element) => element.fmt(f),
                ColonyNetworkCalls::IsColony(element) => element.fmt(f),
                ColonyNetworkCalls::IsInRecoveryMode(element) => element.fmt(f),
                ColonyNetworkCalls::LookupRegisteredENSDomain(element) => element.fmt(f),
                ColonyNetworkCalls::Multicall(element) => element.fmt(f),
                ColonyNetworkCalls::NumRecoveryRoles(element) => element.fmt(f),
                ColonyNetworkCalls::PunishStakers(element) => element.fmt(f),
                ColonyNetworkCalls::RegisterColonyLabel(element) => element.fmt(f),
                ColonyNetworkCalls::RegisterUserLabel(element) => element.fmt(f),
                ColonyNetworkCalls::RemoveRecoveryRole(element) => element.fmt(f),
                ColonyNetworkCalls::Reward(element) => element.fmt(f),
                ColonyNetworkCalls::SetFeeInverse(element) => element.fmt(f),
                ColonyNetworkCalls::SetMiningDelegate(element) => element.fmt(f),
                ColonyNetworkCalls::SetMiningResolver(element) => element.fmt(f),
                ColonyNetworkCalls::SetPayoutWhitelist(element) => element.fmt(f),
                ColonyNetworkCalls::SetRecoveryRole(element) => element.fmt(f),
                ColonyNetworkCalls::SetReplacementReputationUpdateLogEntry(element) => {
                    element.fmt(f)
                }
                ColonyNetworkCalls::SetReputationMiningCycleReward(element) => element.fmt(f),
                ColonyNetworkCalls::SetReputationRootHash(element) => element.fmt(f),
                ColonyNetworkCalls::SetStorageSlotRecovery(element) => element.fmt(f),
                ColonyNetworkCalls::SetTokenLocking(element) => element.fmt(f),
                ColonyNetworkCalls::SetupRegistrar(element) => element.fmt(f),
                ColonyNetworkCalls::StakeForMining(element) => element.fmt(f),
                ColonyNetworkCalls::StartNextCycle(element) => element.fmt(f),
                ColonyNetworkCalls::StartTokenAuction(element) => element.fmt(f),
                ColonyNetworkCalls::SupportsInterface(element) => element.fmt(f),
                ColonyNetworkCalls::UninstallExtension(element) => element.fmt(f),
                ColonyNetworkCalls::UnstakeForMining(element) => element.fmt(f),
                ColonyNetworkCalls::UpdateColonyOrbitDB(element) => element.fmt(f),
                ColonyNetworkCalls::UpdateUserOrbitDB(element) => element.fmt(f),
                ColonyNetworkCalls::UpgradeExtension(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddColonyVersionCall> for ColonyNetworkCalls {
        fn from(var: AddColonyVersionCall) -> Self {
            ColonyNetworkCalls::AddColonyVersion(var)
        }
    }
    impl ::std::convert::From<AddExtensionToNetworkCall> for ColonyNetworkCalls {
        fn from(var: AddExtensionToNetworkCall) -> Self {
            ColonyNetworkCalls::AddExtensionToNetwork(var)
        }
    }
    impl ::std::convert::From<AddSkillCall> for ColonyNetworkCalls {
        fn from(var: AddSkillCall) -> Self {
            ColonyNetworkCalls::AddSkill(var)
        }
    }
    impl ::std::convert::From<AddrCall> for ColonyNetworkCalls {
        fn from(var: AddrCall) -> Self {
            ColonyNetworkCalls::Addr(var)
        }
    }
    impl ::std::convert::From<AppendReputationUpdateLogCall> for ColonyNetworkCalls {
        fn from(var: AppendReputationUpdateLogCall) -> Self {
            ColonyNetworkCalls::AppendReputationUpdateLog(var)
        }
    }
    impl ::std::convert::From<ApproveExitRecoveryCall> for ColonyNetworkCalls {
        fn from(var: ApproveExitRecoveryCall) -> Self {
            ColonyNetworkCalls::ApproveExitRecovery(var)
        }
    }
    impl ::std::convert::From<BurnUnneededRewardsCall> for ColonyNetworkCalls {
        fn from(var: BurnUnneededRewardsCall) -> Self {
            ColonyNetworkCalls::BurnUnneededRewards(var)
        }
    }
    impl ::std::convert::From<CalculateMinerWeightCall> for ColonyNetworkCalls {
        fn from(var: CalculateMinerWeightCall) -> Self {
            ColonyNetworkCalls::CalculateMinerWeight(var)
        }
    }
    impl ::std::convert::From<CheckNotAdditionalProtectedVariableCall> for ColonyNetworkCalls {
        fn from(var: CheckNotAdditionalProtectedVariableCall) -> Self {
            ColonyNetworkCalls::CheckNotAdditionalProtectedVariable(var)
        }
    }
    impl ::std::convert::From<ClaimMiningRewardCall> for ColonyNetworkCalls {
        fn from(var: ClaimMiningRewardCall) -> Self {
            ColonyNetworkCalls::ClaimMiningReward(var)
        }
    }
    impl ::std::convert::From<CreateColony0Call> for ColonyNetworkCalls {
        fn from(var: CreateColony0Call) -> Self {
            ColonyNetworkCalls::CreateColony0(var)
        }
    }
    impl ::std::convert::From<CreateColony2Call> for ColonyNetworkCalls {
        fn from(var: CreateColony2Call) -> Self {
            ColonyNetworkCalls::CreateColony2(var)
        }
    }
    impl ::std::convert::From<CreateColony1Call> for ColonyNetworkCalls {
        fn from(var: CreateColony1Call) -> Self {
            ColonyNetworkCalls::CreateColony1(var)
        }
    }
    impl ::std::convert::From<CreateColony3Call> for ColonyNetworkCalls {
        fn from(var: CreateColony3Call) -> Self {
            ColonyNetworkCalls::CreateColony3(var)
        }
    }
    impl ::std::convert::From<CreateColonyForFrontendCall> for ColonyNetworkCalls {
        fn from(var: CreateColonyForFrontendCall) -> Self {
            ColonyNetworkCalls::CreateColonyForFrontend(var)
        }
    }
    impl ::std::convert::From<CreateMetaColonyCall> for ColonyNetworkCalls {
        fn from(var: CreateMetaColonyCall) -> Self {
            ColonyNetworkCalls::CreateMetaColony(var)
        }
    }
    impl ::std::convert::From<DeployTokenAuthorityCall> for ColonyNetworkCalls {
        fn from(var: DeployTokenAuthorityCall) -> Self {
            ColonyNetworkCalls::DeployTokenAuthority(var)
        }
    }
    impl ::std::convert::From<DeployTokenViaNetworkCall> for ColonyNetworkCalls {
        fn from(var: DeployTokenViaNetworkCall) -> Self {
            ColonyNetworkCalls::DeployTokenViaNetwork(var)
        }
    }
    impl ::std::convert::From<DeprecateExtensionCall> for ColonyNetworkCalls {
        fn from(var: DeprecateExtensionCall) -> Self {
            ColonyNetworkCalls::DeprecateExtension(var)
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
    impl ::std::convert::From<EnterRecoveryModeCall> for ColonyNetworkCalls {
        fn from(var: EnterRecoveryModeCall) -> Self {
            ColonyNetworkCalls::EnterRecoveryMode(var)
        }
    }
    impl ::std::convert::From<ExecuteMetaTransactionCall> for ColonyNetworkCalls {
        fn from(var: ExecuteMetaTransactionCall) -> Self {
            ColonyNetworkCalls::ExecuteMetaTransaction(var)
        }
    }
    impl ::std::convert::From<ExitRecoveryModeCall> for ColonyNetworkCalls {
        fn from(var: ExitRecoveryModeCall) -> Self {
            ColonyNetworkCalls::ExitRecoveryMode(var)
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
    impl ::std::convert::From<GetENSRegistrarCall> for ColonyNetworkCalls {
        fn from(var: GetENSRegistrarCall) -> Self {
            ColonyNetworkCalls::GetENSRegistrar(var)
        }
    }
    impl ::std::convert::From<GetExtensionInstallationCall> for ColonyNetworkCalls {
        fn from(var: GetExtensionInstallationCall) -> Self {
            ColonyNetworkCalls::GetExtensionInstallation(var)
        }
    }
    impl ::std::convert::From<GetExtensionResolverCall> for ColonyNetworkCalls {
        fn from(var: GetExtensionResolverCall) -> Self {
            ColonyNetworkCalls::GetExtensionResolver(var)
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
    impl ::std::convert::From<GetMiningDelegatorCall> for ColonyNetworkCalls {
        fn from(var: GetMiningDelegatorCall) -> Self {
            ColonyNetworkCalls::GetMiningDelegator(var)
        }
    }
    impl ::std::convert::From<GetMiningResolverCall> for ColonyNetworkCalls {
        fn from(var: GetMiningResolverCall) -> Self {
            ColonyNetworkCalls::GetMiningResolver(var)
        }
    }
    impl ::std::convert::From<GetMiningStakeCall> for ColonyNetworkCalls {
        fn from(var: GetMiningStakeCall) -> Self {
            ColonyNetworkCalls::GetMiningStake(var)
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
    impl ::std::convert::From<GetProfileDBAddressCall> for ColonyNetworkCalls {
        fn from(var: GetProfileDBAddressCall) -> Self {
            ColonyNetworkCalls::GetProfileDBAddress(var)
        }
    }
    impl ::std::convert::From<GetReplacementReputationUpdateLogEntryCall> for ColonyNetworkCalls {
        fn from(var: GetReplacementReputationUpdateLogEntryCall) -> Self {
            ColonyNetworkCalls::GetReplacementReputationUpdateLogEntry(var)
        }
    }
    impl ::std::convert::From<GetReplacementReputationUpdateLogsExistCall> for ColonyNetworkCalls {
        fn from(var: GetReplacementReputationUpdateLogsExistCall) -> Self {
            ColonyNetworkCalls::GetReplacementReputationUpdateLogsExist(var)
        }
    }
    impl ::std::convert::From<GetReputationMiningCycleCall> for ColonyNetworkCalls {
        fn from(var: GetReputationMiningCycleCall) -> Self {
            ColonyNetworkCalls::GetReputationMiningCycle(var)
        }
    }
    impl ::std::convert::From<GetReputationMiningCycleRewardCall> for ColonyNetworkCalls {
        fn from(var: GetReputationMiningCycleRewardCall) -> Self {
            ColonyNetworkCalls::GetReputationMiningCycleReward(var)
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
    impl ::std::convert::From<InitialiseReputationMiningCall> for ColonyNetworkCalls {
        fn from(var: InitialiseReputationMiningCall) -> Self {
            ColonyNetworkCalls::InitialiseReputationMining(var)
        }
    }
    impl ::std::convert::From<InitialiseRootLocalSkillCall> for ColonyNetworkCalls {
        fn from(var: InitialiseRootLocalSkillCall) -> Self {
            ColonyNetworkCalls::InitialiseRootLocalSkill(var)
        }
    }
    impl ::std::convert::From<InstallExtensionCall> for ColonyNetworkCalls {
        fn from(var: InstallExtensionCall) -> Self {
            ColonyNetworkCalls::InstallExtension(var)
        }
    }
    impl ::std::convert::From<IsColonyCall> for ColonyNetworkCalls {
        fn from(var: IsColonyCall) -> Self {
            ColonyNetworkCalls::IsColony(var)
        }
    }
    impl ::std::convert::From<IsInRecoveryModeCall> for ColonyNetworkCalls {
        fn from(var: IsInRecoveryModeCall) -> Self {
            ColonyNetworkCalls::IsInRecoveryMode(var)
        }
    }
    impl ::std::convert::From<LookupRegisteredENSDomainCall> for ColonyNetworkCalls {
        fn from(var: LookupRegisteredENSDomainCall) -> Self {
            ColonyNetworkCalls::LookupRegisteredENSDomain(var)
        }
    }
    impl ::std::convert::From<MulticallCall> for ColonyNetworkCalls {
        fn from(var: MulticallCall) -> Self {
            ColonyNetworkCalls::Multicall(var)
        }
    }
    impl ::std::convert::From<NumRecoveryRolesCall> for ColonyNetworkCalls {
        fn from(var: NumRecoveryRolesCall) -> Self {
            ColonyNetworkCalls::NumRecoveryRoles(var)
        }
    }
    impl ::std::convert::From<PunishStakersCall> for ColonyNetworkCalls {
        fn from(var: PunishStakersCall) -> Self {
            ColonyNetworkCalls::PunishStakers(var)
        }
    }
    impl ::std::convert::From<RegisterColonyLabelCall> for ColonyNetworkCalls {
        fn from(var: RegisterColonyLabelCall) -> Self {
            ColonyNetworkCalls::RegisterColonyLabel(var)
        }
    }
    impl ::std::convert::From<RegisterUserLabelCall> for ColonyNetworkCalls {
        fn from(var: RegisterUserLabelCall) -> Self {
            ColonyNetworkCalls::RegisterUserLabel(var)
        }
    }
    impl ::std::convert::From<RemoveRecoveryRoleCall> for ColonyNetworkCalls {
        fn from(var: RemoveRecoveryRoleCall) -> Self {
            ColonyNetworkCalls::RemoveRecoveryRole(var)
        }
    }
    impl ::std::convert::From<RewardCall> for ColonyNetworkCalls {
        fn from(var: RewardCall) -> Self {
            ColonyNetworkCalls::Reward(var)
        }
    }
    impl ::std::convert::From<SetFeeInverseCall> for ColonyNetworkCalls {
        fn from(var: SetFeeInverseCall) -> Self {
            ColonyNetworkCalls::SetFeeInverse(var)
        }
    }
    impl ::std::convert::From<SetMiningDelegateCall> for ColonyNetworkCalls {
        fn from(var: SetMiningDelegateCall) -> Self {
            ColonyNetworkCalls::SetMiningDelegate(var)
        }
    }
    impl ::std::convert::From<SetMiningResolverCall> for ColonyNetworkCalls {
        fn from(var: SetMiningResolverCall) -> Self {
            ColonyNetworkCalls::SetMiningResolver(var)
        }
    }
    impl ::std::convert::From<SetPayoutWhitelistCall> for ColonyNetworkCalls {
        fn from(var: SetPayoutWhitelistCall) -> Self {
            ColonyNetworkCalls::SetPayoutWhitelist(var)
        }
    }
    impl ::std::convert::From<SetRecoveryRoleCall> for ColonyNetworkCalls {
        fn from(var: SetRecoveryRoleCall) -> Self {
            ColonyNetworkCalls::SetRecoveryRole(var)
        }
    }
    impl ::std::convert::From<SetReplacementReputationUpdateLogEntryCall> for ColonyNetworkCalls {
        fn from(var: SetReplacementReputationUpdateLogEntryCall) -> Self {
            ColonyNetworkCalls::SetReplacementReputationUpdateLogEntry(var)
        }
    }
    impl ::std::convert::From<SetReputationMiningCycleRewardCall> for ColonyNetworkCalls {
        fn from(var: SetReputationMiningCycleRewardCall) -> Self {
            ColonyNetworkCalls::SetReputationMiningCycleReward(var)
        }
    }
    impl ::std::convert::From<SetReputationRootHashCall> for ColonyNetworkCalls {
        fn from(var: SetReputationRootHashCall) -> Self {
            ColonyNetworkCalls::SetReputationRootHash(var)
        }
    }
    impl ::std::convert::From<SetStorageSlotRecoveryCall> for ColonyNetworkCalls {
        fn from(var: SetStorageSlotRecoveryCall) -> Self {
            ColonyNetworkCalls::SetStorageSlotRecovery(var)
        }
    }
    impl ::std::convert::From<SetTokenLockingCall> for ColonyNetworkCalls {
        fn from(var: SetTokenLockingCall) -> Self {
            ColonyNetworkCalls::SetTokenLocking(var)
        }
    }
    impl ::std::convert::From<SetupRegistrarCall> for ColonyNetworkCalls {
        fn from(var: SetupRegistrarCall) -> Self {
            ColonyNetworkCalls::SetupRegistrar(var)
        }
    }
    impl ::std::convert::From<StakeForMiningCall> for ColonyNetworkCalls {
        fn from(var: StakeForMiningCall) -> Self {
            ColonyNetworkCalls::StakeForMining(var)
        }
    }
    impl ::std::convert::From<StartNextCycleCall> for ColonyNetworkCalls {
        fn from(var: StartNextCycleCall) -> Self {
            ColonyNetworkCalls::StartNextCycle(var)
        }
    }
    impl ::std::convert::From<StartTokenAuctionCall> for ColonyNetworkCalls {
        fn from(var: StartTokenAuctionCall) -> Self {
            ColonyNetworkCalls::StartTokenAuction(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for ColonyNetworkCalls {
        fn from(var: SupportsInterfaceCall) -> Self {
            ColonyNetworkCalls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<UninstallExtensionCall> for ColonyNetworkCalls {
        fn from(var: UninstallExtensionCall) -> Self {
            ColonyNetworkCalls::UninstallExtension(var)
        }
    }
    impl ::std::convert::From<UnstakeForMiningCall> for ColonyNetworkCalls {
        fn from(var: UnstakeForMiningCall) -> Self {
            ColonyNetworkCalls::UnstakeForMining(var)
        }
    }
    impl ::std::convert::From<UpdateColonyOrbitDBCall> for ColonyNetworkCalls {
        fn from(var: UpdateColonyOrbitDBCall) -> Self {
            ColonyNetworkCalls::UpdateColonyOrbitDB(var)
        }
    }
    impl ::std::convert::From<UpdateUserOrbitDBCall> for ColonyNetworkCalls {
        fn from(var: UpdateUserOrbitDBCall) -> Self {
            ColonyNetworkCalls::UpdateUserOrbitDB(var)
        }
    }
    impl ::std::convert::From<UpgradeExtensionCall> for ColonyNetworkCalls {
        fn from(var: UpgradeExtensionCall) -> Self {
            ColonyNetworkCalls::UpgradeExtension(var)
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
    pub struct AddSkillReturn {
        pub skill_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `addr` function with signature `addr(bytes32)` and selector `[59, 59, 87, 222]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AddrReturn {
        pub address: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `calculateMinerWeight` function with signature `calculateMinerWeight(uint256,uint256)` and selector `[142, 69, 68, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CalculateMinerWeightReturn {
        pub miner_weight: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `createColony` function with signature `createColony(address)` and selector `[8, 235, 13, 44]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CreateColony0Return {
        pub colony_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `createColony` function with signature `createColony(address,uint256,string,string)` and selector `[14, 0, 73, 160]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CreateColony2Return {
        pub colony_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `createColony` function with signature `createColony(address,uint256,string)` and selector `[23, 160, 133, 24]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CreateColony1Return {
        pub colony_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `createColony` function with signature `createColony(address,uint256,string,string,bool)` and selector `[154, 85, 191, 220]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CreateColony3Return {
        pub colony_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `createColonyForFrontend` function with signature `createColonyForFrontend(address,string,string,uint8,uint256,string,string)` and selector `[104, 89, 135, 223]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CreateColonyForFrontendReturn {
        pub token: ethers::core::types::Address,
        pub colony: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `deployTokenAuthority` function with signature `deployTokenAuthority(address,address,address[])` and selector `[90, 197, 68, 8]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DeployTokenAuthorityReturn {
        pub token_authority: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `deployTokenViaNetwork` function with signature `deployTokenViaNetwork(string,string,uint8)` and selector `[78, 81, 105, 26]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DeployTokenViaNetworkReturn {
        pub token: ethers::core::types::Address,
    }
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
    pub struct DeprecateSkillWithDeprecatedReturn {
        pub changed: bool,
    }
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
    pub struct GetChildSkillIdReturn {
        pub child_skill_id: ethers::core::types::U256,
    }
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
    pub struct GetColonyReturn {
        pub colony_address: ethers::core::types::Address,
    }
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
    pub struct GetColonyCountReturn {
        pub count: ethers::core::types::U256,
    }
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
    pub struct GetColonyVersionResolverReturn {
        pub resolver_address: ethers::core::types::Address,
    }
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
    pub struct GetCurrentColonyVersionReturn {
        pub version: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getENSRegistrar` function with signature `getENSRegistrar()` and selector `[165, 228, 37, 54]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetENSRegistrarReturn {
        pub address: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `getExtensionInstallation` function with signature `getExtensionInstallation(bytes32,address)` and selector `[220, 96, 149, 174]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetExtensionInstallationReturn {
        pub installation: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `getExtensionResolver` function with signature `getExtensionResolver(bytes32,uint256)` and selector `[113, 190, 123, 37]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetExtensionResolverReturn {
        pub resolver: ethers::core::types::Address,
    }
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
    pub struct GetMetaColonyReturn {
        pub colony_address: ethers::core::types::Address,
    }
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
    #[doc = "Container type for all return fields from the `getMiningDelegator` function with signature `getMiningDelegator(address)` and selector `[172, 179, 119, 162]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetMiningDelegatorReturn {
        pub delegator: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `getMiningResolver` function with signature `getMiningResolver()` and selector `[116, 159, 21, 205]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetMiningResolverReturn {
        pub mining_resolver_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `getMiningStake` function with signature `getMiningStake(address)` and selector `[9, 255, 179, 203]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetMiningStakeReturn {
        pub info: MiningStake,
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
    pub struct GetParentSkillIdReturn {
        pub parent_skill_id: ethers::core::types::U256,
    }
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
    pub struct GetPayoutWhitelistReturn {
        pub status: bool,
    }
    #[doc = "Container type for all return fields from the `getProfileDBAddress` function with signature `getProfileDBAddress(bytes32)` and selector `[205, 147, 11, 99]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetProfileDBAddressReturn {
        pub orbitdb: String,
    }
    #[doc = "Container type for all return fields from the `getReplacementReputationUpdateLogEntry` function with signature `getReplacementReputationUpdateLogEntry(address,uint256)` and selector `[233, 38, 246, 28]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetReplacementReputationUpdateLogEntryReturn {
        pub reputation_log_entry: ReputationLogEntry,
    }
    #[doc = "Container type for all return fields from the `getReplacementReputationUpdateLogsExist` function with signature `getReplacementReputationUpdateLogsExist(address)` and selector `[29, 91, 20, 227]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetReplacementReputationUpdateLogsExistReturn {
        pub exists: bool,
    }
    #[doc = "Container type for all return fields from the `getReputationMiningCycle` function with signature `getReputationMiningCycle(bool)` and selector `[242, 200, 89, 156]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetReputationMiningCycleReturn {
        pub rep_mining_cycle_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `getReputationMiningCycleReward` function with signature `getReputationMiningCycleReward()` and selector `[111, 81, 216, 92]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetReputationMiningCycleRewardReturn {
        pub amount: ethers::core::types::U256,
    }
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
    pub struct GetReputationMiningSkillIdReturn {
        pub skill_id: ethers::core::types::U256,
    }
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
    pub struct GetReputationRootHashReturn {
        pub root_hash: [u8; 32],
    }
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
    pub struct GetReputationRootHashNLeavesReturn {
        pub n_leaves: ethers::core::types::U256,
    }
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
    pub struct GetReputationRootHashNNodesReturn {
        pub n_nodes: ethers::core::types::U256,
    }
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
    pub struct GetSkillCountReturn {
        pub count: ethers::core::types::U256,
    }
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
    pub struct GetTokenLockingReturn {
        pub locking_address: ethers::core::types::Address,
    }
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
    pub struct InitialiseRootLocalSkillReturn {
        pub root_local_skill_id: ethers::core::types::U256,
    }
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
    pub struct IsColonyReturn {
        pub address_is_colony: bool,
    }
    #[doc = "Container type for all return fields from the `isInRecoveryMode` function with signature `isInRecoveryMode()` and selector `[128, 217, 250, 211]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsInRecoveryModeReturn {
        pub in_recovery_mode: bool,
    }
    #[doc = "Container type for all return fields from the `lookupRegisteredENSDomain` function with signature `lookupRegisteredENSDomain(address)` and selector `[96, 198, 71, 118]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LookupRegisteredENSDomainReturn {
        pub domain: String,
    }
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
    #[doc = "Container type for all return fields from the `numRecoveryRoles` function with signature `numRecoveryRoles()` and selector `[84, 88, 67, 48]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NumRecoveryRolesReturn {
        pub num_roles: u64,
    }
    #[doc = "Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SupportsInterfaceReturn {
        pub status: bool,
    }
    #[doc = "`MiningStake(uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct MiningStake {
        pub amount: ethers::core::types::U256,
        pub timestamp: ethers::core::types::U256,
    }
    #[doc = "`ReputationLogEntry(address,int256,uint256,address,uint128,uint128)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ReputationLogEntry {
        pub user: ethers::core::types::Address,
        pub amount: I256,
        pub skill_id: ethers::core::types::U256,
        pub colony: ethers::core::types::Address,
        pub n_updates: u128,
        pub n_previous_updates: u128,
    }
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
