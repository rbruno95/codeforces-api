use {
    chrono::{serde::ts_seconds, DateTime, Utc},
    serde::Deserialize,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub handle: String,
    pub email: Option<String>,
    pub vk_id: Option<String>,
    pub open_id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub organization: Option<String>,
    pub contribution: i32,
    pub rank: String,
    pub rating: usize,
    pub max_rank: String,
    pub max_rating: usize,
    #[serde(with = "ts_seconds")]
    pub last_online_time_seconds: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub registration_time_seconds: DateTime<Utc>,
    pub friend_of_count: usize,
    pub avatar: String,
    pub title_photo: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialization() {
        let json = r#"{
                "lastName": "Qi",
                "country": "United States",
                "lastOnlineTimeSeconds": 1694923813,
                "city": "Princeton",
                "rating": 3726,
                "friendOfCount": 13656,
                "titlePhoto": "https://userpic.codeforces.org/312472/title/7cf0a442d4071e87.jpg",
                "handle": "Benq",
                "avatar": "https://userpic.codeforces.org/312472/avatar/5716ac69aea8159a.jpg",
                "firstName": "Benjamin",
                "contribution": 44,
                "organization": "MIT",
                "rank": "legendary grandmaster",
                "maxRating": 3813,
                "registrationTimeSeconds": 1435099979,
                "maxRank": "legendary grandmaster"
            }"#;

        dbg!(serde_json::from_str::<User>(json).unwrap());
    }
}
