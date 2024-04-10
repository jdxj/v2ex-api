use reqwest::{header::HeaderMap, ClientBuilder};
use std::error::Error;
use serde::{Serialize, Deserialize};

/// API 域名前缀。
pub const V2EX_API_DOMAIN: &str = "https://www.v2ex.com/api/v2";

#[derive(Debug)]
pub struct Client {
    req_client: reqwest::Client,
}

impl Client {
    pub fn new(token: &str) -> Client {
        let mut bearer = String::from("Bearer ");
        bearer.push_str(token);

        let mut hm = HeaderMap::new();
        hm.append("Authorization", bearer.parse().unwrap());

        let cb = ClientBuilder::new();
        Client {
            req_client: cb.default_headers(hm).build().unwrap(),
        }
    }

    pub async fn get_notifications(&self, req: &GetNotificationsReq) -> Result<GetNotificationsRsp, Box<dyn Error>> {
        let mut page = req.page;
        if page <= 0 {
            page = 1
        }

        let url = format!("{}{}", V2EX_API_DOMAIN, "/notifications");
        let req = self.req_client.get(url)
            .query(&[("p", page)])
            .build()?;

        // println!("url: {:?}", req.url().to_string());

        let bytes = self.req_client.execute(req).await?.bytes().await?;
        let body = serde_json::from_slice(&bytes)?;
        Ok(body)
    }

    pub async fn delete_notifications(&self, req: &DeleteNotificationsReq) -> Result<DeleteNotificationsRsp, Box<dyn Error>> {
        let url = format!("{}{}{}", V2EX_API_DOMAIN, "/notifications/", req.notification_id);
        let req = self.req_client.delete(url)
            .build()?;

        println!("url: {:?}", req.url().to_string());

        let bytes = self.req_client.execute(req).await?.bytes().await?;

        let body = serde_json::from_slice(&bytes)?;
        Ok(body)
    }
}


pub struct DeleteNotificationsReq {
    pub notification_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteNotificationsRsp {
    pub success: bool,
    pub message: String,
}

pub struct GetNotificationsReq {
    pub page: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNotificationsRsp {
    pub success: bool,
    pub message: String,
    // todo: 实现
    // pub result: Vec<()>,
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }

    #[tokio::test]
    async fn get() -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::get("https://httpbin.org/ip")
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        println!("{resp:#?}");
        Ok(())
    }

    fn new() -> Client {
        let current_dir = std::env::current_dir().unwrap();
        println!("{:?}", current_dir);

        let token = fs::read_to_string("token.txt").unwrap();
        Client::new(token.as_str())
    }

    #[tokio::test]
    async fn get_notifications() {
        let c = new();
        match c.get_notifications(&GetNotificationsReq { page: 1 }).await {
            Ok(body) => {
                println!("{:?}", body)
            }
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    }

    #[tokio::test]
    async fn delete_notifications() {
        let c = new();
        match c.delete_notifications(&DeleteNotificationsReq { notification_id: 1 }).await {
            Ok(body) => {
                println!("{:?}", body)
            }
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    }
}
