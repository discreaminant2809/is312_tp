use serde_json::json;

#[tokio::test]
async fn api_signup_login() -> anyhow::Result<()> {
    let client = httpc_test::new_client("http://127.0.0.1:3000")?;

    client
        .do_post(
            "/api/signup",
            json!({
                "username": "123",
                "pwd": "123",
            }),
        )
        .await?
        .print()
        .await?;

    client
        .do_post(
            "/api/login",
            json!({
                "username": "123",
                "pwd": "123",
            }),
        )
        .await?
        .print()
        .await?;

    Ok(())
}
