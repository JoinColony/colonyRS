use ethers::providers::{
    Http, HttpRateLimitRetryPolicy, Provider, RetryClient, RetryClientBuilder,
};
pub use ethers::types::*;
use once_cell::sync::OnceCell;
use reqwest::Error as ReqwestError;
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use std::boxed::Box;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use url::Url;
mod contracts;
use contracts::colony::Domain;

const COLONY_BASE_URL: &str = "https://xdai.colony.io";

// 0x78163f593D1Fa151B4B7cacD146586aD2b686294
const COLONY_NETWORK_ADDRESS: Address = H160([
    120, 22, 63, 89, 61, 31, 161, 81, 180, 183, 202, 205, 20, 101, 134, 173, 43, 104, 98, 148,
]);

/// We want to reuse the same provider for all requests, so we use a `OnceCell`
/// to lazily initialize if it hasn't been initialized yet. The `OnceCell`
/// holds an `Arc` to the provider, so we can clone it and use it in multiple
/// places.
static PROVIDER: OnceCell<Arc<Provider<RetryClient<Http>>>> = OnceCell::new();
static CLIENT: OnceCell<Arc<reqwest::Client>> = OnceCell::new();

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReputationNoProof {
    pub key: String,
    pub reputation_amount: String,
    pub value: String,
}

