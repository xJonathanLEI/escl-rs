use std::io::Write;

use escl::{Scanner, Url};

#[tokio::main]
async fn main() {
    let scanner_ip =
        std::env::var("SCANNER_IP").expect("scanner IP not available from env var SCANNER_IP");

    let scanner = Scanner::new(
        Url::parse(&format!("http://{}/eSCL", scanner_ip)).expect("invalid scanner URL"),
    );

    let capabilities = scanner
        .capabilities()
        .await
        .expect("unable to fetch capabilities");
    println!("Scanner capabilities: \n{:#?}", capabilities);

    let job = scanner
        .scan(&escl::settings::ScanSettings {
            version: capabilities.version,
            intent: None,
            scan_regions: None,
            input_source: None,
            color_mode: None,
            blank_page_detection: None,
        })
        .await
        .expect("unable to submit scan job");
    println!("Scan job URL: {}", job.job_url());

    let status = scanner.status().await.expect("unable to fetch status");
    println!("Scanner status: \n{:#?}", status);

    println!("Downloading scanned page...");
    let image = job
        .next_document()
        .await
        .expect("unable to fetch scanned page")
        .expect("at least one page should be available");

    let mut image_file = std::fs::File::create("./scan.jpg").expect("unable to create output file");
    image_file
        .write_all(&image)
        .expect("unable to write to output file");
}
