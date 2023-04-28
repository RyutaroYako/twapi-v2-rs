use anyhow::Result;
use twapi_v2::api::{execute_twitter, get_2_dm_conversations_dm_conversation_id_dm_events};

// BEARER_CODE=XXXXX DM_CONVERSATION_ID=XXXXX cargo test test_get_2_dm_conversations_dm_conversation_id_dm_events -- --nocapture --test-threads=1

#[tokio::test]
async fn test_get_2_dm_conversations_dm_conversation_id_dm_events() -> Result<()> {
    let dm_conversation_id = match std::env::var("DM_CONVERSATION_ID") {
        Ok(dm_conversation_id) => dm_conversation_id,
        _ => return Ok(()),
    };
    let bearer_code = std::env::var("BEARER_CODE").unwrap_or_default();
    let builder = get_2_dm_conversations_dm_conversation_id_dm_events::Api::all(
        &bearer_code,
        &dm_conversation_id,
    )
    .build();
    let (res, _rate_limit) = execute_twitter::<serde_json::Value>(builder).await?;
    println!("{}", serde_json::to_string(&res).unwrap());
    let response = serde_json::from_value::<
        get_2_dm_conversations_dm_conversation_id_dm_events::Response,
    >(res)?;
    assert_eq!(response.is_empty_extra(), true);
    Ok(())
}
