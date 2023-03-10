use dotenvy::dotenv;
use std::env;

/// Returns the value of the environment variable for the Key specified in the first argument.
/// - If the value does not exist, an error is raised and the Key for which the value does not exist is reported by cargo.
///
/// ### Example:
/// ```
/// async fn main() {
///    println!("{}", get_env("HOSTNAME"))
/// }
/// ```
pub fn get_env(key: &str) -> String {
    dotenv().ok();
    env::var(key).unwrap_or_else(|_| panic!("Key:[{key}] value is not set."))
}

pub async fn create_shorten(target: String) -> String {
    let res = reqwest::get(format!(
        "https://is.gd/create.php?format=simple&url={}",
        target
    ))
    .await
    .unwrap()
    .text()
    .await
    .unwrap();
    return res;
}
