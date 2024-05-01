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
    Address: RichText,
});

pub_struct!(Debug, Deserialize; RichText {
    rich_text: Vec<PlainText>,
});

pub_struct!(Debug, Deserialize; PlainText {
    plain_text: String,
});
