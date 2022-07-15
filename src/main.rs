use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Checking for GITHUB_ACTIONS...");
    let name = "GITHUB_ACTIONS";
    match env::var(name) {
        Ok(v) => println!("{}: {}", name, v),
        Err(e) => panic!("${} is not set ({})", name, e)
    }
    let ret_token = env::var("ACTIONS_ID_TOKEN_REQUEST_TOKEN");
    let req_url = env::var("ACTIONS_ID_TOKEN_REQUEST_URL");
    // if we don't have both, error
    print!("Checking for ACTIONS_ID_TOKEN_REQUEST_TOKEN / ACTIONS_ID_TOKEN_REQUEST_TOKEN...");
    if ret_token.is_err() || req_url.is_err() {
        panic!("ACTIONS_ID_TOKEN_REQUEST_TOKEN and ACTIONS_ID_TOKEN_REQUEST_URL must be set");
    } else {
        println!("ACTIONS_ID_TOKEN_REQUEST_TOKEN: {}", ret_token.as_ref().unwrap());
        println!("ACTIONS_ID_TOKEN_REQUEST_URL: {}", req_url.as_ref().unwrap());
    }

    let client = reqwest::blocking::Client::new();
    let response = client.get(&req_url.unwrap()).send()?;
    let token = response.text()?;
    println!("Got response: {}", token);
    Ok(())
}
