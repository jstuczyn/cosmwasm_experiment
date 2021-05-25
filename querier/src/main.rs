use crate::models::QueryResponse;
use common::{PagedResponse, QueryMsg, SomeData};
use serde::Deserialize;

pub mod models;
pub mod serde_helpers;

struct Client {
    reqwest_client: reqwest::Client,
    validator_url: String,
    contract_address: String,
}

impl Client {
    fn base_query_path(&self) -> String {
        format!(
            "{}/wasm/contract/{}/smart",
            self.validator_url, self.contract_address
        )
    }

    async fn query_validator<T>(&self, query: String) -> T
    where
        for<'a> T: Deserialize<'a>,
    {
        let query_url = format!("{}/{}?encoding=base64", self.base_query_path(), query);

        let query_response: QueryResponse<T> = self
            .reqwest_client
            .get(query_url)
            .send()
            .await
            .expect("error handling here")
            .json()
            .await
            .expect("error handling here");

        match query_response {
            QueryResponse::Ok(smart_res) => smart_res.result.smart,
            QueryResponse::Error(err) => panic!("{:?}", err),
        }
    }

    async fn get_data_paged(&self, start_after: Option<String>) -> PagedResponse {
        let query_content_json =
            serde_json::to_string(&QueryMsg::GrabData { start_after }).unwrap();

        let query_content = base64::encode(query_content_json);

        self.query_validator(query_content).await
    }

    pub async fn get_data(&self) -> Vec<SomeData> {
        let mut data = Vec::new();
        let mut start_after = None;
        loop {
            let mut paged_response = self.get_data_paged(start_after.take()).await;
            data.append(&mut paged_response.data);

            if let Some(start_after_res) = paged_response.start_next_after {
                start_after = Some(start_after_res)
            } else {
                break;
            }
        }

        data
    }
}

#[tokio::main]
async fn main() {
    let reqwest_client = reqwest::Client::new();
    let client = Client {
        reqwest_client,
        validator_url: "http://localhost:1317".to_string(),
        contract_address: "hal13n6gf4ctcljuklthyx8cpm9fx9a9educqzfdpr".to_string(),
    };

    let mut i = 0;
    loop {
        let data = client.get_data().await;
        println!("{}, Got {} pieces of data", i, data.len());
        i += 1;
    }
}
