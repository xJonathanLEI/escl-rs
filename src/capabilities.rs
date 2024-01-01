use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScannerCapabilities {
    pub version: String,
    pub make_and_model: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    pub serial_number: String,
    #[serde(rename = "UUID")]
    pub uuid: String,
    #[serde(rename = "AdminURI")]
    pub admin_uri: String,
    #[serde(rename = "IconURI")]
    pub icon_uri: String,
    #[serde(default, skip_serializing_if = "Certifications::is_empty")]
    pub certifications: Certifications,
    pub platen: Platen,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression_factor_support: Option<CompressionFactorSupport>,
    #[serde(default, skip_serializing_if = "SupportedMediaTypes::is_empty")]
    pub supported_media_types: SupportedMediaTypes,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sharpen_support: Option<SharpenSupport>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Certifications {
    pub certification: Vec<Certification>,
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
    #[serde(default, skip_serializing_if = "ContentTypes::is_empty")]
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

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    pub color_space: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CcdChannels {
    pub ccd_channel: Vec<CcdChannel>,
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

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SupportedMediaTypes {
    pub media_type: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SharpenSupport {
    pub min: u32,
    pub max: u32,
    pub normal: u32,
    pub step: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorMode {
    /// Binary monochrome scanning. Valid only for certain DocumentFormat/DocumentFormatExt values -
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
pub enum CcdChannel {
    /// Use the Red CCD
    Red,
    /// Use the Green CCD
    Green,
    /// Use the Blue CCD
    Blue,
    /// Weighted combination of the three color channels optimized for photos
    NTSC,
    /// A dedicated Gray CCD array in the hardware (optimized for documents)
    GrayCcd,
    /// An emulated Gray CCD mode where each CCD line are given even weight (1/3 R, 1/3 G, 1/3 B)
    /// (optimized for documents).
    GrayCcdEmulated,
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

struct ColorModeVisitor;
struct ContentTypeVisitor;
struct CcdChannelVisitor;
struct ScanIntentVisitor;

impl Certifications {
    fn is_empty(&self) -> bool {
        self.certification.is_empty()
    }
}

impl ContentTypes {
    fn is_empty(&self) -> bool {
        self.content_type.is_empty()
    }
}

impl SupportedMediaTypes {
    fn is_empty(&self) -> bool {
        self.media_type.is_empty()
    }
}

impl ColorModes {
    /// Gets the highest support quality RGB color mode. If no RGB color mode is supported, `None`
    // is returned.
    pub fn color(&self) -> Option<ColorMode> {
        if self.color_mode.contains(&ColorMode::RGB48) {
            Some(ColorMode::RGB48)
        } else if self.color_mode.contains(&ColorMode::RGB24) {
            Some(ColorMode::RGB24)
        } else {
            None
        }
    }
}

impl Serialize for ColorMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Self::BlackAndWhite1 => "BlackAndWhite1",
            Self::Grayscale8 => "Grayscale8",
            Self::Grayscale16 => "Grayscale16",
            Self::RGB24 => "RGB24",
            Self::RGB48 => "RGB48",
        })
    }
}

impl<'de> Deserialize<'de> for ColorMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ColorModeVisitor)
    }
}

impl<'de> Visitor<'de> for ColorModeVisitor {
    type Value = ColorMode;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(match v {
            "BlackAndWhite1" => ColorMode::BlackAndWhite1,
            "Grayscale8" => ColorMode::Grayscale8,
            "Grayscale16" => ColorMode::Grayscale16,
            "RGB24" => ColorMode::RGB24,
            "RGB48" => ColorMode::RGB48,
            _ => {
                return Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(v),
                    &"valid ColorMode value",
                ))
            }
        })
    }
}

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

impl Serialize for CcdChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            CcdChannel::Red => "Red",
            CcdChannel::Green => "Green",
            CcdChannel::Blue => "Blue",
            CcdChannel::NTSC => "NTSC",
            CcdChannel::GrayCcd => "GrayCcd",
            CcdChannel::GrayCcdEmulated => "GrayCcdEmulated",
        })
    }
}

impl<'de> Deserialize<'de> for CcdChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(CcdChannelVisitor)
    }
}

impl<'de> Visitor<'de> for CcdChannelVisitor {
    type Value = CcdChannel;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(match v {
            "Red" => CcdChannel::Red,
            "Green" => CcdChannel::Green,
            "Blue" => CcdChannel::Blue,
            "NTSC" => CcdChannel::NTSC,
            "GrayCcd" => CcdChannel::GrayCcd,
            "GrayCcdEmulated" => CcdChannel::GrayCcdEmulated,
            _ => {
                return Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(v),
                    &"valid ColorMode value",
                ))
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_capabilities_deser() {
        for raw_xml in [
            include_str!("../test-data/capabilities/brother_mfc_j497dw.xml"),
            include_str!("../test-data/capabilities/canon_ts5300_series.xml"),
        ]
        .into_iter()
        {
            serde_xml_rs::from_str::<ScannerCapabilities>(raw_xml)
                .expect("capabilities deserializing failure");
        }
    }
}
