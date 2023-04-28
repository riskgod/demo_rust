use reqwest::Error;

// 定义一个异步函数，用于获取网页内容
async fn fetch_web_page(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let content = response.text().await?;
    Ok(content)
}

// 主函数
#[tokio::main]
async fn main() {
    let url = "https://www.baidu.com";

    match fetch_web_page(url).await {
        Ok(content) => println!("Fetched content: {}", content),
        Err(error) => eprintln!("Error fetching {}: {}", url, error),
    }
}
