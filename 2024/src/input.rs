use reqwest::blocking::Client;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::Path;

pub fn load_input(year: usize, day: usize, token_file: Option<&str>) -> String {

    let token_path = token_file.unwrap_or(".aoctoken");
    let session = read_to_string(token_path)
        .expect("Failed to read session token")
        .trim()
        .to_string();

    let cache_dir = Path::new(".aocinput");
    if !cache_dir.exists() {
        create_dir_all(cache_dir).expect("Failed to create cache directory");
    }

    let cache_file = cache_dir.join(format!("{}-day{}.txt", year, day));
    if cache_file.exists()
    {
        read_to_string(cache_file).expect("Failed to read cached input")
    } else {
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
        let client = Client::new();
        let response = client
            .get(url)
            .header("Cookie", format!("session={}", session))
            .send()
            .expect("Failed to fetch input")
            .text()
            .expect("Failed to read response");

        write(&cache_file, &response).expect("Failed to cache input");
        response
    }
}

