use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use reqwest;
use reqwest::Error as RequestError;

use crate::common;

const FABRIC_VERSION: &str = "0.11.2";

fn get_fabric_binary_path() -> String{
    return common::tmp() + format!("\\fabric-installer-{}.jar", FABRIC_VERSION).as_str();
}

async fn get_fabric_binary_bytes() -> Result<Vec<u8>, RequestError> {
    let url = format!("https://maven.fabricmc.net/net/fabricmc/fabric-installer/{v}/fabric-installer-{v}.jar", v=FABRIC_VERSION);

    let client = reqwest::Client::new();

    let resp = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.")
        .header("Accept-Language", "en-GB,en;q=0.6")
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
        .send().await?;

    let content_length = resp.content_length().unwrap_or(2137).try_into().unwrap();
    let body = resp.bytes().await?;

    println!("Reqwest B:{} CL:{}", body.len(), content_length);

    let mut content = vec![0 as u8; content_length];

    content.copy_from_slice(&body[0..content_length]);

    Ok(content)
}

fn save_fabric_binary_bytes(bytes: &[u8]) -> std::io::Result<()> {
    let filepath = get_fabric_binary_path();

    let mut file = File::create(filepath)?;
    file.write_all(bytes)?;

    Ok(())
}

fn run_fabric_binary() {
    let filepath = get_fabric_binary_path();

    let cmd = format!("java -jar {}", filepath);
    println!("CMD: {}", cmd);
    let _ = Command::new("javaw").args(["-jar", &filepath]).output().expect("Some err with cmd");
}

pub async fn download_and_run_fabric_binary() -> Result<(), String> {
    let filepath = get_fabric_binary_path();
    let path = Path::new(&filepath);
    
    if !path.exists() {
        let bytes_res = get_fabric_binary_bytes().await;
        if bytes_res.is_err() {
            return Err(format!("Something went wrong with request, {}", bytes_res.err().unwrap()));
        }
        let _ = save_fabric_binary_bytes(bytes_res.unwrap().as_ref());
    }

    run_fabric_binary();

    Ok(())
}