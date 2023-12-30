use serde::{de::Visitor, Deserialize, Serialize};

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
    pub color_mode: Vec<ColorMode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContentTypes {
    pub content_type: Vec<ContentType>,
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
    pub intent: Vec<ScanIntent>,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorMode {
    /// Binary monochrome scanning. Valid only for certain DocumentFormat/DocumentFormatExt values –
    /// like 'application/octet-stream', 'image/tiff' that can support single-bit scans. For
    /// document format not supporting BlackAndWhite1 color mode, scanner SHOULD report a 409
    /// Conflict error.
    BlackAndWhite1,
    /// 8-bit grayscale
    Grayscale8,
    /// 16-bit grayscale
    Grayscale16,
    /// 8-bit per channel RGB
    RGB24,
    /// 16-bit per channel RGB
    RGB48,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentType {
    Photo,
    Text,
    TextAndPhoto,
    LineArt,
    Magazine,
    Halftone,
    Auto,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScanIntent {
    /// Scanning optimized for text.
    Document,
    /// A composite document with mixed text/graphic/photo content.
    TextAndGraphic,
    /// Scanning optimized for photo
    Photo,
    /// Scanning optimized for performance (fast output)
    Preview,
    /// Scanning optimized for 3 dimensional objects - objects with depth
    Object,
    /// Scanning optimized for a business card
    BusinessCard,
    Custom(String),
}

struct ContentTypeVisitor;
struct ScanIntentVisitor;

impl Serialize for ContentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Self::Photo => "Photo",
            Self::Text => "Text",
            Self::TextAndPhoto => "TextAndPhoto",
            Self::LineArt => "LineArt",
            Self::Magazine => "Magazine",
            Self::Halftone => "Halftone",
            Self::Auto => "Auto",
            Self::Custom(custom) => custom,
        })
    }
}

impl<'de> Deserialize<'de> for ContentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ContentTypeVisitor)
    }
}

impl<'de> Visitor<'de> for ContentTypeVisitor {
    type Value = ContentType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(match v {
            "Photo" => ContentType::Photo,
            "Text" => ContentType::Text,
            "TextAndPhoto" => ContentType::TextAndPhoto,
            "LineArt" => ContentType::LineArt,
            "Magazine" => ContentType::Magazine,
            "Halftone" => ContentType::Halftone,
            "Auto" => ContentType::Auto,
            custom => ContentType::Custom(custom.to_owned()),
        })
    }
}
impl Serialize for ScanIntent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Self::Document => "Document",
            Self::TextAndGraphic => "TextAndGraphic",
            Self::Photo => "Photo",
            Self::Preview => "Preview",
            Self::Object => "Object",
            Self::BusinessCard => "BusinessCard",
            Self::Custom(custom) => custom,
        })
    }
}

impl<'de> Deserialize<'de> for ScanIntent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ScanIntentVisitor)
    }
}

impl<'de> Visitor<'de> for ScanIntentVisitor {
    type Value = ScanIntent;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(match v {
            "Document" => ScanIntent::Document,
            "TextAndGraphic" => ScanIntent::TextAndGraphic,
            "Photo" => ScanIntent::Photo,
            "Preview" => ScanIntent::Preview,
            "Object" => ScanIntent::Object,
            "BusinessCard" => ScanIntent::BusinessCard,
            custom => ScanIntent::Custom(custom.to_owned()),
        })
    }
}
