use reqwest::{header::HeaderMap, ClientBuilder};
use std::error::Error;
use serde::{Serialize, Deserialize};

/// API 域名前缀.
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

    /// 获取最新的提醒.
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

    /// 删除指定的提醒.
    pub async fn delete_notifications(&self, req: &DeleteNotificationsReq) -> Result<DeleteNotificationsRsp, Box<dyn Error>> {
        let url = format!("{}{}{}", V2EX_API_DOMAIN, "/notifications/", req.notification_id);
        let req = self.req_client.delete(url)
            .build()?;

        // println!("url: {:?}", req.url().to_string());

        let bytes = self.req_client.execute(req).await?.bytes().await?;

        let body = serde_json::from_slice(&bytes)?;
        Ok(body)
    }

    /// 获取自己的 Profile.
    pub async fn get_member(&self) -> Result<GetMemberRsp, Box<dyn Error>> {
        let url = format!("{}{}", V2EX_API_DOMAIN, "/member");
        let req = self.req_client.get(url).build()?;

        // println!("url: {:?}", req.url().to_string());

        let bytes = self.req_client.execute(req).await?.bytes().await?;

        let body = serde_json::from_slice(&bytes)?;
        Ok(body)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMemberRsp {
    pub success: bool,
    pub result: Member,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    pub id: u32,
    pub username: String,
    pub url: String,
    pub website: Option<String>,
    pub twitter: Option<String>,
    pub psn: Option<String>,
    pub github: Option<String>,
    pub btc: Option<String>,
    pub location: Option<String>,
    pub tagline: Option<String>,
    pub bio: Option<String>,
    pub avatar_mini: String,
    pub avatar_normal: String,
    pub avatar_large: String,
    pub created: i64,
    pub last_modified: i64,
}

pub struct DeleteNotificationsReq {
    pub notification_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteNotificationsRsp {
    #[serde(flatten)]
    pub status: Status,
}

pub struct GetNotificationsReq {
    pub page: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNotificationsRsp {
    #[serde(flatten)]
    pub status: Status,
    // todo: 实现
    // pub result: Vec<()>,
}

/// 请求结果状态.
#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub success: bool,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

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

    #[tokio::test]
    async fn get_member() {
        let c = new();
        match c.get_member().await {
            Ok(body) => {
                println!("{:?}", body)
            }
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    }
}
