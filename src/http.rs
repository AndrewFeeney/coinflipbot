async fn get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;

    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock};

    #[tokio::test]
    async fn test_get() {
        let mock_response = "Hello, world!";
        let mock_server = mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body(mock_response)
            .create();

        let response = get(&format!("{}/test", mockito::server_url())).await.unwrap();
        assert_eq!(response, mock_response);

        mock_server.assert();
    }
}
