use ergo_lib::ergotree_ir::chain::address::{AddressEncoder, NetworkAddress, NetworkPrefix};
use reqwest::Url;
use serde::{Deserialize, Serialize};

const LOCAL_NODE_URL: &str = "https://localhost:9053/";

const MAINNET_EXPLORER_URL: &str = "https://explorer.ergoplatform.com/";
const TESTNET_EXPLORER_URL: &str = "https://testnet.ergoplatform.com/";

const MAINNET_EXPLORER_API_URL: &str = "https://api.ergoplatform.com/";
const TESTNET_EXPLORER_API_URL: &str = "https://api-testnet.ergoplatform.com/";

fn default_explorer_api_url(network_prefix: NetworkPrefix) -> Url {
    let url_str = match network_prefix {
        NetworkPrefix::Mainnet => MAINNET_EXPLORER_API_URL,
        NetworkPrefix::Testnet => TESTNET_EXPLORER_API_URL,
    };
    Url::parse(url_str).unwrap()
}

fn default_explorer_url(network_prefix: NetworkPrefix) -> Url {
    let url_str = match network_prefix {
        NetworkPrefix::Mainnet => MAINNET_EXPLORER_URL,
        NetworkPrefix::Testnet => TESTNET_EXPLORER_URL,
    };
    Url::parse(url_str).unwrap()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Endpoints {
    pub node_url: Url,
    pub explorer_url: Url
}

impl Default for Endpoints {
    fn default() -> Self {
        let network: NetworkPrefix = NetworkPrefix::Testnet; // TODO make as parameter.
        let explorer_url: Url = default_explorer_api_url(network);
        Endpoints {
            node_url: Url::parse(LOCAL_NODE_URL).unwrap(),
            explorer_url: explorer_url.clone()
        }
    }
}