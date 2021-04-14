use reqwest::{multipart::Form, Client, Response};
use serde::{de::DeserializeOwned, Serialize};

use crate::{requests::ResponseResult, RequestError};

use super::TelegramResponse;

pub async fn request_multipart<T>(
    client: &Client,
    token: &str,
    method_name: &str,
    params: Form,
) -> ResponseResult<T>
    where
        T: DeserializeOwned,
{
    let response = client
        .post(&super::method_url(token, method_name))
        .multipart(params)
        .send()
        .await
        .map_err(RequestError::NetworkError)?;

    process_response(response).await
}

pub async fn request_json<T, P>(
    client: &Client,
    token: &str,
    method_name: &str,
    params: &P,
) -> ResponseResult<T>
    where
        T: DeserializeOwned + Serialize + std::fmt::Debug + Clone,
        P: Serialize,
{
    let response = client
        .post(&super::method_url(token, method_name))
        .json(params)
        .send()
        .await
        .map_err(RequestError::NetworkError)?;

    match process_response(response).await {
        Ok(v) => {
            // let req = serde_json::to_string(&params).unwrap();
            // let res = serde_json::to_string(&v).unwrap();
            // println!("edwin 43, {} req, {}", method_name, req);
            // println!("edwin 44, {} res, {}", method_name, res);
            Ok(v)
        }
        Err(err) => {
            Err(err)
        }
    }
}

async fn process_response<T>(response: Response) -> ResponseResult<T>
    where
        T: DeserializeOwned,
{
    let url = response.url().to_string();
    let s = &response.text().await.map_err(RequestError::NetworkError)?;

    match serde_json::from_str::<TelegramResponse<T>>(s) {
        Ok(v) => {
            // log::debug!("url={}, body={}", url, s);
            v.into() }
        Err(err) => {
            log::error!("url={}, body={}, err={}", url, s, err.to_string());
            Err(RequestError::InvalidJson(err))
        }
    }

    // let x = serde_json::from_str::<TelegramResponse<T>>(s)
    //     .map_err(RequestError::InvalidJson)?
    //     .into();
    // x
}
