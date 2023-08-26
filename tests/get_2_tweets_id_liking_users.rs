use anyhow::Result;
use twapi_v2::api::{execute_twitter, get_2_tweets_id_liking_users, BearerAuth};

// BEARER_CODE=XXXXX cargo test test_get_2_tweets_id_liking_users -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_tweets_id_liking_users() -> Result<()> {
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let bearer_auth = BearerAuth::new(bearer_code);
    let builder =
        get_2_tweets_id_liking_users::Api::open("1617696866413719556").build(&bearer_auth);
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_tweets_id_liking_users::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);

    /*
    let builder = get_2_tweets_id_liking_users::Api::open("1617696866413719556")
        .pagination_token(&response.meta.unwrap().next_token.unwrap())
        .build(&bearer_auth);
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("\n{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<get_2_tweets_id_liking_users::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
     */

    Ok(())
}
