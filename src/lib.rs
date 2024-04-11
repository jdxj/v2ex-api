use reqwest::{header::HeaderMap, ClientBuilder};
use std::error::Error;
use std::collections::HashMap;
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

    /// 查看当前使用的令牌.
    pub async fn get_token(&self) -> Result<GetTokenRsp, Box<dyn Error>> {
        let url = format!("{}{}", V2EX_API_DOMAIN, "/token");
        let req = self.req_client.get(url).build()?;

        // println!("url: {:?}", req.url().to_string());

        let bytes = self.req_client.execute(req).await?.bytes().await?;
        let body = serde_json::from_slice(&bytes)?;
        Ok(body)
    }

    /// 创建新的令牌.
    /// 在系统中最多创建 10 个 Personal Access Token.
    pub async fn post_tokens(&self, req: &PostTokensReq) -> Result<PostTokensRsp, Box<dyn Error>> {
        let mut data = HashMap::new();
        data.insert("scope", req.scope.as_str());
        data.insert("expiration", req.expiration.as_str());

        let url = format!("{}{}", V2EX_API_DOMAIN, "/tokens");
        let req = self.req_client.post(url)
            .json(&data)
            .build()?;

        // println!("url: {:?}", req.url().to_string());

        let bytes = self.req_client.execute(req).await?.bytes().await?;
        let body = serde_json::from_slice(&bytes)?;
        Ok(body)
    }

    /// 获取指定节点.
    pub async fn get_nodes(&self, req: &GetNodesReq) -> Result<GetNodesRsp, Box<dyn Error>> {
        let url = format!("{}{}{}", V2EX_API_DOMAIN, "/nodes/", req.node_name);
        let req = self.req_client.get(url).build()?;

        // println!("url: {:?}", req.url().to_string());

        let bytes = self.req_client.execute(req).await?.bytes().await?;
        let body = serde_json::from_slice(&bytes)?;
        Ok(body)
    }

    /// 获取指定节点下的主题.
    pub async fn get_nodes_topics(&self, req: &GetNodesTopicsReq) -> Result<GetNodesTopicsRsp, Box<dyn Error>>{
        let mut page = req.page;
        if page <= 0 {
            page = 1
        }

        let url = format!("{}/nodes/{}/topics", V2EX_API_DOMAIN, req.node_name);
        let req = self.req_client.get(url)
            .query(&[("p", page)])
            .build()?;

        // println!("url: {:?}", req.url().to_string());

        let bytes = self.req_client.execute(req).await?.bytes().await?;
        let body = serde_json::from_slice(&bytes)?;
        Ok(body)
    }

    /// 获取指定主题.
    pub async fn get_topics(&self, req: &GetTopicsReq) -> Result<GetTopicsRsp, Box<dyn Error>> {
        let url = format!("{}/topics/{}", V2EX_API_DOMAIN, req.topic_id);
        let req = self.req_client.get(url).build()?;

        // println!("url: {:?}", req.url().to_string());

        let bytes = self.req_client.execute(req).await?.bytes().await?;
        let body = serde_json::from_slice(&bytes)?;
        Ok(body)
    }

    /// 获取指定主题下的回复.
    pub async fn get_topics_replies(&self, req: &GetTopicsRepliesReq) -> Result<GetTopicsRepliesRsp, Box<dyn Error>> {
        let mut page = req.page;
        if page <= 0 {
            page = 1
        }

        let url = format!("{}/topics/{}/replies", V2EX_API_DOMAIN, req.topic_id);
        let req = self.req_client.get(url)
            .query(&[("p", page)])
            .build()?;

        // println!("url: {:?}", req.url().to_string());

        let bytes = self.req_client.execute(req).await?.bytes().await?;
        let body = serde_json::from_slice(&bytes)?;
        Ok(body)
    }
}

