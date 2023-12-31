use std::{io::Write, time::Duration};

use escl::{
    discover,
    settings::{ContentRegionUnits, InputSource, ScanRegion, ScanRegions},
    Scanner,
};

#[tokio::main]
async fn main() {
    println!("Looking for scanners in LAN...");

    let scanner_services = discover(Duration::from_secs(5))
        .await
        .expect("error discovering scanner services");

    let chosen_service = scanner_services.first().expect("no scanner found in LAN");
    println!(
        "Scanner chosen: {} ({})",
        chosen_service.name(),
        chosen_service.url()
    );

    let scanner: Scanner = chosen_service.into();

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
            document_format_ext: Some("image/jpeg".to_owned()),
            x_resolution: None,
            y_resolution: None,
            input_source: Some(InputSource::Platen),
            color_mode: capabilities
                .platen
                .platen_input_caps
                .setting_profiles
                .setting_profile
                .color_modes
                .color(),
            compression_factor: None,
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