#[derive(Debug, Error)]
pub enum ColonyError {
    #[error("The provided address is not a valid address")]
    InvalidAddress,
    #[error("An error occurred while communicating with the provider")]
    ContractError(#[from] ethers::contract::ContractError<Provider<RetryClient<Http>>>),
    #[error("An error occurred while communicating with the colony http endpoint")]
    HttpError(#[from] ReqwestError),
    #[error("The provided signature is invalid")]
    InvalidSignature(#[from] SignatureError),
}

/// Returns an arc with generated Provider moved into, is is meant to be called
/// by each other function that tries to initialize the provider.
fn init_provider() -> Arc<Provider<RetryClient<Http>>> {
    let url = Url::from_str(&format!("{}/rpc/", COLONY_BASE_URL)).expect(&format!(
        "our constant url did not parse: {}/rpc/. This should not happen",
        COLONY_BASE_URL
    ));
    let http = Http::new(url);
    let client = RetryClientBuilder::default()
        .rate_limit_retries(10)
        .timeout_retries(10)
        .initial_backoff(Duration::from_millis(500))
        .build(http, Box::new(HttpRateLimitRetryPolicy::default()));
    Arc::new(Provider::new(client))
}

fn init_reqwest() -> Arc<Client> {
    let client = ClientBuilder::new().build().unwrap();
    Arc::new(client)
}

pub async fn get_reputation_root_hash() -> Result<TxHash, ColonyError> {
    let provider = PROVIDER.get_or_init(init_provider).clone();
    let network = contracts::colony_network::ColonyNetwork::new(COLONY_NETWORK_ADDRESS, provider);
    Ok(TxHash(network.get_reputation_root_hash().call().await?))
}

pub async fn get_domain(colony_address: &Address, id: u64) -> Result<Domain, ColonyError> {
    let provider = PROVIDER.get_or_init(init_provider).clone();
    let colony = contracts::colony::Colony::new(*colony_address, provider);
    Ok(colony.get_domain(id.into()).call().await?)
}

pub async fn get_reputation_in_domain(
    colony_address: &Address,
    user_address: &Address,
    domain_id: u64,
) -> Result<ReputationNoProof, ColonyError> {
    let root_hash = get_reputation_root_hash().await?;
    let domain = get_domain(colony_address, domain_id).await?;

    let endpoint = format!(
        "{}/reputation/xdai/{:?}/{:?}/{:?}/{:?}/noProof",
        COLONY_BASE_URL, root_hash, colony_address, domain.skill_id, user_address
    );
    eprintln!("endpoint: {}", endpoint);

    let client = CLIENT.get_or_init(init_reqwest);
    // let response = reqwest::get(endpoint).await?;
    let response = client.get(endpoint).send().await?;
    response.json().await.map_err(|e| e.into())
}

pub fn validate_signature(
    address: &Address,
    message: &str,
    signature: &str,
) -> Result<(), ColonyError> {
    let signature = Signature::from_str(signature)?;
    Ok(signature.verify(message, *address)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_message() {
        let user_address = "0x5D0275ceC343f973e168C161BAde8a12676c7925"
            .parse::<Address>()
            .unwrap();

        let message_str = "TEST";

        let message_sig = "0x640464c74b706d940aa4000aba2ec6a78bc69f6155137f656d7b5bbcc08a90814880240be2194cd9768ae48277c97b36c188cabab676aa5d6cab0c0e6214a8081c";

        assert!(validate_signature(&user_address, &message_str, &message_sig).is_ok());
    }

    #[tokio::test]
    async fn test_get_reputation_root_hash() {
        let hash = get_reputation_root_hash().await.unwrap();
        assert_eq!(format!("{hash:?}").len(), 66,);
    }
    #[tokio::test]
    async fn test_get_domain() {
        let colony_address = "0x364B3153A24bb9ECa28B8c7aCeB15E3942eb4fc5"
            .parse::<Address>()
            .unwrap();
        let domain = get_domain(&colony_address, 1).await.unwrap();
        assert_eq!(
            domain,
            Domain {
                skill_id: 3862.into(),
                funding_pot_id: 1.into()
            }
        );
    }

    #[tokio::test]
    async fn test_get_user_reputation_in_domain() {
        let colony_address = "0x364B3153A24bb9ECa28B8c7aCeB15E3942eb4fc5"
            .parse::<Address>()
            .unwrap();
        let user_address = "0x0AEFF664e8d75c13801be16bCfE8143Bf422135A"
            .parse::<Address>()
            .unwrap();
        let reputation = get_reputation_in_domain(&colony_address, &user_address, 1)
            .await
            .unwrap();
        assert_eq!(reputation.key, "0x364b3153a24bb9eca28b8c7aceb15e3942eb4fc50000000000000000000000000000000000000000000000000000000000000f160aeff664e8d75c13801be16bcfe8143bf422135a".to_string());
        assert_eq!(reputation.value, "0x0000000000000000000000000000000000000000000000041fc0add9b32492350000000000000000000000000000000000000000000000000000000000000f1a".to_string());
    }

    #[tokio::test]
    async fn test_get_whole_reputation_in_domain() {
        let colony_address = "0x364B3153A24bb9ECa28B8c7aCeB15E3942eb4fc5"
            .parse::<Address>()
            .unwrap();
        let user_address = Address::zero();
        let reputation = get_reputation_in_domain(&colony_address, &user_address, 1)
            .await
            .unwrap();
        assert_eq!(reputation.key, "0x364b3153a24bb9eca28b8c7aceb15e3942eb4fc50000000000000000000000000000000000000000000000000000000000000f160000000000000000000000000000000000000000".to_string());
        assert_eq!(reputation.value, "0x00000000000000000000000000000000000000000000000dc51a96a5089f4bfc0000000000000000000000000000000000000000000000000000000000000f19".to_string());
    }

    #[tokio::test]
    async fn test_get_no_reputation_in_domain() {
        let colony_address = "0x364B3153A24bb9ECa28B8c7aCeB15E3942eb4fc5"
            .parse::<Address>()
            .unwrap();
        let user_address = "0xcB313f361847e245954FD338Cb21b5F4225b17d1"
            .parse::<Address>()
            .unwrap();
        let reputation = get_reputation_in_domain(&colony_address, &user_address, 1).await;
        assert!(reputation.is_err());
    }

    #[tokio::test]
    async fn test_ether_scan() {
        let client =
            ethers::etherscan::Client::new(Chain::Mainnet, "4STDKJBKM789RAXJB9VEC5ICN2TTJX2R1R")
                .unwrap();
        let address = Address::from_str("0x0535f1f43Ee274123291bbAB284948CAED46C65D").unwrap();
        let balance = client
            .get_ether_balance_single(&address, None)
            .await
            .unwrap();
        assert_eq!(balance.balance, "0".to_string());
    }
}
