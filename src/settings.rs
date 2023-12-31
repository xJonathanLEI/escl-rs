use serde::{de::Visitor, Deserialize, Serialize};

use crate::capabilities::{ColorMode, ScanIntent};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "scan:ScanSettings")]
pub struct ScanSettings {
    #[serde(rename = "pwg:Version")]
    pub version: String,
    #[serde(
        rename = "scan:Intent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub intent: Option<ScanIntent>,
    #[serde(
        rename = "pwg:ScanRegions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scan_regions: Option<ScanRegions>,
    #[serde(
        rename = "scan:DocumentFormatExt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub document_format_ext: Option<String>,
    #[serde(
        rename = "pwg:InputSource",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub input_source: Option<InputSource>,
    #[serde(
        rename = "scan:XResolution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub x_resolution: Option<u32>,
    #[serde(
        rename = "scan:YResolution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub y_resolution: Option<u32>,
    #[serde(
        rename = "scan:ColorMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub color_mode: Option<ColorMode>,
    #[serde(
        rename = "scan:CompressionFactor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub compression_factor: Option<u32>,
    #[serde(
        rename = "scan:BlankPageDetection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blank_page_detection: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScanRegions {
    #[serde(rename = "pwg:ScanRegion")]
    pub scan_region: ScanRegion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScanRegion {
    #[serde(rename = "pwg:Height")]
    pub height: u32,
    #[serde(rename = "pwg:ContentRegionUnits")]
    pub content_region_units: ContentRegionUnits,
    #[serde(rename = "pwg:Width")]
    pub width: u32,
    #[serde(rename = "pwg:XOffset")]
    pub x_offset: u32,
    #[serde(rename = "pwg:YOffset")]
    pub y_offset: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputSource {
    /// Glass flat bed
    Platen,
    /// ADF - Automatic Document Feeder
    Feeder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentRegionUnits {
    ThreeHundredthsOfInches,
}

struct InputSourceVisitor;
struct ContentRegionUnitsVisitor;

impl Serialize for InputSource {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Self::Platen => "Platen",
            Self::Feeder => "Feeder",
        })
    }
}

impl<'de> Deserialize<'de> for InputSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(InputSourceVisitor)
    }
}

impl<'de> Visitor<'de> for InputSourceVisitor {
    type Value = InputSource;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(match v {
            "Platen" => InputSource::Platen,
            "Feeder" => InputSource::Feeder,
            _ => {
                return Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(v),
                    &"valid InputSource",
                ))
            }
        })
    }
}

impl Serialize for ContentRegionUnits {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Self::ThreeHundredthsOfInches => "escl:ThreeHundredthsOfInches",
        })
    }
}

impl<'de> Deserialize<'de> for ContentRegionUnits {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ContentRegionUnitsVisitor)
    }
}

impl<'de> Visitor<'de> for ContentRegionUnitsVisitor {
    type Value = ContentRegionUnits;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(match v {
            "escl:ThreeHundredthsOfInches" => ContentRegionUnits::ThreeHundredthsOfInches,
            _ => {
                return Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(v),
                    &"valid ContentRegionUnits",
                ))
            }
        })
    }
}