pub struct GetTopicsRepliesReq {
    pub topic_id: u32,
    pub page: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTopicsRepliesRsp {
    #[serde(flatten)]
    pub status: Status,
    pub result: Vec<TopicReply>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopicReply {
    pub id: u32,
    pub content: String,
    pub content_rendered: String,
    pub created: i64,
    pub member: Member,
}

pub struct GetTopicsReq {
    pub topic_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTopicsRsp {
    #[serde(flatten)]
    pub status: Status,
    pub result: TopicDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopicDetails {
    #[serde(flatten)]
    pub details: NodeTopic,
    pub member: Member,
    pub node: NodeDetails,
    // todo: 没有数据, 无法定义
    // pub supplements: Vec<?>,
}

pub struct GetNodesTopicsReq {
    pub node_name: String,
    pub page: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNodesTopicsRsp {
    #[serde(flatten)]
    pub status: Status,
    pub result: Vec<NodeTopic>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeTopic {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub content_rendered: String,
    pub syntax: u8,
    pub url: String,
    pub replies: u32,
    pub last_reply_by: String,
    pub created: i64,
    pub last_modified: i64,
    pub last_touched: i64,
}

pub struct GetNodesReq {
    pub node_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNodesRsp {
    #[serde(flatten)]
    pub status: Status,
    pub result: NodeDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeDetails {
    pub id: u16,
    pub url: String,
    pub name: String,
    pub title: String,
    pub header: String,
    pub footer: String,
    pub avatar: String,
    pub topics: u32,
    pub created: i64,
    pub last_modified: i64,
}

pub struct PostTokensReq {
    pub scope: Scope,
    pub expiration: Expiration,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostTokensRsp {
    pub success: bool,
    pub result: Token,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub token: String,
}

pub enum Scope {
    Everything,
    /// 不能用于进一步创建新的 token.
    Regular,
}

impl Scope {
    fn as_str(&self) -> &str {
        match self {
            Scope::Everything => "everything",
            Scope::Regular => "regular"
        }
    }
}

pub enum Expiration {
    Day30,
    Day60,
    Day90,
    Day180,
}

impl Expiration {
    fn as_str(&self) -> &str {
        match self {
            Expiration::Day30 => "2592000",
            Expiration::Day60 => "5184000",
            Expiration::Day90 => "7776000",
            Expiration::Day180 => "15552000",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTokenRsp {
    #[serde(flatten)]
    pub status: Status,
    pub result: TokenDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenDetails {
    pub token: String,
    pub scope: String,
    pub expiration: i64,
    pub good_for_days: u8,
    pub total_used: u32,
    pub last_used: i64,
    pub created: i64,
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
    pub avatar: Option<String>,
    pub avatar_mini: Option<String>,
    pub avatar_normal: Option<String>,
    pub avatar_large: Option<String>,
    pub created: i64,
    pub last_modified: Option<i64>,
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
    // todo: 没有数据, 无法定义
    // pub result: Vec<?>,
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

    #[tokio::test]
    async fn get_token() {
        let c = new();
        match c.get_token().await {
            Ok(body) => {
                println!("{:?}", body)
            }
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    }

    #[tokio::test]
    async fn post_tokens() {
        let c = new();
        let req = PostTokensReq {
            scope: Scope::Regular,
            expiration: Expiration::Day30,
        };
        match c.post_tokens(&req).await {
            Ok(body) => {
                println!("{:?}", body)
            }
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    }

    #[tokio::test]
    async fn get_nodes() {
        let c = new();
        let req = GetNodesReq {
            node_name: "rust".to_string(),
        };
        match c.get_nodes(&req).await {
            Ok(body) => {
                println!("{:?}", body)
            }
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    }

    #[tokio::test]
    async fn get_nodes_topics() {
        let c = new();
        let req = GetNodesTopicsReq {
            node_name: "rust".to_string(),
            page: 1,
        };
        match c.get_nodes_topics(&req).await {
            Ok(body) => {
                println!("{:?}", body)
            }
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    }

    #[tokio::test]
    async fn get_topics() {
        let c = new();
        let req = GetTopicsReq {
            topic_id: 1029068,
        };
        match c.get_topics(&req).await {
            Ok(body) => {
                println!("{:?}", body)
            }
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    }

    #[tokio::test]
    async fn get_topics_replies() {
        let c = new();
        let req = GetTopicsRepliesReq {
            topic_id: 1029068,
            page: 1,
        };
        match c.get_topics_replies(&req).await {
            Ok(body) => {
                println!("{:?}", body)
            }
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    }
}
