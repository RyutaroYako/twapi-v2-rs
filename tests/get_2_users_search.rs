use anyhow::Result;
use twapi_v2::api::{execute_twitter, get_2_users_search, BearerAuthentication};

// BEARER_CODE=XXXXX cargo test test_get_2_users_search --all-features -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_users_search() -> Result<()> {
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuthentication::new(bearer_code);
    let builder = get_2_users_search::Api::open("東京").build(&bearer_auth);
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_users_search::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
