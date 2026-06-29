//! Smoke test for Garage (S3-compatible object storage).
//!
//! Make sure Garage is running and its layout is applied (see ../../GARAGE.md),
//! the bucket exists, and backend/.env has the S3_* and AWS_* values filled in.
//! Then run:
//!
//!     cargo run --example garage_smoke
//!
//! It connects, uploads a small file, lists the bucket, and downloads the file
//! back. This is the in-project version of the standalone Garage test — proof
//! that the backend can talk to storage before we wire it into the app proper.

use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Region;
use aws_sdk_s3::primitives::ByteStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load backend/.env so S3_* and AWS_* are available, just like the app does.
    dotenvy::dotenv().ok();

    let endpoint = std::env::var("S3_ENDPOINT").unwrap_or_else(|_| "http://localhost:3900".into());
    let region = std::env::var("S3_REGION").unwrap_or_else(|_| "garage".into());
    let bucket = std::env::var("S3_BUCKET").unwrap_or_else(|_| "test-bucket".into());

    // AWS_ACCESS_KEY_ID / AWS_SECRET_ACCESS_KEY are read from the environment
    // automatically by the default config loader.
    let base = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(region))
        .endpoint_url(endpoint.clone())
        .load()
        .await;

    // force_path_style is required for Garage — it does not support
    // virtual-hosted-style addressing (bucket.host), only path-style (host/bucket).
    let s3_config = aws_sdk_s3::config::Builder::from(&base)
        .force_path_style(true)
        .build();
    let client = Client::from_conf(s3_config);

    println!("Connecting to {endpoint}, bucket '{bucket}'");

    // 1) Upload a file
    client
        .put_object()
        .bucket(bucket.clone())
        .key("rust-test.txt")
        .body(ByteStream::from_static(b"hei fra rust"))
        .send()
        .await?;
    println!("Uploaded rust-test.txt");

    // 2) List the bucket contents
    let listed = client.list_objects_v2().bucket(bucket.clone()).send().await?;
    println!("Objects in '{bucket}':");
    for obj in listed.contents() {
        println!(
            "  {} ({} bytes)",
            obj.key().unwrap_or("?"),
            obj.size().unwrap_or(0)
        );
    }

    // 3) Download it again and print the contents
    let got = client
        .get_object()
        .bucket(bucket.clone())
        .key("rust-test.txt")
        .send()
        .await?;
    let bytes = got.body.collect().await?.into_bytes();
    println!("Downloaded: {}", String::from_utf8_lossy(&bytes));

    Ok(())
}
