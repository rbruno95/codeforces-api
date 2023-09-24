mod user;

use super::{ClientResponse, CodeforcesClient};

use user::User;

impl CodeforcesClient {
    // return the same result obtained by calling get and avoid the use of unwrap
    pub fn user_info(&self, handles: &[&str]) -> ClientResponse<User> {
        self.get::<User>(format!("user.info?handles={}", handles.join(";")))
            .unwrap()
    }
}
