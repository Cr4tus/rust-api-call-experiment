# API Calls Exeperiments using Rust

## What does the app do ?
Once it is running, the programm makes a request to "https://api.github.com/repos/owner/repo/stargazers" (replace **owner** within the url with: **rust-lang-nursery** and **repo** with: **rust-cookbook**) and displays the body of the response.

## Programming Languages:
- Rust

## Frameworks & Libraries:
- serde - https://serde.rs
- serde-json - https://github.com/serde-rs/json | https://docs.rs/serde_json/latest/serde_json/
- tokio - https://tokio.rs
- reqwest - https://docs.rs/reqwest/latest/reqwest/

## Implementation:
The main function uses tokio annotation to make it **async** and returns a core::Result:
```rust
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
```
First of all, the url is being constructed, then the **reqwest** client which will make the **GET** request and provide the response that will be unwrapped. After these steps, the response's body will be deserialized as a vector of:
```rust
#[derive(serde::Deserialize, Debug)]
struct UserLoginData {
    id: u32,
    login: String,
}
```
This struct has been annotated using serde's annotations that are only available once we specify the features within **Cargo.toml** like this:
```toml
serde = { version = "1.0", features = ["derive"] }
```
The final step is to display the data.