mod users;

pub struct CodeforcesClient {
    http_client: reqwest::blocking::Client,
}

const BASE_URL: &'static str = "https://codeforces.com/api/";

impl CodeforcesClient {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::blocking::Client::new(),
        }
    }

    // Return a more general error to be able to use question mark for serde_json::Error as well
    pub(crate) fn get<T: for<'d> serde::Deserialize<'d>>(
        &self,
        url: String,
    ) -> reqwest::Result<ClientResponse<T>> {
        let query = self.http_client.get(format!("{BASE_URL}{url}")).build()?;
        let response = self.http_client.execute(query);
        let data = response?.text()?;

        Ok(serde_json::from_str::<ClientResponse<T>>(&data).unwrap())
    }
}

#[derive(serde::Deserialize, Debug)]
pub enum Status {
    OK,
    FAILED,
}

#[derive(serde::Deserialize, Debug)]
pub struct ClientResponse<T> {
    status: Status,
    result: Option<Vec<T>>,
    comment: Option<String>,
}
