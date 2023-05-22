extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate tokio;

#[derive(serde::Deserialize, Debug)]
struct UserLoginData {
    id: u32,
    login: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );

    let response = reqwest::Client::new()
        .get(&url)
        .header(reqwest::header::USER_AGENT, "rust web-api-client demo")
        .send()
        .await?;

    let users: Vec<UserLoginData> = response.json().await?;

    println!("{:#?}", users);

    Ok(())
}
