use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::process;

#[derive(Parser)]
#[clap(about, version, author)]
struct TaktCtl {
    /// API Key
    #[clap(long = "api-key")]
    api_key: String,

    /// API Key Secret
    #[clap(long = "api-key-secret")]
    api_key_secret: String,

    /// ID of the Organization
    #[clap(short = 'o', long = "organization-id")]
    organization_id: String,

    /// API Endpoint to use for Takt
    #[clap(
        short = 'e',
        long = "endpoint",
        default_value = "https://api.alpha.takt.io"
    )]
    endpoint: String,

    #[clap(subcommand)]
    command: TaktCtlCommands,
}

#[derive(Subcommand)]
enum TaktCtlCommands {
    /// Upload a new file to a Takt Feed
    Upload {
        /// ID of the Takt Feed
        #[clap(short = 'f', long = "feed-id")]
        feed_id: String,

        // Patht to the File
        #[clap(parse(from_os_str))]
        path: std::path::PathBuf,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct UploadResult {
    organization_id: String,
    feed_id: String,
    feed_import_id: String,
}

fn main() {
    let args = TaktCtl::parse();

    match &args.command {
        TaktCtlCommands::Upload { feed_id, path } => {
            upload_command(&args, feed_id, path);
        }
    }
}

fn upload_command(args: &TaktCtl, feed_id: &std::string::String, path: &std::path::PathBuf) {
    println!("Uploading file via Takt API...");

    let url = format!(
        "{}/v1/organizations/{organization_id}/feeds/{feed_id}/upload",
        args.endpoint,
        organization_id = args.organization_id,
        feed_id = feed_id
    );

    let api_key = args.api_key.clone();
    let api_key_secret = args.api_key_secret.clone();

    let form = reqwest::blocking::multipart::Form::new()
        .text("apiKey", api_key)
        .text("apiKeySecret", api_key_secret)
        .file("file", path)
        .unwrap();

    let response = reqwest::blocking::Client::new()
        .post(&url)
        .multipart(form)
        .send()
        .unwrap();

    if response.status().is_success() {
        let result: UploadResult = response.json().unwrap();
        println!(
            "{} {}! {}",
            "The file has been uploaded as import".color("green"),
            &result.feed_import_id.color("green"),
            "The file should begin processing soon.".color("green")
        );
        process::exit(0);
    } else if response.status() == reqwest::StatusCode::PAYLOAD_TOO_LARGE {
        println!(
            "{}",
            "The file you are attempting to upload is too large.".color("red")
        );
        process::exit(1);
    } else if response.status().is_server_error() {
        println!(
            "{}",
            "There was an unknown error on the server!".color("red"),
        );
        process::exit(1);
    } else {
        println!(
            "{}",
            "There was an error while uploading the file!".color("red"),
        );
        process::exit(1);
    }
}
