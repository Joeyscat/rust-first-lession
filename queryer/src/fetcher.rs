use anyhow::{anyhow, Result};
use async_trait::async_trait;
use tokio::fs;
use tracing::info;

// Rust 的 async trait 还没有稳定，可以用 async_trait 宏
#[async_trait]
pub trait Fetch {
    type Error;
    async fn fetch(&self) -> Result<String, Self::Error>;
}

/// 从文件源或者 http 源中获取数据，组成 DataFrame
pub async fn retrieve_data(source: impl AsRef<str>) -> Result<String> {
    let name = source.as_ref();
    match &name[..4] {
        "http" => UrlFetcher(name).fetch().await,
        "file" => FileFetcher(name).fetch().await,
        _ => return Err(anyhow!("We only support http/https/file at the moment")),
    }
}

struct UrlFetcher<'a>(pub(crate) &'a str);
struct FileFetcher<'a>(pub(crate) &'a str);

#[async_trait]
impl<'a> Fetch for UrlFetcher<'a> {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error> {
        // Ok(reqwest::Client::builder()
        //     .proxy(reqwest::Proxy::all("http://172.27.64.1:7890")?)
        //     .build()?
        //     .get(self.0)
        //     .send()
        //     .await?
        //     .text()
        //     .await?)

        Ok(reqwest::get(self.0).await?.text().await?)
    }
}

#[async_trait]
impl<'a> Fetch for FileFetcher<'a> {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error> {
        info!("reading: {}", &self.0[7..]);
        Ok(fs::read_to_string(&self.0[7..]).await?)
    }
}
