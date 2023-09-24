use codeforces_sdk::codeforces_client::CodeforcesClient;

fn main() {
    let codeforces_client = CodeforcesClient::new();
    // for i in 0..10 {
    let x = codeforces_client.user_info(&["Benq", "tourist", "pesteapata"]);
    println!("{:#?}", x);
    // println!("{i}")
    // }
}
