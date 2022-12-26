use quick_xml::de::DeError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::str;

mod read_pma;

// const WAYPOINT_XML: &str = include_str!("../Defaults/WAYPOINT.xml");
// const AIRFIELD_XML: &str = include_str!("../Defaults/AIRFIELD.xml");
// const COM1_XML: &str = include_str!("../Defaults/COM_1.xml");
// const COM2_XML: &str = include_str!("../Defaults/COM_2.xml");
// const DATALINK_XML: &str = include_str!("../Defaults/Datalink.xml");
// const FROUTE_XML: &str = include_str!("../Defaults/F_ROUTE.xml");
// const RECCELEG_XML: &str = include_str!("../Defaults/RecceLeg.xml");
// const RECCETGT_XML: &str = include_str!("../Defaults/RecceTgt.xml");

// Rename all the fields of this struct variant according to the given case convention.
// The possible values are "lowercase", "UPPERCASE", "PascalCase", "camelCase", "snake_case",
// "SCREAMING_SNAKE_CASE", "kebab-case", "SCREAMING-KEBAB-CASE"

trait Format {
    fn fmt(&self) -> String;
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
struct Waypoint {
    #[serde(rename = "Waypoint_Rec")]
    pub waypoint_rec: Vec<Wpt>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Wpt {
    #[serde(rename = "ID")]
    id: u8,
    #[serde(rename = "LAT")]
    lat: f64,
    #[serde(rename = "LONG")]
    long: f64,
    #[serde(rename = "Elev")]
    elev: i32,
    #[serde(rename = "TOF_V")]
    tof_v: TofV,
    #[serde(rename = "Name")]
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct TofV {
    validity: bool,
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct COM1 {
    #[serde(rename = "COM1_PRESET")]
    pub com1_preset: Vec<Freq>,
}

#[derive(Debug, Deserialize, Serialize)]
struct COM2 {
    #[serde(rename = "COM2_PRESET")]
    pub com2_preset: Vec<Freq>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Freq {
    #[serde(rename = "ID")]
    pub id: u8,
    #[serde(rename = "Freq")]
    pub freq: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Ils {
    #[serde(rename = "ILS_Rec")]
    ils_rec: Vec<ILSRec>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
struct ILSRec {
    id: u8,
    #[serde(rename = "Freq")]
    freq: ILSFreq,
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Description")]
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ILSFreq {
    #[serde(rename = "MHz")]
    mhz: u16,
    #[serde(rename = "kHz")]
    khz: u16,
}

fn main() -> Result<(), DeError> {
    let content = read_pma::read_file("./Defaults/DTC_RECCE copy.txt");

    let mut pma_file = read_pma::PMAFile::default();
    pma_file.read(&content);

    //   let testing: WAYPOINT = deserialize_xml(WAYPOINT_XML)?;

    println!("{:?}", &pma_file);

    Ok(())
}

pub fn deserialize_xml<T: DeserializeOwned>(string_slice: &str) -> Result<T, DeError> {
    quick_xml::de::from_str(string_slice)
}

// m/s * 3600 / 1852
