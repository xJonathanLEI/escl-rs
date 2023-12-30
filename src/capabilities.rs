use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScannerCapabilities {
    pub version: String,
    pub make_and_model: String,
    pub manufacturer: String,
    pub serial_number: String,
    #[serde(rename = "UUID")]
    pub uuid: String,
    #[serde(rename = "AdminURI")]
    pub admin_uri: String,
    #[serde(rename = "IconURI")]
    pub icon_uri: String,
    pub certifications: Certifications,
    pub platen: Platen,
    pub compression_factor_support: CompressionFactorSupport,
    pub supported_media_types: SupportedMediaTypes,
    pub sharpen_support: SharpenSupport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Certifications {
    pub certification: Certification,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Certification {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Platen {
    pub platen_input_caps: PlatenInputCaps,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlatenInputCaps {
    pub min_width: u32,
    pub max_width: u32,
    pub min_height: u32,
    pub max_height: u32,
    pub max_scan_regions: u32,
    pub setting_profiles: SettingProfiles,
    pub supported_intents: SupportedIntents,
    pub max_optical_x_resolution: u32,
    pub max_optical_y_resolution: u32,
    pub risky_left_margin: u32,
    pub risky_right_margin: u32,
    pub risky_top_margin: u32,
    pub risky_bottom_margin: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SettingProfiles {
    pub setting_profile: SettingProfile,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SettingProfile {
    pub color_modes: ColorModes,
    pub content_types: ContentTypes,
    pub document_formats: DocumentFormats,
    pub supported_resolutions: SupportedResolutions,
    pub color_spaces: ColorSpaces,
    pub ccd_channels: CcdChannels,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ColorModes {
    pub color_mode: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContentTypes {
    pub content_type: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DocumentFormats {
    pub document_format: Vec<String>,
    pub document_format_ext: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SupportedResolutions {
    pub discrete_resolutions: DiscreteResolutions,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DiscreteResolutions {
    pub discrete_resolution: Vec<DiscreteResolution>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DiscreteResolution {
    pub x_resolution: u32,
    pub y_resolution: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ColorSpaces {
    pub color_space: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CcdChannels {
    pub ccd_channel: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SupportedIntents {
    pub intent: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CompressionFactorSupport {
    pub min: u32,
    pub max: u32,
    pub normal: u32,
    pub step: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SupportedMediaTypes {
    pub media_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SharpenSupport {
    pub min: u32,
    pub max: u32,
    pub normal: u32,
    pub step: u32,
}
