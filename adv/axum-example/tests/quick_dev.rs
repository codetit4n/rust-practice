#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    //hc.do_get("/hello?name=lokesh").await?.print().await?;
    hc.do_get("/hello2/lohit").await?.print().await?;

    //hc.do_get("/src/main.rs").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username":"lokesh",
            "pwd":"pass"
        }),
    );

    req_login.await?.print().await?;

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
        "title": "Ticket New"
        }),
    );

    req_create_ticket.await?.print().await?;

    //hc.do_delete("/api/tickets/1").await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
