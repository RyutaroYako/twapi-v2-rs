use anyhow::Result;
use twapi_v2::api::{execute_twitter, post_2_users_id_bookmarks};

// BEARER_CODE=XXXXX TWEET_ID=XXXXX cargo test test_post_2_users_id_bookmarks -- --nocapture --test-threads=1

#[tokio::test]
async fn test_post_2_users_id_bookmarks() -> Result<()> {
    let tweet_id = match std::env::var("TWEET_ID") {
        Ok(tweet_id) => tweet_id,
        _ => return Ok(()),
    };
    let body = post_2_users_id_bookmarks::Body { tweet_id };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let builder = post_2_users_id_bookmarks::Api::new(&bearer_code, "19522946", body).build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<post_2_users_id_bookmarks::Response>(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
