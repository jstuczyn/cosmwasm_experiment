use crate::serde_helpers::{de_i64_from_str, de_query_response_from_str};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(bound = "for<'a> T: Deserialize<'a>")]
pub struct SmartQueryResult<T>
where
    for<'a> T: Deserialize<'a>,
{
    #[serde(deserialize_with = "de_query_response_from_str")]
    pub smart: T,
}

#[derive(Deserialize, Debug)]
#[serde(bound = "for<'a> T: Deserialize<'a>")]
pub struct SmartQueryResponse<T>
where
    for<'a> T: Deserialize<'a>,
{
    #[serde(deserialize_with = "de_i64_from_str")]
    pub height: i64,
    pub result: SmartQueryResult<T>,
}

#[derive(Deserialize, Debug)]
pub struct SmartQueryError {
    pub error: String,
}

#[derive(Deserialize, Debug)]
#[serde(bound = "for<'a> T: Deserialize<'a>")]
#[serde(untagged)]
pub enum QueryResponse<T>
where
    for<'a> T: Deserialize<'a>,
{
    Ok(SmartQueryResponse<T>),
    Error(SmartQueryError),
}
