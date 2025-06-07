use std::{fs::File, io::Write, process::Command, time::{SystemTime, UNIX_EPOCH}};

use crate::common;

use reqwest;
use reqwest::Error as RequestError;
use scraper::{Html, Selector};

pub async fn fetch_mc_versions() -> Result<Vec<String>, RequestError> {
    let body = reqwest::get("https://files.minecraftforge.net/net/minecraftforge/forge/index_1.19.4.html").await?.text().await?;

    let mut out = vec![];
    
    let document = Html::parse_document(&body);
    let selector = Selector::parse(".nav-collapsible > li").unwrap();
    for version in document.select(&selector) {
        for txt in version.text() {
            if !txt.trim().is_empty() {
                out.push(String::from(txt.trim()));
            }
        }
    }
    
    Ok(out)
}

pub async fn fetch_forge_versions(mc_version: String) -> Result<Vec<String>, RequestError> {
    let url = format!("https://files.minecraftforge.net/net/minecraftforge/forge/index_{}.html", mc_version);

    println!("FORGE URL {}", url);

    let body = reqwest::get(url).await?.text().await?;
    
    let mut out = vec![];

    let document = Html::parse_document(&body);
    let selector = Selector::parse(".download-version").unwrap();

    for version in document.select(&selector) {
        for txt in version.text() {
            if !txt.trim().is_empty() {
                println!("FORGE-V: {}", txt.trim());
                out.push(String::from(txt.trim()));
            }
        }
    }

    Ok(out)
}

async fn get_forge_binary_bytes(mc_version: &str, forge_version: &str) -> Result<Vec<u8>, RequestError> {
    let url = format!("https://maven.minecraftforge.net/net/minecraftforge/forge/{mcv}-{fv}/forge-{mcv}-{fv}-installer.jar", mcv=mc_version, fv=forge_version);
    
    println!("URL: {}", url);

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

fn get_forge_binary_path(mc_version: &str, forge_version: &str) -> String{
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    return common::tmp() + format!("\\forge-{mcv}-{fv}-installer-{t}.jar", mcv=mc_version, fv=forge_version, t=timestamp).as_str();
}

fn download_forge_binary(mc_version: &str, forge_version: &str, bytes: &[u8]) -> std::io::Result<()> {


    let filepath = get_forge_binary_path(mc_version, forge_version);
    println!("FP: {}", filepath);

    let mut file = File::create(filepath)?;
    file.write_all(bytes)?;

    Ok(())
}

fn run_forge_binary(mc_version: &str, forge_version: &str) {
    let filepath = get_forge_binary_path(mc_version, forge_version);

    let cmd = format!("java -jar {}", filepath);
    println!("CMD: {}", cmd);
    let _ = Command::new("javaw").args(["-jar", &filepath]).output().expect("Some err with cmd");
}

pub async fn download_and_run_forge_binary(mc_version: &str, forge_version: &str) -> Result<(), String> {
    let bytes_res = get_forge_binary_bytes(mc_version, forge_version).await;
    if bytes_res.is_err() {
        return Err(format!("Something went wrong with request, {}", bytes_res.err().unwrap()));
    }
    
    let download_res = download_forge_binary(mc_version, forge_version, bytes_res.ok().unwrap().as_ref());
    if download_res.is_err() {
        return Err(format!("Something went wrong with bytes save, {}", download_res.err().unwrap() ))
    }
    
    run_forge_binary(mc_version, forge_version);

    Ok(())
}