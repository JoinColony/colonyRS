pub use colony::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod colony {
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
    #[doc = "Colony was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"txHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"metadata\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Annotation\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"skillId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"amount\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ArbitraryReputationUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ArbitraryTransaction\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address[]\",\"name\":\"users\",\"type\":\"address[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256[]\",\"name\":\"amounts\",\"type\":\"int256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ColonyBootstrapped\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"payoutRemainder\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ColonyFundsClaimed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"fromPot\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"toPot\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ColonyFundsMovedBetweenFundingPots\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"colonyNetwork\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ColonyInitialised\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"metadata\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ColonyMetadata\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"metadata\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ColonyMetadataDelta\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"rewardInverse\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ColonyRewardInverseSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"domainId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint8\",\"name\":\"role\",\"type\":\"uint8\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"setTo\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ColonyRoleSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"oldVersion\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"newVersion\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ColonyUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"domainId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DomainAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"domainId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"deprecated\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DomainDeprecated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"domainId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"metadata\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DomainMetadata\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"expenditureId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExpenditureAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"expenditureId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ExpenditureCancelled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"expenditureId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"slot\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"claimDelay\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExpenditureClaimDelaySet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"expenditureId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ExpenditureFinalized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"globalClaimDelay\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExpenditureGlobalClaimDelaySet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"expenditureId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ExpenditureLocked\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"expenditureId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"metadata\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExpenditureMetadataSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"expenditureId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"slot\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"int256\",\"name\":\"payoutModifier\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExpenditurePayoutModifierSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"expenditureId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"slot\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExpenditurePayoutSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"expenditureId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"slot\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ExpenditureRecipientSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"expenditureId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"slot\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"skillId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ExpenditureSkillSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"expenditureId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"storageSlot\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool[]\",\"name\":\"mask\",\"type\":\"bool[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32[]\",\"name\":\"keys\",\"type\":\"bytes32[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"value\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ExpenditureStateChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"expenditureId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ExpenditureTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fundingPotId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FundingPotAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"localSkillId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LocalSkillAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"localSkillId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"deprecated\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LocalSkillDeprecated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"userAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address payable\",\"name\":\"relayerAddress\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MetaTransactionExecuted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"paymentId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PaymentAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"paymentId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PaymentFinalized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"paymentId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PaymentPayoutSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"paymentId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PaymentRecipientSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"paymentId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"skillId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PaymentSkillSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"fundingPotId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PayoutClaimed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RecoveryModeEntered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RecoveryModeExitApproved\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RecoveryModeExited\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"setTo\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RecoveryRoleSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"slot\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"fromValue\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"toValue\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RecoveryStorageSlotSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"rewardPayoutId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"fee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"rewardRemainder\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RewardPayoutClaimed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"rewardPayoutId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RewardPayoutCycleEnded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"rewardPayoutId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RewardPayoutCycleStarted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"taskId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TaskAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"taskId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"specificationHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TaskBriefSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"taskId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TaskCanceled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"reviewerAddresses\",\"type\":\"address[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TaskChangedViaSignatures\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"taskId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TaskCompleted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"taskId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"deliverableHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TaskDeliverableSubmitted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"taskId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"dueDate\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TaskDueDateSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"taskId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TaskFinalized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"taskId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"enum ColonyDataTypes.TaskRole\",\"name\":\"role\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TaskPayoutSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"taskId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"enum ColonyDataTypes.TaskRole\",\"name\":\"role\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TaskRoleUserSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"taskId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"skillId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TaskSkillSet\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"taskId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"enum ColonyDataTypes.TaskRole\",\"name\":\"role\",\"type\":\"uint8\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint8\",\"name\":\"rating\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TaskWorkRatingRevealed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenUnlocked\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokensBurned\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"agent\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"who\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokensMinted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approveExitRecovery\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_slot\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"checkNotAdditionalProtectedVariable\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enterRecoveryMode\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"userAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"payload\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"sigR\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"sigS\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"sigV\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"executeMetaTransaction\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"exitRecoveryMode\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"userAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMetatransactionNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isInRecoveryMode\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"inRecoveryMode\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes[]\",\"name\":\"data\",\"type\":\"bytes[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"multicall\",\"outputs\":[{\"internalType\":\"bytes[]\",\"name\":\"results\",\"type\":\"bytes[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"numRecoveryRoles\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"numRoles\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeRecoveryRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRecoveryRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_slot\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_value\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setStorageSlotRecovery\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"authority\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"colonyAuthority\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"colonyOwner\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"colonyVersion\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_newVersion\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgrade\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"finishUpgrade\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getColonyNetwork\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"colonyNetwork\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_action\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"makeArbitraryTransaction\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"_targets\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"_actions\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_strict\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"makeArbitraryTransactions\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_action\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"makeSingleArbitraryTransaction\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"success\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_txHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_metadata\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"annotateTransaction\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_setTo\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRootRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_setTo\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setArbitrationRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_setTo\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setArchitectureRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_setTo\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setFundingRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_setTo\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAdministrationRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_roles\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setUserRoles\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"enum ColonyDataTypes.ColonyRole\",\"name\":\"_role\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasUserRole\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"hasRole\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"enum ColonyDataTypes.ColonyRole\",\"name\":\"_role\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childDomainId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasInheritedUserRole\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"hasRole\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childDomainId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"userCanSetRoles\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"canSet\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domain\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getUserRoles\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"roles\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"_sig\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCapabilityRoles\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"roles\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_amount\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"emitDomainReputationReward\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_amount\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"emitSkillReputationReward\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_amount\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"emitDomainReputationPenalty\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_amount\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"emitSkillReputationPenalty\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_colonyNetworkAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialiseColony\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_metadata\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"editColony\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_metadataDelta\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"editColonyByDelta\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"_users\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"int256[]\",\"name\":\"_amount\",\"type\":\"int256[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"bootstrapColony\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_wad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintTokens\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_guy\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_wad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mintTokensFor\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"lockToken\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"timesLocked\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lockId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unlockTokenForUser\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"colonyName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"orbitdb\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerColonyLabel\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"orbitdb\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateColonyOrbitDB\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"extensionId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"version\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"installExtension\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"extensionId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newVersion\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"upgradeExtension\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"extensionId\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"deprecated\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deprecateExtension\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"extensionId\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"uninstallExtension\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialiseRootLocalSkill\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addLocalSkill\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"localSkillId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"deprecated\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deprecateLocalSkill\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRootLocalSkill\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"rootLocalSkill\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_parentDomainId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addDomain\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_parentDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_metadata\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addDomain\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_metadata\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"editDomain\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_deprecated\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deprecateDomain\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDomain\",\"outputs\":[{\"internalType\":\"struct ColonyDataTypes.Domain\",\"name\":\"domain\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"skillId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fundingPotId\",\"type\":\"uint256\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDomainCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"count\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"key\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"value\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"branchMask\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"siblings\",\"type\":\"bytes32[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"verifyReputationProof\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"isValid\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_globalClaimDelay\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDefaultGlobalClaimDelay\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"makeExpenditure\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"expenditureId\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferExpenditure\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferExpenditureViaArbitration\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"cancelExpenditure\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"lockExpenditure\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"finalizeExpenditure\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_metadata\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpenditureMetadata\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_metadata\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpenditureMetadata\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_slot\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"_recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpenditureRecipient\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"_slots\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"address payable[]\",\"name\":\"_recipients\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpenditureRecipients\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"_slots\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"_amounts\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpenditurePayouts\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_slot\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpenditurePayout\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_slot\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpenditurePayout\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_slot\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpenditureSkill\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"_slots\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"_skillIds\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpenditureSkills\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_slot\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_claimDelay\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpenditureClaimDelay\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"_slots\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"_claimDelays\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpenditureClaimDelays\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"_slots\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"int256[]\",\"name\":\"_payoutModifiers\",\"type\":\"int256[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpenditurePayoutModifiers\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"_recipientSlots\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"address payable[]\",\"name\":\"_recipients\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"_skillIdSlots\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"_skillIds\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"_claimDelaySlots\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"_claimDelays\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"_payoutModifierSlots\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"int256[]\",\"name\":\"_payoutModifiers\",\"type\":\"int256[]\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"_payoutTokens\",\"type\":\"address[]\",\"components\":[]},{\"internalType\":\"uint256[][]\",\"name\":\"_payoutSlots\",\"type\":\"uint256[][]\",\"components\":[]},{\"internalType\":\"uint256[][]\",\"name\":\"_payoutValues\",\"type\":\"uint256[][]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpenditureValues\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_storageSlot\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool[]\",\"name\":\"_mask\",\"type\":\"bool[]\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"_keys\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_value\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setExpenditureState\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_slot\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimExpenditurePayout\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getExpenditureCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"count\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getExpenditure\",\"outputs\":[{\"internalType\":\"struct ColonyDataTypes.Expenditure\",\"name\":\"expenditure\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ColonyDataTypes.ExpenditureStatus\",\"name\":\"status\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fundingPotId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"finalizedTimestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"globalClaimDelay\",\"type\":\"uint256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_slot\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getExpenditureSlot\",\"outputs\":[{\"internalType\":\"struct ColonyDataTypes.ExpenditureSlot\",\"name\":\"expenditureSlot\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"claimDelay\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"payoutModifier\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"skills\",\"type\":\"uint256[]\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_slot\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getExpenditureSlotPayout\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"_recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addPayment\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"paymentId\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"finalizePayment\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"_recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPaymentRecipient\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPaymentSkill\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPaymentPayout\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPayment\",\"outputs\":[{\"internalType\":\"struct ColonyDataTypes.Payment\",\"name\":\"payment\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"finalized\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fundingPotId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"skills\",\"type\":\"uint256[]\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimPayment\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getPaymentCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"count\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_specificationHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_dueDate\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"makeTask\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTaskCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"count\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTaskChangeNonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint8[]\",\"name\":\"_sigV\",\"type\":\"uint8[]\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"_sigR\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"_sigS\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"uint8[]\",\"name\":\"_mode\",\"type\":\"uint8[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeTaskChange\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint8[]\",\"name\":\"_sigV\",\"type\":\"uint8[]\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"_sigR\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"_sigS\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"uint8[]\",\"name\":\"_mode\",\"type\":\"uint8[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"executeTaskRoleAssignment\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_role\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_ratingSecret\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitTaskWorkRating\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_role\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_rating\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_salt\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"revealTaskWorkRating\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_salt\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"generateSecret\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"secret\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTaskWorkRatingSecretsInfo\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"nSecrets\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lastSubmittedAt\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_role\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTaskWorkRatingSecret\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"secret\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTaskManagerRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTaskEvaluatorRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTaskWorkerRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeTaskEvaluatorRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeTaskWorkerRole\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_skillId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTaskSkill\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_specificationHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTaskBrief\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_dueDate\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTaskDueDate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_deliverableHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitTaskDeliverable\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_deliverableHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"_ratingSecret\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitTaskDeliverableAndRating\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"finalizeTask\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"cancelTask\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"completeTask\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTask\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"specificationHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"deliverableHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"enum ColonyDataTypes.TaskStatus\",\"name\":\"status\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"dueDate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"fundingPotId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"completionTimestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"skillIds\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_role\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTaskRole\",\"outputs\":[{\"internalType\":\"struct ColonyDataTypes.Role\",\"name\":\"role\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address payable\",\"name\":\"user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"rateFail\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"enum ColonyDataTypes.TaskRatings\",\"name\":\"rating\",\"type\":\"uint8\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_rewardInverse\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRewardInverse\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRewardInverse\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"rewardInverse\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_role\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTaskPayout\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTaskManagerPayout\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTaskEvaluatorPayout\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTaskWorkerPayout\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_managerAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_evaluatorAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_workerAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAllTaskPayouts\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_role\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimTaskPayout\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"key\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"value\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"branchMask\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"siblings\",\"type\":\"bytes32[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"startNextRewardPayout\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_payoutId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[7]\",\"name\":\"_squareRoots\",\"type\":\"uint256[7]\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"key\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"value\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"branchMask\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"siblings\",\"type\":\"bytes32[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimRewardPayout\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_payoutId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRewardPayoutInfo\",\"outputs\":[{\"internalType\":\"struct ColonyDataTypes.RewardPayoutCycle\",\"name\":\"rewardPayoutCycle\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bytes32\",\"name\":\"reputationState\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"colonyWideReputation\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalTokens\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"tokenAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"blockTimestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amountRemaining\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"finalized\",\"type\":\"bool\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_payoutId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"finalizeRewardPayout\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_id\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFundingPot\",\"outputs\":[{\"internalType\":\"enum ColonyDataTypes.FundingPotAssociatedType\",\"name\":\"associatedType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"associatedTypeId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"payoutsWeCannotMake\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFundingPotCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"count\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_potId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFundingPotBalance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"balance\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_potId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getFundingPotPayout\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"payout\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_fromChildSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_toChildSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_fromPot\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_toPot\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"moveFundsBetweenPots\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_fromChildSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_toChildSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_fromPot\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_toPot\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"moveFundsBetweenPots\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimColonyFunds\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getNonRewardPotsTotal\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_approvee\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approveStake\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"obligateStake\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deobligateStake\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_permissionDomainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_childSkillIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_obligator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferStake\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_obligator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getApproval\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"approval\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_user\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_obligator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_domainId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getObligation\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"obligation\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_fundingPotId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDomainFromFundingPot\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"domainId\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"burnTokens\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unlockToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateApprovalAmount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTokenApproval\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTotalTokenApproval\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static COLONY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct Colony<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Colony<M> {
        fn clone(&self) -> Self {
            Colony(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Colony<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Colony<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Colony))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Colony<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), COLONY_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `addDomain` (0x1f7e3ebe) function"]
        pub fn add_domain(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            parent_domain_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [31, 126, 62, 190],
                    (permission_domain_id, child_skill_index, parent_domain_id),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addDomain` (0xdf91f650) function"]
        pub fn add_domain_with_permission_domain_id_and_child_skill_index_and_metadata(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            parent_domain_id: ethers::core::types::U256,
            metadata: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [223, 145, 246, 80],
                    (
                        permission_domain_id,
                        child_skill_index,
                        parent_domain_id,
                        metadata,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addLocalSkill` (0x64055d04) function"]
        pub fn add_local_skill(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 5, 93, 4], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addPayment` (0x4f8df643) function"]
        pub fn add_payment(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            domain_id: ethers::core::types::U256,
            skill_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [79, 141, 246, 67],
                    (
                        permission_domain_id,
                        child_skill_index,
                        recipient,
                        token,
                        amount,
                        domain_id,
                        skill_id,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `annotateTransaction` (0x6c8ec8b5) function"]
        pub fn annotate_transaction(
            &self,
            tx_hash: [u8; 32],
            metadata: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 142, 200, 181], (tx_hash, metadata))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approveExitRecovery` (0x31292ba7) function"]
        pub fn approve_exit_recovery(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([49, 41, 43, 167], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approveStake` (0x354f33d8) function"]
        pub fn approve_stake(
            &self,
            approvee: ethers::core::types::Address,
            domain_id: ethers::core::types::U256,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 79, 51, 216], (approvee, domain_id, amount))
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
        #[doc = "Calls the contract's `bootstrapColony` (0xe723d631) function"]
        pub fn bootstrap_colony(
            &self,
            users: ::std::vec::Vec<ethers::core::types::Address>,
            amount: ::std::vec::Vec<I256>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 35, 214, 49], (users, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burnTokens` (0x0d1118ce) function"]
        pub fn burn_tokens(
            &self,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 17, 24, 206], (token, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancelExpenditure` (0x3ddd42d0) function"]
        pub fn cancel_expenditure(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 221, 66, 208], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancelTask` (0x7eec20a8) function"]
        pub fn cancel_task(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([126, 236, 32, 168], id)
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
        #[doc = "Calls the contract's `claimColonyFunds` (0x89224a1e) function"]
        pub fn claim_colony_funds(
            &self,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 34, 74, 30], token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimExpenditurePayout` (0x2fe96899) function"]
        pub fn claim_expenditure_payout(
            &self,
            id: ethers::core::types::U256,
            slot: ethers::core::types::U256,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 233, 104, 153], (id, slot, token))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimPayment` (0x0f4c52f3) function"]
        pub fn claim_payment(
            &self,
            id: ethers::core::types::U256,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 76, 82, 243], (id, token))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimRewardPayout` (0x6fc14a90) function"]
        pub fn claim_reward_payout(
            &self,
            payout_id: ethers::core::types::U256,
            square_roots: [ethers::core::types::U256; 7usize],
            key: ethers::core::types::Bytes,
            value: ethers::core::types::Bytes,
            branch_mask: ethers::core::types::U256,
            siblings: ::std::vec::Vec<[u8; 32]>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [111, 193, 74, 144],
                    (payout_id, square_roots, key, value, branch_mask, siblings),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimTaskPayout` (0x7c506b0b) function"]
        pub fn claim_task_payout(
            &self,
            id: ethers::core::types::U256,
            role: u8,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 80, 107, 11], (id, role, token))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `completeTask` (0xe1e29558) function"]
        pub fn complete_task(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 226, 149, 88], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deobligateStake` (0x37a68e67) function"]
        pub fn deobligate_stake(
            &self,
            user: ethers::core::types::Address,
            domain_id: ethers::core::types::U256,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([55, 166, 142, 103], (user, domain_id, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deprecateDomain` (0xe70e808d) function"]
        pub fn deprecate_domain(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            domain_id: ethers::core::types::U256,
            deprecated: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [231, 14, 128, 141],
                    (
                        permission_domain_id,
                        child_skill_index,
                        domain_id,
                        deprecated,
                    ),
                )
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
        #[doc = "Calls the contract's `deprecateLocalSkill` (0xa3fb7384) function"]
        pub fn deprecate_local_skill(
            &self,
            local_skill_id: ethers::core::types::U256,
            deprecated: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 251, 115, 132], (local_skill_id, deprecated))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `editColony` (0x102b1b35) function"]
        pub fn edit_colony(
            &self,
            metadata: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 43, 27, 53], metadata)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `editColonyByDelta` (0xa72828b1) function"]
        pub fn edit_colony_by_delta(
            &self,
            metadata_delta: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 40, 40, 177], metadata_delta)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `editDomain` (0x6406b868) function"]
        pub fn edit_domain(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            domain_id: ethers::core::types::U256,
            metadata: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [100, 6, 184, 104],
                    (permission_domain_id, child_skill_index, domain_id, metadata),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `emitDomainReputationPenalty` (0xfce891a5) function"]
        pub fn emit_domain_reputation_penalty(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            domain_id: ethers::core::types::U256,
            user: ethers::core::types::Address,
            amount: I256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [252, 232, 145, 165],
                    (
                        permission_domain_id,
                        child_skill_index,
                        domain_id,
                        user,
                        amount,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `emitDomainReputationReward` (0x944c650f) function"]
        pub fn emit_domain_reputation_reward(
            &self,
            domain_id: ethers::core::types::U256,
            user: ethers::core::types::Address,
            amount: I256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([148, 76, 101, 15], (domain_id, user, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `emitSkillReputationPenalty` (0x5d860695) function"]
        pub fn emit_skill_reputation_penalty(
            &self,
            skill_id: ethers::core::types::U256,
            user: ethers::core::types::Address,
            amount: I256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([93, 134, 6, 149], (skill_id, user, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `emitSkillReputationReward` (0x596d368e) function"]
        pub fn emit_skill_reputation_reward(
            &self,
            skill_id: ethers::core::types::U256,
            user: ethers::core::types::Address,
            amount: I256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 109, 54, 142], (skill_id, user, amount))
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
        #[doc = "Calls the contract's `executeTaskChange` (0x74a2200e) function"]
        pub fn execute_task_change(
            &self,
            sig_v: ::std::vec::Vec<u8>,
            sig_r: ::std::vec::Vec<[u8; 32]>,
            sig_s: ::std::vec::Vec<[u8; 32]>,
            mode: ::std::vec::Vec<u8>,
            value: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([116, 162, 32, 14], (sig_v, sig_r, sig_s, mode, value, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeTaskRoleAssignment` (0x24aa4948) function"]
        pub fn execute_task_role_assignment(
            &self,
            sig_v: ::std::vec::Vec<u8>,
            sig_r: ::std::vec::Vec<[u8; 32]>,
            sig_s: ::std::vec::Vec<[u8; 32]>,
            mode: ::std::vec::Vec<u8>,
            value: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 170, 73, 72], (sig_v, sig_r, sig_s, mode, value, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exitRecoveryMode` (0x38e5b7ae) function"]
        pub fn exit_recovery_mode(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 229, 183, 174], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `finalizeExpenditure` (0xb3e75ec6) function"]
        pub fn finalize_expenditure(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([179, 231, 94, 198], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `finalizePayment` (0xd961a420) function"]
        pub fn finalize_payment(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [217, 97, 164, 32],
                    (permission_domain_id, child_skill_index, id),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `finalizeRewardPayout` (0x51c38cd9) function"]
        pub fn finalize_reward_payout(
            &self,
            payout_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 195, 140, 217], payout_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `finalizeTask` (0x88488c33) function"]
        pub fn finalize_task(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([136, 72, 140, 51], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `finishUpgrade` (0x340df28f) function"]
        pub fn finish_upgrade(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([52, 13, 242, 143], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `generateSecret` (0x1ae3aaa4) function"]
        pub fn generate_secret(
            &self,
            salt: [u8; 32],
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([26, 227, 170, 164], (salt, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getApproval` (0x312d371d) function"]
        pub fn get_approval(
            &self,
            user: ethers::core::types::Address,
            obligator: ethers::core::types::Address,
            domain_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([49, 45, 55, 29], (user, obligator, domain_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCapabilityRoles` (0x2dfbb083) function"]
        pub fn get_capability_roles(
            &self,
            sig: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([45, 251, 176, 131], sig)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getColonyNetwork` (0x4e5e8395) function"]
        pub fn get_colony_network(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([78, 94, 131, 149], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getDomain` (0x1a7a98e2) function"]
        pub fn get_domain(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, Domain> {
            self.0
                .method_hash([26, 122, 152, 226], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getDomainCount` (0x13e91f08) function"]
        pub fn get_domain_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([19, 233, 31, 8], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getDomainFromFundingPot` (0xb61b5317) function"]
        pub fn get_domain_from_funding_pot(
            &self,
            funding_pot_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([182, 27, 83, 23], funding_pot_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getExpenditure` (0xe96aad86) function"]
        pub fn get_expenditure(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, Expenditure> {
            self.0
                .method_hash([233, 106, 173, 134], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getExpenditureCount` (0x7a4e6ff1) function"]
        pub fn get_expenditure_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([122, 78, 111, 241], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getExpenditureSlot` (0x994b2281) function"]
        pub fn get_expenditure_slot(
            &self,
            id: ethers::core::types::U256,
            slot: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ExpenditureSlot> {
            self.0
                .method_hash([153, 75, 34, 129], (id, slot))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getExpenditureSlotPayout` (0x29abcdce) function"]
        pub fn get_expenditure_slot_payout(
            &self,
            id: ethers::core::types::U256,
            slot: ethers::core::types::U256,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([41, 171, 205, 206], (id, slot, token))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getFundingPot` (0x7c1735a8) function"]
        pub fn get_funding_pot(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (u8, ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([124, 23, 53, 168], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getFundingPotBalance` (0xaae3bddd) function"]
        pub fn get_funding_pot_balance(
            &self,
            pot_id: ethers::core::types::U256,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([170, 227, 189, 221], (pot_id, token))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getFundingPotCount` (0x25f759e3) function"]
        pub fn get_funding_pot_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([37, 247, 89, 227], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getFundingPotPayout` (0xc5d0886e) function"]
        pub fn get_funding_pot_payout(
            &self,
            pot_id: ethers::core::types::U256,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([197, 208, 136, 110], (pot_id, token))
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
        #[doc = "Calls the contract's `getNonRewardPotsTotal` (0x34fa7aa2) function"]
        pub fn get_non_reward_pots_total(
            &self,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([52, 250, 122, 162], token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getObligation` (0x5f2b8cb2) function"]
        pub fn get_obligation(
            &self,
            user: ethers::core::types::Address,
            obligator: ethers::core::types::Address,
            domain_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([95, 43, 140, 178], (user, obligator, domain_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPayment` (0x3280a836) function"]
        pub fn get_payment(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, Payment> {
            self.0
                .method_hash([50, 128, 168, 54], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPaymentCount` (0xaf32a03f) function"]
        pub fn get_payment_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([175, 50, 160, 63], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRewardInverse` (0x4cb164c7) function"]
        pub fn get_reward_inverse(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([76, 177, 100, 199], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRewardPayoutInfo` (0x3644a3fd) function"]
        pub fn get_reward_payout_info(
            &self,
            payout_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, RewardPayoutCycle> {
            self.0
                .method_hash([54, 68, 163, 253], payout_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRootLocalSkill` (0xbda67023) function"]
        pub fn get_root_local_skill(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([189, 166, 112, 35], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTask` (0x1d65e77e) function"]
        pub fn get_task(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                [u8; 32],
                [u8; 32],
                u8,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ::std::vec::Vec<ethers::core::types::U256>,
            ),
        > {
            self.0
                .method_hash([29, 101, 231, 126], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTaskChangeNonce` (0xce7cee49) function"]
        pub fn get_task_change_nonce(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([206, 124, 238, 73], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTaskCount` (0xc17a340e) function"]
        pub fn get_task_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([193, 122, 52, 14], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTaskPayout` (0x057dda2c) function"]
        pub fn get_task_payout(
            &self,
            id: ethers::core::types::U256,
            role: u8,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([5, 125, 218, 44], (id, role, token))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTaskRole` (0x1e8954de) function"]
        pub fn get_task_role(
            &self,
            id: ethers::core::types::U256,
            role: u8,
        ) -> ethers::contract::builders::ContractCall<M, Role> {
            self.0
                .method_hash([30, 137, 84, 222], (id, role))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTaskWorkRatingSecret` (0xad4e908c) function"]
        pub fn get_task_work_rating_secret(
            &self,
            id: ethers::core::types::U256,
            role: u8,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([173, 78, 144, 140], (id, role))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTaskWorkRatingSecretsInfo` (0x01f8fb15) function"]
        pub fn get_task_work_rating_secrets_info(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (ethers::core::types::U256, ethers::core::types::U256),
        > {
            self.0
                .method_hash([1, 248, 251, 21], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getToken` (0x21df0da7) function"]
        pub fn get_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([33, 223, 13, 167], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTokenApproval` (0x65dfedc4) function"]
        pub fn get_token_approval(
            &self,
            token: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([101, 223, 237, 196], (token, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTotalTokenApproval` (0xfe359995) function"]
        pub fn get_total_token_approval(
            &self,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([254, 53, 153, 149], token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUserRoles` (0x595b03f0) function"]
        pub fn get_user_roles(
            &self,
            user: ethers::core::types::Address,
            domain: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([89, 91, 3, 240], (user, domain))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasInheritedUserRole` (0x3354f138) function"]
        pub fn has_inherited_user_role(
            &self,
            user: ethers::core::types::Address,
            domain_id: ethers::core::types::U256,
            role: u8,
            child_skill_index: ethers::core::types::U256,
            child_domain_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [51, 84, 241, 56],
                    (user, domain_id, role, child_skill_index, child_domain_id),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasUserRole` (0xab2f7ae2) function"]
        pub fn has_user_role(
            &self,
            user: ethers::core::types::Address,
            domain_id: ethers::core::types::U256,
            role: u8,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([171, 47, 122, 226], (user, domain_id, role))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialiseColony` (0x79d95b92) function"]
        pub fn initialise_colony(
            &self,
            colony_network_address: ethers::core::types::Address,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 217, 91, 146], (colony_network_address, token))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialiseRootLocalSkill` (0x2501ab2c) function"]
        pub fn initialise_root_local_skill(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
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
        #[doc = "Calls the contract's `isInRecoveryMode` (0x80d9fad3) function"]
        pub fn is_in_recovery_mode(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([128, 217, 250, 211], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lockExpenditure` (0x7883b770) function"]
        pub fn lock_expenditure(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 131, 183, 112], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lockToken` (0xbca7a9e2) function"]
        pub fn lock_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([188, 167, 169, 226], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `makeArbitraryTransaction` (0x67c351c2) function"]
        pub fn make_arbitrary_transaction(
            &self,
            to: ethers::core::types::Address,
            action: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([103, 195, 81, 194], (to, action))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `makeArbitraryTransactions` (0xc6356efe) function"]
        pub fn make_arbitrary_transactions(
            &self,
            targets: ::std::vec::Vec<ethers::core::types::Address>,
            actions: ::std::vec::Vec<ethers::core::types::Bytes>,
            strict: bool,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([198, 53, 110, 254], (targets, actions, strict))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `makeExpenditure` (0xa5487a37) function"]
        pub fn make_expenditure(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            domain_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [165, 72, 122, 55],
                    (permission_domain_id, child_skill_index, domain_id),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `makeSingleArbitraryTransaction` (0x663b482d) function"]
        pub fn make_single_arbitrary_transaction(
            &self,
            target: ethers::core::types::Address,
            action: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([102, 59, 72, 45], (target, action))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `makeTask` (0x663cb633) function"]
        pub fn make_task(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            specification_hash: [u8; 32],
            domain_id: ethers::core::types::U256,
            skill_id: ethers::core::types::U256,
            due_date: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [102, 60, 182, 51],
                    (
                        permission_domain_id,
                        child_skill_index,
                        specification_hash,
                        domain_id,
                        skill_id,
                        due_date,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintTokens` (0x97304ced) function"]
        pub fn mint_tokens(
            &self,
            wad: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([151, 48, 76, 237], wad)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintTokensFor` (0x8f6811a3) function"]
        pub fn mint_tokens_for(
            &self,
            guy: ethers::core::types::Address,
            wad: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([143, 104, 17, 163], (guy, wad))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `moveFundsBetweenPots` (0x3e77c549) function"]
        pub fn move_funds_between_pots_with_permission_domain_id_and_child_skill_index_and_domain_id_and_from_child_skill_index_and_to_child_skill_index_and_from_pot_and_to_pot(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            domain_id: ethers::core::types::U256,
            from_child_skill_index: ethers::core::types::U256,
            to_child_skill_index: ethers::core::types::U256,
            from_pot: ethers::core::types::U256,
            to_pot: ethers::core::types::U256,
            amount: ethers::core::types::U256,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [62, 119, 197, 73],
                    (
                        permission_domain_id,
                        child_skill_index,
                        domain_id,
                        from_child_skill_index,
                        to_child_skill_index,
                        from_pot,
                        to_pot,
                        amount,
                        token,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `moveFundsBetweenPots` (0xc68d19ea) function"]
        pub fn move_funds_between_pots(
            &self,
            permission_domain_id: ethers::core::types::U256,
            from_child_skill_index: ethers::core::types::U256,
            to_child_skill_index: ethers::core::types::U256,
            from_pot: ethers::core::types::U256,
            to_pot: ethers::core::types::U256,
            amount: ethers::core::types::U256,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [198, 141, 25, 234],
                    (
                        permission_domain_id,
                        from_child_skill_index,
                        to_child_skill_index,
                        from_pot,
                        to_pot,
                        amount,
                        token,
                    ),
                )
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
        #[doc = "Calls the contract's `obligateStake` (0x096abb49) function"]
        pub fn obligate_stake(
            &self,
            user: ethers::core::types::Address,
            domain_id: ethers::core::types::U256,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 106, 187, 73], (user, domain_id, amount))
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
        #[doc = "Calls the contract's `removeRecoveryRole` (0x99cf7cc7) function"]
        pub fn remove_recovery_role(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 207, 124, 199], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeTaskEvaluatorRole` (0xa1c60e63) function"]
        pub fn remove_task_evaluator_role(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 198, 14, 99], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeTaskWorkerRole` (0x46889ecb) function"]
        pub fn remove_task_worker_role(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 136, 158, 203], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revealTaskWorkRating` (0x934a1dc7) function"]
        pub fn reveal_task_work_rating(
            &self,
            id: ethers::core::types::U256,
            role: u8,
            rating: u8,
            salt: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([147, 74, 29, 199], (id, role, rating, salt))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAdministrationRole` (0x63331de3) function"]
        pub fn set_administration_role(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            user: ethers::core::types::Address,
            domain_id: ethers::core::types::U256,
            set_to: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [99, 51, 29, 227],
                    (
                        permission_domain_id,
                        child_skill_index,
                        user,
                        domain_id,
                        set_to,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAllTaskPayouts` (0x5dbe69b4) function"]
        pub fn set_all_task_payouts(
            &self,
            id: ethers::core::types::U256,
            token: ethers::core::types::Address,
            manager_amount: ethers::core::types::U256,
            evaluator_amount: ethers::core::types::U256,
            worker_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [93, 190, 105, 180],
                    (id, token, manager_amount, evaluator_amount, worker_amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setArbitrationRole` (0xb7e9b3f2) function"]
        pub fn set_arbitration_role(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            user: ethers::core::types::Address,
            domain_id: ethers::core::types::U256,
            set_to: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [183, 233, 179, 242],
                    (
                        permission_domain_id,
                        child_skill_index,
                        user,
                        domain_id,
                        set_to,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setArchitectureRole` (0xe9c75e47) function"]
        pub fn set_architecture_role(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            user: ethers::core::types::Address,
            domain_id: ethers::core::types::U256,
            set_to: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [233, 199, 94, 71],
                    (
                        permission_domain_id,
                        child_skill_index,
                        user,
                        domain_id,
                        set_to,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDefaultGlobalClaimDelay` (0x8c2c0421) function"]
        pub fn set_default_global_claim_delay(
            &self,
            global_claim_delay: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 44, 4, 33], global_claim_delay)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExpenditureClaimDelay` (0x09895c21) function"]
        pub fn set_expenditure_claim_delay(
            &self,
            id: ethers::core::types::U256,
            slot: ethers::core::types::U256,
            claim_delay: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 137, 92, 33], (id, slot, claim_delay))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExpenditureClaimDelays` (0x5bdb365d) function"]
        pub fn set_expenditure_claim_delays(
            &self,
            id: ethers::core::types::U256,
            slots: ::std::vec::Vec<ethers::core::types::U256>,
            claim_delays: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([91, 219, 54, 93], (id, slots, claim_delays))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExpenditureMetadata` (0x6b9bf027) function"]
        pub fn set_expenditure_metadata_with_permission_domain_id_and_child_skill_index(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            id: ethers::core::types::U256,
            metadata: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [107, 155, 240, 39],
                    (permission_domain_id, child_skill_index, id, metadata),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExpenditureMetadata` (0xe68fc17d) function"]
        pub fn set_expenditure_metadata(
            &self,
            id: ethers::core::types::U256,
            metadata: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 143, 193, 125], (id, metadata))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExpenditurePayout` (0x476d1d05) function"]
        pub fn set_expenditure_payout(
            &self,
            id: ethers::core::types::U256,
            slot: ethers::core::types::U256,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 109, 29, 5], (id, slot, token, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExpenditurePayout` (0xbae82ec9) function"]
        pub fn set_expenditure_payout_with_permission_domain_id_and_child_skill_index_and_id_and_slot(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            id: ethers::core::types::U256,
            slot: ethers::core::types::U256,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [186, 232, 46, 201],
                    (
                        permission_domain_id,
                        child_skill_index,
                        id,
                        slot,
                        token,
                        amount,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExpenditurePayoutModifiers` (0x89126bb7) function"]
        pub fn set_expenditure_payout_modifiers(
            &self,
            id: ethers::core::types::U256,
            slots: ::std::vec::Vec<ethers::core::types::U256>,
            payout_modifiers: ::std::vec::Vec<I256>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 18, 107, 183], (id, slots, payout_modifiers))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExpenditurePayouts` (0xfc9bdbfe) function"]
        pub fn set_expenditure_payouts(
            &self,
            id: ethers::core::types::U256,
            slots: ::std::vec::Vec<ethers::core::types::U256>,
            token: ethers::core::types::Address,
            amounts: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 155, 219, 254], (id, slots, token, amounts))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExpenditureRecipient` (0x17b46cb7) function"]
        pub fn set_expenditure_recipient(
            &self,
            id: ethers::core::types::U256,
            slot: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 180, 108, 183], (id, slot, recipient))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExpenditureRecipients` (0xceba22fe) function"]
        pub fn set_expenditure_recipients(
            &self,
            id: ethers::core::types::U256,
            slots: ::std::vec::Vec<ethers::core::types::U256>,
            recipients: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 186, 34, 254], (id, slots, recipients))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExpenditureSkill` (0xfa13ed72) function"]
        pub fn set_expenditure_skill(
            &self,
            id: ethers::core::types::U256,
            slot: ethers::core::types::U256,
            skill_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 19, 237, 114], (id, slot, skill_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExpenditureSkills` (0xb58179f9) function"]
        pub fn set_expenditure_skills(
            &self,
            id: ethers::core::types::U256,
            slots: ::std::vec::Vec<ethers::core::types::U256>,
            skill_ids: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 129, 121, 249], (id, slots, skill_ids))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExpenditureState` (0xc9a2ce7c) function"]
        pub fn set_expenditure_state(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            id: ethers::core::types::U256,
            storage_slot: ethers::core::types::U256,
            mask: ::std::vec::Vec<bool>,
            keys: ::std::vec::Vec<[u8; 32]>,
            value: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [201, 162, 206, 124],
                    (
                        permission_domain_id,
                        child_skill_index,
                        id,
                        storage_slot,
                        mask,
                        keys,
                        value,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setExpenditureValues` (0x1c9f6c6d) function"]
        pub fn set_expenditure_values(
            &self,
            id: ethers::core::types::U256,
            recipient_slots: ::std::vec::Vec<ethers::core::types::U256>,
            recipients: ::std::vec::Vec<ethers::core::types::Address>,
            skill_id_slots: ::std::vec::Vec<ethers::core::types::U256>,
            skill_ids: ::std::vec::Vec<ethers::core::types::U256>,
            claim_delay_slots: ::std::vec::Vec<ethers::core::types::U256>,
            claim_delays: ::std::vec::Vec<ethers::core::types::U256>,
            payout_modifier_slots: ::std::vec::Vec<ethers::core::types::U256>,
            payout_modifiers: ::std::vec::Vec<I256>,
            payout_tokens: ::std::vec::Vec<ethers::core::types::Address>,
            payout_slots: ::std::vec::Vec<::std::vec::Vec<ethers::core::types::U256>>,
            payout_values: ::std::vec::Vec<::std::vec::Vec<ethers::core::types::U256>>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [28, 159, 108, 109],
                    (
                        id,
                        recipient_slots,
                        recipients,
                        skill_id_slots,
                        skill_ids,
                        claim_delay_slots,
                        claim_delays,
                        payout_modifier_slots,
                        payout_modifiers,
                        payout_tokens,
                        payout_slots,
                        payout_values,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFundingRole` (0x9552a1e5) function"]
        pub fn set_funding_role(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            user: ethers::core::types::Address,
            domain_id: ethers::core::types::U256,
            set_to: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [149, 82, 161, 229],
                    (
                        permission_domain_id,
                        child_skill_index,
                        user,
                        domain_id,
                        set_to,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPaymentPayout` (0x54a9a082) function"]
        pub fn set_payment_payout(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            id: ethers::core::types::U256,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [84, 169, 160, 130],
                    (permission_domain_id, child_skill_index, id, token, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPaymentRecipient` (0x8904ba90) function"]
        pub fn set_payment_recipient(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            id: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [137, 4, 186, 144],
                    (permission_domain_id, child_skill_index, id, recipient),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPaymentSkill` (0x255fc39c) function"]
        pub fn set_payment_skill(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            id: ethers::core::types::U256,
            skill_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [37, 95, 195, 156],
                    (permission_domain_id, child_skill_index, id, skill_id),
                )
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
        #[doc = "Calls the contract's `setRewardInverse` (0x2b949c99) function"]
        pub fn set_reward_inverse(
            &self,
            reward_inverse: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([43, 148, 156, 153], reward_inverse)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRootRole` (0xe178f31a) function"]
        pub fn set_root_role(
            &self,
            user: ethers::core::types::Address,
            set_to: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 120, 243, 26], (user, set_to))
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
        #[doc = "Calls the contract's `setTaskBrief` (0xda4db249) function"]
        pub fn set_task_brief(
            &self,
            id: ethers::core::types::U256,
            specification_hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([218, 77, 178, 73], (id, specification_hash))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTaskDueDate` (0xcae960fe) function"]
        pub fn set_task_due_date(
            &self,
            id: ethers::core::types::U256,
            due_date: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 233, 96, 254], (id, due_date))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTaskEvaluatorPayout` (0x6fb0794f) function"]
        pub fn set_task_evaluator_payout(
            &self,
            id: ethers::core::types::U256,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 176, 121, 79], (id, token, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTaskEvaluatorRole` (0xe104695a) function"]
        pub fn set_task_evaluator_role(
            &self,
            id: ethers::core::types::U256,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 4, 105, 90], (id, user))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTaskManagerPayout` (0x6aaf4a77) function"]
        pub fn set_task_manager_payout(
            &self,
            id: ethers::core::types::U256,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([106, 175, 74, 119], (id, token, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTaskManagerRole` (0xb35383e3) function"]
        pub fn set_task_manager_role(
            &self,
            id: ethers::core::types::U256,
            user: ethers::core::types::Address,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [179, 83, 131, 227],
                    (id, user, permission_domain_id, child_skill_index),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTaskSkill` (0xb8984c5a) function"]
        pub fn set_task_skill(
            &self,
            id: ethers::core::types::U256,
            skill_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 152, 76, 90], (id, skill_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTaskWorkerPayout` (0x2cf62b39) function"]
        pub fn set_task_worker_payout(
            &self,
            id: ethers::core::types::U256,
            token: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([44, 246, 43, 57], (id, token, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTaskWorkerRole` (0x74773439) function"]
        pub fn set_task_worker_role(
            &self,
            id: ethers::core::types::U256,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([116, 119, 52, 57], (id, user))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setUserRoles` (0xcd4a72cb) function"]
        pub fn set_user_roles(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            user: ethers::core::types::Address,
            domain_id: ethers::core::types::U256,
            roles: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [205, 74, 114, 203],
                    (
                        permission_domain_id,
                        child_skill_index,
                        user,
                        domain_id,
                        roles,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `startNextRewardPayout` (0xe6f519ce) function"]
        pub fn start_next_reward_payout(
            &self,
            token: ethers::core::types::Address,
            key: ethers::core::types::Bytes,
            value: ethers::core::types::Bytes,
            branch_mask: ethers::core::types::U256,
            siblings: ::std::vec::Vec<[u8; 32]>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [230, 245, 25, 206],
                    (token, key, value, branch_mask, siblings),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `submitTaskDeliverable` (0x50030964) function"]
        pub fn submit_task_deliverable(
            &self,
            id: ethers::core::types::U256,
            deliverable_hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([80, 3, 9, 100], (id, deliverable_hash))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `submitTaskDeliverableAndRating` (0x194f523d) function"]
        pub fn submit_task_deliverable_and_rating(
            &self,
            id: ethers::core::types::U256,
            deliverable_hash: [u8; 32],
            rating_secret: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 79, 82, 61], (id, deliverable_hash, rating_secret))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `submitTaskWorkRating` (0x3feb369c) function"]
        pub fn submit_task_work_rating(
            &self,
            id: ethers::core::types::U256,
            role: u8,
            rating_secret: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 235, 54, 156], (id, role, rating_secret))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferExpenditure` (0xfe40fbe5) function"]
        pub fn transfer_expenditure(
            &self,
            id: ethers::core::types::U256,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 64, 251, 229], (id, new_owner))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferExpenditureViaArbitration` (0x2c176360) function"]
        pub fn transfer_expenditure_via_arbitration(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            id: ethers::core::types::U256,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [44, 23, 99, 96],
                    (permission_domain_id, child_skill_index, id, new_owner),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferStake` (0x446c1aa2) function"]
        pub fn transfer_stake(
            &self,
            permission_domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            obligator: ethers::core::types::Address,
            user: ethers::core::types::Address,
            domain_id: ethers::core::types::U256,
            amount: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [68, 108, 26, 162],
                    (
                        permission_domain_id,
                        child_skill_index,
                        obligator,
                        user,
                        domain_id,
                        amount,
                        recipient,
                    ),
                )
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
        #[doc = "Calls the contract's `unlockToken` (0x18a24b5b) function"]
        pub fn unlock_token(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 162, 75, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unlockTokenForUser` (0x73c08dde) function"]
        pub fn unlock_token_for_user(
            &self,
            user: ethers::core::types::Address,
            lock_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([115, 192, 141, 222], (user, lock_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateApprovalAmount` (0x811c4f9b) function"]
        pub fn update_approval_amount(
            &self,
            token: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 28, 79, 155], (token, spender))
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
        #[doc = "Calls the contract's `upgrade` (0x45977d03) function"]
        pub fn upgrade(
            &self,
            new_version: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([69, 151, 125, 3], new_version)
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
        #[doc = "Calls the contract's `userCanSetRoles` (0xf4a8d851) function"]
        pub fn user_can_set_roles(
            &self,
            user: ethers::core::types::Address,
            domain_id: ethers::core::types::U256,
            child_skill_index: ethers::core::types::U256,
            child_domain_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [244, 168, 216, 81],
                    (user, domain_id, child_skill_index, child_domain_id),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifyReputationProof` (0x96d933e0) function"]
        pub fn verify_reputation_proof(
            &self,
            key: ethers::core::types::Bytes,
            value: ethers::core::types::Bytes,
            branch_mask: ethers::core::types::U256,
            siblings: ::std::vec::Vec<[u8; 32]>,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([150, 217, 51, 224], (key, value, branch_mask, siblings))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `version` (0x54fd4d50) function"]
        pub fn version(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Annotation` event"]
        pub fn annotation_filter(&self) -> ethers::contract::builders::Event<M, AnnotationFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ArbitraryReputationUpdate` event"]
        pub fn arbitrary_reputation_update_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ArbitraryReputationUpdateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ArbitraryTransaction` event"]
        pub fn arbitrary_transaction_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ArbitraryTransactionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ColonyBootstrapped` event"]
        pub fn colony_bootstrapped_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ColonyBootstrappedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ColonyFundsClaimed` event"]
        pub fn colony_funds_claimed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ColonyFundsClaimedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ColonyFundsMovedBetweenFundingPots` event"]
        pub fn colony_funds_moved_between_funding_pots_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ColonyFundsMovedBetweenFundingPotsFilter>
        {
            self.0.event()
        }
        #[doc = "Gets the contract's `ColonyInitialised` event"]
        pub fn colony_initialised_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ColonyInitialisedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ColonyMetadata` event"]
        pub fn colony_metadata_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ColonyMetadataFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ColonyMetadataDelta` event"]
        pub fn colony_metadata_delta_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ColonyMetadataDeltaFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ColonyRewardInverseSet` event"]
        pub fn colony_reward_inverse_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ColonyRewardInverseSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ColonyRoleSet` event"]
        pub fn colony_role_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ColonyRoleSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ColonyUpgraded` event"]
        pub fn colony_upgraded_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ColonyUpgradedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DomainAdded` event"]
        pub fn domain_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DomainAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DomainDeprecated` event"]
        pub fn domain_deprecated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DomainDeprecatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DomainMetadata` event"]
        pub fn domain_metadata_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DomainMetadataFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExpenditureAdded` event"]
        pub fn expenditure_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExpenditureAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExpenditureCancelled` event"]
        pub fn expenditure_cancelled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExpenditureCancelledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExpenditureClaimDelaySet` event"]
        pub fn expenditure_claim_delay_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExpenditureClaimDelaySetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExpenditureFinalized` event"]
        pub fn expenditure_finalized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExpenditureFinalizedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExpenditureGlobalClaimDelaySet` event"]
        pub fn expenditure_global_claim_delay_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExpenditureGlobalClaimDelaySetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExpenditureLocked` event"]
        pub fn expenditure_locked_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExpenditureLockedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExpenditureMetadataSet` event"]
        pub fn expenditure_metadata_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExpenditureMetadataSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExpenditurePayoutModifierSet` event"]
        pub fn expenditure_payout_modifier_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExpenditurePayoutModifierSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExpenditurePayoutSet` event"]
        pub fn expenditure_payout_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExpenditurePayoutSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExpenditureRecipientSet` event"]
        pub fn expenditure_recipient_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExpenditureRecipientSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExpenditureSkillSet` event"]
        pub fn expenditure_skill_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExpenditureSkillSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExpenditureStateChanged` event"]
        pub fn expenditure_state_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExpenditureStateChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExpenditureTransferred` event"]
        pub fn expenditure_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExpenditureTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FundingPotAdded` event"]
        pub fn funding_pot_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FundingPotAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LocalSkillAdded` event"]
        pub fn local_skill_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LocalSkillAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LocalSkillDeprecated` event"]
        pub fn local_skill_deprecated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LocalSkillDeprecatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MetaTransactionExecuted` event"]
        pub fn meta_transaction_executed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MetaTransactionExecutedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PaymentAdded` event"]
        pub fn payment_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PaymentAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PaymentFinalized` event"]
        pub fn payment_finalized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PaymentFinalizedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PaymentPayoutSet` event"]
        pub fn payment_payout_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PaymentPayoutSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PaymentRecipientSet` event"]
        pub fn payment_recipient_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PaymentRecipientSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PaymentSkillSet` event"]
        pub fn payment_skill_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PaymentSkillSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PayoutClaimed` event"]
        pub fn payout_claimed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PayoutClaimedFilter> {
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
        #[doc = "Gets the contract's `RewardPayoutClaimed` event"]
        pub fn reward_payout_claimed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RewardPayoutClaimedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RewardPayoutCycleEnded` event"]
        pub fn reward_payout_cycle_ended_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RewardPayoutCycleEndedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RewardPayoutCycleStarted` event"]
        pub fn reward_payout_cycle_started_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RewardPayoutCycleStartedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TaskAdded` event"]
        pub fn task_added_filter(&self) -> ethers::contract::builders::Event<M, TaskAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TaskBriefSet` event"]
        pub fn task_brief_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TaskBriefSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TaskCanceled` event"]
        pub fn task_canceled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TaskCanceledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TaskChangedViaSignatures` event"]
        pub fn task_changed_via_signatures_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TaskChangedViaSignaturesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TaskCompleted` event"]
        pub fn task_completed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TaskCompletedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TaskDeliverableSubmitted` event"]
        pub fn task_deliverable_submitted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TaskDeliverableSubmittedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TaskDueDateSet` event"]
        pub fn task_due_date_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TaskDueDateSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TaskFinalized` event"]
        pub fn task_finalized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TaskFinalizedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TaskPayoutSet` event"]
        pub fn task_payout_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TaskPayoutSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TaskRoleUserSet` event"]
        pub fn task_role_user_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TaskRoleUserSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TaskSkillSet` event"]
        pub fn task_skill_set_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TaskSkillSetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TaskWorkRatingRevealed` event"]
        pub fn task_work_rating_revealed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TaskWorkRatingRevealedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokenUnlocked` event"]
        pub fn token_unlocked_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokenUnlockedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokensBurned` event"]
        pub fn tokens_burned_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokensBurnedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TokensMinted` event"]
        pub fn tokens_minted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TokensMintedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ColonyEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Colony<M> {
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
    #[ethevent(name = "Annotation", abi = "Annotation(address,bytes32,string)")]
    pub struct AnnotationFilter {
        #[ethevent(indexed)]
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tx_hash: [u8; 32],
        pub metadata: String,
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
        name = "ArbitraryReputationUpdate",
        abi = "ArbitraryReputationUpdate(address,address,uint256,int256)"
    )]
    pub struct ArbitraryReputationUpdateFilter {
        pub agent: ethers::core::types::Address,
        pub user: ethers::core::types::Address,
        pub skill_id: ethers::core::types::U256,
        pub amount: I256,
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
        name = "ArbitraryTransaction",
        abi = "ArbitraryTransaction(address,bytes,bool)"
    )]
    pub struct ArbitraryTransactionFilter {
        pub target: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
        pub success: bool,
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
        name = "ColonyBootstrapped",
        abi = "ColonyBootstrapped(address,address[],int256[])"
    )]
    pub struct ColonyBootstrappedFilter {
        pub agent: ethers::core::types::Address,
        pub users: Vec<ethers::core::types::Address>,
        pub amounts: Vec<I256>,
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
        name = "ColonyFundsClaimed",
        abi = "ColonyFundsClaimed(address,address,uint256,uint256)"
    )]
    pub struct ColonyFundsClaimedFilter {
        pub agent: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub fee: ethers::core::types::U256,
        pub payout_remainder: ethers::core::types::U256,
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
        name = "ColonyFundsMovedBetweenFundingPots",
        abi = "ColonyFundsMovedBetweenFundingPots(address,uint256,uint256,uint256,address)"
    )]
    pub struct ColonyFundsMovedBetweenFundingPotsFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from_pot: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub to_pot: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
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
        name = "ColonyInitialised",
        abi = "ColonyInitialised(address,address,address)"
    )]
    pub struct ColonyInitialisedFilter {
        pub agent: ethers::core::types::Address,
        pub colony_network: ethers::core::types::Address,
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
    #[ethevent(name = "ColonyMetadata", abi = "ColonyMetadata(address,string)")]
    pub struct ColonyMetadataFilter {
        pub agent: ethers::core::types::Address,
        pub metadata: String,
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
        name = "ColonyMetadataDelta",
        abi = "ColonyMetadataDelta(address,string)"
    )]
    pub struct ColonyMetadataDeltaFilter {
        pub agent: ethers::core::types::Address,
        pub metadata: String,
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
        name = "ColonyRewardInverseSet",
        abi = "ColonyRewardInverseSet(address,uint256)"
    )]
    pub struct ColonyRewardInverseSetFilter {
        pub agent: ethers::core::types::Address,
        pub reward_inverse: ethers::core::types::U256,
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
        name = "ColonyRoleSet",
        abi = "ColonyRoleSet(address,address,uint256,uint8,bool)"
    )]
    pub struct ColonyRoleSetFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub domain_id: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub role: u8,
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
        name = "ColonyUpgraded",
        abi = "ColonyUpgraded(address,uint256,uint256)"
    )]
    pub struct ColonyUpgradedFilter {
        pub agent: ethers::core::types::Address,
        pub old_version: ethers::core::types::U256,
        pub new_version: ethers::core::types::U256,
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
    #[ethevent(name = "DomainAdded", abi = "DomainAdded(address,uint256)")]
    pub struct DomainAddedFilter {
        pub agent: ethers::core::types::Address,
        pub domain_id: ethers::core::types::U256,
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
        name = "DomainDeprecated",
        abi = "DomainDeprecated(address,uint256,bool)"
    )]
    pub struct DomainDeprecatedFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub domain_id: ethers::core::types::U256,
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
        name = "DomainMetadata",
        abi = "DomainMetadata(address,uint256,string)"
    )]
    pub struct DomainMetadataFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub domain_id: ethers::core::types::U256,
        pub metadata: String,
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
    #[ethevent(name = "ExpenditureAdded", abi = "ExpenditureAdded(address,uint256)")]
    pub struct ExpenditureAddedFilter {
        pub agent: ethers::core::types::Address,
        pub expenditure_id: ethers::core::types::U256,
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
        name = "ExpenditureCancelled",
        abi = "ExpenditureCancelled(address,uint256)"
    )]
    pub struct ExpenditureCancelledFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub expenditure_id: ethers::core::types::U256,
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
        name = "ExpenditureClaimDelaySet",
        abi = "ExpenditureClaimDelaySet(address,uint256,uint256,uint256)"
    )]
    pub struct ExpenditureClaimDelaySetFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub expenditure_id: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub slot: ethers::core::types::U256,
        pub claim_delay: ethers::core::types::U256,
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
        name = "ExpenditureFinalized",
        abi = "ExpenditureFinalized(address,uint256)"
    )]
    pub struct ExpenditureFinalizedFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub expenditure_id: ethers::core::types::U256,
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
        name = "ExpenditureGlobalClaimDelaySet",
        abi = "ExpenditureGlobalClaimDelaySet(address,uint256)"
    )]
    pub struct ExpenditureGlobalClaimDelaySetFilter {
        pub agent: ethers::core::types::Address,
        pub global_claim_delay: ethers::core::types::U256,
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
    #[ethevent(name = "ExpenditureLocked", abi = "ExpenditureLocked(address,uint256)")]
    pub struct ExpenditureLockedFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub expenditure_id: ethers::core::types::U256,
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
        name = "ExpenditureMetadataSet",
        abi = "ExpenditureMetadataSet(address,uint256,string)"
    )]
    pub struct ExpenditureMetadataSetFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub expenditure_id: ethers::core::types::U256,
        pub metadata: String,
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
        name = "ExpenditurePayoutModifierSet",
        abi = "ExpenditurePayoutModifierSet(address,uint256,uint256,int256)"
    )]
    pub struct ExpenditurePayoutModifierSetFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub expenditure_id: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub slot: ethers::core::types::U256,
        pub payout_modifier: I256,
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
        name = "ExpenditurePayoutSet",
        abi = "ExpenditurePayoutSet(address,uint256,uint256,address,uint256)"
    )]
    pub struct ExpenditurePayoutSetFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub expenditure_id: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub slot: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
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
        name = "ExpenditureRecipientSet",
        abi = "ExpenditureRecipientSet(address,uint256,uint256,address)"
    )]
    pub struct ExpenditureRecipientSetFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub expenditure_id: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub slot: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub recipient: ethers::core::types::Address,
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
        name = "ExpenditureSkillSet",
        abi = "ExpenditureSkillSet(address,uint256,uint256,uint256)"
    )]
    pub struct ExpenditureSkillSetFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub expenditure_id: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub slot: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub skill_id: ethers::core::types::U256,
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
        name = "ExpenditureStateChanged",
        abi = "ExpenditureStateChanged(address,uint256,uint256,bool[],bytes32[],bytes32)"
    )]
    pub struct ExpenditureStateChangedFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub expenditure_id: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub storage_slot: ethers::core::types::U256,
        pub mask: Vec<bool>,
        pub keys: Vec<[u8; 32]>,
        pub value: [u8; 32],
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
        name = "ExpenditureTransferred",
        abi = "ExpenditureTransferred(address,uint256,address)"
    )]
    pub struct ExpenditureTransferredFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub expenditure_id: ethers::core::types::U256,
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
    #[ethevent(name = "FundingPotAdded", abi = "FundingPotAdded(uint256)")]
    pub struct FundingPotAddedFilter {
        pub funding_pot_id: ethers::core::types::U256,
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
    #[ethevent(name = "LocalSkillAdded", abi = "LocalSkillAdded(address,uint256)")]
    pub struct LocalSkillAddedFilter {
        pub agent: ethers::core::types::Address,
        pub local_skill_id: ethers::core::types::U256,
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
        name = "LocalSkillDeprecated",
        abi = "LocalSkillDeprecated(address,uint256,bool)"
    )]
    pub struct LocalSkillDeprecatedFilter {
        pub agent: ethers::core::types::Address,
        pub local_skill_id: ethers::core::types::U256,
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
    #[ethevent(name = "PaymentAdded", abi = "PaymentAdded(address,uint256)")]
    pub struct PaymentAddedFilter {
        pub agent: ethers::core::types::Address,
        pub payment_id: ethers::core::types::U256,
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
    #[ethevent(name = "PaymentFinalized", abi = "PaymentFinalized(address,uint256)")]
    pub struct PaymentFinalizedFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub payment_id: ethers::core::types::U256,
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
        name = "PaymentPayoutSet",
        abi = "PaymentPayoutSet(address,uint256,address,uint256)"
    )]
    pub struct PaymentPayoutSetFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub payment_id: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
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
        name = "PaymentRecipientSet",
        abi = "PaymentRecipientSet(address,uint256,address)"
    )]
    pub struct PaymentRecipientSetFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub payment_id: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
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
        name = "PaymentSkillSet",
        abi = "PaymentSkillSet(address,uint256,uint256)"
    )]
    pub struct PaymentSkillSetFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub payment_id: ethers::core::types::U256,
        pub skill_id: ethers::core::types::U256,
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
        name = "PayoutClaimed",
        abi = "PayoutClaimed(address,uint256,address,uint256)"
    )]
    pub struct PayoutClaimedFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub funding_pot_id: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
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
        name = "RewardPayoutClaimed",
        abi = "RewardPayoutClaimed(uint256,address,uint256,uint256)"
    )]
    pub struct RewardPayoutClaimedFilter {
        pub reward_payout_id: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
        pub fee: ethers::core::types::U256,
        pub reward_remainder: ethers::core::types::U256,
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
        name = "RewardPayoutCycleEnded",
        abi = "RewardPayoutCycleEnded(address,uint256)"
    )]
    pub struct RewardPayoutCycleEndedFilter {
        pub agent: ethers::core::types::Address,
        pub reward_payout_id: ethers::core::types::U256,
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
        name = "RewardPayoutCycleStarted",
        abi = "RewardPayoutCycleStarted(address,uint256)"
    )]
    pub struct RewardPayoutCycleStartedFilter {
        pub agent: ethers::core::types::Address,
        pub reward_payout_id: ethers::core::types::U256,
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
    #[ethevent(name = "TaskAdded", abi = "TaskAdded(address,uint256)")]
    pub struct TaskAddedFilter {
        pub agent: ethers::core::types::Address,
        pub task_id: ethers::core::types::U256,
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
    #[ethevent(name = "TaskBriefSet", abi = "TaskBriefSet(uint256,bytes32)")]
    pub struct TaskBriefSetFilter {
        #[ethevent(indexed)]
        pub task_id: ethers::core::types::U256,
        pub specification_hash: [u8; 32],
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
    #[ethevent(name = "TaskCanceled", abi = "TaskCanceled(uint256)")]
    pub struct TaskCanceledFilter {
        #[ethevent(indexed)]
        pub task_id: ethers::core::types::U256,
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
        name = "TaskChangedViaSignatures",
        abi = "TaskChangedViaSignatures(address[])"
    )]
    pub struct TaskChangedViaSignaturesFilter {
        pub reviewer_addresses: Vec<ethers::core::types::Address>,
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
    #[ethevent(name = "TaskCompleted", abi = "TaskCompleted(address,uint256)")]
    pub struct TaskCompletedFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub task_id: ethers::core::types::U256,
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
        name = "TaskDeliverableSubmitted",
        abi = "TaskDeliverableSubmitted(address,uint256,bytes32)"
    )]
    pub struct TaskDeliverableSubmittedFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub task_id: ethers::core::types::U256,
        pub deliverable_hash: [u8; 32],
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
    #[ethevent(name = "TaskDueDateSet", abi = "TaskDueDateSet(uint256,uint256)")]
    pub struct TaskDueDateSetFilter {
        #[ethevent(indexed)]
        pub task_id: ethers::core::types::U256,
        pub due_date: ethers::core::types::U256,
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
    #[ethevent(name = "TaskFinalized", abi = "TaskFinalized(address,uint256)")]
    pub struct TaskFinalizedFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub task_id: ethers::core::types::U256,
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
        name = "TaskPayoutSet",
        abi = "TaskPayoutSet(uint256,uint8,address,uint256)"
    )]
    pub struct TaskPayoutSetFilter {
        #[ethevent(indexed)]
        pub task_id: ethers::core::types::U256,
        pub role: u8,
        pub token: ethers::core::types::Address,
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
        name = "TaskRoleUserSet",
        abi = "TaskRoleUserSet(uint256,uint8,address)"
    )]
    pub struct TaskRoleUserSetFilter {
        #[ethevent(indexed)]
        pub task_id: ethers::core::types::U256,
        pub role: u8,
        #[ethevent(indexed)]
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
    #[ethevent(name = "TaskSkillSet", abi = "TaskSkillSet(uint256,uint256)")]
    pub struct TaskSkillSetFilter {
        #[ethevent(indexed)]
        pub task_id: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub skill_id: ethers::core::types::U256,
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
        name = "TaskWorkRatingRevealed",
        abi = "TaskWorkRatingRevealed(address,uint256,uint8,uint8)"
    )]
    pub struct TaskWorkRatingRevealedFilter {
        pub agent: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub task_id: ethers::core::types::U256,
        pub role: u8,
        pub rating: u8,
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
    #[ethevent(name = "TokenUnlocked", abi = "TokenUnlocked(address)")]
    pub struct TokenUnlockedFilter {
        pub agent: ethers::core::types::Address,
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
    #[ethevent(name = "TokensBurned", abi = "TokensBurned(address,address,uint256)")]
    pub struct TokensBurnedFilter {
        pub agent: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
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
    #[ethevent(name = "TokensMinted", abi = "TokensMinted(address,address,uint256)")]
    pub struct TokensMintedFilter {
        pub agent: ethers::core::types::Address,
        pub who: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ColonyEvents {
        AnnotationFilter(AnnotationFilter),
        ArbitraryReputationUpdateFilter(ArbitraryReputationUpdateFilter),
        ArbitraryTransactionFilter(ArbitraryTransactionFilter),
        ColonyBootstrappedFilter(ColonyBootstrappedFilter),
        ColonyFundsClaimedFilter(ColonyFundsClaimedFilter),
        ColonyFundsMovedBetweenFundingPotsFilter(ColonyFundsMovedBetweenFundingPotsFilter),
        ColonyInitialisedFilter(ColonyInitialisedFilter),
        ColonyMetadataFilter(ColonyMetadataFilter),
        ColonyMetadataDeltaFilter(ColonyMetadataDeltaFilter),
        ColonyRewardInverseSetFilter(ColonyRewardInverseSetFilter),
        ColonyRoleSetFilter(ColonyRoleSetFilter),
        ColonyUpgradedFilter(ColonyUpgradedFilter),
        DomainAddedFilter(DomainAddedFilter),
        DomainDeprecatedFilter(DomainDeprecatedFilter),
        DomainMetadataFilter(DomainMetadataFilter),
        ExpenditureAddedFilter(ExpenditureAddedFilter),
        ExpenditureCancelledFilter(ExpenditureCancelledFilter),
        ExpenditureClaimDelaySetFilter(ExpenditureClaimDelaySetFilter),
        ExpenditureFinalizedFilter(ExpenditureFinalizedFilter),
        ExpenditureGlobalClaimDelaySetFilter(ExpenditureGlobalClaimDelaySetFilter),
        ExpenditureLockedFilter(ExpenditureLockedFilter),
        ExpenditureMetadataSetFilter(ExpenditureMetadataSetFilter),
        ExpenditurePayoutModifierSetFilter(ExpenditurePayoutModifierSetFilter),
        ExpenditurePayoutSetFilter(ExpenditurePayoutSetFilter),
        ExpenditureRecipientSetFilter(ExpenditureRecipientSetFilter),
        ExpenditureSkillSetFilter(ExpenditureSkillSetFilter),
        ExpenditureStateChangedFilter(ExpenditureStateChangedFilter),
        ExpenditureTransferredFilter(ExpenditureTransferredFilter),
        FundingPotAddedFilter(FundingPotAddedFilter),
        LocalSkillAddedFilter(LocalSkillAddedFilter),
        LocalSkillDeprecatedFilter(LocalSkillDeprecatedFilter),
        MetaTransactionExecutedFilter(MetaTransactionExecutedFilter),
        PaymentAddedFilter(PaymentAddedFilter),
        PaymentFinalizedFilter(PaymentFinalizedFilter),
        PaymentPayoutSetFilter(PaymentPayoutSetFilter),
        PaymentRecipientSetFilter(PaymentRecipientSetFilter),
        PaymentSkillSetFilter(PaymentSkillSetFilter),
        PayoutClaimedFilter(PayoutClaimedFilter),
        RecoveryModeEnteredFilter(RecoveryModeEnteredFilter),
        RecoveryModeExitApprovedFilter(RecoveryModeExitApprovedFilter),
        RecoveryModeExitedFilter(RecoveryModeExitedFilter),
        RecoveryRoleSetFilter(RecoveryRoleSetFilter),
        RecoveryStorageSlotSetFilter(RecoveryStorageSlotSetFilter),
        RewardPayoutClaimedFilter(RewardPayoutClaimedFilter),
        RewardPayoutCycleEndedFilter(RewardPayoutCycleEndedFilter),
        RewardPayoutCycleStartedFilter(RewardPayoutCycleStartedFilter),
        TaskAddedFilter(TaskAddedFilter),
        TaskBriefSetFilter(TaskBriefSetFilter),
        TaskCanceledFilter(TaskCanceledFilter),
        TaskChangedViaSignaturesFilter(TaskChangedViaSignaturesFilter),
        TaskCompletedFilter(TaskCompletedFilter),
        TaskDeliverableSubmittedFilter(TaskDeliverableSubmittedFilter),
        TaskDueDateSetFilter(TaskDueDateSetFilter),
        TaskFinalizedFilter(TaskFinalizedFilter),
        TaskPayoutSetFilter(TaskPayoutSetFilter),
        TaskRoleUserSetFilter(TaskRoleUserSetFilter),
        TaskSkillSetFilter(TaskSkillSetFilter),
        TaskWorkRatingRevealedFilter(TaskWorkRatingRevealedFilter),
        TokenUnlockedFilter(TokenUnlockedFilter),
        TokensBurnedFilter(TokensBurnedFilter),
        TokensMintedFilter(TokensMintedFilter),
    }
    impl ethers::contract::EthLogDecode for ColonyEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AnnotationFilter::decode_log(log) {
                return Ok(ColonyEvents::AnnotationFilter(decoded));
            }
            if let Ok(decoded) = ArbitraryReputationUpdateFilter::decode_log(log) {
                return Ok(ColonyEvents::ArbitraryReputationUpdateFilter(decoded));
            }
            if let Ok(decoded) = ArbitraryTransactionFilter::decode_log(log) {
                return Ok(ColonyEvents::ArbitraryTransactionFilter(decoded));
            }
            if let Ok(decoded) = ColonyBootstrappedFilter::decode_log(log) {
                return Ok(ColonyEvents::ColonyBootstrappedFilter(decoded));
            }
            if let Ok(decoded) = ColonyFundsClaimedFilter::decode_log(log) {
                return Ok(ColonyEvents::ColonyFundsClaimedFilter(decoded));
            }
            if let Ok(decoded) = ColonyFundsMovedBetweenFundingPotsFilter::decode_log(log) {
                return Ok(ColonyEvents::ColonyFundsMovedBetweenFundingPotsFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ColonyInitialisedFilter::decode_log(log) {
                return Ok(ColonyEvents::ColonyInitialisedFilter(decoded));
            }
            if let Ok(decoded) = ColonyMetadataFilter::decode_log(log) {
                return Ok(ColonyEvents::ColonyMetadataFilter(decoded));
            }
            if let Ok(decoded) = ColonyMetadataDeltaFilter::decode_log(log) {
                return Ok(ColonyEvents::ColonyMetadataDeltaFilter(decoded));
            }
            if let Ok(decoded) = ColonyRewardInverseSetFilter::decode_log(log) {
                return Ok(ColonyEvents::ColonyRewardInverseSetFilter(decoded));
            }
            if let Ok(decoded) = ColonyRoleSetFilter::decode_log(log) {
                return Ok(ColonyEvents::ColonyRoleSetFilter(decoded));
            }
            if let Ok(decoded) = ColonyUpgradedFilter::decode_log(log) {
                return Ok(ColonyEvents::ColonyUpgradedFilter(decoded));
            }
            if let Ok(decoded) = DomainAddedFilter::decode_log(log) {
                return Ok(ColonyEvents::DomainAddedFilter(decoded));
            }
            if let Ok(decoded) = DomainDeprecatedFilter::decode_log(log) {
                return Ok(ColonyEvents::DomainDeprecatedFilter(decoded));
            }
            if let Ok(decoded) = DomainMetadataFilter::decode_log(log) {
                return Ok(ColonyEvents::DomainMetadataFilter(decoded));
            }
            if let Ok(decoded) = ExpenditureAddedFilter::decode_log(log) {
                return Ok(ColonyEvents::ExpenditureAddedFilter(decoded));
            }
            if let Ok(decoded) = ExpenditureCancelledFilter::decode_log(log) {
                return Ok(ColonyEvents::ExpenditureCancelledFilter(decoded));
            }
            if let Ok(decoded) = ExpenditureClaimDelaySetFilter::decode_log(log) {
                return Ok(ColonyEvents::ExpenditureClaimDelaySetFilter(decoded));
            }
            if let Ok(decoded) = ExpenditureFinalizedFilter::decode_log(log) {
                return Ok(ColonyEvents::ExpenditureFinalizedFilter(decoded));
            }
            if let Ok(decoded) = ExpenditureGlobalClaimDelaySetFilter::decode_log(log) {
                return Ok(ColonyEvents::ExpenditureGlobalClaimDelaySetFilter(decoded));
            }
            if let Ok(decoded) = ExpenditureLockedFilter::decode_log(log) {
                return Ok(ColonyEvents::ExpenditureLockedFilter(decoded));
            }
            if let Ok(decoded) = ExpenditureMetadataSetFilter::decode_log(log) {
                return Ok(ColonyEvents::ExpenditureMetadataSetFilter(decoded));
            }
            if let Ok(decoded) = ExpenditurePayoutModifierSetFilter::decode_log(log) {
                return Ok(ColonyEvents::ExpenditurePayoutModifierSetFilter(decoded));
            }
            if let Ok(decoded) = ExpenditurePayoutSetFilter::decode_log(log) {
                return Ok(ColonyEvents::ExpenditurePayoutSetFilter(decoded));
            }
            if let Ok(decoded) = ExpenditureRecipientSetFilter::decode_log(log) {
                return Ok(ColonyEvents::ExpenditureRecipientSetFilter(decoded));
            }
            if let Ok(decoded) = ExpenditureSkillSetFilter::decode_log(log) {
                return Ok(ColonyEvents::ExpenditureSkillSetFilter(decoded));
            }
            if let Ok(decoded) = ExpenditureStateChangedFilter::decode_log(log) {
                return Ok(ColonyEvents::ExpenditureStateChangedFilter(decoded));
            }
            if let Ok(decoded) = ExpenditureTransferredFilter::decode_log(log) {
                return Ok(ColonyEvents::ExpenditureTransferredFilter(decoded));
            }
            if let Ok(decoded) = FundingPotAddedFilter::decode_log(log) {
                return Ok(ColonyEvents::FundingPotAddedFilter(decoded));
            }
            if let Ok(decoded) = LocalSkillAddedFilter::decode_log(log) {
                return Ok(ColonyEvents::LocalSkillAddedFilter(decoded));
            }
            if let Ok(decoded) = LocalSkillDeprecatedFilter::decode_log(log) {
                return Ok(ColonyEvents::LocalSkillDeprecatedFilter(decoded));
            }
            if let Ok(decoded) = MetaTransactionExecutedFilter::decode_log(log) {
                return Ok(ColonyEvents::MetaTransactionExecutedFilter(decoded));
            }
            if let Ok(decoded) = PaymentAddedFilter::decode_log(log) {
                return Ok(ColonyEvents::PaymentAddedFilter(decoded));
            }
            if let Ok(decoded) = PaymentFinalizedFilter::decode_log(log) {
                return Ok(ColonyEvents::PaymentFinalizedFilter(decoded));
            }
            if let Ok(decoded) = PaymentPayoutSetFilter::decode_log(log) {
                return Ok(ColonyEvents::PaymentPayoutSetFilter(decoded));
            }
            if let Ok(decoded) = PaymentRecipientSetFilter::decode_log(log) {
                return Ok(ColonyEvents::PaymentRecipientSetFilter(decoded));
            }
            if let Ok(decoded) = PaymentSkillSetFilter::decode_log(log) {
                return Ok(ColonyEvents::PaymentSkillSetFilter(decoded));
            }
            if let Ok(decoded) = PayoutClaimedFilter::decode_log(log) {
                return Ok(ColonyEvents::PayoutClaimedFilter(decoded));
            }
            if let Ok(decoded) = RecoveryModeEnteredFilter::decode_log(log) {
                return Ok(ColonyEvents::RecoveryModeEnteredFilter(decoded));
            }
            if let Ok(decoded) = RecoveryModeExitApprovedFilter::decode_log(log) {
                return Ok(ColonyEvents::RecoveryModeExitApprovedFilter(decoded));
            }
            if let Ok(decoded) = RecoveryModeExitedFilter::decode_log(log) {
                return Ok(ColonyEvents::RecoveryModeExitedFilter(decoded));
            }
            if let Ok(decoded) = RecoveryRoleSetFilter::decode_log(log) {
                return Ok(ColonyEvents::RecoveryRoleSetFilter(decoded));
            }
            if let Ok(decoded) = RecoveryStorageSlotSetFilter::decode_log(log) {
                return Ok(ColonyEvents::RecoveryStorageSlotSetFilter(decoded));
            }
            if let Ok(decoded) = RewardPayoutClaimedFilter::decode_log(log) {
                return Ok(ColonyEvents::RewardPayoutClaimedFilter(decoded));
            }
            if let Ok(decoded) = RewardPayoutCycleEndedFilter::decode_log(log) {
                return Ok(ColonyEvents::RewardPayoutCycleEndedFilter(decoded));
            }
            if let Ok(decoded) = RewardPayoutCycleStartedFilter::decode_log(log) {
                return Ok(ColonyEvents::RewardPayoutCycleStartedFilter(decoded));
            }
            if let Ok(decoded) = TaskAddedFilter::decode_log(log) {
                return Ok(ColonyEvents::TaskAddedFilter(decoded));
            }
            if let Ok(decoded) = TaskBriefSetFilter::decode_log(log) {
                return Ok(ColonyEvents::TaskBriefSetFilter(decoded));
            }
            if let Ok(decoded) = TaskCanceledFilter::decode_log(log) {
                return Ok(ColonyEvents::TaskCanceledFilter(decoded));
            }
            if let Ok(decoded) = TaskChangedViaSignaturesFilter::decode_log(log) {
                return Ok(ColonyEvents::TaskChangedViaSignaturesFilter(decoded));
            }
            if let Ok(decoded) = TaskCompletedFilter::decode_log(log) {
                return Ok(ColonyEvents::TaskCompletedFilter(decoded));
            }
            if let Ok(decoded) = TaskDeliverableSubmittedFilter::decode_log(log) {
                return Ok(ColonyEvents::TaskDeliverableSubmittedFilter(decoded));
            }
            if let Ok(decoded) = TaskDueDateSetFilter::decode_log(log) {
                return Ok(ColonyEvents::TaskDueDateSetFilter(decoded));
            }
            if let Ok(decoded) = TaskFinalizedFilter::decode_log(log) {
                return Ok(ColonyEvents::TaskFinalizedFilter(decoded));
            }
            if let Ok(decoded) = TaskPayoutSetFilter::decode_log(log) {
                return Ok(ColonyEvents::TaskPayoutSetFilter(decoded));
            }
            if let Ok(decoded) = TaskRoleUserSetFilter::decode_log(log) {
                return Ok(ColonyEvents::TaskRoleUserSetFilter(decoded));
            }
            if let Ok(decoded) = TaskSkillSetFilter::decode_log(log) {
                return Ok(ColonyEvents::TaskSkillSetFilter(decoded));
            }
            if let Ok(decoded) = TaskWorkRatingRevealedFilter::decode_log(log) {
                return Ok(ColonyEvents::TaskWorkRatingRevealedFilter(decoded));
            }
            if let Ok(decoded) = TokenUnlockedFilter::decode_log(log) {
                return Ok(ColonyEvents::TokenUnlockedFilter(decoded));
            }
            if let Ok(decoded) = TokensBurnedFilter::decode_log(log) {
                return Ok(ColonyEvents::TokensBurnedFilter(decoded));
            }
            if let Ok(decoded) = TokensMintedFilter::decode_log(log) {
                return Ok(ColonyEvents::TokensMintedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ColonyEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ColonyEvents::AnnotationFilter(element) => element.fmt(f),
                ColonyEvents::ArbitraryReputationUpdateFilter(element) => element.fmt(f),
                ColonyEvents::ArbitraryTransactionFilter(element) => element.fmt(f),
                ColonyEvents::ColonyBootstrappedFilter(element) => element.fmt(f),
                ColonyEvents::ColonyFundsClaimedFilter(element) => element.fmt(f),
                ColonyEvents::ColonyFundsMovedBetweenFundingPotsFilter(element) => element.fmt(f),
                ColonyEvents::ColonyInitialisedFilter(element) => element.fmt(f),
                ColonyEvents::ColonyMetadataFilter(element) => element.fmt(f),
                ColonyEvents::ColonyMetadataDeltaFilter(element) => element.fmt(f),
                ColonyEvents::ColonyRewardInverseSetFilter(element) => element.fmt(f),
                ColonyEvents::ColonyRoleSetFilter(element) => element.fmt(f),
                ColonyEvents::ColonyUpgradedFilter(element) => element.fmt(f),
                ColonyEvents::DomainAddedFilter(element) => element.fmt(f),
                ColonyEvents::DomainDeprecatedFilter(element) => element.fmt(f),
                ColonyEvents::DomainMetadataFilter(element) => element.fmt(f),
                ColonyEvents::ExpenditureAddedFilter(element) => element.fmt(f),
                ColonyEvents::ExpenditureCancelledFilter(element) => element.fmt(f),
                ColonyEvents::ExpenditureClaimDelaySetFilter(element) => element.fmt(f),
                ColonyEvents::ExpenditureFinalizedFilter(element) => element.fmt(f),
                ColonyEvents::ExpenditureGlobalClaimDelaySetFilter(element) => element.fmt(f),
                ColonyEvents::ExpenditureLockedFilter(element) => element.fmt(f),
                ColonyEvents::ExpenditureMetadataSetFilter(element) => element.fmt(f),
                ColonyEvents::ExpenditurePayoutModifierSetFilter(element) => element.fmt(f),
                ColonyEvents::ExpenditurePayoutSetFilter(element) => element.fmt(f),
                ColonyEvents::ExpenditureRecipientSetFilter(element) => element.fmt(f),
                ColonyEvents::ExpenditureSkillSetFilter(element) => element.fmt(f),
                ColonyEvents::ExpenditureStateChangedFilter(element) => element.fmt(f),
                ColonyEvents::ExpenditureTransferredFilter(element) => element.fmt(f),
                ColonyEvents::FundingPotAddedFilter(element) => element.fmt(f),
                ColonyEvents::LocalSkillAddedFilter(element) => element.fmt(f),
                ColonyEvents::LocalSkillDeprecatedFilter(element) => element.fmt(f),
                ColonyEvents::MetaTransactionExecutedFilter(element) => element.fmt(f),
                ColonyEvents::PaymentAddedFilter(element) => element.fmt(f),
                ColonyEvents::PaymentFinalizedFilter(element) => element.fmt(f),
                ColonyEvents::PaymentPayoutSetFilter(element) => element.fmt(f),
                ColonyEvents::PaymentRecipientSetFilter(element) => element.fmt(f),
                ColonyEvents::PaymentSkillSetFilter(element) => element.fmt(f),
                ColonyEvents::PayoutClaimedFilter(element) => element.fmt(f),
                ColonyEvents::RecoveryModeEnteredFilter(element) => element.fmt(f),
                ColonyEvents::RecoveryModeExitApprovedFilter(element) => element.fmt(f),
                ColonyEvents::RecoveryModeExitedFilter(element) => element.fmt(f),
                ColonyEvents::RecoveryRoleSetFilter(element) => element.fmt(f),
                ColonyEvents::RecoveryStorageSlotSetFilter(element) => element.fmt(f),
                ColonyEvents::RewardPayoutClaimedFilter(element) => element.fmt(f),
                ColonyEvents::RewardPayoutCycleEndedFilter(element) => element.fmt(f),
                ColonyEvents::RewardPayoutCycleStartedFilter(element) => element.fmt(f),
                ColonyEvents::TaskAddedFilter(element) => element.fmt(f),
                ColonyEvents::TaskBriefSetFilter(element) => element.fmt(f),
                ColonyEvents::TaskCanceledFilter(element) => element.fmt(f),
                ColonyEvents::TaskChangedViaSignaturesFilter(element) => element.fmt(f),
                ColonyEvents::TaskCompletedFilter(element) => element.fmt(f),
                ColonyEvents::TaskDeliverableSubmittedFilter(element) => element.fmt(f),
                ColonyEvents::TaskDueDateSetFilter(element) => element.fmt(f),
                ColonyEvents::TaskFinalizedFilter(element) => element.fmt(f),
                ColonyEvents::TaskPayoutSetFilter(element) => element.fmt(f),
                ColonyEvents::TaskRoleUserSetFilter(element) => element.fmt(f),
                ColonyEvents::TaskSkillSetFilter(element) => element.fmt(f),
                ColonyEvents::TaskWorkRatingRevealedFilter(element) => element.fmt(f),
                ColonyEvents::TokenUnlockedFilter(element) => element.fmt(f),
                ColonyEvents::TokensBurnedFilter(element) => element.fmt(f),
                ColonyEvents::TokensMintedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addDomain` function with signature `addDomain(uint256,uint256,uint256)` and selector `[31, 126, 62, 190]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "addDomain", abi = "addDomain(uint256,uint256,uint256)")]
    pub struct AddDomainCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub parent_domain_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `addDomain` function with signature `addDomain(uint256,uint256,uint256,string)` and selector `[223, 145, 246, 80]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "addDomain", abi = "addDomain(uint256,uint256,uint256,string)")]
    pub struct AddDomainWithPermissionDomainIdAndChildSkillIndexAndMetadataCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub parent_domain_id: ethers::core::types::U256,
        pub metadata: String,
    }
    #[doc = "Container type for all input parameters for the `addLocalSkill` function with signature `addLocalSkill()` and selector `[100, 5, 93, 4]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "addLocalSkill", abi = "addLocalSkill()")]
    pub struct AddLocalSkillCall;
    #[doc = "Container type for all input parameters for the `addPayment` function with signature `addPayment(uint256,uint256,address,address,uint256,uint256,uint256)` and selector `[79, 141, 246, 67]`"]
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
        name = "addPayment",
        abi = "addPayment(uint256,uint256,address,address,uint256,uint256,uint256)"
    )]
    pub struct AddPaymentCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub domain_id: ethers::core::types::U256,
        pub skill_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `annotateTransaction` function with signature `annotateTransaction(bytes32,string)` and selector `[108, 142, 200, 181]`"]
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
        name = "annotateTransaction",
        abi = "annotateTransaction(bytes32,string)"
    )]
    pub struct AnnotateTransactionCall {
        pub tx_hash: [u8; 32],
        pub metadata: String,
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
    #[doc = "Container type for all input parameters for the `approveStake` function with signature `approveStake(address,uint256,uint256)` and selector `[53, 79, 51, 216]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "approveStake", abi = "approveStake(address,uint256,uint256)")]
    pub struct ApproveStakeCall {
        pub approvee: ethers::core::types::Address,
        pub domain_id: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `bootstrapColony` function with signature `bootstrapColony(address[],int256[])` and selector `[231, 35, 214, 49]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "bootstrapColony", abi = "bootstrapColony(address[],int256[])")]
    pub struct BootstrapColonyCall {
        pub users: ::std::vec::Vec<ethers::core::types::Address>,
        pub amount: ::std::vec::Vec<I256>,
    }
    #[doc = "Container type for all input parameters for the `burnTokens` function with signature `burnTokens(address,uint256)` and selector `[13, 17, 24, 206]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "burnTokens", abi = "burnTokens(address,uint256)")]
    pub struct BurnTokensCall {
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `cancelExpenditure` function with signature `cancelExpenditure(uint256)` and selector `[61, 221, 66, 208]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "cancelExpenditure", abi = "cancelExpenditure(uint256)")]
    pub struct CancelExpenditureCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `cancelTask` function with signature `cancelTask(uint256)` and selector `[126, 236, 32, 168]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "cancelTask", abi = "cancelTask(uint256)")]
    pub struct CancelTaskCall {
        pub id: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `claimColonyFunds` function with signature `claimColonyFunds(address)` and selector `[137, 34, 74, 30]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "claimColonyFunds", abi = "claimColonyFunds(address)")]
    pub struct ClaimColonyFundsCall {
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `claimExpenditurePayout` function with signature `claimExpenditurePayout(uint256,uint256,address)` and selector `[47, 233, 104, 153]`"]
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
        name = "claimExpenditurePayout",
        abi = "claimExpenditurePayout(uint256,uint256,address)"
    )]
    pub struct ClaimExpenditurePayoutCall {
        pub id: ethers::core::types::U256,
        pub slot: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `claimPayment` function with signature `claimPayment(uint256,address)` and selector `[15, 76, 82, 243]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "claimPayment", abi = "claimPayment(uint256,address)")]
    pub struct ClaimPaymentCall {
        pub id: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `claimRewardPayout` function with signature `claimRewardPayout(uint256,uint256[7],bytes,bytes,uint256,bytes32[])` and selector `[111, 193, 74, 144]`"]
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
        name = "claimRewardPayout",
        abi = "claimRewardPayout(uint256,uint256[7],bytes,bytes,uint256,bytes32[])"
    )]
    pub struct ClaimRewardPayoutCall {
        pub payout_id: ethers::core::types::U256,
        pub square_roots: [ethers::core::types::U256; 7usize],
        pub key: ethers::core::types::Bytes,
        pub value: ethers::core::types::Bytes,
        pub branch_mask: ethers::core::types::U256,
        pub siblings: ::std::vec::Vec<[u8; 32]>,
    }
    #[doc = "Container type for all input parameters for the `claimTaskPayout` function with signature `claimTaskPayout(uint256,uint8,address)` and selector `[124, 80, 107, 11]`"]
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
        name = "claimTaskPayout",
        abi = "claimTaskPayout(uint256,uint8,address)"
    )]
    pub struct ClaimTaskPayoutCall {
        pub id: ethers::core::types::U256,
        pub role: u8,
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `completeTask` function with signature `completeTask(uint256)` and selector `[225, 226, 149, 88]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "completeTask", abi = "completeTask(uint256)")]
    pub struct CompleteTaskCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `deobligateStake` function with signature `deobligateStake(address,uint256,uint256)` and selector `[55, 166, 142, 103]`"]
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
        name = "deobligateStake",
        abi = "deobligateStake(address,uint256,uint256)"
    )]
    pub struct DeobligateStakeCall {
        pub user: ethers::core::types::Address,
        pub domain_id: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `deprecateDomain` function with signature `deprecateDomain(uint256,uint256,uint256,bool)` and selector `[231, 14, 128, 141]`"]
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
        name = "deprecateDomain",
        abi = "deprecateDomain(uint256,uint256,uint256,bool)"
    )]
    pub struct DeprecateDomainCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub domain_id: ethers::core::types::U256,
        pub deprecated: bool,
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
    #[doc = "Container type for all input parameters for the `deprecateLocalSkill` function with signature `deprecateLocalSkill(uint256,bool)` and selector `[163, 251, 115, 132]`"]
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
        name = "deprecateLocalSkill",
        abi = "deprecateLocalSkill(uint256,bool)"
    )]
    pub struct DeprecateLocalSkillCall {
        pub local_skill_id: ethers::core::types::U256,
        pub deprecated: bool,
    }
    #[doc = "Container type for all input parameters for the `editColony` function with signature `editColony(string)` and selector `[16, 43, 27, 53]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "editColony", abi = "editColony(string)")]
    pub struct EditColonyCall {
        pub metadata: String,
    }
    #[doc = "Container type for all input parameters for the `editColonyByDelta` function with signature `editColonyByDelta(string)` and selector `[167, 40, 40, 177]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "editColonyByDelta", abi = "editColonyByDelta(string)")]
    pub struct EditColonyByDeltaCall {
        pub metadata_delta: String,
    }
    #[doc = "Container type for all input parameters for the `editDomain` function with signature `editDomain(uint256,uint256,uint256,string)` and selector `[100, 6, 184, 104]`"]
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
        name = "editDomain",
        abi = "editDomain(uint256,uint256,uint256,string)"
    )]
    pub struct EditDomainCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub domain_id: ethers::core::types::U256,
        pub metadata: String,
    }
    #[doc = "Container type for all input parameters for the `emitDomainReputationPenalty` function with signature `emitDomainReputationPenalty(uint256,uint256,uint256,address,int256)` and selector `[252, 232, 145, 165]`"]
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
        name = "emitDomainReputationPenalty",
        abi = "emitDomainReputationPenalty(uint256,uint256,uint256,address,int256)"
    )]
    pub struct EmitDomainReputationPenaltyCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub domain_id: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
        pub amount: I256,
    }
    #[doc = "Container type for all input parameters for the `emitDomainReputationReward` function with signature `emitDomainReputationReward(uint256,address,int256)` and selector `[148, 76, 101, 15]`"]
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
        name = "emitDomainReputationReward",
        abi = "emitDomainReputationReward(uint256,address,int256)"
    )]
    pub struct EmitDomainReputationRewardCall {
        pub domain_id: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
        pub amount: I256,
    }
    #[doc = "Container type for all input parameters for the `emitSkillReputationPenalty` function with signature `emitSkillReputationPenalty(uint256,address,int256)` and selector `[93, 134, 6, 149]`"]
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
        name = "emitSkillReputationPenalty",
        abi = "emitSkillReputationPenalty(uint256,address,int256)"
    )]
    pub struct EmitSkillReputationPenaltyCall {
        pub skill_id: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
        pub amount: I256,
    }
    #[doc = "Container type for all input parameters for the `emitSkillReputationReward` function with signature `emitSkillReputationReward(uint256,address,int256)` and selector `[89, 109, 54, 142]`"]
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
        name = "emitSkillReputationReward",
        abi = "emitSkillReputationReward(uint256,address,int256)"
    )]
    pub struct EmitSkillReputationRewardCall {
        pub skill_id: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
        pub amount: I256,
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
    #[doc = "Container type for all input parameters for the `executeTaskChange` function with signature `executeTaskChange(uint8[],bytes32[],bytes32[],uint8[],uint256,bytes)` and selector `[116, 162, 32, 14]`"]
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
        name = "executeTaskChange",
        abi = "executeTaskChange(uint8[],bytes32[],bytes32[],uint8[],uint256,bytes)"
    )]
    pub struct ExecuteTaskChangeCall {
        pub sig_v: ::std::vec::Vec<u8>,
        pub sig_r: ::std::vec::Vec<[u8; 32]>,
        pub sig_s: ::std::vec::Vec<[u8; 32]>,
        pub mode: ::std::vec::Vec<u8>,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `executeTaskRoleAssignment` function with signature `executeTaskRoleAssignment(uint8[],bytes32[],bytes32[],uint8[],uint256,bytes)` and selector `[36, 170, 73, 72]`"]
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
        name = "executeTaskRoleAssignment",
        abi = "executeTaskRoleAssignment(uint8[],bytes32[],bytes32[],uint8[],uint256,bytes)"
    )]
    pub struct ExecuteTaskRoleAssignmentCall {
        pub sig_v: ::std::vec::Vec<u8>,
        pub sig_r: ::std::vec::Vec<[u8; 32]>,
        pub sig_s: ::std::vec::Vec<[u8; 32]>,
        pub mode: ::std::vec::Vec<u8>,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
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
    #[doc = "Container type for all input parameters for the `finalizeExpenditure` function with signature `finalizeExpenditure(uint256)` and selector `[179, 231, 94, 198]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "finalizeExpenditure", abi = "finalizeExpenditure(uint256)")]
    pub struct FinalizeExpenditureCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `finalizePayment` function with signature `finalizePayment(uint256,uint256,uint256)` and selector `[217, 97, 164, 32]`"]
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
        name = "finalizePayment",
        abi = "finalizePayment(uint256,uint256,uint256)"
    )]
    pub struct FinalizePaymentCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `finalizeRewardPayout` function with signature `finalizeRewardPayout(uint256)` and selector `[81, 195, 140, 217]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "finalizeRewardPayout", abi = "finalizeRewardPayout(uint256)")]
    pub struct FinalizeRewardPayoutCall {
        pub payout_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `finalizeTask` function with signature `finalizeTask(uint256)` and selector `[136, 72, 140, 51]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "finalizeTask", abi = "finalizeTask(uint256)")]
    pub struct FinalizeTaskCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `finishUpgrade` function with signature `finishUpgrade()` and selector `[52, 13, 242, 143]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "finishUpgrade", abi = "finishUpgrade()")]
    pub struct FinishUpgradeCall;
    #[doc = "Container type for all input parameters for the `generateSecret` function with signature `generateSecret(bytes32,uint256)` and selector `[26, 227, 170, 164]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "generateSecret", abi = "generateSecret(bytes32,uint256)")]
    pub struct GenerateSecretCall {
        pub salt: [u8; 32],
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getApproval` function with signature `getApproval(address,address,uint256)` and selector `[49, 45, 55, 29]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getApproval", abi = "getApproval(address,address,uint256)")]
    pub struct GetApprovalCall {
        pub user: ethers::core::types::Address,
        pub obligator: ethers::core::types::Address,
        pub domain_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getCapabilityRoles` function with signature `getCapabilityRoles(bytes4)` and selector `[45, 251, 176, 131]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getCapabilityRoles", abi = "getCapabilityRoles(bytes4)")]
    pub struct GetCapabilityRolesCall {
        pub sig: [u8; 4],
    }
    #[doc = "Container type for all input parameters for the `getColonyNetwork` function with signature `getColonyNetwork()` and selector `[78, 94, 131, 149]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getColonyNetwork", abi = "getColonyNetwork()")]
    pub struct GetColonyNetworkCall;
    #[doc = "Container type for all input parameters for the `getDomain` function with signature `getDomain(uint256)` and selector `[26, 122, 152, 226]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getDomain", abi = "getDomain(uint256)")]
    pub struct GetDomainCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getDomainCount` function with signature `getDomainCount()` and selector `[19, 233, 31, 8]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getDomainCount", abi = "getDomainCount()")]
    pub struct GetDomainCountCall;
    #[doc = "Container type for all input parameters for the `getDomainFromFundingPot` function with signature `getDomainFromFundingPot(uint256)` and selector `[182, 27, 83, 23]`"]
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
        name = "getDomainFromFundingPot",
        abi = "getDomainFromFundingPot(uint256)"
    )]
    pub struct GetDomainFromFundingPotCall {
        pub funding_pot_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getExpenditure` function with signature `getExpenditure(uint256)` and selector `[233, 106, 173, 134]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getExpenditure", abi = "getExpenditure(uint256)")]
    pub struct GetExpenditureCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getExpenditureCount` function with signature `getExpenditureCount()` and selector `[122, 78, 111, 241]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getExpenditureCount", abi = "getExpenditureCount()")]
    pub struct GetExpenditureCountCall;
    #[doc = "Container type for all input parameters for the `getExpenditureSlot` function with signature `getExpenditureSlot(uint256,uint256)` and selector `[153, 75, 34, 129]`"]
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
        name = "getExpenditureSlot",
        abi = "getExpenditureSlot(uint256,uint256)"
    )]
    pub struct GetExpenditureSlotCall {
        pub id: ethers::core::types::U256,
        pub slot: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getExpenditureSlotPayout` function with signature `getExpenditureSlotPayout(uint256,uint256,address)` and selector `[41, 171, 205, 206]`"]
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
        name = "getExpenditureSlotPayout",
        abi = "getExpenditureSlotPayout(uint256,uint256,address)"
    )]
    pub struct GetExpenditureSlotPayoutCall {
        pub id: ethers::core::types::U256,
        pub slot: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getFundingPot` function with signature `getFundingPot(uint256)` and selector `[124, 23, 53, 168]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getFundingPot", abi = "getFundingPot(uint256)")]
    pub struct GetFundingPotCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getFundingPotBalance` function with signature `getFundingPotBalance(uint256,address)` and selector `[170, 227, 189, 221]`"]
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
        name = "getFundingPotBalance",
        abi = "getFundingPotBalance(uint256,address)"
    )]
    pub struct GetFundingPotBalanceCall {
        pub pot_id: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getFundingPotCount` function with signature `getFundingPotCount()` and selector `[37, 247, 89, 227]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getFundingPotCount", abi = "getFundingPotCount()")]
    pub struct GetFundingPotCountCall;
    #[doc = "Container type for all input parameters for the `getFundingPotPayout` function with signature `getFundingPotPayout(uint256,address)` and selector `[197, 208, 136, 110]`"]
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
        name = "getFundingPotPayout",
        abi = "getFundingPotPayout(uint256,address)"
    )]
    pub struct GetFundingPotPayoutCall {
        pub pot_id: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
    }
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
    #[doc = "Container type for all input parameters for the `getNonRewardPotsTotal` function with signature `getNonRewardPotsTotal(address)` and selector `[52, 250, 122, 162]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getNonRewardPotsTotal", abi = "getNonRewardPotsTotal(address)")]
    pub struct GetNonRewardPotsTotalCall {
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getObligation` function with signature `getObligation(address,address,uint256)` and selector `[95, 43, 140, 178]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getObligation", abi = "getObligation(address,address,uint256)")]
    pub struct GetObligationCall {
        pub user: ethers::core::types::Address,
        pub obligator: ethers::core::types::Address,
        pub domain_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getPayment` function with signature `getPayment(uint256)` and selector `[50, 128, 168, 54]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPayment", abi = "getPayment(uint256)")]
    pub struct GetPaymentCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getPaymentCount` function with signature `getPaymentCount()` and selector `[175, 50, 160, 63]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPaymentCount", abi = "getPaymentCount()")]
    pub struct GetPaymentCountCall;
    #[doc = "Container type for all input parameters for the `getRewardInverse` function with signature `getRewardInverse()` and selector `[76, 177, 100, 199]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getRewardInverse", abi = "getRewardInverse()")]
    pub struct GetRewardInverseCall;
    #[doc = "Container type for all input parameters for the `getRewardPayoutInfo` function with signature `getRewardPayoutInfo(uint256)` and selector `[54, 68, 163, 253]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getRewardPayoutInfo", abi = "getRewardPayoutInfo(uint256)")]
    pub struct GetRewardPayoutInfoCall {
        pub payout_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getRootLocalSkill` function with signature `getRootLocalSkill()` and selector `[189, 166, 112, 35]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getRootLocalSkill", abi = "getRootLocalSkill()")]
    pub struct GetRootLocalSkillCall;
    #[doc = "Container type for all input parameters for the `getTask` function with signature `getTask(uint256)` and selector `[29, 101, 231, 126]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getTask", abi = "getTask(uint256)")]
    pub struct GetTaskCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getTaskChangeNonce` function with signature `getTaskChangeNonce(uint256)` and selector `[206, 124, 238, 73]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getTaskChangeNonce", abi = "getTaskChangeNonce(uint256)")]
    pub struct GetTaskChangeNonceCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getTaskCount` function with signature `getTaskCount()` and selector `[193, 122, 52, 14]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getTaskCount", abi = "getTaskCount()")]
    pub struct GetTaskCountCall;
    #[doc = "Container type for all input parameters for the `getTaskPayout` function with signature `getTaskPayout(uint256,uint8,address)` and selector `[5, 125, 218, 44]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getTaskPayout", abi = "getTaskPayout(uint256,uint8,address)")]
    pub struct GetTaskPayoutCall {
        pub id: ethers::core::types::U256,
        pub role: u8,
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getTaskRole` function with signature `getTaskRole(uint256,uint8)` and selector `[30, 137, 84, 222]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getTaskRole", abi = "getTaskRole(uint256,uint8)")]
    pub struct GetTaskRoleCall {
        pub id: ethers::core::types::U256,
        pub role: u8,
    }
    #[doc = "Container type for all input parameters for the `getTaskWorkRatingSecret` function with signature `getTaskWorkRatingSecret(uint256,uint8)` and selector `[173, 78, 144, 140]`"]
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
        name = "getTaskWorkRatingSecret",
        abi = "getTaskWorkRatingSecret(uint256,uint8)"
    )]
    pub struct GetTaskWorkRatingSecretCall {
        pub id: ethers::core::types::U256,
        pub role: u8,
    }
    #[doc = "Container type for all input parameters for the `getTaskWorkRatingSecretsInfo` function with signature `getTaskWorkRatingSecretsInfo(uint256)` and selector `[1, 248, 251, 21]`"]
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
        name = "getTaskWorkRatingSecretsInfo",
        abi = "getTaskWorkRatingSecretsInfo(uint256)"
    )]
    pub struct GetTaskWorkRatingSecretsInfoCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getToken` function with signature `getToken()` and selector `[33, 223, 13, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getToken", abi = "getToken()")]
    pub struct GetTokenCall;
    #[doc = "Container type for all input parameters for the `getTokenApproval` function with signature `getTokenApproval(address,address)` and selector `[101, 223, 237, 196]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getTokenApproval", abi = "getTokenApproval(address,address)")]
    pub struct GetTokenApprovalCall {
        pub token: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getTotalTokenApproval` function with signature `getTotalTokenApproval(address)` and selector `[254, 53, 153, 149]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getTotalTokenApproval", abi = "getTotalTokenApproval(address)")]
    pub struct GetTotalTokenApprovalCall {
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getUserRoles` function with signature `getUserRoles(address,uint256)` and selector `[89, 91, 3, 240]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getUserRoles", abi = "getUserRoles(address,uint256)")]
    pub struct GetUserRolesCall {
        pub user: ethers::core::types::Address,
        pub domain: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `hasInheritedUserRole` function with signature `hasInheritedUserRole(address,uint256,uint8,uint256,uint256)` and selector `[51, 84, 241, 56]`"]
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
        name = "hasInheritedUserRole",
        abi = "hasInheritedUserRole(address,uint256,uint8,uint256,uint256)"
    )]
    pub struct HasInheritedUserRoleCall {
        pub user: ethers::core::types::Address,
        pub domain_id: ethers::core::types::U256,
        pub role: u8,
        pub child_skill_index: ethers::core::types::U256,
        pub child_domain_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `hasUserRole` function with signature `hasUserRole(address,uint256,uint8)` and selector `[171, 47, 122, 226]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "hasUserRole", abi = "hasUserRole(address,uint256,uint8)")]
    pub struct HasUserRoleCall {
        pub user: ethers::core::types::Address,
        pub domain_id: ethers::core::types::U256,
        pub role: u8,
    }
    #[doc = "Container type for all input parameters for the `initialiseColony` function with signature `initialiseColony(address,address)` and selector `[121, 217, 91, 146]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initialiseColony", abi = "initialiseColony(address,address)")]
    pub struct InitialiseColonyCall {
        pub colony_network_address: ethers::core::types::Address,
        pub token: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `lockExpenditure` function with signature `lockExpenditure(uint256)` and selector `[120, 131, 183, 112]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lockExpenditure", abi = "lockExpenditure(uint256)")]
    pub struct LockExpenditureCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `lockToken` function with signature `lockToken()` and selector `[188, 167, 169, 226]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lockToken", abi = "lockToken()")]
    pub struct LockTokenCall;
    #[doc = "Container type for all input parameters for the `makeArbitraryTransaction` function with signature `makeArbitraryTransaction(address,bytes)` and selector `[103, 195, 81, 194]`"]
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
        name = "makeArbitraryTransaction",
        abi = "makeArbitraryTransaction(address,bytes)"
    )]
    pub struct MakeArbitraryTransactionCall {
        pub to: ethers::core::types::Address,
        pub action: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `makeArbitraryTransactions` function with signature `makeArbitraryTransactions(address[],bytes[],bool)` and selector `[198, 53, 110, 254]`"]
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
        name = "makeArbitraryTransactions",
        abi = "makeArbitraryTransactions(address[],bytes[],bool)"
    )]
    pub struct MakeArbitraryTransactionsCall {
        pub targets: ::std::vec::Vec<ethers::core::types::Address>,
        pub actions: ::std::vec::Vec<ethers::core::types::Bytes>,
        pub strict: bool,
    }
    #[doc = "Container type for all input parameters for the `makeExpenditure` function with signature `makeExpenditure(uint256,uint256,uint256)` and selector `[165, 72, 122, 55]`"]
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
        name = "makeExpenditure",
        abi = "makeExpenditure(uint256,uint256,uint256)"
    )]
    pub struct MakeExpenditureCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub domain_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `makeSingleArbitraryTransaction` function with signature `makeSingleArbitraryTransaction(address,bytes)` and selector `[102, 59, 72, 45]`"]
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
        name = "makeSingleArbitraryTransaction",
        abi = "makeSingleArbitraryTransaction(address,bytes)"
    )]
    pub struct MakeSingleArbitraryTransactionCall {
        pub target: ethers::core::types::Address,
        pub action: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `makeTask` function with signature `makeTask(uint256,uint256,bytes32,uint256,uint256,uint256)` and selector `[102, 60, 182, 51]`"]
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
        name = "makeTask",
        abi = "makeTask(uint256,uint256,bytes32,uint256,uint256,uint256)"
    )]
    pub struct MakeTaskCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub specification_hash: [u8; 32],
        pub domain_id: ethers::core::types::U256,
        pub skill_id: ethers::core::types::U256,
        pub due_date: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `mintTokens` function with signature `mintTokens(uint256)` and selector `[151, 48, 76, 237]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "mintTokens", abi = "mintTokens(uint256)")]
    pub struct MintTokensCall {
        pub wad: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `mintTokensFor` function with signature `mintTokensFor(address,uint256)` and selector `[143, 104, 17, 163]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "mintTokensFor", abi = "mintTokensFor(address,uint256)")]
    pub struct MintTokensForCall {
        pub guy: ethers::core::types::Address,
        pub wad: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `moveFundsBetweenPots` function with signature `moveFundsBetweenPots(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,address)` and selector `[62, 119, 197, 73]`"]
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
        name = "moveFundsBetweenPots",
        abi = "moveFundsBetweenPots(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,address)"
    )]
    pub struct MoveFundsBetweenPotsWithPermissionDomainIdAndChildSkillIndexAndDomainIdAndFromChildSkillIndexAndToChildSkillIndexAndFromPotAndToPotCall
    {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub domain_id: ethers::core::types::U256,
        pub from_child_skill_index: ethers::core::types::U256,
        pub to_child_skill_index: ethers::core::types::U256,
        pub from_pot: ethers::core::types::U256,
        pub to_pot: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `moveFundsBetweenPots` function with signature `moveFundsBetweenPots(uint256,uint256,uint256,uint256,uint256,uint256,address)` and selector `[198, 141, 25, 234]`"]
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
        name = "moveFundsBetweenPots",
        abi = "moveFundsBetweenPots(uint256,uint256,uint256,uint256,uint256,uint256,address)"
    )]
    pub struct MoveFundsBetweenPotsCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub from_child_skill_index: ethers::core::types::U256,
        pub to_child_skill_index: ethers::core::types::U256,
        pub from_pot: ethers::core::types::U256,
        pub to_pot: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `obligateStake` function with signature `obligateStake(address,uint256,uint256)` and selector `[9, 106, 187, 73]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "obligateStake", abi = "obligateStake(address,uint256,uint256)")]
    pub struct ObligateStakeCall {
        pub user: ethers::core::types::Address,
        pub domain_id: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `removeTaskEvaluatorRole` function with signature `removeTaskEvaluatorRole(uint256)` and selector `[161, 198, 14, 99]`"]
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
        name = "removeTaskEvaluatorRole",
        abi = "removeTaskEvaluatorRole(uint256)"
    )]
    pub struct RemoveTaskEvaluatorRoleCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `removeTaskWorkerRole` function with signature `removeTaskWorkerRole(uint256)` and selector `[70, 136, 158, 203]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "removeTaskWorkerRole", abi = "removeTaskWorkerRole(uint256)")]
    pub struct RemoveTaskWorkerRoleCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `revealTaskWorkRating` function with signature `revealTaskWorkRating(uint256,uint8,uint8,bytes32)` and selector `[147, 74, 29, 199]`"]
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
        name = "revealTaskWorkRating",
        abi = "revealTaskWorkRating(uint256,uint8,uint8,bytes32)"
    )]
    pub struct RevealTaskWorkRatingCall {
        pub id: ethers::core::types::U256,
        pub role: u8,
        pub rating: u8,
        pub salt: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `setAdministrationRole` function with signature `setAdministrationRole(uint256,uint256,address,uint256,bool)` and selector `[99, 51, 29, 227]`"]
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
        name = "setAdministrationRole",
        abi = "setAdministrationRole(uint256,uint256,address,uint256,bool)"
    )]
    pub struct SetAdministrationRoleCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
        pub domain_id: ethers::core::types::U256,
        pub set_to: bool,
    }
    #[doc = "Container type for all input parameters for the `setAllTaskPayouts` function with signature `setAllTaskPayouts(uint256,address,uint256,uint256,uint256)` and selector `[93, 190, 105, 180]`"]
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
        name = "setAllTaskPayouts",
        abi = "setAllTaskPayouts(uint256,address,uint256,uint256,uint256)"
    )]
    pub struct SetAllTaskPayoutsCall {
        pub id: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
        pub manager_amount: ethers::core::types::U256,
        pub evaluator_amount: ethers::core::types::U256,
        pub worker_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setArbitrationRole` function with signature `setArbitrationRole(uint256,uint256,address,uint256,bool)` and selector `[183, 233, 179, 242]`"]
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
        name = "setArbitrationRole",
        abi = "setArbitrationRole(uint256,uint256,address,uint256,bool)"
    )]
    pub struct SetArbitrationRoleCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
        pub domain_id: ethers::core::types::U256,
        pub set_to: bool,
    }
    #[doc = "Container type for all input parameters for the `setArchitectureRole` function with signature `setArchitectureRole(uint256,uint256,address,uint256,bool)` and selector `[233, 199, 94, 71]`"]
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
        name = "setArchitectureRole",
        abi = "setArchitectureRole(uint256,uint256,address,uint256,bool)"
    )]
    pub struct SetArchitectureRoleCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
        pub domain_id: ethers::core::types::U256,
        pub set_to: bool,
    }
    #[doc = "Container type for all input parameters for the `setDefaultGlobalClaimDelay` function with signature `setDefaultGlobalClaimDelay(uint256)` and selector `[140, 44, 4, 33]`"]
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
        name = "setDefaultGlobalClaimDelay",
        abi = "setDefaultGlobalClaimDelay(uint256)"
    )]
    pub struct SetDefaultGlobalClaimDelayCall {
        pub global_claim_delay: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setExpenditureClaimDelay` function with signature `setExpenditureClaimDelay(uint256,uint256,uint256)` and selector `[9, 137, 92, 33]`"]
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
        name = "setExpenditureClaimDelay",
        abi = "setExpenditureClaimDelay(uint256,uint256,uint256)"
    )]
    pub struct SetExpenditureClaimDelayCall {
        pub id: ethers::core::types::U256,
        pub slot: ethers::core::types::U256,
        pub claim_delay: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setExpenditureClaimDelays` function with signature `setExpenditureClaimDelays(uint256,uint256[],uint256[])` and selector `[91, 219, 54, 93]`"]
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
        name = "setExpenditureClaimDelays",
        abi = "setExpenditureClaimDelays(uint256,uint256[],uint256[])"
    )]
    pub struct SetExpenditureClaimDelaysCall {
        pub id: ethers::core::types::U256,
        pub slots: ::std::vec::Vec<ethers::core::types::U256>,
        pub claim_delays: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `setExpenditureMetadata` function with signature `setExpenditureMetadata(uint256,uint256,uint256,string)` and selector `[107, 155, 240, 39]`"]
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
        name = "setExpenditureMetadata",
        abi = "setExpenditureMetadata(uint256,uint256,uint256,string)"
    )]
    pub struct SetExpenditureMetadataWithPermissionDomainIdAndChildSkillIndexCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub id: ethers::core::types::U256,
        pub metadata: String,
    }
    #[doc = "Container type for all input parameters for the `setExpenditureMetadata` function with signature `setExpenditureMetadata(uint256,string)` and selector `[230, 143, 193, 125]`"]
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
        name = "setExpenditureMetadata",
        abi = "setExpenditureMetadata(uint256,string)"
    )]
    pub struct SetExpenditureMetadataCall {
        pub id: ethers::core::types::U256,
        pub metadata: String,
    }
    #[doc = "Container type for all input parameters for the `setExpenditurePayout` function with signature `setExpenditurePayout(uint256,uint256,address,uint256)` and selector `[71, 109, 29, 5]`"]
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
        name = "setExpenditurePayout",
        abi = "setExpenditurePayout(uint256,uint256,address,uint256)"
    )]
    pub struct SetExpenditurePayoutCall {
        pub id: ethers::core::types::U256,
        pub slot: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setExpenditurePayout` function with signature `setExpenditurePayout(uint256,uint256,uint256,uint256,address,uint256)` and selector `[186, 232, 46, 201]`"]
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
        name = "setExpenditurePayout",
        abi = "setExpenditurePayout(uint256,uint256,uint256,uint256,address,uint256)"
    )]
    pub struct SetExpenditurePayoutWithPermissionDomainIdAndChildSkillIndexAndIdAndSlotCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub id: ethers::core::types::U256,
        pub slot: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setExpenditurePayoutModifiers` function with signature `setExpenditurePayoutModifiers(uint256,uint256[],int256[])` and selector `[137, 18, 107, 183]`"]
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
        name = "setExpenditurePayoutModifiers",
        abi = "setExpenditurePayoutModifiers(uint256,uint256[],int256[])"
    )]
    pub struct SetExpenditurePayoutModifiersCall {
        pub id: ethers::core::types::U256,
        pub slots: ::std::vec::Vec<ethers::core::types::U256>,
        pub payout_modifiers: ::std::vec::Vec<I256>,
    }
    #[doc = "Container type for all input parameters for the `setExpenditurePayouts` function with signature `setExpenditurePayouts(uint256,uint256[],address,uint256[])` and selector `[252, 155, 219, 254]`"]
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
        name = "setExpenditurePayouts",
        abi = "setExpenditurePayouts(uint256,uint256[],address,uint256[])"
    )]
    pub struct SetExpenditurePayoutsCall {
        pub id: ethers::core::types::U256,
        pub slots: ::std::vec::Vec<ethers::core::types::U256>,
        pub token: ethers::core::types::Address,
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `setExpenditureRecipient` function with signature `setExpenditureRecipient(uint256,uint256,address)` and selector `[23, 180, 108, 183]`"]
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
        name = "setExpenditureRecipient",
        abi = "setExpenditureRecipient(uint256,uint256,address)"
    )]
    pub struct SetExpenditureRecipientCall {
        pub id: ethers::core::types::U256,
        pub slot: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setExpenditureRecipients` function with signature `setExpenditureRecipients(uint256,uint256[],address[])` and selector `[206, 186, 34, 254]`"]
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
        name = "setExpenditureRecipients",
        abi = "setExpenditureRecipients(uint256,uint256[],address[])"
    )]
    pub struct SetExpenditureRecipientsCall {
        pub id: ethers::core::types::U256,
        pub slots: ::std::vec::Vec<ethers::core::types::U256>,
        pub recipients: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `setExpenditureSkill` function with signature `setExpenditureSkill(uint256,uint256,uint256)` and selector `[250, 19, 237, 114]`"]
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
        name = "setExpenditureSkill",
        abi = "setExpenditureSkill(uint256,uint256,uint256)"
    )]
    pub struct SetExpenditureSkillCall {
        pub id: ethers::core::types::U256,
        pub slot: ethers::core::types::U256,
        pub skill_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setExpenditureSkills` function with signature `setExpenditureSkills(uint256,uint256[],uint256[])` and selector `[181, 129, 121, 249]`"]
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
        name = "setExpenditureSkills",
        abi = "setExpenditureSkills(uint256,uint256[],uint256[])"
    )]
    pub struct SetExpenditureSkillsCall {
        pub id: ethers::core::types::U256,
        pub slots: ::std::vec::Vec<ethers::core::types::U256>,
        pub skill_ids: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `setExpenditureState` function with signature `setExpenditureState(uint256,uint256,uint256,uint256,bool[],bytes32[],bytes32)` and selector `[201, 162, 206, 124]`"]
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
        name = "setExpenditureState",
        abi = "setExpenditureState(uint256,uint256,uint256,uint256,bool[],bytes32[],bytes32)"
    )]
    pub struct SetExpenditureStateCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub id: ethers::core::types::U256,
        pub storage_slot: ethers::core::types::U256,
        pub mask: ::std::vec::Vec<bool>,
        pub keys: ::std::vec::Vec<[u8; 32]>,
        pub value: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `setExpenditureValues` function with signature `setExpenditureValues(uint256,uint256[],address[],uint256[],uint256[],uint256[],uint256[],uint256[],int256[],address[],uint256[][],uint256[][])` and selector `[28, 159, 108, 109]`"]
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
        name = "setExpenditureValues",
        abi = "setExpenditureValues(uint256,uint256[],address[],uint256[],uint256[],uint256[],uint256[],uint256[],int256[],address[],uint256[][],uint256[][])"
    )]
    pub struct SetExpenditureValuesCall {
        pub id: ethers::core::types::U256,
        pub recipient_slots: ::std::vec::Vec<ethers::core::types::U256>,
        pub recipients: ::std::vec::Vec<ethers::core::types::Address>,
        pub skill_id_slots: ::std::vec::Vec<ethers::core::types::U256>,
        pub skill_ids: ::std::vec::Vec<ethers::core::types::U256>,
        pub claim_delay_slots: ::std::vec::Vec<ethers::core::types::U256>,
        pub claim_delays: ::std::vec::Vec<ethers::core::types::U256>,
        pub payout_modifier_slots: ::std::vec::Vec<ethers::core::types::U256>,
        pub payout_modifiers: ::std::vec::Vec<I256>,
        pub payout_tokens: ::std::vec::Vec<ethers::core::types::Address>,
        pub payout_slots: ::std::vec::Vec<::std::vec::Vec<ethers::core::types::U256>>,
        pub payout_values: ::std::vec::Vec<::std::vec::Vec<ethers::core::types::U256>>,
    }
    #[doc = "Container type for all input parameters for the `setFundingRole` function with signature `setFundingRole(uint256,uint256,address,uint256,bool)` and selector `[149, 82, 161, 229]`"]
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
        name = "setFundingRole",
        abi = "setFundingRole(uint256,uint256,address,uint256,bool)"
    )]
    pub struct SetFundingRoleCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
        pub domain_id: ethers::core::types::U256,
        pub set_to: bool,
    }
    #[doc = "Container type for all input parameters for the `setPaymentPayout` function with signature `setPaymentPayout(uint256,uint256,uint256,address,uint256)` and selector `[84, 169, 160, 130]`"]
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
        name = "setPaymentPayout",
        abi = "setPaymentPayout(uint256,uint256,uint256,address,uint256)"
    )]
    pub struct SetPaymentPayoutCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub id: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setPaymentRecipient` function with signature `setPaymentRecipient(uint256,uint256,uint256,address)` and selector `[137, 4, 186, 144]`"]
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
        name = "setPaymentRecipient",
        abi = "setPaymentRecipient(uint256,uint256,uint256,address)"
    )]
    pub struct SetPaymentRecipientCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub id: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setPaymentSkill` function with signature `setPaymentSkill(uint256,uint256,uint256,uint256)` and selector `[37, 95, 195, 156]`"]
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
        name = "setPaymentSkill",
        abi = "setPaymentSkill(uint256,uint256,uint256,uint256)"
    )]
    pub struct SetPaymentSkillCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub id: ethers::core::types::U256,
        pub skill_id: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `setRewardInverse` function with signature `setRewardInverse(uint256)` and selector `[43, 148, 156, 153]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setRewardInverse", abi = "setRewardInverse(uint256)")]
    pub struct SetRewardInverseCall {
        pub reward_inverse: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setRootRole` function with signature `setRootRole(address,bool)` and selector `[225, 120, 243, 26]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setRootRole", abi = "setRootRole(address,bool)")]
    pub struct SetRootRoleCall {
        pub user: ethers::core::types::Address,
        pub set_to: bool,
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
    #[doc = "Container type for all input parameters for the `setTaskBrief` function with signature `setTaskBrief(uint256,bytes32)` and selector `[218, 77, 178, 73]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setTaskBrief", abi = "setTaskBrief(uint256,bytes32)")]
    pub struct SetTaskBriefCall {
        pub id: ethers::core::types::U256,
        pub specification_hash: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `setTaskDueDate` function with signature `setTaskDueDate(uint256,uint256)` and selector `[202, 233, 96, 254]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setTaskDueDate", abi = "setTaskDueDate(uint256,uint256)")]
    pub struct SetTaskDueDateCall {
        pub id: ethers::core::types::U256,
        pub due_date: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setTaskEvaluatorPayout` function with signature `setTaskEvaluatorPayout(uint256,address,uint256)` and selector `[111, 176, 121, 79]`"]
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
        name = "setTaskEvaluatorPayout",
        abi = "setTaskEvaluatorPayout(uint256,address,uint256)"
    )]
    pub struct SetTaskEvaluatorPayoutCall {
        pub id: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setTaskEvaluatorRole` function with signature `setTaskEvaluatorRole(uint256,address)` and selector `[225, 4, 105, 90]`"]
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
        name = "setTaskEvaluatorRole",
        abi = "setTaskEvaluatorRole(uint256,address)"
    )]
    pub struct SetTaskEvaluatorRoleCall {
        pub id: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setTaskManagerPayout` function with signature `setTaskManagerPayout(uint256,address,uint256)` and selector `[106, 175, 74, 119]`"]
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
        name = "setTaskManagerPayout",
        abi = "setTaskManagerPayout(uint256,address,uint256)"
    )]
    pub struct SetTaskManagerPayoutCall {
        pub id: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setTaskManagerRole` function with signature `setTaskManagerRole(uint256,address,uint256,uint256)` and selector `[179, 83, 131, 227]`"]
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
        name = "setTaskManagerRole",
        abi = "setTaskManagerRole(uint256,address,uint256,uint256)"
    )]
    pub struct SetTaskManagerRoleCall {
        pub id: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setTaskSkill` function with signature `setTaskSkill(uint256,uint256)` and selector `[184, 152, 76, 90]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setTaskSkill", abi = "setTaskSkill(uint256,uint256)")]
    pub struct SetTaskSkillCall {
        pub id: ethers::core::types::U256,
        pub skill_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setTaskWorkerPayout` function with signature `setTaskWorkerPayout(uint256,address,uint256)` and selector `[44, 246, 43, 57]`"]
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
        name = "setTaskWorkerPayout",
        abi = "setTaskWorkerPayout(uint256,address,uint256)"
    )]
    pub struct SetTaskWorkerPayoutCall {
        pub id: ethers::core::types::U256,
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setTaskWorkerRole` function with signature `setTaskWorkerRole(uint256,address)` and selector `[116, 119, 52, 57]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setTaskWorkerRole", abi = "setTaskWorkerRole(uint256,address)")]
    pub struct SetTaskWorkerRoleCall {
        pub id: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setUserRoles` function with signature `setUserRoles(uint256,uint256,address,uint256,bytes32)` and selector `[205, 74, 114, 203]`"]
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
        name = "setUserRoles",
        abi = "setUserRoles(uint256,uint256,address,uint256,bytes32)"
    )]
    pub struct SetUserRolesCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub user: ethers::core::types::Address,
        pub domain_id: ethers::core::types::U256,
        pub roles: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `startNextRewardPayout` function with signature `startNextRewardPayout(address,bytes,bytes,uint256,bytes32[])` and selector `[230, 245, 25, 206]`"]
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
        name = "startNextRewardPayout",
        abi = "startNextRewardPayout(address,bytes,bytes,uint256,bytes32[])"
    )]
    pub struct StartNextRewardPayoutCall {
        pub token: ethers::core::types::Address,
        pub key: ethers::core::types::Bytes,
        pub value: ethers::core::types::Bytes,
        pub branch_mask: ethers::core::types::U256,
        pub siblings: ::std::vec::Vec<[u8; 32]>,
    }
    #[doc = "Container type for all input parameters for the `submitTaskDeliverable` function with signature `submitTaskDeliverable(uint256,bytes32)` and selector `[80, 3, 9, 100]`"]
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
        name = "submitTaskDeliverable",
        abi = "submitTaskDeliverable(uint256,bytes32)"
    )]
    pub struct SubmitTaskDeliverableCall {
        pub id: ethers::core::types::U256,
        pub deliverable_hash: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `submitTaskDeliverableAndRating` function with signature `submitTaskDeliverableAndRating(uint256,bytes32,bytes32)` and selector `[25, 79, 82, 61]`"]
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
        name = "submitTaskDeliverableAndRating",
        abi = "submitTaskDeliverableAndRating(uint256,bytes32,bytes32)"
    )]
    pub struct SubmitTaskDeliverableAndRatingCall {
        pub id: ethers::core::types::U256,
        pub deliverable_hash: [u8; 32],
        pub rating_secret: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `submitTaskWorkRating` function with signature `submitTaskWorkRating(uint256,uint8,bytes32)` and selector `[63, 235, 54, 156]`"]
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
        name = "submitTaskWorkRating",
        abi = "submitTaskWorkRating(uint256,uint8,bytes32)"
    )]
    pub struct SubmitTaskWorkRatingCall {
        pub id: ethers::core::types::U256,
        pub role: u8,
        pub rating_secret: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `transferExpenditure` function with signature `transferExpenditure(uint256,address)` and selector `[254, 64, 251, 229]`"]
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
        name = "transferExpenditure",
        abi = "transferExpenditure(uint256,address)"
    )]
    pub struct TransferExpenditureCall {
        pub id: ethers::core::types::U256,
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `transferExpenditureViaArbitration` function with signature `transferExpenditureViaArbitration(uint256,uint256,uint256,address)` and selector `[44, 23, 99, 96]`"]
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
        name = "transferExpenditureViaArbitration",
        abi = "transferExpenditureViaArbitration(uint256,uint256,uint256,address)"
    )]
    pub struct TransferExpenditureViaArbitrationCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub id: ethers::core::types::U256,
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `transferStake` function with signature `transferStake(uint256,uint256,address,address,uint256,uint256,address)` and selector `[68, 108, 26, 162]`"]
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
        name = "transferStake",
        abi = "transferStake(uint256,uint256,address,address,uint256,uint256,address)"
    )]
    pub struct TransferStakeCall {
        pub permission_domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub obligator: ethers::core::types::Address,
        pub user: ethers::core::types::Address,
        pub domain_id: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `unlockToken` function with signature `unlockToken()` and selector `[24, 162, 75, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "unlockToken", abi = "unlockToken()")]
    pub struct UnlockTokenCall;
    #[doc = "Container type for all input parameters for the `unlockTokenForUser` function with signature `unlockTokenForUser(address,uint256)` and selector `[115, 192, 141, 222]`"]
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
        name = "unlockTokenForUser",
        abi = "unlockTokenForUser(address,uint256)"
    )]
    pub struct UnlockTokenForUserCall {
        pub user: ethers::core::types::Address,
        pub lock_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `updateApprovalAmount` function with signature `updateApprovalAmount(address,address)` and selector `[129, 28, 79, 155]`"]
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
        name = "updateApprovalAmount",
        abi = "updateApprovalAmount(address,address)"
    )]
    pub struct UpdateApprovalAmountCall {
        pub token: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `upgrade` function with signature `upgrade(uint256)` and selector `[69, 151, 125, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "upgrade", abi = "upgrade(uint256)")]
    pub struct UpgradeCall {
        pub new_version: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `userCanSetRoles` function with signature `userCanSetRoles(address,uint256,uint256,uint256)` and selector `[244, 168, 216, 81]`"]
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
        name = "userCanSetRoles",
        abi = "userCanSetRoles(address,uint256,uint256,uint256)"
    )]
    pub struct UserCanSetRolesCall {
        pub user: ethers::core::types::Address,
        pub domain_id: ethers::core::types::U256,
        pub child_skill_index: ethers::core::types::U256,
        pub child_domain_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `verifyReputationProof` function with signature `verifyReputationProof(bytes,bytes,uint256,bytes32[])` and selector `[150, 217, 51, 224]`"]
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
        name = "verifyReputationProof",
        abi = "verifyReputationProof(bytes,bytes,uint256,bytes32[])"
    )]
    pub struct VerifyReputationProofCall {
        pub key: ethers::core::types::Bytes,
        pub value: ethers::core::types::Bytes,
        pub branch_mask: ethers::core::types::U256,
        pub siblings: ::std::vec::Vec<[u8; 32]>,
    }
    #[doc = "Container type for all input parameters for the `version` function with signature `version()` and selector `[84, 253, 77, 80]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ColonyCalls {
        AddDomain (AddDomainCall) , AddDomainWithPermissionDomainIdAndChildSkillIndexAndMetadata (AddDomainWithPermissionDomainIdAndChildSkillIndexAndMetadataCall) , AddLocalSkill (AddLocalSkillCall) , AddPayment (AddPaymentCall) , AnnotateTransaction (AnnotateTransactionCall) , ApproveExitRecovery (ApproveExitRecoveryCall) , ApproveStake (ApproveStakeCall) , Authority (AuthorityCall) , BootstrapColony (BootstrapColonyCall) , BurnTokens (BurnTokensCall) , CancelExpenditure (CancelExpenditureCall) , CancelTask (CancelTaskCall) , CheckNotAdditionalProtectedVariable (CheckNotAdditionalProtectedVariableCall) , ClaimColonyFunds (ClaimColonyFundsCall) , ClaimExpenditurePayout (ClaimExpenditurePayoutCall) , ClaimPayment (ClaimPaymentCall) , ClaimRewardPayout (ClaimRewardPayoutCall) , ClaimTaskPayout (ClaimTaskPayoutCall) , CompleteTask (CompleteTaskCall) , DeobligateStake (DeobligateStakeCall) , DeprecateDomain (DeprecateDomainCall) , DeprecateExtension (DeprecateExtensionCall) , DeprecateLocalSkill (DeprecateLocalSkillCall) , EditColony (EditColonyCall) , EditColonyByDelta (EditColonyByDeltaCall) , EditDomain (EditDomainCall) , EmitDomainReputationPenalty (EmitDomainReputationPenaltyCall) , EmitDomainReputationReward (EmitDomainReputationRewardCall) , EmitSkillReputationPenalty (EmitSkillReputationPenaltyCall) , EmitSkillReputationReward (EmitSkillReputationRewardCall) , EnterRecoveryMode (EnterRecoveryModeCall) , ExecuteMetaTransaction (ExecuteMetaTransactionCall) , ExecuteTaskChange (ExecuteTaskChangeCall) , ExecuteTaskRoleAssignment (ExecuteTaskRoleAssignmentCall) , ExitRecoveryMode (ExitRecoveryModeCall) , FinalizeExpenditure (FinalizeExpenditureCall) , FinalizePayment (FinalizePaymentCall) , FinalizeRewardPayout (FinalizeRewardPayoutCall) , FinalizeTask (FinalizeTaskCall) , FinishUpgrade (FinishUpgradeCall) , GenerateSecret (GenerateSecretCall) , GetApproval (GetApprovalCall) , GetCapabilityRoles (GetCapabilityRolesCall) , GetColonyNetwork (GetColonyNetworkCall) , GetDomain (GetDomainCall) , GetDomainCount (GetDomainCountCall) , GetDomainFromFundingPot (GetDomainFromFundingPotCall) , GetExpenditure (GetExpenditureCall) , GetExpenditureCount (GetExpenditureCountCall) , GetExpenditureSlot (GetExpenditureSlotCall) , GetExpenditureSlotPayout (GetExpenditureSlotPayoutCall) , GetFundingPot (GetFundingPotCall) , GetFundingPotBalance (GetFundingPotBalanceCall) , GetFundingPotCount (GetFundingPotCountCall) , GetFundingPotPayout (GetFundingPotPayoutCall) , GetMetatransactionNonce (GetMetatransactionNonceCall) , GetNonRewardPotsTotal (GetNonRewardPotsTotalCall) , GetObligation (GetObligationCall) , GetPayment (GetPaymentCall) , GetPaymentCount (GetPaymentCountCall) , GetRewardInverse (GetRewardInverseCall) , GetRewardPayoutInfo (GetRewardPayoutInfoCall) , GetRootLocalSkill (GetRootLocalSkillCall) , GetTask (GetTaskCall) , GetTaskChangeNonce (GetTaskChangeNonceCall) , GetTaskCount (GetTaskCountCall) , GetTaskPayout (GetTaskPayoutCall) , GetTaskRole (GetTaskRoleCall) , GetTaskWorkRatingSecret (GetTaskWorkRatingSecretCall) , GetTaskWorkRatingSecretsInfo (GetTaskWorkRatingSecretsInfoCall) , GetToken (GetTokenCall) , GetTokenApproval (GetTokenApprovalCall) , GetTotalTokenApproval (GetTotalTokenApprovalCall) , GetUserRoles (GetUserRolesCall) , HasInheritedUserRole (HasInheritedUserRoleCall) , HasUserRole (HasUserRoleCall) , InitialiseColony (InitialiseColonyCall) , InitialiseRootLocalSkill (InitialiseRootLocalSkillCall) , InstallExtension (InstallExtensionCall) , IsInRecoveryMode (IsInRecoveryModeCall) , LockExpenditure (LockExpenditureCall) , LockToken (LockTokenCall) , MakeArbitraryTransaction (MakeArbitraryTransactionCall) , MakeArbitraryTransactions (MakeArbitraryTransactionsCall) , MakeExpenditure (MakeExpenditureCall) , MakeSingleArbitraryTransaction (MakeSingleArbitraryTransactionCall) , MakeTask (MakeTaskCall) , MintTokens (MintTokensCall) , MintTokensFor (MintTokensForCall) , MoveFundsBetweenPotsWithPermissionDomainIdAndChildSkillIndexAndDomainIdAndFromChildSkillIndexAndToChildSkillIndexAndFromPotAndToPot (MoveFundsBetweenPotsWithPermissionDomainIdAndChildSkillIndexAndDomainIdAndFromChildSkillIndexAndToChildSkillIndexAndFromPotAndToPotCall) , MoveFundsBetweenPots (MoveFundsBetweenPotsCall) , Multicall (MulticallCall) , NumRecoveryRoles (NumRecoveryRolesCall) , ObligateStake (ObligateStakeCall) , Owner (OwnerCall) , RegisterColonyLabel (RegisterColonyLabelCall) , RemoveRecoveryRole (RemoveRecoveryRoleCall) , RemoveTaskEvaluatorRole (RemoveTaskEvaluatorRoleCall) , RemoveTaskWorkerRole (RemoveTaskWorkerRoleCall) , RevealTaskWorkRating (RevealTaskWorkRatingCall) , SetAdministrationRole (SetAdministrationRoleCall) , SetAllTaskPayouts (SetAllTaskPayoutsCall) , SetArbitrationRole (SetArbitrationRoleCall) , SetArchitectureRole (SetArchitectureRoleCall) , SetDefaultGlobalClaimDelay (SetDefaultGlobalClaimDelayCall) , SetExpenditureClaimDelay (SetExpenditureClaimDelayCall) , SetExpenditureClaimDelays (SetExpenditureClaimDelaysCall) , SetExpenditureMetadataWithPermissionDomainIdAndChildSkillIndex (SetExpenditureMetadataWithPermissionDomainIdAndChildSkillIndexCall) , SetExpenditureMetadata (SetExpenditureMetadataCall) , SetExpenditurePayout (SetExpenditurePayoutCall) , SetExpenditurePayoutWithPermissionDomainIdAndChildSkillIndexAndIdAndSlot (SetExpenditurePayoutWithPermissionDomainIdAndChildSkillIndexAndIdAndSlotCall) , SetExpenditurePayoutModifiers (SetExpenditurePayoutModifiersCall) , SetExpenditurePayouts (SetExpenditurePayoutsCall) , SetExpenditureRecipient (SetExpenditureRecipientCall) , SetExpenditureRecipients (SetExpenditureRecipientsCall) , SetExpenditureSkill (SetExpenditureSkillCall) , SetExpenditureSkills (SetExpenditureSkillsCall) , SetExpenditureState (SetExpenditureStateCall) , SetExpenditureValues (SetExpenditureValuesCall) , SetFundingRole (SetFundingRoleCall) , SetPaymentPayout (SetPaymentPayoutCall) , SetPaymentRecipient (SetPaymentRecipientCall) , SetPaymentSkill (SetPaymentSkillCall) , SetRecoveryRole (SetRecoveryRoleCall) , SetRewardInverse (SetRewardInverseCall) , SetRootRole (SetRootRoleCall) , SetStorageSlotRecovery (SetStorageSlotRecoveryCall) , SetTaskBrief (SetTaskBriefCall) , SetTaskDueDate (SetTaskDueDateCall) , SetTaskEvaluatorPayout (SetTaskEvaluatorPayoutCall) , SetTaskEvaluatorRole (SetTaskEvaluatorRoleCall) , SetTaskManagerPayout (SetTaskManagerPayoutCall) , SetTaskManagerRole (SetTaskManagerRoleCall) , SetTaskSkill (SetTaskSkillCall) , SetTaskWorkerPayout (SetTaskWorkerPayoutCall) , SetTaskWorkerRole (SetTaskWorkerRoleCall) , SetUserRoles (SetUserRolesCall) , StartNextRewardPayout (StartNextRewardPayoutCall) , SubmitTaskDeliverable (SubmitTaskDeliverableCall) , SubmitTaskDeliverableAndRating (SubmitTaskDeliverableAndRatingCall) , SubmitTaskWorkRating (SubmitTaskWorkRatingCall) , TransferExpenditure (TransferExpenditureCall) , TransferExpenditureViaArbitration (TransferExpenditureViaArbitrationCall) , TransferStake (TransferStakeCall) , UninstallExtension (UninstallExtensionCall) , UnlockToken (UnlockTokenCall) , UnlockTokenForUser (UnlockTokenForUserCall) , UpdateApprovalAmount (UpdateApprovalAmountCall) , UpdateColonyOrbitDB (UpdateColonyOrbitDBCall) , Upgrade (UpgradeCall) , UpgradeExtension (UpgradeExtensionCall) , UserCanSetRoles (UserCanSetRolesCall) , VerifyReputationProof (VerifyReputationProofCall) , Version (VersionCall) }
    impl ethers::core::abi::AbiDecode for ColonyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddDomainCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::AddDomain(decoded));
            }
            if let Ok (decoded) = < AddDomainWithPermissionDomainIdAndChildSkillIndexAndMetadataCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (ColonyCalls :: AddDomainWithPermissionDomainIdAndChildSkillIndexAndMetadata (decoded)) }
            if let Ok(decoded) =
                <AddLocalSkillCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::AddLocalSkill(decoded));
            }
            if let Ok(decoded) =
                <AddPaymentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::AddPayment(decoded));
            }
            if let Ok(decoded) =
                <AnnotateTransactionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::AnnotateTransaction(decoded));
            }
            if let Ok(decoded) =
                <ApproveExitRecoveryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::ApproveExitRecovery(decoded));
            }
            if let Ok(decoded) =
                <ApproveStakeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::ApproveStake(decoded));
            }
            if let Ok(decoded) =
                <AuthorityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::Authority(decoded));
            }
            if let Ok(decoded) =
                <BootstrapColonyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::BootstrapColony(decoded));
            }
            if let Ok(decoded) =
                <BurnTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::BurnTokens(decoded));
            }
            if let Ok(decoded) =
                <CancelExpenditureCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::CancelExpenditure(decoded));
            }
            if let Ok(decoded) =
                <CancelTaskCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::CancelTask(decoded));
            }
            if let Ok(decoded) =
                <CheckNotAdditionalProtectedVariableCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyCalls::CheckNotAdditionalProtectedVariable(decoded));
            }
            if let Ok(decoded) =
                <ClaimColonyFundsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::ClaimColonyFunds(decoded));
            }
            if let Ok(decoded) =
                <ClaimExpenditurePayoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::ClaimExpenditurePayout(decoded));
            }
            if let Ok(decoded) =
                <ClaimPaymentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::ClaimPayment(decoded));
            }
            if let Ok(decoded) =
                <ClaimRewardPayoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::ClaimRewardPayout(decoded));
            }
            if let Ok(decoded) =
                <ClaimTaskPayoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::ClaimTaskPayout(decoded));
            }
            if let Ok(decoded) =
                <CompleteTaskCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::CompleteTask(decoded));
            }
            if let Ok(decoded) =
                <DeobligateStakeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::DeobligateStake(decoded));
            }
            if let Ok(decoded) =
                <DeprecateDomainCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::DeprecateDomain(decoded));
            }
            if let Ok(decoded) =
                <DeprecateExtensionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::DeprecateExtension(decoded));
            }
            if let Ok(decoded) =
                <DeprecateLocalSkillCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::DeprecateLocalSkill(decoded));
            }
            if let Ok(decoded) =
                <EditColonyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::EditColony(decoded));
            }
            if let Ok(decoded) =
                <EditColonyByDeltaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::EditColonyByDelta(decoded));
            }
            if let Ok(decoded) =
                <EditDomainCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::EditDomain(decoded));
            }
            if let Ok(decoded) =
                <EmitDomainReputationPenaltyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyCalls::EmitDomainReputationPenalty(decoded));
            }
            if let Ok(decoded) =
                <EmitDomainReputationRewardCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyCalls::EmitDomainReputationReward(decoded));
            }
            if let Ok(decoded) =
                <EmitSkillReputationPenaltyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyCalls::EmitSkillReputationPenalty(decoded));
            }
            if let Ok(decoded) =
                <EmitSkillReputationRewardCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyCalls::EmitSkillReputationReward(decoded));
            }
            if let Ok(decoded) =
                <EnterRecoveryModeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::EnterRecoveryMode(decoded));
            }
            if let Ok(decoded) =
                <ExecuteMetaTransactionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::ExecuteMetaTransaction(decoded));
            }
            if let Ok(decoded) =
                <ExecuteTaskChangeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::ExecuteTaskChange(decoded));
            }
            if let Ok(decoded) =
                <ExecuteTaskRoleAssignmentCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyCalls::ExecuteTaskRoleAssignment(decoded));
            }
            if let Ok(decoded) =
                <ExitRecoveryModeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::ExitRecoveryMode(decoded));
            }
            if let Ok(decoded) =
                <FinalizeExpenditureCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::FinalizeExpenditure(decoded));
            }
            if let Ok(decoded) =
                <FinalizePaymentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::FinalizePayment(decoded));
            }
            if let Ok(decoded) =
                <FinalizeRewardPayoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::FinalizeRewardPayout(decoded));
            }
            if let Ok(decoded) =
                <FinalizeTaskCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::FinalizeTask(decoded));
            }
            if let Ok(decoded) =
                <FinishUpgradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::FinishUpgrade(decoded));
            }
            if let Ok(decoded) =
                <GenerateSecretCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GenerateSecret(decoded));
            }
            if let Ok(decoded) =
                <GetApprovalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetApproval(decoded));
            }
            if let Ok(decoded) =
                <GetCapabilityRolesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetCapabilityRoles(decoded));
            }
            if let Ok(decoded) =
                <GetColonyNetworkCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetColonyNetwork(decoded));
            }
            if let Ok(decoded) =
                <GetDomainCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetDomain(decoded));
            }
            if let Ok(decoded) =
                <GetDomainCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetDomainCount(decoded));
            }
            if let Ok(decoded) =
                <GetDomainFromFundingPotCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetDomainFromFundingPot(decoded));
            }
            if let Ok(decoded) =
                <GetExpenditureCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetExpenditure(decoded));
            }
            if let Ok(decoded) =
                <GetExpenditureCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetExpenditureCount(decoded));
            }
            if let Ok(decoded) =
                <GetExpenditureSlotCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetExpenditureSlot(decoded));
            }
            if let Ok(decoded) =
                <GetExpenditureSlotPayoutCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyCalls::GetExpenditureSlotPayout(decoded));
            }
            if let Ok(decoded) =
                <GetFundingPotCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetFundingPot(decoded));
            }
            if let Ok(decoded) =
                <GetFundingPotBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetFundingPotBalance(decoded));
            }
            if let Ok(decoded) =
                <GetFundingPotCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetFundingPotCount(decoded));
            }
            if let Ok(decoded) =
                <GetFundingPotPayoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetFundingPotPayout(decoded));
            }
            if let Ok(decoded) =
                <GetMetatransactionNonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetMetatransactionNonce(decoded));
            }
            if let Ok(decoded) =
                <GetNonRewardPotsTotalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetNonRewardPotsTotal(decoded));
            }
            if let Ok(decoded) =
                <GetObligationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetObligation(decoded));
            }
            if let Ok(decoded) =
                <GetPaymentCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetPayment(decoded));
            }
            if let Ok(decoded) =
                <GetPaymentCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetPaymentCount(decoded));
            }
            if let Ok(decoded) =
                <GetRewardInverseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetRewardInverse(decoded));
            }
            if let Ok(decoded) =
                <GetRewardPayoutInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetRewardPayoutInfo(decoded));
            }
            if let Ok(decoded) =
                <GetRootLocalSkillCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetRootLocalSkill(decoded));
            }
            if let Ok(decoded) =
                <GetTaskCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetTask(decoded));
            }
            if let Ok(decoded) =
                <GetTaskChangeNonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetTaskChangeNonce(decoded));
            }
            if let Ok(decoded) =
                <GetTaskCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetTaskCount(decoded));
            }
            if let Ok(decoded) =
                <GetTaskPayoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetTaskPayout(decoded));
            }
            if let Ok(decoded) =
                <GetTaskRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetTaskRole(decoded));
            }
            if let Ok(decoded) =
                <GetTaskWorkRatingSecretCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetTaskWorkRatingSecret(decoded));
            }
            if let Ok(decoded) =
                <GetTaskWorkRatingSecretsInfoCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyCalls::GetTaskWorkRatingSecretsInfo(decoded));
            }
            if let Ok(decoded) =
                <GetTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetToken(decoded));
            }
            if let Ok(decoded) =
                <GetTokenApprovalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetTokenApproval(decoded));
            }
            if let Ok(decoded) =
                <GetTotalTokenApprovalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetTotalTokenApproval(decoded));
            }
            if let Ok(decoded) =
                <GetUserRolesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::GetUserRoles(decoded));
            }
            if let Ok(decoded) =
                <HasInheritedUserRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::HasInheritedUserRole(decoded));
            }
            if let Ok(decoded) =
                <HasUserRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::HasUserRole(decoded));
            }
            if let Ok(decoded) =
                <InitialiseColonyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::InitialiseColony(decoded));
            }
            if let Ok(decoded) =
                <InitialiseRootLocalSkillCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyCalls::InitialiseRootLocalSkill(decoded));
            }
            if let Ok(decoded) =
                <InstallExtensionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::InstallExtension(decoded));
            }
            if let Ok(decoded) =
                <IsInRecoveryModeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::IsInRecoveryMode(decoded));
            }
            if let Ok(decoded) =
                <LockExpenditureCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::LockExpenditure(decoded));
            }
            if let Ok(decoded) =
                <LockTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::LockToken(decoded));
            }
            if let Ok(decoded) =
                <MakeArbitraryTransactionCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyCalls::MakeArbitraryTransaction(decoded));
            }
            if let Ok(decoded) =
                <MakeArbitraryTransactionsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyCalls::MakeArbitraryTransactions(decoded));
            }
            if let Ok(decoded) =
                <MakeExpenditureCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::MakeExpenditure(decoded));
            }
            if let Ok(decoded) =
                <MakeSingleArbitraryTransactionCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyCalls::MakeSingleArbitraryTransaction(decoded));
            }
            if let Ok(decoded) =
                <MakeTaskCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::MakeTask(decoded));
            }
            if let Ok(decoded) =
                <MintTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::MintTokens(decoded));
            }
            if let Ok(decoded) =
                <MintTokensForCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::MintTokensFor(decoded));
            }
            if let Ok (decoded) = < MoveFundsBetweenPotsWithPermissionDomainIdAndChildSkillIndexAndDomainIdAndFromChildSkillIndexAndToChildSkillIndexAndFromPotAndToPotCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (ColonyCalls :: MoveFundsBetweenPotsWithPermissionDomainIdAndChildSkillIndexAndDomainIdAndFromChildSkillIndexAndToChildSkillIndexAndFromPotAndToPot (decoded)) }
            if let Ok(decoded) =
                <MoveFundsBetweenPotsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::MoveFundsBetweenPots(decoded));
            }
            if let Ok(decoded) =
                <MulticallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::Multicall(decoded));
            }
            if let Ok(decoded) =
                <NumRecoveryRolesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::NumRecoveryRoles(decoded));
            }
            if let Ok(decoded) =
                <ObligateStakeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::ObligateStake(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RegisterColonyLabelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::RegisterColonyLabel(decoded));
            }
            if let Ok(decoded) =
                <RemoveRecoveryRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::RemoveRecoveryRole(decoded));
            }
            if let Ok(decoded) =
                <RemoveTaskEvaluatorRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::RemoveTaskEvaluatorRole(decoded));
            }
            if let Ok(decoded) =
                <RemoveTaskWorkerRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::RemoveTaskWorkerRole(decoded));
            }
            if let Ok(decoded) =
                <RevealTaskWorkRatingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::RevealTaskWorkRating(decoded));
            }
            if let Ok(decoded) =
                <SetAdministrationRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetAdministrationRole(decoded));
            }
            if let Ok(decoded) =
                <SetAllTaskPayoutsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetAllTaskPayouts(decoded));
            }
            if let Ok(decoded) =
                <SetArbitrationRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetArbitrationRole(decoded));
            }
            if let Ok(decoded) =
                <SetArchitectureRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetArchitectureRole(decoded));
            }
            if let Ok(decoded) =
                <SetDefaultGlobalClaimDelayCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyCalls::SetDefaultGlobalClaimDelay(decoded));
            }
            if let Ok(decoded) =
                <SetExpenditureClaimDelayCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyCalls::SetExpenditureClaimDelay(decoded));
            }
            if let Ok(decoded) =
                <SetExpenditureClaimDelaysCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyCalls::SetExpenditureClaimDelays(decoded));
            }
            if let Ok (decoded) = < SetExpenditureMetadataWithPermissionDomainIdAndChildSkillIndexCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (ColonyCalls :: SetExpenditureMetadataWithPermissionDomainIdAndChildSkillIndex (decoded)) }
            if let Ok(decoded) =
                <SetExpenditureMetadataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetExpenditureMetadata(decoded));
            }
            if let Ok(decoded) =
                <SetExpenditurePayoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetExpenditurePayout(decoded));
            }
            if let Ok (decoded) = < SetExpenditurePayoutWithPermissionDomainIdAndChildSkillIndexAndIdAndSlotCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (ColonyCalls :: SetExpenditurePayoutWithPermissionDomainIdAndChildSkillIndexAndIdAndSlot (decoded)) }
            if let Ok(decoded) =
                <SetExpenditurePayoutModifiersCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyCalls::SetExpenditurePayoutModifiers(decoded));
            }
            if let Ok(decoded) =
                <SetExpenditurePayoutsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetExpenditurePayouts(decoded));
            }
            if let Ok(decoded) =
                <SetExpenditureRecipientCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetExpenditureRecipient(decoded));
            }
            if let Ok(decoded) =
                <SetExpenditureRecipientsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyCalls::SetExpenditureRecipients(decoded));
            }
            if let Ok(decoded) =
                <SetExpenditureSkillCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetExpenditureSkill(decoded));
            }
            if let Ok(decoded) =
                <SetExpenditureSkillsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetExpenditureSkills(decoded));
            }
            if let Ok(decoded) =
                <SetExpenditureStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetExpenditureState(decoded));
            }
            if let Ok(decoded) =
                <SetExpenditureValuesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetExpenditureValues(decoded));
            }
            if let Ok(decoded) =
                <SetFundingRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetFundingRole(decoded));
            }
            if let Ok(decoded) =
                <SetPaymentPayoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetPaymentPayout(decoded));
            }
            if let Ok(decoded) =
                <SetPaymentRecipientCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetPaymentRecipient(decoded));
            }
            if let Ok(decoded) =
                <SetPaymentSkillCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetPaymentSkill(decoded));
            }
            if let Ok(decoded) =
                <SetRecoveryRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetRecoveryRole(decoded));
            }
            if let Ok(decoded) =
                <SetRewardInverseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetRewardInverse(decoded));
            }
            if let Ok(decoded) =
                <SetRootRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetRootRole(decoded));
            }
            if let Ok(decoded) =
                <SetStorageSlotRecoveryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetStorageSlotRecovery(decoded));
            }
            if let Ok(decoded) =
                <SetTaskBriefCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetTaskBrief(decoded));
            }
            if let Ok(decoded) =
                <SetTaskDueDateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetTaskDueDate(decoded));
            }
            if let Ok(decoded) =
                <SetTaskEvaluatorPayoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetTaskEvaluatorPayout(decoded));
            }
            if let Ok(decoded) =
                <SetTaskEvaluatorRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetTaskEvaluatorRole(decoded));
            }
            if let Ok(decoded) =
                <SetTaskManagerPayoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetTaskManagerPayout(decoded));
            }
            if let Ok(decoded) =
                <SetTaskManagerRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetTaskManagerRole(decoded));
            }
            if let Ok(decoded) =
                <SetTaskSkillCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetTaskSkill(decoded));
            }
            if let Ok(decoded) =
                <SetTaskWorkerPayoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetTaskWorkerPayout(decoded));
            }
            if let Ok(decoded) =
                <SetTaskWorkerRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetTaskWorkerRole(decoded));
            }
            if let Ok(decoded) =
                <SetUserRolesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SetUserRoles(decoded));
            }
            if let Ok(decoded) =
                <StartNextRewardPayoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::StartNextRewardPayout(decoded));
            }
            if let Ok(decoded) =
                <SubmitTaskDeliverableCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SubmitTaskDeliverable(decoded));
            }
            if let Ok(decoded) =
                <SubmitTaskDeliverableAndRatingCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyCalls::SubmitTaskDeliverableAndRating(decoded));
            }
            if let Ok(decoded) =
                <SubmitTaskWorkRatingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::SubmitTaskWorkRating(decoded));
            }
            if let Ok(decoded) =
                <TransferExpenditureCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::TransferExpenditure(decoded));
            }
            if let Ok(decoded) =
                <TransferExpenditureViaArbitrationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ColonyCalls::TransferExpenditureViaArbitration(decoded));
            }
            if let Ok(decoded) =
                <TransferStakeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::TransferStake(decoded));
            }
            if let Ok(decoded) =
                <UninstallExtensionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::UninstallExtension(decoded));
            }
            if let Ok(decoded) =
                <UnlockTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::UnlockToken(decoded));
            }
            if let Ok(decoded) =
                <UnlockTokenForUserCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::UnlockTokenForUser(decoded));
            }
            if let Ok(decoded) =
                <UpdateApprovalAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::UpdateApprovalAmount(decoded));
            }
            if let Ok(decoded) =
                <UpdateColonyOrbitDBCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::UpdateColonyOrbitDB(decoded));
            }
            if let Ok(decoded) =
                <UpgradeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::Upgrade(decoded));
            }
            if let Ok(decoded) =
                <UpgradeExtensionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::UpgradeExtension(decoded));
            }
            if let Ok(decoded) =
                <UserCanSetRolesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::UserCanSetRoles(decoded));
            }
            if let Ok(decoded) =
                <VerifyReputationProofCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::VerifyReputationProof(decoded));
            }
            if let Ok(decoded) =
                <VersionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ColonyCalls::Version(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ColonyCalls {
        fn encode(self) -> Vec<u8> {
            match self { ColonyCalls :: AddDomain (element) => element . encode () , ColonyCalls :: AddDomainWithPermissionDomainIdAndChildSkillIndexAndMetadata (element) => element . encode () , ColonyCalls :: AddLocalSkill (element) => element . encode () , ColonyCalls :: AddPayment (element) => element . encode () , ColonyCalls :: AnnotateTransaction (element) => element . encode () , ColonyCalls :: ApproveExitRecovery (element) => element . encode () , ColonyCalls :: ApproveStake (element) => element . encode () , ColonyCalls :: Authority (element) => element . encode () , ColonyCalls :: BootstrapColony (element) => element . encode () , ColonyCalls :: BurnTokens (element) => element . encode () , ColonyCalls :: CancelExpenditure (element) => element . encode () , ColonyCalls :: CancelTask (element) => element . encode () , ColonyCalls :: CheckNotAdditionalProtectedVariable (element) => element . encode () , ColonyCalls :: ClaimColonyFunds (element) => element . encode () , ColonyCalls :: ClaimExpenditurePayout (element) => element . encode () , ColonyCalls :: ClaimPayment (element) => element . encode () , ColonyCalls :: ClaimRewardPayout (element) => element . encode () , ColonyCalls :: ClaimTaskPayout (element) => element . encode () , ColonyCalls :: CompleteTask (element) => element . encode () , ColonyCalls :: DeobligateStake (element) => element . encode () , ColonyCalls :: DeprecateDomain (element) => element . encode () , ColonyCalls :: DeprecateExtension (element) => element . encode () , ColonyCalls :: DeprecateLocalSkill (element) => element . encode () , ColonyCalls :: EditColony (element) => element . encode () , ColonyCalls :: EditColonyByDelta (element) => element . encode () , ColonyCalls :: EditDomain (element) => element . encode () , ColonyCalls :: EmitDomainReputationPenalty (element) => element . encode () , ColonyCalls :: EmitDomainReputationReward (element) => element . encode () , ColonyCalls :: EmitSkillReputationPenalty (element) => element . encode () , ColonyCalls :: EmitSkillReputationReward (element) => element . encode () , ColonyCalls :: EnterRecoveryMode (element) => element . encode () , ColonyCalls :: ExecuteMetaTransaction (element) => element . encode () , ColonyCalls :: ExecuteTaskChange (element) => element . encode () , ColonyCalls :: ExecuteTaskRoleAssignment (element) => element . encode () , ColonyCalls :: ExitRecoveryMode (element) => element . encode () , ColonyCalls :: FinalizeExpenditure (element) => element . encode () , ColonyCalls :: FinalizePayment (element) => element . encode () , ColonyCalls :: FinalizeRewardPayout (element) => element . encode () , ColonyCalls :: FinalizeTask (element) => element . encode () , ColonyCalls :: FinishUpgrade (element) => element . encode () , ColonyCalls :: GenerateSecret (element) => element . encode () , ColonyCalls :: GetApproval (element) => element . encode () , ColonyCalls :: GetCapabilityRoles (element) => element . encode () , ColonyCalls :: GetColonyNetwork (element) => element . encode () , ColonyCalls :: GetDomain (element) => element . encode () , ColonyCalls :: GetDomainCount (element) => element . encode () , ColonyCalls :: GetDomainFromFundingPot (element) => element . encode () , ColonyCalls :: GetExpenditure (element) => element . encode () , ColonyCalls :: GetExpenditureCount (element) => element . encode () , ColonyCalls :: GetExpenditureSlot (element) => element . encode () , ColonyCalls :: GetExpenditureSlotPayout (element) => element . encode () , ColonyCalls :: GetFundingPot (element) => element . encode () , ColonyCalls :: GetFundingPotBalance (element) => element . encode () , ColonyCalls :: GetFundingPotCount (element) => element . encode () , ColonyCalls :: GetFundingPotPayout (element) => element . encode () , ColonyCalls :: GetMetatransactionNonce (element) => element . encode () , ColonyCalls :: GetNonRewardPotsTotal (element) => element . encode () , ColonyCalls :: GetObligation (element) => element . encode () , ColonyCalls :: GetPayment (element) => element . encode () , ColonyCalls :: GetPaymentCount (element) => element . encode () , ColonyCalls :: GetRewardInverse (element) => element . encode () , ColonyCalls :: GetRewardPayoutInfo (element) => element . encode () , ColonyCalls :: GetRootLocalSkill (element) => element . encode () , ColonyCalls :: GetTask (element) => element . encode () , ColonyCalls :: GetTaskChangeNonce (element) => element . encode () , ColonyCalls :: GetTaskCount (element) => element . encode () , ColonyCalls :: GetTaskPayout (element) => element . encode () , ColonyCalls :: GetTaskRole (element) => element . encode () , ColonyCalls :: GetTaskWorkRatingSecret (element) => element . encode () , ColonyCalls :: GetTaskWorkRatingSecretsInfo (element) => element . encode () , ColonyCalls :: GetToken (element) => element . encode () , ColonyCalls :: GetTokenApproval (element) => element . encode () , ColonyCalls :: GetTotalTokenApproval (element) => element . encode () , ColonyCalls :: GetUserRoles (element) => element . encode () , ColonyCalls :: HasInheritedUserRole (element) => element . encode () , ColonyCalls :: HasUserRole (element) => element . encode () , ColonyCalls :: InitialiseColony (element) => element . encode () , ColonyCalls :: InitialiseRootLocalSkill (element) => element . encode () , ColonyCalls :: InstallExtension (element) => element . encode () , ColonyCalls :: IsInRecoveryMode (element) => element . encode () , ColonyCalls :: LockExpenditure (element) => element . encode () , ColonyCalls :: LockToken (element) => element . encode () , ColonyCalls :: MakeArbitraryTransaction (element) => element . encode () , ColonyCalls :: MakeArbitraryTransactions (element) => element . encode () , ColonyCalls :: MakeExpenditure (element) => element . encode () , ColonyCalls :: MakeSingleArbitraryTransaction (element) => element . encode () , ColonyCalls :: MakeTask (element) => element . encode () , ColonyCalls :: MintTokens (element) => element . encode () , ColonyCalls :: MintTokensFor (element) => element . encode () , ColonyCalls :: MoveFundsBetweenPotsWithPermissionDomainIdAndChildSkillIndexAndDomainIdAndFromChildSkillIndexAndToChildSkillIndexAndFromPotAndToPot (element) => element . encode () , ColonyCalls :: MoveFundsBetweenPots (element) => element . encode () , ColonyCalls :: Multicall (element) => element . encode () , ColonyCalls :: NumRecoveryRoles (element) => element . encode () , ColonyCalls :: ObligateStake (element) => element . encode () , ColonyCalls :: Owner (element) => element . encode () , ColonyCalls :: RegisterColonyLabel (element) => element . encode () , ColonyCalls :: RemoveRecoveryRole (element) => element . encode () , ColonyCalls :: RemoveTaskEvaluatorRole (element) => element . encode () , ColonyCalls :: RemoveTaskWorkerRole (element) => element . encode () , ColonyCalls :: RevealTaskWorkRating (element) => element . encode () , ColonyCalls :: SetAdministrationRole (element) => element . encode () , ColonyCalls :: SetAllTaskPayouts (element) => element . encode () , ColonyCalls :: SetArbitrationRole (element) => element . encode () , ColonyCalls :: SetArchitectureRole (element) => element . encode () , ColonyCalls :: SetDefaultGlobalClaimDelay (element) => element . encode () , ColonyCalls :: SetExpenditureClaimDelay (element) => element . encode () , ColonyCalls :: SetExpenditureClaimDelays (element) => element . encode () , ColonyCalls :: SetExpenditureMetadataWithPermissionDomainIdAndChildSkillIndex (element) => element . encode () , ColonyCalls :: SetExpenditureMetadata (element) => element . encode () , ColonyCalls :: SetExpenditurePayout (element) => element . encode () , ColonyCalls :: SetExpenditurePayoutWithPermissionDomainIdAndChildSkillIndexAndIdAndSlot (element) => element . encode () , ColonyCalls :: SetExpenditurePayoutModifiers (element) => element . encode () , ColonyCalls :: SetExpenditurePayouts (element) => element . encode () , ColonyCalls :: SetExpenditureRecipient (element) => element . encode () , ColonyCalls :: SetExpenditureRecipients (element) => element . encode () , ColonyCalls :: SetExpenditureSkill (element) => element . encode () , ColonyCalls :: SetExpenditureSkills (element) => element . encode () , ColonyCalls :: SetExpenditureState (element) => element . encode () , ColonyCalls :: SetExpenditureValues (element) => element . encode () , ColonyCalls :: SetFundingRole (element) => element . encode () , ColonyCalls :: SetPaymentPayout (element) => element . encode () , ColonyCalls :: SetPaymentRecipient (element) => element . encode () , ColonyCalls :: SetPaymentSkill (element) => element . encode () , ColonyCalls :: SetRecoveryRole (element) => element . encode () , ColonyCalls :: SetRewardInverse (element) => element . encode () , ColonyCalls :: SetRootRole (element) => element . encode () , ColonyCalls :: SetStorageSlotRecovery (element) => element . encode () , ColonyCalls :: SetTaskBrief (element) => element . encode () , ColonyCalls :: SetTaskDueDate (element) => element . encode () , ColonyCalls :: SetTaskEvaluatorPayout (element) => element . encode () , ColonyCalls :: SetTaskEvaluatorRole (element) => element . encode () , ColonyCalls :: SetTaskManagerPayout (element) => element . encode () , ColonyCalls :: SetTaskManagerRole (element) => element . encode () , ColonyCalls :: SetTaskSkill (element) => element . encode () , ColonyCalls :: SetTaskWorkerPayout (element) => element . encode () , ColonyCalls :: SetTaskWorkerRole (element) => element . encode () , ColonyCalls :: SetUserRoles (element) => element . encode () , ColonyCalls :: StartNextRewardPayout (element) => element . encode () , ColonyCalls :: SubmitTaskDeliverable (element) => element . encode () , ColonyCalls :: SubmitTaskDeliverableAndRating (element) => element . encode () , ColonyCalls :: SubmitTaskWorkRating (element) => element . encode () , ColonyCalls :: TransferExpenditure (element) => element . encode () , ColonyCalls :: TransferExpenditureViaArbitration (element) => element . encode () , ColonyCalls :: TransferStake (element) => element . encode () , ColonyCalls :: UninstallExtension (element) => element . encode () , ColonyCalls :: UnlockToken (element) => element . encode () , ColonyCalls :: UnlockTokenForUser (element) => element . encode () , ColonyCalls :: UpdateApprovalAmount (element) => element . encode () , ColonyCalls :: UpdateColonyOrbitDB (element) => element . encode () , ColonyCalls :: Upgrade (element) => element . encode () , ColonyCalls :: UpgradeExtension (element) => element . encode () , ColonyCalls :: UserCanSetRoles (element) => element . encode () , ColonyCalls :: VerifyReputationProof (element) => element . encode () , ColonyCalls :: Version (element) => element . encode () }
        }
    }
    impl ::std::fmt::Display for ColonyCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self { ColonyCalls :: AddDomain (element) => element . fmt (f) , ColonyCalls :: AddDomainWithPermissionDomainIdAndChildSkillIndexAndMetadata (element) => element . fmt (f) , ColonyCalls :: AddLocalSkill (element) => element . fmt (f) , ColonyCalls :: AddPayment (element) => element . fmt (f) , ColonyCalls :: AnnotateTransaction (element) => element . fmt (f) , ColonyCalls :: ApproveExitRecovery (element) => element . fmt (f) , ColonyCalls :: ApproveStake (element) => element . fmt (f) , ColonyCalls :: Authority (element) => element . fmt (f) , ColonyCalls :: BootstrapColony (element) => element . fmt (f) , ColonyCalls :: BurnTokens (element) => element . fmt (f) , ColonyCalls :: CancelExpenditure (element) => element . fmt (f) , ColonyCalls :: CancelTask (element) => element . fmt (f) , ColonyCalls :: CheckNotAdditionalProtectedVariable (element) => element . fmt (f) , ColonyCalls :: ClaimColonyFunds (element) => element . fmt (f) , ColonyCalls :: ClaimExpenditurePayout (element) => element . fmt (f) , ColonyCalls :: ClaimPayment (element) => element . fmt (f) , ColonyCalls :: ClaimRewardPayout (element) => element . fmt (f) , ColonyCalls :: ClaimTaskPayout (element) => element . fmt (f) , ColonyCalls :: CompleteTask (element) => element . fmt (f) , ColonyCalls :: DeobligateStake (element) => element . fmt (f) , ColonyCalls :: DeprecateDomain (element) => element . fmt (f) , ColonyCalls :: DeprecateExtension (element) => element . fmt (f) , ColonyCalls :: DeprecateLocalSkill (element) => element . fmt (f) , ColonyCalls :: EditColony (element) => element . fmt (f) , ColonyCalls :: EditColonyByDelta (element) => element . fmt (f) , ColonyCalls :: EditDomain (element) => element . fmt (f) , ColonyCalls :: EmitDomainReputationPenalty (element) => element . fmt (f) , ColonyCalls :: EmitDomainReputationReward (element) => element . fmt (f) , ColonyCalls :: EmitSkillReputationPenalty (element) => element . fmt (f) , ColonyCalls :: EmitSkillReputationReward (element) => element . fmt (f) , ColonyCalls :: EnterRecoveryMode (element) => element . fmt (f) , ColonyCalls :: ExecuteMetaTransaction (element) => element . fmt (f) , ColonyCalls :: ExecuteTaskChange (element) => element . fmt (f) , ColonyCalls :: ExecuteTaskRoleAssignment (element) => element . fmt (f) , ColonyCalls :: ExitRecoveryMode (element) => element . fmt (f) , ColonyCalls :: FinalizeExpenditure (element) => element . fmt (f) , ColonyCalls :: FinalizePayment (element) => element . fmt (f) , ColonyCalls :: FinalizeRewardPayout (element) => element . fmt (f) , ColonyCalls :: FinalizeTask (element) => element . fmt (f) , ColonyCalls :: FinishUpgrade (element) => element . fmt (f) , ColonyCalls :: GenerateSecret (element) => element . fmt (f) , ColonyCalls :: GetApproval (element) => element . fmt (f) , ColonyCalls :: GetCapabilityRoles (element) => element . fmt (f) , ColonyCalls :: GetColonyNetwork (element) => element . fmt (f) , ColonyCalls :: GetDomain (element) => element . fmt (f) , ColonyCalls :: GetDomainCount (element) => element . fmt (f) , ColonyCalls :: GetDomainFromFundingPot (element) => element . fmt (f) , ColonyCalls :: GetExpenditure (element) => element . fmt (f) , ColonyCalls :: GetExpenditureCount (element) => element . fmt (f) , ColonyCalls :: GetExpenditureSlot (element) => element . fmt (f) , ColonyCalls :: GetExpenditureSlotPayout (element) => element . fmt (f) , ColonyCalls :: GetFundingPot (element) => element . fmt (f) , ColonyCalls :: GetFundingPotBalance (element) => element . fmt (f) , ColonyCalls :: GetFundingPotCount (element) => element . fmt (f) , ColonyCalls :: GetFundingPotPayout (element) => element . fmt (f) , ColonyCalls :: GetMetatransactionNonce (element) => element . fmt (f) , ColonyCalls :: GetNonRewardPotsTotal (element) => element . fmt (f) , ColonyCalls :: GetObligation (element) => element . fmt (f) , ColonyCalls :: GetPayment (element) => element . fmt (f) , ColonyCalls :: GetPaymentCount (element) => element . fmt (f) , ColonyCalls :: GetRewardInverse (element) => element . fmt (f) , ColonyCalls :: GetRewardPayoutInfo (element) => element . fmt (f) , ColonyCalls :: GetRootLocalSkill (element) => element . fmt (f) , ColonyCalls :: GetTask (element) => element . fmt (f) , ColonyCalls :: GetTaskChangeNonce (element) => element . fmt (f) , ColonyCalls :: GetTaskCount (element) => element . fmt (f) , ColonyCalls :: GetTaskPayout (element) => element . fmt (f) , ColonyCalls :: GetTaskRole (element) => element . fmt (f) , ColonyCalls :: GetTaskWorkRatingSecret (element) => element . fmt (f) , ColonyCalls :: GetTaskWorkRatingSecretsInfo (element) => element . fmt (f) , ColonyCalls :: GetToken (element) => element . fmt (f) , ColonyCalls :: GetTokenApproval (element) => element . fmt (f) , ColonyCalls :: GetTotalTokenApproval (element) => element . fmt (f) , ColonyCalls :: GetUserRoles (element) => element . fmt (f) , ColonyCalls :: HasInheritedUserRole (element) => element . fmt (f) , ColonyCalls :: HasUserRole (element) => element . fmt (f) , ColonyCalls :: InitialiseColony (element) => element . fmt (f) , ColonyCalls :: InitialiseRootLocalSkill (element) => element . fmt (f) , ColonyCalls :: InstallExtension (element) => element . fmt (f) , ColonyCalls :: IsInRecoveryMode (element) => element . fmt (f) , ColonyCalls :: LockExpenditure (element) => element . fmt (f) , ColonyCalls :: LockToken (element) => element . fmt (f) , ColonyCalls :: MakeArbitraryTransaction (element) => element . fmt (f) , ColonyCalls :: MakeArbitraryTransactions (element) => element . fmt (f) , ColonyCalls :: MakeExpenditure (element) => element . fmt (f) , ColonyCalls :: MakeSingleArbitraryTransaction (element) => element . fmt (f) , ColonyCalls :: MakeTask (element) => element . fmt (f) , ColonyCalls :: MintTokens (element) => element . fmt (f) , ColonyCalls :: MintTokensFor (element) => element . fmt (f) , ColonyCalls :: MoveFundsBetweenPotsWithPermissionDomainIdAndChildSkillIndexAndDomainIdAndFromChildSkillIndexAndToChildSkillIndexAndFromPotAndToPot (element) => element . fmt (f) , ColonyCalls :: MoveFundsBetweenPots (element) => element . fmt (f) , ColonyCalls :: Multicall (element) => element . fmt (f) , ColonyCalls :: NumRecoveryRoles (element) => element . fmt (f) , ColonyCalls :: ObligateStake (element) => element . fmt (f) , ColonyCalls :: Owner (element) => element . fmt (f) , ColonyCalls :: RegisterColonyLabel (element) => element . fmt (f) , ColonyCalls :: RemoveRecoveryRole (element) => element . fmt (f) , ColonyCalls :: RemoveTaskEvaluatorRole (element) => element . fmt (f) , ColonyCalls :: RemoveTaskWorkerRole (element) => element . fmt (f) , ColonyCalls :: RevealTaskWorkRating (element) => element . fmt (f) , ColonyCalls :: SetAdministrationRole (element) => element . fmt (f) , ColonyCalls :: SetAllTaskPayouts (element) => element . fmt (f) , ColonyCalls :: SetArbitrationRole (element) => element . fmt (f) , ColonyCalls :: SetArchitectureRole (element) => element . fmt (f) , ColonyCalls :: SetDefaultGlobalClaimDelay (element) => element . fmt (f) , ColonyCalls :: SetExpenditureClaimDelay (element) => element . fmt (f) , ColonyCalls :: SetExpenditureClaimDelays (element) => element . fmt (f) , ColonyCalls :: SetExpenditureMetadataWithPermissionDomainIdAndChildSkillIndex (element) => element . fmt (f) , ColonyCalls :: SetExpenditureMetadata (element) => element . fmt (f) , ColonyCalls :: SetExpenditurePayout (element) => element . fmt (f) , ColonyCalls :: SetExpenditurePayoutWithPermissionDomainIdAndChildSkillIndexAndIdAndSlot (element) => element . fmt (f) , ColonyCalls :: SetExpenditurePayoutModifiers (element) => element . fmt (f) , ColonyCalls :: SetExpenditurePayouts (element) => element . fmt (f) , ColonyCalls :: SetExpenditureRecipient (element) => element . fmt (f) , ColonyCalls :: SetExpenditureRecipients (element) => element . fmt (f) , ColonyCalls :: SetExpenditureSkill (element) => element . fmt (f) , ColonyCalls :: SetExpenditureSkills (element) => element . fmt (f) , ColonyCalls :: SetExpenditureState (element) => element . fmt (f) , ColonyCalls :: SetExpenditureValues (element) => element . fmt (f) , ColonyCalls :: SetFundingRole (element) => element . fmt (f) , ColonyCalls :: SetPaymentPayout (element) => element . fmt (f) , ColonyCalls :: SetPaymentRecipient (element) => element . fmt (f) , ColonyCalls :: SetPaymentSkill (element) => element . fmt (f) , ColonyCalls :: SetRecoveryRole (element) => element . fmt (f) , ColonyCalls :: SetRewardInverse (element) => element . fmt (f) , ColonyCalls :: SetRootRole (element) => element . fmt (f) , ColonyCalls :: SetStorageSlotRecovery (element) => element . fmt (f) , ColonyCalls :: SetTaskBrief (element) => element . fmt (f) , ColonyCalls :: SetTaskDueDate (element) => element . fmt (f) , ColonyCalls :: SetTaskEvaluatorPayout (element) => element . fmt (f) , ColonyCalls :: SetTaskEvaluatorRole (element) => element . fmt (f) , ColonyCalls :: SetTaskManagerPayout (element) => element . fmt (f) , ColonyCalls :: SetTaskManagerRole (element) => element . fmt (f) , ColonyCalls :: SetTaskSkill (element) => element . fmt (f) , ColonyCalls :: SetTaskWorkerPayout (element) => element . fmt (f) , ColonyCalls :: SetTaskWorkerRole (element) => element . fmt (f) , ColonyCalls :: SetUserRoles (element) => element . fmt (f) , ColonyCalls :: StartNextRewardPayout (element) => element . fmt (f) , ColonyCalls :: SubmitTaskDeliverable (element) => element . fmt (f) , ColonyCalls :: SubmitTaskDeliverableAndRating (element) => element . fmt (f) , ColonyCalls :: SubmitTaskWorkRating (element) => element . fmt (f) , ColonyCalls :: TransferExpenditure (element) => element . fmt (f) , ColonyCalls :: TransferExpenditureViaArbitration (element) => element . fmt (f) , ColonyCalls :: TransferStake (element) => element . fmt (f) , ColonyCalls :: UninstallExtension (element) => element . fmt (f) , ColonyCalls :: UnlockToken (element) => element . fmt (f) , ColonyCalls :: UnlockTokenForUser (element) => element . fmt (f) , ColonyCalls :: UpdateApprovalAmount (element) => element . fmt (f) , ColonyCalls :: UpdateColonyOrbitDB (element) => element . fmt (f) , ColonyCalls :: Upgrade (element) => element . fmt (f) , ColonyCalls :: UpgradeExtension (element) => element . fmt (f) , ColonyCalls :: UserCanSetRoles (element) => element . fmt (f) , ColonyCalls :: VerifyReputationProof (element) => element . fmt (f) , ColonyCalls :: Version (element) => element . fmt (f) }
        }
    }
    impl ::std::convert::From<AddDomainCall> for ColonyCalls {
        fn from(var: AddDomainCall) -> Self {
            ColonyCalls::AddDomain(var)
        }
    }
    impl ::std::convert::From<AddDomainWithPermissionDomainIdAndChildSkillIndexAndMetadataCall>
        for ColonyCalls
    {
        fn from(var: AddDomainWithPermissionDomainIdAndChildSkillIndexAndMetadataCall) -> Self {
            ColonyCalls::AddDomainWithPermissionDomainIdAndChildSkillIndexAndMetadata(var)
        }
    }
    impl ::std::convert::From<AddLocalSkillCall> for ColonyCalls {
        fn from(var: AddLocalSkillCall) -> Self {
            ColonyCalls::AddLocalSkill(var)
        }
    }
    impl ::std::convert::From<AddPaymentCall> for ColonyCalls {
        fn from(var: AddPaymentCall) -> Self {
            ColonyCalls::AddPayment(var)
        }
    }
    impl ::std::convert::From<AnnotateTransactionCall> for ColonyCalls {
        fn from(var: AnnotateTransactionCall) -> Self {
            ColonyCalls::AnnotateTransaction(var)
        }
    }
    impl ::std::convert::From<ApproveExitRecoveryCall> for ColonyCalls {
        fn from(var: ApproveExitRecoveryCall) -> Self {
            ColonyCalls::ApproveExitRecovery(var)
        }
    }
    impl ::std::convert::From<ApproveStakeCall> for ColonyCalls {
        fn from(var: ApproveStakeCall) -> Self {
            ColonyCalls::ApproveStake(var)
        }
    }
    impl ::std::convert::From<AuthorityCall> for ColonyCalls {
        fn from(var: AuthorityCall) -> Self {
            ColonyCalls::Authority(var)
        }
    }
    impl ::std::convert::From<BootstrapColonyCall> for ColonyCalls {
        fn from(var: BootstrapColonyCall) -> Self {
            ColonyCalls::BootstrapColony(var)
        }
    }
    impl ::std::convert::From<BurnTokensCall> for ColonyCalls {
        fn from(var: BurnTokensCall) -> Self {
            ColonyCalls::BurnTokens(var)
        }
    }
    impl ::std::convert::From<CancelExpenditureCall> for ColonyCalls {
        fn from(var: CancelExpenditureCall) -> Self {
            ColonyCalls::CancelExpenditure(var)
        }
    }
    impl ::std::convert::From<CancelTaskCall> for ColonyCalls {
        fn from(var: CancelTaskCall) -> Self {
            ColonyCalls::CancelTask(var)
        }
    }
    impl ::std::convert::From<CheckNotAdditionalProtectedVariableCall> for ColonyCalls {
        fn from(var: CheckNotAdditionalProtectedVariableCall) -> Self {
            ColonyCalls::CheckNotAdditionalProtectedVariable(var)
        }
    }
    impl ::std::convert::From<ClaimColonyFundsCall> for ColonyCalls {
        fn from(var: ClaimColonyFundsCall) -> Self {
            ColonyCalls::ClaimColonyFunds(var)
        }
    }
    impl ::std::convert::From<ClaimExpenditurePayoutCall> for ColonyCalls {
        fn from(var: ClaimExpenditurePayoutCall) -> Self {
            ColonyCalls::ClaimExpenditurePayout(var)
        }
    }
    impl ::std::convert::From<ClaimPaymentCall> for ColonyCalls {
        fn from(var: ClaimPaymentCall) -> Self {
            ColonyCalls::ClaimPayment(var)
        }
    }
    impl ::std::convert::From<ClaimRewardPayoutCall> for ColonyCalls {
        fn from(var: ClaimRewardPayoutCall) -> Self {
            ColonyCalls::ClaimRewardPayout(var)
        }
    }
    impl ::std::convert::From<ClaimTaskPayoutCall> for ColonyCalls {
        fn from(var: ClaimTaskPayoutCall) -> Self {
            ColonyCalls::ClaimTaskPayout(var)
        }
    }
    impl ::std::convert::From<CompleteTaskCall> for ColonyCalls {
        fn from(var: CompleteTaskCall) -> Self {
            ColonyCalls::CompleteTask(var)
        }
    }
    impl ::std::convert::From<DeobligateStakeCall> for ColonyCalls {
        fn from(var: DeobligateStakeCall) -> Self {
            ColonyCalls::DeobligateStake(var)
        }
    }
    impl ::std::convert::From<DeprecateDomainCall> for ColonyCalls {
        fn from(var: DeprecateDomainCall) -> Self {
            ColonyCalls::DeprecateDomain(var)
        }
    }
    impl ::std::convert::From<DeprecateExtensionCall> for ColonyCalls {
        fn from(var: DeprecateExtensionCall) -> Self {
            ColonyCalls::DeprecateExtension(var)
        }
    }
    impl ::std::convert::From<DeprecateLocalSkillCall> for ColonyCalls {
        fn from(var: DeprecateLocalSkillCall) -> Self {
            ColonyCalls::DeprecateLocalSkill(var)
        }
    }
    impl ::std::convert::From<EditColonyCall> for ColonyCalls {
        fn from(var: EditColonyCall) -> Self {
            ColonyCalls::EditColony(var)
        }
    }
    impl ::std::convert::From<EditColonyByDeltaCall> for ColonyCalls {
        fn from(var: EditColonyByDeltaCall) -> Self {
            ColonyCalls::EditColonyByDelta(var)
        }
    }
    impl ::std::convert::From<EditDomainCall> for ColonyCalls {
        fn from(var: EditDomainCall) -> Self {
            ColonyCalls::EditDomain(var)
        }
    }
    impl ::std::convert::From<EmitDomainReputationPenaltyCall> for ColonyCalls {
        fn from(var: EmitDomainReputationPenaltyCall) -> Self {
            ColonyCalls::EmitDomainReputationPenalty(var)
        }
    }
    impl ::std::convert::From<EmitDomainReputationRewardCall> for ColonyCalls {
        fn from(var: EmitDomainReputationRewardCall) -> Self {
            ColonyCalls::EmitDomainReputationReward(var)
        }
    }
    impl ::std::convert::From<EmitSkillReputationPenaltyCall> for ColonyCalls {
        fn from(var: EmitSkillReputationPenaltyCall) -> Self {
            ColonyCalls::EmitSkillReputationPenalty(var)
        }
    }
    impl ::std::convert::From<EmitSkillReputationRewardCall> for ColonyCalls {
        fn from(var: EmitSkillReputationRewardCall) -> Self {
            ColonyCalls::EmitSkillReputationReward(var)
        }
    }
    impl ::std::convert::From<EnterRecoveryModeCall> for ColonyCalls {
        fn from(var: EnterRecoveryModeCall) -> Self {
            ColonyCalls::EnterRecoveryMode(var)
        }
    }
    impl ::std::convert::From<ExecuteMetaTransactionCall> for ColonyCalls {
        fn from(var: ExecuteMetaTransactionCall) -> Self {
            ColonyCalls::ExecuteMetaTransaction(var)
        }
    }
    impl ::std::convert::From<ExecuteTaskChangeCall> for ColonyCalls {
        fn from(var: ExecuteTaskChangeCall) -> Self {
            ColonyCalls::ExecuteTaskChange(var)
        }
    }
    impl ::std::convert::From<ExecuteTaskRoleAssignmentCall> for ColonyCalls {
        fn from(var: ExecuteTaskRoleAssignmentCall) -> Self {
            ColonyCalls::ExecuteTaskRoleAssignment(var)
        }
    }
    impl ::std::convert::From<ExitRecoveryModeCall> for ColonyCalls {
        fn from(var: ExitRecoveryModeCall) -> Self {
            ColonyCalls::ExitRecoveryMode(var)
        }
    }
    impl ::std::convert::From<FinalizeExpenditureCall> for ColonyCalls {
        fn from(var: FinalizeExpenditureCall) -> Self {
            ColonyCalls::FinalizeExpenditure(var)
        }
    }
    impl ::std::convert::From<FinalizePaymentCall> for ColonyCalls {
        fn from(var: FinalizePaymentCall) -> Self {
            ColonyCalls::FinalizePayment(var)
        }
    }
    impl ::std::convert::From<FinalizeRewardPayoutCall> for ColonyCalls {
        fn from(var: FinalizeRewardPayoutCall) -> Self {
            ColonyCalls::FinalizeRewardPayout(var)
        }
    }
    impl ::std::convert::From<FinalizeTaskCall> for ColonyCalls {
        fn from(var: FinalizeTaskCall) -> Self {
            ColonyCalls::FinalizeTask(var)
        }
    }
    impl ::std::convert::From<FinishUpgradeCall> for ColonyCalls {
        fn from(var: FinishUpgradeCall) -> Self {
            ColonyCalls::FinishUpgrade(var)
        }
    }
    impl ::std::convert::From<GenerateSecretCall> for ColonyCalls {
        fn from(var: GenerateSecretCall) -> Self {
            ColonyCalls::GenerateSecret(var)
        }
    }
    impl ::std::convert::From<GetApprovalCall> for ColonyCalls {
        fn from(var: GetApprovalCall) -> Self {
            ColonyCalls::GetApproval(var)
        }
    }
    impl ::std::convert::From<GetCapabilityRolesCall> for ColonyCalls {
        fn from(var: GetCapabilityRolesCall) -> Self {
            ColonyCalls::GetCapabilityRoles(var)
        }
    }
    impl ::std::convert::From<GetColonyNetworkCall> for ColonyCalls {
        fn from(var: GetColonyNetworkCall) -> Self {
            ColonyCalls::GetColonyNetwork(var)
        }
    }
    impl ::std::convert::From<GetDomainCall> for ColonyCalls {
        fn from(var: GetDomainCall) -> Self {
            ColonyCalls::GetDomain(var)
        }
    }
    impl ::std::convert::From<GetDomainCountCall> for ColonyCalls {
        fn from(var: GetDomainCountCall) -> Self {
            ColonyCalls::GetDomainCount(var)
        }
    }
    impl ::std::convert::From<GetDomainFromFundingPotCall> for ColonyCalls {
        fn from(var: GetDomainFromFundingPotCall) -> Self {
            ColonyCalls::GetDomainFromFundingPot(var)
        }
    }
    impl ::std::convert::From<GetExpenditureCall> for ColonyCalls {
        fn from(var: GetExpenditureCall) -> Self {
            ColonyCalls::GetExpenditure(var)
        }
    }
    impl ::std::convert::From<GetExpenditureCountCall> for ColonyCalls {
        fn from(var: GetExpenditureCountCall) -> Self {
            ColonyCalls::GetExpenditureCount(var)
        }
    }
    impl ::std::convert::From<GetExpenditureSlotCall> for ColonyCalls {
        fn from(var: GetExpenditureSlotCall) -> Self {
            ColonyCalls::GetExpenditureSlot(var)
        }
    }
    impl ::std::convert::From<GetExpenditureSlotPayoutCall> for ColonyCalls {
        fn from(var: GetExpenditureSlotPayoutCall) -> Self {
            ColonyCalls::GetExpenditureSlotPayout(var)
        }
    }
    impl ::std::convert::From<GetFundingPotCall> for ColonyCalls {
        fn from(var: GetFundingPotCall) -> Self {
            ColonyCalls::GetFundingPot(var)
        }
    }
    impl ::std::convert::From<GetFundingPotBalanceCall> for ColonyCalls {
        fn from(var: GetFundingPotBalanceCall) -> Self {
            ColonyCalls::GetFundingPotBalance(var)
        }
    }
    impl ::std::convert::From<GetFundingPotCountCall> for ColonyCalls {
        fn from(var: GetFundingPotCountCall) -> Self {
            ColonyCalls::GetFundingPotCount(var)
        }
    }
    impl ::std::convert::From<GetFundingPotPayoutCall> for ColonyCalls {
        fn from(var: GetFundingPotPayoutCall) -> Self {
            ColonyCalls::GetFundingPotPayout(var)
        }
    }
    impl ::std::convert::From<GetMetatransactionNonceCall> for ColonyCalls {
        fn from(var: GetMetatransactionNonceCall) -> Self {
            ColonyCalls::GetMetatransactionNonce(var)
        }
    }
    impl ::std::convert::From<GetNonRewardPotsTotalCall> for ColonyCalls {
        fn from(var: GetNonRewardPotsTotalCall) -> Self {
            ColonyCalls::GetNonRewardPotsTotal(var)
        }
    }
    impl ::std::convert::From<GetObligationCall> for ColonyCalls {
        fn from(var: GetObligationCall) -> Self {
            ColonyCalls::GetObligation(var)
        }
    }
    impl ::std::convert::From<GetPaymentCall> for ColonyCalls {
        fn from(var: GetPaymentCall) -> Self {
            ColonyCalls::GetPayment(var)
        }
    }
    impl ::std::convert::From<GetPaymentCountCall> for ColonyCalls {
        fn from(var: GetPaymentCountCall) -> Self {
            ColonyCalls::GetPaymentCount(var)
        }
    }
    impl ::std::convert::From<GetRewardInverseCall> for ColonyCalls {
        fn from(var: GetRewardInverseCall) -> Self {
            ColonyCalls::GetRewardInverse(var)
        }
    }
    impl ::std::convert::From<GetRewardPayoutInfoCall> for ColonyCalls {
        fn from(var: GetRewardPayoutInfoCall) -> Self {
            ColonyCalls::GetRewardPayoutInfo(var)
        }
    }
    impl ::std::convert::From<GetRootLocalSkillCall> for ColonyCalls {
        fn from(var: GetRootLocalSkillCall) -> Self {
            ColonyCalls::GetRootLocalSkill(var)
        }
    }
    impl ::std::convert::From<GetTaskCall> for ColonyCalls {
        fn from(var: GetTaskCall) -> Self {
            ColonyCalls::GetTask(var)
        }
    }
    impl ::std::convert::From<GetTaskChangeNonceCall> for ColonyCalls {
        fn from(var: GetTaskChangeNonceCall) -> Self {
            ColonyCalls::GetTaskChangeNonce(var)
        }
    }
    impl ::std::convert::From<GetTaskCountCall> for ColonyCalls {
        fn from(var: GetTaskCountCall) -> Self {
            ColonyCalls::GetTaskCount(var)
        }
    }
    impl ::std::convert::From<GetTaskPayoutCall> for ColonyCalls {
        fn from(var: GetTaskPayoutCall) -> Self {
            ColonyCalls::GetTaskPayout(var)
        }
    }
    impl ::std::convert::From<GetTaskRoleCall> for ColonyCalls {
        fn from(var: GetTaskRoleCall) -> Self {
            ColonyCalls::GetTaskRole(var)
        }
    }
    impl ::std::convert::From<GetTaskWorkRatingSecretCall> for ColonyCalls {
        fn from(var: GetTaskWorkRatingSecretCall) -> Self {
            ColonyCalls::GetTaskWorkRatingSecret(var)
        }
    }
    impl ::std::convert::From<GetTaskWorkRatingSecretsInfoCall> for ColonyCalls {
        fn from(var: GetTaskWorkRatingSecretsInfoCall) -> Self {
            ColonyCalls::GetTaskWorkRatingSecretsInfo(var)
        }
    }
    impl ::std::convert::From<GetTokenCall> for ColonyCalls {
        fn from(var: GetTokenCall) -> Self {
            ColonyCalls::GetToken(var)
        }
    }
    impl ::std::convert::From<GetTokenApprovalCall> for ColonyCalls {
        fn from(var: GetTokenApprovalCall) -> Self {
            ColonyCalls::GetTokenApproval(var)
        }
    }
    impl ::std::convert::From<GetTotalTokenApprovalCall> for ColonyCalls {
        fn from(var: GetTotalTokenApprovalCall) -> Self {
            ColonyCalls::GetTotalTokenApproval(var)
        }
    }
    impl ::std::convert::From<GetUserRolesCall> for ColonyCalls {
        fn from(var: GetUserRolesCall) -> Self {
            ColonyCalls::GetUserRoles(var)
        }
    }
    impl ::std::convert::From<HasInheritedUserRoleCall> for ColonyCalls {
        fn from(var: HasInheritedUserRoleCall) -> Self {
            ColonyCalls::HasInheritedUserRole(var)
        }
    }
    impl ::std::convert::From<HasUserRoleCall> for ColonyCalls {
        fn from(var: HasUserRoleCall) -> Self {
            ColonyCalls::HasUserRole(var)
        }
    }
    impl ::std::convert::From<InitialiseColonyCall> for ColonyCalls {
        fn from(var: InitialiseColonyCall) -> Self {
            ColonyCalls::InitialiseColony(var)
        }
    }
    impl ::std::convert::From<InitialiseRootLocalSkillCall> for ColonyCalls {
        fn from(var: InitialiseRootLocalSkillCall) -> Self {
            ColonyCalls::InitialiseRootLocalSkill(var)
        }
    }
    impl ::std::convert::From<InstallExtensionCall> for ColonyCalls {
        fn from(var: InstallExtensionCall) -> Self {
            ColonyCalls::InstallExtension(var)
        }
    }
    impl ::std::convert::From<IsInRecoveryModeCall> for ColonyCalls {
        fn from(var: IsInRecoveryModeCall) -> Self {
            ColonyCalls::IsInRecoveryMode(var)
        }
    }
    impl ::std::convert::From<LockExpenditureCall> for ColonyCalls {
        fn from(var: LockExpenditureCall) -> Self {
            ColonyCalls::LockExpenditure(var)
        }
    }
    impl ::std::convert::From<LockTokenCall> for ColonyCalls {
        fn from(var: LockTokenCall) -> Self {
            ColonyCalls::LockToken(var)
        }
    }
    impl ::std::convert::From<MakeArbitraryTransactionCall> for ColonyCalls {
        fn from(var: MakeArbitraryTransactionCall) -> Self {
            ColonyCalls::MakeArbitraryTransaction(var)
        }
    }
    impl ::std::convert::From<MakeArbitraryTransactionsCall> for ColonyCalls {
        fn from(var: MakeArbitraryTransactionsCall) -> Self {
            ColonyCalls::MakeArbitraryTransactions(var)
        }
    }
    impl ::std::convert::From<MakeExpenditureCall> for ColonyCalls {
        fn from(var: MakeExpenditureCall) -> Self {
            ColonyCalls::MakeExpenditure(var)
        }
    }
    impl ::std::convert::From<MakeSingleArbitraryTransactionCall> for ColonyCalls {
        fn from(var: MakeSingleArbitraryTransactionCall) -> Self {
            ColonyCalls::MakeSingleArbitraryTransaction(var)
        }
    }
    impl ::std::convert::From<MakeTaskCall> for ColonyCalls {
        fn from(var: MakeTaskCall) -> Self {
            ColonyCalls::MakeTask(var)
        }
    }
    impl ::std::convert::From<MintTokensCall> for ColonyCalls {
        fn from(var: MintTokensCall) -> Self {
            ColonyCalls::MintTokens(var)
        }
    }
    impl ::std::convert::From<MintTokensForCall> for ColonyCalls {
        fn from(var: MintTokensForCall) -> Self {
            ColonyCalls::MintTokensFor(var)
        }
    }
    impl :: std :: convert :: From < MoveFundsBetweenPotsWithPermissionDomainIdAndChildSkillIndexAndDomainIdAndFromChildSkillIndexAndToChildSkillIndexAndFromPotAndToPotCall > for ColonyCalls { fn from (var : MoveFundsBetweenPotsWithPermissionDomainIdAndChildSkillIndexAndDomainIdAndFromChildSkillIndexAndToChildSkillIndexAndFromPotAndToPotCall) -> Self { ColonyCalls :: MoveFundsBetweenPotsWithPermissionDomainIdAndChildSkillIndexAndDomainIdAndFromChildSkillIndexAndToChildSkillIndexAndFromPotAndToPot (var) } }
    impl ::std::convert::From<MoveFundsBetweenPotsCall> for ColonyCalls {
        fn from(var: MoveFundsBetweenPotsCall) -> Self {
            ColonyCalls::MoveFundsBetweenPots(var)
        }
    }
    impl ::std::convert::From<MulticallCall> for ColonyCalls {
        fn from(var: MulticallCall) -> Self {
            ColonyCalls::Multicall(var)
        }
    }
    impl ::std::convert::From<NumRecoveryRolesCall> for ColonyCalls {
        fn from(var: NumRecoveryRolesCall) -> Self {
            ColonyCalls::NumRecoveryRoles(var)
        }
    }
    impl ::std::convert::From<ObligateStakeCall> for ColonyCalls {
        fn from(var: ObligateStakeCall) -> Self {
            ColonyCalls::ObligateStake(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for ColonyCalls {
        fn from(var: OwnerCall) -> Self {
            ColonyCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RegisterColonyLabelCall> for ColonyCalls {
        fn from(var: RegisterColonyLabelCall) -> Self {
            ColonyCalls::RegisterColonyLabel(var)
        }
    }
    impl ::std::convert::From<RemoveRecoveryRoleCall> for ColonyCalls {
        fn from(var: RemoveRecoveryRoleCall) -> Self {
            ColonyCalls::RemoveRecoveryRole(var)
        }
    }
    impl ::std::convert::From<RemoveTaskEvaluatorRoleCall> for ColonyCalls {
        fn from(var: RemoveTaskEvaluatorRoleCall) -> Self {
            ColonyCalls::RemoveTaskEvaluatorRole(var)
        }
    }
    impl ::std::convert::From<RemoveTaskWorkerRoleCall> for ColonyCalls {
        fn from(var: RemoveTaskWorkerRoleCall) -> Self {
            ColonyCalls::RemoveTaskWorkerRole(var)
        }
    }
    impl ::std::convert::From<RevealTaskWorkRatingCall> for ColonyCalls {
        fn from(var: RevealTaskWorkRatingCall) -> Self {
            ColonyCalls::RevealTaskWorkRating(var)
        }
    }
    impl ::std::convert::From<SetAdministrationRoleCall> for ColonyCalls {
        fn from(var: SetAdministrationRoleCall) -> Self {
            ColonyCalls::SetAdministrationRole(var)
        }
    }
    impl ::std::convert::From<SetAllTaskPayoutsCall> for ColonyCalls {
        fn from(var: SetAllTaskPayoutsCall) -> Self {
            ColonyCalls::SetAllTaskPayouts(var)
        }
    }
    impl ::std::convert::From<SetArbitrationRoleCall> for ColonyCalls {
        fn from(var: SetArbitrationRoleCall) -> Self {
            ColonyCalls::SetArbitrationRole(var)
        }
    }
    impl ::std::convert::From<SetArchitectureRoleCall> for ColonyCalls {
        fn from(var: SetArchitectureRoleCall) -> Self {
            ColonyCalls::SetArchitectureRole(var)
        }
    }
    impl ::std::convert::From<SetDefaultGlobalClaimDelayCall> for ColonyCalls {
        fn from(var: SetDefaultGlobalClaimDelayCall) -> Self {
            ColonyCalls::SetDefaultGlobalClaimDelay(var)
        }
    }
    impl ::std::convert::From<SetExpenditureClaimDelayCall> for ColonyCalls {
        fn from(var: SetExpenditureClaimDelayCall) -> Self {
            ColonyCalls::SetExpenditureClaimDelay(var)
        }
    }
    impl ::std::convert::From<SetExpenditureClaimDelaysCall> for ColonyCalls {
        fn from(var: SetExpenditureClaimDelaysCall) -> Self {
            ColonyCalls::SetExpenditureClaimDelays(var)
        }
    }
    impl ::std::convert::From<SetExpenditureMetadataWithPermissionDomainIdAndChildSkillIndexCall>
        for ColonyCalls
    {
        fn from(var: SetExpenditureMetadataWithPermissionDomainIdAndChildSkillIndexCall) -> Self {
            ColonyCalls::SetExpenditureMetadataWithPermissionDomainIdAndChildSkillIndex(var)
        }
    }
    impl ::std::convert::From<SetExpenditureMetadataCall> for ColonyCalls {
        fn from(var: SetExpenditureMetadataCall) -> Self {
            ColonyCalls::SetExpenditureMetadata(var)
        }
    }
    impl ::std::convert::From<SetExpenditurePayoutCall> for ColonyCalls {
        fn from(var: SetExpenditurePayoutCall) -> Self {
            ColonyCalls::SetExpenditurePayout(var)
        }
    }
    impl
        ::std::convert::From<
            SetExpenditurePayoutWithPermissionDomainIdAndChildSkillIndexAndIdAndSlotCall,
        > for ColonyCalls
    {
        fn from(
            var: SetExpenditurePayoutWithPermissionDomainIdAndChildSkillIndexAndIdAndSlotCall,
        ) -> Self {
            ColonyCalls::SetExpenditurePayoutWithPermissionDomainIdAndChildSkillIndexAndIdAndSlot(
                var,
            )
        }
    }
    impl ::std::convert::From<SetExpenditurePayoutModifiersCall> for ColonyCalls {
        fn from(var: SetExpenditurePayoutModifiersCall) -> Self {
            ColonyCalls::SetExpenditurePayoutModifiers(var)
        }
    }
    impl ::std::convert::From<SetExpenditurePayoutsCall> for ColonyCalls {
        fn from(var: SetExpenditurePayoutsCall) -> Self {
            ColonyCalls::SetExpenditurePayouts(var)
        }
    }
    impl ::std::convert::From<SetExpenditureRecipientCall> for ColonyCalls {
        fn from(var: SetExpenditureRecipientCall) -> Self {
            ColonyCalls::SetExpenditureRecipient(var)
        }
    }
    impl ::std::convert::From<SetExpenditureRecipientsCall> for ColonyCalls {
        fn from(var: SetExpenditureRecipientsCall) -> Self {
            ColonyCalls::SetExpenditureRecipients(var)
        }
    }
    impl ::std::convert::From<SetExpenditureSkillCall> for ColonyCalls {
        fn from(var: SetExpenditureSkillCall) -> Self {
            ColonyCalls::SetExpenditureSkill(var)
        }
    }
    impl ::std::convert::From<SetExpenditureSkillsCall> for ColonyCalls {
        fn from(var: SetExpenditureSkillsCall) -> Self {
            ColonyCalls::SetExpenditureSkills(var)
        }
    }
    impl ::std::convert::From<SetExpenditureStateCall> for ColonyCalls {
        fn from(var: SetExpenditureStateCall) -> Self {
            ColonyCalls::SetExpenditureState(var)
        }
    }
    impl ::std::convert::From<SetExpenditureValuesCall> for ColonyCalls {
        fn from(var: SetExpenditureValuesCall) -> Self {
            ColonyCalls::SetExpenditureValues(var)
        }
    }
    impl ::std::convert::From<SetFundingRoleCall> for ColonyCalls {
        fn from(var: SetFundingRoleCall) -> Self {
            ColonyCalls::SetFundingRole(var)
        }
    }
    impl ::std::convert::From<SetPaymentPayoutCall> for ColonyCalls {
        fn from(var: SetPaymentPayoutCall) -> Self {
            ColonyCalls::SetPaymentPayout(var)
        }
    }
    impl ::std::convert::From<SetPaymentRecipientCall> for ColonyCalls {
        fn from(var: SetPaymentRecipientCall) -> Self {
            ColonyCalls::SetPaymentRecipient(var)
        }
    }
    impl ::std::convert::From<SetPaymentSkillCall> for ColonyCalls {
        fn from(var: SetPaymentSkillCall) -> Self {
            ColonyCalls::SetPaymentSkill(var)
        }
    }
    impl ::std::convert::From<SetRecoveryRoleCall> for ColonyCalls {
        fn from(var: SetRecoveryRoleCall) -> Self {
            ColonyCalls::SetRecoveryRole(var)
        }
    }
    impl ::std::convert::From<SetRewardInverseCall> for ColonyCalls {
        fn from(var: SetRewardInverseCall) -> Self {
            ColonyCalls::SetRewardInverse(var)
        }
    }
    impl ::std::convert::From<SetRootRoleCall> for ColonyCalls {
        fn from(var: SetRootRoleCall) -> Self {
            ColonyCalls::SetRootRole(var)
        }
    }
    impl ::std::convert::From<SetStorageSlotRecoveryCall> for ColonyCalls {
        fn from(var: SetStorageSlotRecoveryCall) -> Self {
            ColonyCalls::SetStorageSlotRecovery(var)
        }
    }
    impl ::std::convert::From<SetTaskBriefCall> for ColonyCalls {
        fn from(var: SetTaskBriefCall) -> Self {
            ColonyCalls::SetTaskBrief(var)
        }
    }
    impl ::std::convert::From<SetTaskDueDateCall> for ColonyCalls {
        fn from(var: SetTaskDueDateCall) -> Self {
            ColonyCalls::SetTaskDueDate(var)
        }
    }
    impl ::std::convert::From<SetTaskEvaluatorPayoutCall> for ColonyCalls {
        fn from(var: SetTaskEvaluatorPayoutCall) -> Self {
            ColonyCalls::SetTaskEvaluatorPayout(var)
        }
    }
    impl ::std::convert::From<SetTaskEvaluatorRoleCall> for ColonyCalls {
        fn from(var: SetTaskEvaluatorRoleCall) -> Self {
            ColonyCalls::SetTaskEvaluatorRole(var)
        }
    }
    impl ::std::convert::From<SetTaskManagerPayoutCall> for ColonyCalls {
        fn from(var: SetTaskManagerPayoutCall) -> Self {
            ColonyCalls::SetTaskManagerPayout(var)
        }
    }
    impl ::std::convert::From<SetTaskManagerRoleCall> for ColonyCalls {
        fn from(var: SetTaskManagerRoleCall) -> Self {
            ColonyCalls::SetTaskManagerRole(var)
        }
    }
    impl ::std::convert::From<SetTaskSkillCall> for ColonyCalls {
        fn from(var: SetTaskSkillCall) -> Self {
            ColonyCalls::SetTaskSkill(var)
        }
    }
    impl ::std::convert::From<SetTaskWorkerPayoutCall> for ColonyCalls {
        fn from(var: SetTaskWorkerPayoutCall) -> Self {
            ColonyCalls::SetTaskWorkerPayout(var)
        }
    }
    impl ::std::convert::From<SetTaskWorkerRoleCall> for ColonyCalls {
        fn from(var: SetTaskWorkerRoleCall) -> Self {
            ColonyCalls::SetTaskWorkerRole(var)
        }
    }
    impl ::std::convert::From<SetUserRolesCall> for ColonyCalls {
        fn from(var: SetUserRolesCall) -> Self {
            ColonyCalls::SetUserRoles(var)
        }
    }
    impl ::std::convert::From<StartNextRewardPayoutCall> for ColonyCalls {
        fn from(var: StartNextRewardPayoutCall) -> Self {
            ColonyCalls::StartNextRewardPayout(var)
        }
    }
    impl ::std::convert::From<SubmitTaskDeliverableCall> for ColonyCalls {
        fn from(var: SubmitTaskDeliverableCall) -> Self {
            ColonyCalls::SubmitTaskDeliverable(var)
        }
    }
    impl ::std::convert::From<SubmitTaskDeliverableAndRatingCall> for ColonyCalls {
        fn from(var: SubmitTaskDeliverableAndRatingCall) -> Self {
            ColonyCalls::SubmitTaskDeliverableAndRating(var)
        }
    }
    impl ::std::convert::From<SubmitTaskWorkRatingCall> for ColonyCalls {
        fn from(var: SubmitTaskWorkRatingCall) -> Self {
            ColonyCalls::SubmitTaskWorkRating(var)
        }
    }
    impl ::std::convert::From<TransferExpenditureCall> for ColonyCalls {
        fn from(var: TransferExpenditureCall) -> Self {
            ColonyCalls::TransferExpenditure(var)
        }
    }
    impl ::std::convert::From<TransferExpenditureViaArbitrationCall> for ColonyCalls {
        fn from(var: TransferExpenditureViaArbitrationCall) -> Self {
            ColonyCalls::TransferExpenditureViaArbitration(var)
        }
    }
    impl ::std::convert::From<TransferStakeCall> for ColonyCalls {
        fn from(var: TransferStakeCall) -> Self {
            ColonyCalls::TransferStake(var)
        }
    }
    impl ::std::convert::From<UninstallExtensionCall> for ColonyCalls {
        fn from(var: UninstallExtensionCall) -> Self {
            ColonyCalls::UninstallExtension(var)
        }
    }
    impl ::std::convert::From<UnlockTokenCall> for ColonyCalls {
        fn from(var: UnlockTokenCall) -> Self {
            ColonyCalls::UnlockToken(var)
        }
    }
    impl ::std::convert::From<UnlockTokenForUserCall> for ColonyCalls {
        fn from(var: UnlockTokenForUserCall) -> Self {
            ColonyCalls::UnlockTokenForUser(var)
        }
    }
    impl ::std::convert::From<UpdateApprovalAmountCall> for ColonyCalls {
        fn from(var: UpdateApprovalAmountCall) -> Self {
            ColonyCalls::UpdateApprovalAmount(var)
        }
    }
    impl ::std::convert::From<UpdateColonyOrbitDBCall> for ColonyCalls {
        fn from(var: UpdateColonyOrbitDBCall) -> Self {
            ColonyCalls::UpdateColonyOrbitDB(var)
        }
    }
    impl ::std::convert::From<UpgradeCall> for ColonyCalls {
        fn from(var: UpgradeCall) -> Self {
            ColonyCalls::Upgrade(var)
        }
    }
    impl ::std::convert::From<UpgradeExtensionCall> for ColonyCalls {
        fn from(var: UpgradeExtensionCall) -> Self {
            ColonyCalls::UpgradeExtension(var)
        }
    }
    impl ::std::convert::From<UserCanSetRolesCall> for ColonyCalls {
        fn from(var: UserCanSetRolesCall) -> Self {
            ColonyCalls::UserCanSetRoles(var)
        }
    }
    impl ::std::convert::From<VerifyReputationProofCall> for ColonyCalls {
        fn from(var: VerifyReputationProofCall) -> Self {
            ColonyCalls::VerifyReputationProof(var)
        }
    }
    impl ::std::convert::From<VersionCall> for ColonyCalls {
        fn from(var: VersionCall) -> Self {
            ColonyCalls::Version(var)
        }
    }
    #[doc = "Container type for all return fields from the `addPayment` function with signature `addPayment(uint256,uint256,address,address,uint256,uint256,uint256)` and selector `[79, 141, 246, 67]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AddPaymentReturn {
        pub payment_id: ethers::core::types::U256,
    }
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
    pub struct AuthorityReturn {
        pub colony_authority: ethers::core::types::Address,
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
    #[doc = "Container type for all return fields from the `generateSecret` function with signature `generateSecret(bytes32,uint256)` and selector `[26, 227, 170, 164]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GenerateSecretReturn {
        pub secret: [u8; 32],
    }
    #[doc = "Container type for all return fields from the `getApproval` function with signature `getApproval(address,address,uint256)` and selector `[49, 45, 55, 29]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetApprovalReturn {
        pub approval: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getCapabilityRoles` function with signature `getCapabilityRoles(bytes4)` and selector `[45, 251, 176, 131]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCapabilityRolesReturn {
        pub roles: [u8; 32],
    }
    #[doc = "Container type for all return fields from the `getColonyNetwork` function with signature `getColonyNetwork()` and selector `[78, 94, 131, 149]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetColonyNetworkReturn {
        pub colony_network: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `getDomain` function with signature `getDomain(uint256)` and selector `[26, 122, 152, 226]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetDomainReturn {
        pub domain: Domain,
    }
    #[doc = "Container type for all return fields from the `getDomainCount` function with signature `getDomainCount()` and selector `[19, 233, 31, 8]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetDomainCountReturn {
        pub count: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getDomainFromFundingPot` function with signature `getDomainFromFundingPot(uint256)` and selector `[182, 27, 83, 23]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetDomainFromFundingPotReturn {
        pub domain_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getExpenditure` function with signature `getExpenditure(uint256)` and selector `[233, 106, 173, 134]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetExpenditureReturn {
        pub expenditure: Expenditure,
    }
    #[doc = "Container type for all return fields from the `getExpenditureCount` function with signature `getExpenditureCount()` and selector `[122, 78, 111, 241]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetExpenditureCountReturn {
        pub count: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getExpenditureSlot` function with signature `getExpenditureSlot(uint256,uint256)` and selector `[153, 75, 34, 129]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetExpenditureSlotReturn {
        pub expenditure_slot: ExpenditureSlot,
    }
    #[doc = "Container type for all return fields from the `getExpenditureSlotPayout` function with signature `getExpenditureSlotPayout(uint256,uint256,address)` and selector `[41, 171, 205, 206]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetExpenditureSlotPayoutReturn {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getFundingPot` function with signature `getFundingPot(uint256)` and selector `[124, 23, 53, 168]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetFundingPotReturn {
        pub associated_type: u8,
        pub associated_type_id: ethers::core::types::U256,
        pub payouts_we_cannot_make: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getFundingPotBalance` function with signature `getFundingPotBalance(uint256,address)` and selector `[170, 227, 189, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetFundingPotBalanceReturn {
        pub balance: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getFundingPotCount` function with signature `getFundingPotCount()` and selector `[37, 247, 89, 227]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetFundingPotCountReturn {
        pub count: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getFundingPotPayout` function with signature `getFundingPotPayout(uint256,address)` and selector `[197, 208, 136, 110]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetFundingPotPayoutReturn {
        pub payout: ethers::core::types::U256,
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
    #[doc = "Container type for all return fields from the `getNonRewardPotsTotal` function with signature `getNonRewardPotsTotal(address)` and selector `[52, 250, 122, 162]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetNonRewardPotsTotalReturn {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getObligation` function with signature `getObligation(address,address,uint256)` and selector `[95, 43, 140, 178]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetObligationReturn {
        pub obligation: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getPayment` function with signature `getPayment(uint256)` and selector `[50, 128, 168, 54]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetPaymentReturn {
        pub payment: Payment,
    }
    #[doc = "Container type for all return fields from the `getPaymentCount` function with signature `getPaymentCount()` and selector `[175, 50, 160, 63]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetPaymentCountReturn {
        pub count: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getRewardInverse` function with signature `getRewardInverse()` and selector `[76, 177, 100, 199]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetRewardInverseReturn {
        pub reward_inverse: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getRewardPayoutInfo` function with signature `getRewardPayoutInfo(uint256)` and selector `[54, 68, 163, 253]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetRewardPayoutInfoReturn {
        pub reward_payout_cycle: RewardPayoutCycle,
    }
    #[doc = "Container type for all return fields from the `getRootLocalSkill` function with signature `getRootLocalSkill()` and selector `[189, 166, 112, 35]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetRootLocalSkillReturn {
        pub root_local_skill: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getTask` function with signature `getTask(uint256)` and selector `[29, 101, 231, 126]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetTaskReturn {
        pub specification_hash: [u8; 32],
        pub deliverable_hash: [u8; 32],
        pub status: u8,
        pub due_date: ethers::core::types::U256,
        pub funding_pot_id: ethers::core::types::U256,
        pub completion_timestamp: ethers::core::types::U256,
        pub domain_id: ethers::core::types::U256,
        pub skill_ids: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all return fields from the `getTaskChangeNonce` function with signature `getTaskChangeNonce(uint256)` and selector `[206, 124, 238, 73]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetTaskChangeNonceReturn {
        pub nonce: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getTaskCount` function with signature `getTaskCount()` and selector `[193, 122, 52, 14]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetTaskCountReturn {
        pub count: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getTaskPayout` function with signature `getTaskPayout(uint256,uint8,address)` and selector `[5, 125, 218, 44]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetTaskPayoutReturn {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getTaskRole` function with signature `getTaskRole(uint256,uint8)` and selector `[30, 137, 84, 222]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetTaskRoleReturn {
        pub role: Role,
    }
    #[doc = "Container type for all return fields from the `getTaskWorkRatingSecret` function with signature `getTaskWorkRatingSecret(uint256,uint8)` and selector `[173, 78, 144, 140]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetTaskWorkRatingSecretReturn {
        pub secret: [u8; 32],
    }
    #[doc = "Container type for all return fields from the `getTaskWorkRatingSecretsInfo` function with signature `getTaskWorkRatingSecretsInfo(uint256)` and selector `[1, 248, 251, 21]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetTaskWorkRatingSecretsInfoReturn {
        pub n_secrets: ethers::core::types::U256,
        pub last_submitted_at: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getToken` function with signature `getToken()` and selector `[33, 223, 13, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetTokenReturn {
        pub token_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `getTokenApproval` function with signature `getTokenApproval(address,address)` and selector `[101, 223, 237, 196]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetTokenApprovalReturn {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getTotalTokenApproval` function with signature `getTotalTokenApproval(address)` and selector `[254, 53, 153, 149]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetTotalTokenApprovalReturn {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getUserRoles` function with signature `getUserRoles(address,uint256)` and selector `[89, 91, 3, 240]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetUserRolesReturn {
        pub roles: [u8; 32],
    }
    #[doc = "Container type for all return fields from the `hasInheritedUserRole` function with signature `hasInheritedUserRole(address,uint256,uint8,uint256,uint256)` and selector `[51, 84, 241, 56]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct HasInheritedUserRoleReturn {
        pub has_role: bool,
    }
    #[doc = "Container type for all return fields from the `hasUserRole` function with signature `hasUserRole(address,uint256,uint8)` and selector `[171, 47, 122, 226]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct HasUserRoleReturn {
        pub has_role: bool,
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
    #[doc = "Container type for all return fields from the `lockToken` function with signature `lockToken()` and selector `[188, 167, 169, 226]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LockTokenReturn {
        pub times_locked: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `makeArbitraryTransaction` function with signature `makeArbitraryTransaction(address,bytes)` and selector `[103, 195, 81, 194]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MakeArbitraryTransactionReturn {
        pub success: bool,
    }
    #[doc = "Container type for all return fields from the `makeArbitraryTransactions` function with signature `makeArbitraryTransactions(address[],bytes[],bool)` and selector `[198, 53, 110, 254]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MakeArbitraryTransactionsReturn {
        pub success: bool,
    }
    #[doc = "Container type for all return fields from the `makeExpenditure` function with signature `makeExpenditure(uint256,uint256,uint256)` and selector `[165, 72, 122, 55]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MakeExpenditureReturn {
        pub expenditure_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `makeSingleArbitraryTransaction` function with signature `makeSingleArbitraryTransaction(address,bytes)` and selector `[102, 59, 72, 45]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MakeSingleArbitraryTransactionReturn {
        pub success: bool,
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
    pub struct OwnerReturn {
        pub colony_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `userCanSetRoles` function with signature `userCanSetRoles(address,uint256,uint256,uint256)` and selector `[244, 168, 216, 81]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct UserCanSetRolesReturn {
        pub can_set: bool,
    }
    #[doc = "Container type for all return fields from the `verifyReputationProof` function with signature `verifyReputationProof(bytes,bytes,uint256,bytes32[])` and selector `[150, 217, 51, 224]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VerifyReputationProofReturn {
        pub is_valid: bool,
    }
    #[doc = "Container type for all return fields from the `version` function with signature `version()` and selector `[84, 253, 77, 80]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VersionReturn {
        pub colony_version: ethers::core::types::U256,
    }
    #[doc = "`Domain(uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Domain {
        pub skill_id: ethers::core::types::U256,
        pub funding_pot_id: ethers::core::types::U256,
    }
    #[doc = "`Expenditure(uint8,address,uint256,uint256,uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Expenditure {
        pub status: u8,
        pub owner: ethers::core::types::Address,
        pub funding_pot_id: ethers::core::types::U256,
        pub domain_id: ethers::core::types::U256,
        pub finalized_timestamp: ethers::core::types::U256,
        pub global_claim_delay: ethers::core::types::U256,
    }
    #[doc = "`ExpenditureSlot(address,uint256,int256,uint256[])`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ExpenditureSlot {
        pub recipient: ethers::core::types::Address,
        pub claim_delay: ethers::core::types::U256,
        pub payout_modifier: I256,
        pub skills: Vec<ethers::core::types::U256>,
    }
    #[doc = "`Payment(address,bool,uint256,uint256,uint256[])`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Payment {
        pub recipient: ethers::core::types::Address,
        pub finalized: bool,
        pub funding_pot_id: ethers::core::types::U256,
        pub domain_id: ethers::core::types::U256,
        pub skills: Vec<ethers::core::types::U256>,
    }
    #[doc = "`RewardPayoutCycle(bytes32,uint256,uint256,uint256,address,uint256,uint256,bool)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct RewardPayoutCycle {
        pub reputation_state: [u8; 32],
        pub colony_wide_reputation: ethers::core::types::U256,
        pub total_tokens: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
        pub token_address: ethers::core::types::Address,
        pub block_timestamp: ethers::core::types::U256,
        pub amount_remaining: ethers::core::types::U256,
        pub finalized: bool,
    }
    #[doc = "`Role(address,bool,uint8)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Role {
        pub user: ethers::core::types::Address,
        pub rate_fail: bool,
        pub rating: u8,
    }
}
