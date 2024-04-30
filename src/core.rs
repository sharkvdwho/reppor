use http::{header::{HeaderMap, USER_AGENT, AUTHORIZATION, ACCEPT}, HeaderValue};
use serde_json::value::Value;

const URL: &str = "https://api.github.com/orgs";
const H2K: &'static str = "X-GitHub-Api-Version";
const H2V: HeaderValue =  HeaderValue::from_static("2022-11-28");
const H3V: HeaderValue = HeaderValue::from_static("application/vnd.github+json");
const H4V: HeaderValue = HeaderValue::from_static("reppor");

pub async fn get_repos(orgnization: &str, token: &str) -> Result<(), reqwest::Error> 
{

    let mut headers = HeaderMap::new(); 
    let access_token = HeaderValue::from_str(&format!("Bearer {}", token)).expect("Invalid access token");
    headers.insert(AUTHORIZATION, access_token);
    headers.insert(H2K, H2V);
    headers.insert(ACCEPT, H3V);
    headers.insert(USER_AGENT, H4V);

    let clinet = reqwest::Client::new();
    let orgnization_repos = clinet.get(URL.to_string() + "/" + orgnization + "/repos").headers(headers).send().await?.json::<Value>().await?;   

    if let Some(repos) = orgnization_repos.as_array() {
        for repo in repos {
            if let Some(repo_name) = repo["name"].as_str() {
                println!("https://github.com/{}/{}", &orgnization ,repo_name);
            }
        }
    } else {
        println!("No repositories found");
    }

    Ok(())
}

