use std::io::Write;

use escl::{
    settings::{ContentRegionUnits, InputSource, ScanRegion, ScanRegions},
    Scanner, Url,
};

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
            scan_regions: Some(ScanRegions {
                scan_region: ScanRegion {
                    height: capabilities.platen.platen_input_caps.max_height,
                    content_region_units: ContentRegionUnits::ThreeHundredthsOfInches,
                    width: capabilities.platen.platen_input_caps.max_width,
                    x_offset: 0,
                    y_offset: 0,
                },
            }),
            input_source: Some(InputSource::Platen),
            color_mode: capabilities
                .platen
                .platen_input_caps
                .setting_profiles
                .setting_profile
                .color_modes
                .color(),
            blank_page_detection: Some(false),
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
