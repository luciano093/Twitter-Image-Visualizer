use crate::error::Error;
use super::Error as TwitterApiError;

pub async fn fetch_guest_token(client: &reqwest::Client) -> Result<String, Error> {
    let url = "https://api.twitter.com/1.1/guest/activate.json";

    let response = client.post(url)
        .header("authorization", "Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA")
        .send()
        .await?;

    let response = response.bytes().await?;

    let json: serde_json::Value = serde_json::from_slice(&response)?;

    let string = json["guest_token"].to_string();

    // remove trailing '"'
    let mut chars = string.chars();
    chars.next();
    chars.next_back();
    Ok(chars.collect())
}

/// Returns `Ok(None)` if the response is successful but the server send a blank message
pub async fn fetch_rest_id(client: &reqwest::Client, guest_token: &str, user: &str) -> Result<String, Error> {
    let url = format!("https://twitter.com/i/api/graphql/9zwVLJ48lmVUk8u_Gh9DmA/ProfileSpotlightsQuery?variables=%7B%22screen_name%22%3A%22{}%22%7D", user);

    let response = client.get(url)
        .header("authorization", "Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA")
        .header("x-guest-token", guest_token)
        .send().await?;

    let response = response.bytes().await?;

    if response.len() == 11 {
        return Err(Error::TwitterApi(TwitterApiError::NoUserFound));
    }

    let json: serde_json::Value = serde_json::from_slice(&response)?;

    if let Some(error) = json.pointer("/errors/0/message") {
        if error == "Bad guest token" {
            return Err(Error::TwitterApi(TwitterApiError::BadToken));
        }
    }

    let result_pointer = json.pointer("/data/user_result_by_screen_name/result").ok_or_else(|| {
        json.pointer("/data/user/result").unwrap()
    }).unwrap();

    if result_pointer.pointer("/__typename").unwrap().to_string() == "\"UserUnavailable\"" {
        return Err(Error::TwitterApi(TwitterApiError::UserUnavailable));
    }

    let string = result_pointer.pointer("/rest_id").unwrap().to_string();
    let mut chars = string.chars();
    chars.next();
    chars.next_back();

    Ok(chars.collect())
}

pub async fn get_media(client: &reqwest::Client, guest_token: &str, rest_id: &str, count: usize, cursor: Option<String>) -> Result<serde_json::Value, Error> {
    let url = if let Some(cursor) = cursor {
        let cursor = urlencoding::encode(&cursor);
        format!("https://twitter.com/i/api/graphql/d_ONZLUHGCsErBCriRsLXg/UserMedia?variables=%7B%22userId%22%3A%22{}%22%2C%22count%22%3A{}%2C%22cursor%22%3A{}%2C%22includePromotedContent%22%3Afalse%2C%22withQuickPromoteEligibilityTweetFields%22%3Afalse%2C%22withDownvotePerspective%22%3Afalse%2C%22withReactionsMetadata%22%3Afalse%2C%22withReactionsPerspective%22%3Afalse%2C%22withVoice%22%3Afalse%2C%22withV2Timeline%22%3Afalse%7D&features=%7B%22blue_business_profile_image_shape_enabled%22%3Afalse%2C%22responsive_web_graphql_exclude_directive_enabled%22%3Afalse%2C%22verified_phone_label_enabled%22%3Afalse%2C%22responsive_web_graphql_timeline_navigation_enabled%22%3Afalse%2C%22responsive_web_graphql_skip_user_profile_image_extensions_enabled%22%3Atrue%2C%22tweetypie_unmention_optimization_enabled%22%3Atrue%2C%22vibe_api_enabled%22%3Afalse%2C%22responsive_web_edit_tweet_api_enabled%22%3Afalse%2C%22graphql_is_translatable_rweb_tweet_is_translatable_enabled%22%3Afalse%2C%22view_counts_everywhere_api_enabled%22%3Afalse%2C%22longform_notetweets_consumption_enabled%22%3Atrue%2C%22tweet_awards_web_tipping_enabled%22%3Afalse%2C%22freedom_of_speech_not_reach_fetch_enabled%22%3Afalse%2C%22standardized_nudges_misinfo%22%3Afalse%2C%22tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled%22%3Afalse%2C%22interactive_text_enabled%22%3Afalse%2C%22responsive_web_text_conversations_enabled%22%3Afalse%2C%22longform_notetweets_richtext_consumption_enabled%22%3Afalse%2C%22responsive_web_enhance_cards_enabled%22%3Afalse%7D", rest_id, count, cursor)
    } else {
        format!("https://twitter.com/i/api/graphql/d_ONZLUHGCsErBCriRsLXg/UserMedia?variables=%7B%22userId%22%3A%22{}%22%2C%22count%22%3A{}%2C%22includePromotedContent%22%3Afalse%2C%22withQuickPromoteEligibilityTweetFields%22%3Afalse%2C%22withDownvotePerspective%22%3Afalse%2C%22withReactionsMetadata%22%3Afalse%2C%22withReactionsPerspective%22%3Afalse%2C%22withVoice%22%3Afalse%2C%22withV2Timeline%22%3Afalse%7D&features=%7B%22blue_business_profile_image_shape_enabled%22%3Afalse%2C%22responsive_web_graphql_exclude_directive_enabled%22%3Afalse%2C%22verified_phone_label_enabled%22%3Afalse%2C%22responsive_web_graphql_timeline_navigation_enabled%22%3Afalse%2C%22responsive_web_graphql_skip_user_profile_image_extensions_enabled%22%3Atrue%2C%22tweetypie_unmention_optimization_enabled%22%3Atrue%2C%22vibe_api_enabled%22%3Afalse%2C%22responsive_web_edit_tweet_api_enabled%22%3Afalse%2C%22graphql_is_translatable_rweb_tweet_is_translatable_enabled%22%3Afalse%2C%22view_counts_everywhere_api_enabled%22%3Afalse%2C%22longform_notetweets_consumption_enabled%22%3Atrue%2C%22tweet_awards_web_tipping_enabled%22%3Afalse%2C%22freedom_of_speech_not_reach_fetch_enabled%22%3Afalse%2C%22standardized_nudges_misinfo%22%3Afalse%2C%22tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled%22%3Afalse%2C%22interactive_text_enabled%22%3Afalse%2C%22responsive_web_text_conversations_enabled%22%3Afalse%2C%22longform_notetweets_richtext_consumption_enabled%22%3Afalse%2C%22responsive_web_enhance_cards_enabled%22%3Afalse%7D", rest_id, count)
    };

    let response = client.get(url)
        .header("authorization", "Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA")
        .header("x-guest-token", guest_token)
        .send()
        .await?;
    
    let response = response.bytes().await?;
    
    let json: serde_json::Value = serde_json::from_slice(&response)?;

    Ok(json.pointer("/data/user/result/timeline/timeline/instructions/0/entries").expect(&json.to_string()).to_owned())
}