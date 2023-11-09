use serde::Deserialize;
use starknet::providers::{jsonrpc::HttpTransport, JsonRpcClient};

use crate::config::Config;

pub_struct!(;AppState {
    conf: Config,
    provider: JsonRpcClient<HttpTransport>,
});

pub_struct!(Deserialize; ResolveQuery {
    domain: String,
});

pub_struct!(Debug, Deserialize; ApiResponse {
    results: Vec<ResultItem>,
});

pub_struct!(Debug, Deserialize; ResultItem {
    properties: Properties,
});

pub_struct!(Debug, Deserialize; Properties {
    Address: Address,
});

pub_struct!(Debug, Deserialize; Address {
    rich_text: Vec<RichText>,
});

pub_struct!(Debug, Deserialize; RichText {
    plain_text: String,
});
