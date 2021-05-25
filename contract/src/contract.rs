use common::{PagedResponse, QueryMsg, SomeData};
use cosmwasm_std::{
    to_binary, Deps, DepsMut, Env, HandleResponse, InitResponse, MessageInfo, Order, QueryResponse,
    StdError, StdResult, Storage,
};
use cosmwasm_storage::{bucket, bucket_read, Bucket};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const DATA_PREFIX: &[u8] = b"somedata";

pub fn get_data_bucket(storage: &mut dyn Storage) -> Bucket<SomeData> {
    bucket(storage, DATA_PREFIX)
}

pub fn store_data(storage: &mut dyn Storage, key: &[u8], data: &SomeData) -> StdResult<()> {
    let mut bucket = get_data_bucket(storage);
    bucket.save(key, data)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {}

// put some data into the bucket on init
pub fn init(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InitMsg,
) -> Result<InitResponse, ContractError> {
    for i in 0..1000 {
        let key = format!("datakey{}", i).to_string();
        let data = SomeData {
            id: i,
            field1: "somefield1".to_string(),
            field2: "somefield2".to_string(),
            field3: "somefieldashumanaddr".into(),
            field4: "somefield4".to_string(),
            field5: "somefield5".to_string(),
            key_field: key.clone(),
        };
        store_data(deps.storage, key.as_bytes(), &data)?;
    }

    Ok(InitResponse::default())
}

const PAGE_LIMIT: usize = 100;

pub fn query_data_paged(deps: Deps, start_after: Option<String>) -> StdResult<PagedResponse> {
    let start = calculate_start_value(start_after);

    let data = bucket_read(deps.storage, DATA_PREFIX)
        .range(start.as_deref(), None, Order::Ascending)
        .take(PAGE_LIMIT)
        .map(|res| res.map(|item| item.1))
        .collect::<StdResult<Vec<SomeData>>>()?;

    let start_next_after = data.last().map(|data| data.key_field.clone());

    Ok(PagedResponse {
        data,
        per_page: PAGE_LIMIT,
        start_next_after,
    })
}

/// Adds a 0 byte to terminate the `start_after` value given. This allows CosmWasm
/// to get the succeeding key as the start of the next page.
fn calculate_start_value(start_after: Option<String>) -> Option<Vec<u8>> {
    start_after.as_ref().map(|addr| {
        let mut bytes = addr.as_bytes().to_owned();
        bytes.push(0);
        bytes
    })
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    let query_res = match msg {
        QueryMsg::GrabData { start_after } => to_binary(&query_data_paged(deps, start_after)?),
    };

    Ok(query_res?)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct HandleMsg {}

// irrelevant for the purpose
pub fn handle(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: HandleMsg,
) -> Result<HandleResponse, ContractError> {
    Ok(Default::default())
}

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
}
