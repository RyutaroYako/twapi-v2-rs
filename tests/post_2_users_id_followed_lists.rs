use anyhow::Result;
use twapi_v2::api::{execute_twitter, post_2_users_id_followed_lists, BearerAuthentication};

// BEARER_CODE=XXXXX LIST_ID=XXXXX cargo test test_post_2_users_id_followed_lists -- --nocapture --test-threads=1

#[tokio::test]
async fn test_post_2_users_id_followed_lists() -> Result<()> {
    let list_id = match std::env::var("LIST_ID") {
        Ok(list_id) => list_id,
        _ => return Ok(()),
    };
    let body = post_2_users_id_followed_lists::Body { list_id };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuthentication::new(bearer_code);
    let builder =
        post_2_users_id_followed_lists::Api::new("1660518823991336966", body).build(&bearer_auth);
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<post_2_users_id_followed_lists::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
