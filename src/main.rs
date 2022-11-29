use quick_xml::de::DeError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

const WAYPOINT_XML: &str = include_str!("../Defaults/WAYPOINT.xml");
const AIRFIELD_XML: &str = include_str!("../Defaults/AIRFIELD.xml");
const COM1_XML: &str = include_str!("../Defaults/COM_1.xml");
const COM2_XML: &str = include_str!("../Defaults/COM_2.xml");
const Datalink_XML: &str = include_str!("../Defaults/Datalink.xml");
const FROUTE_XML: &str = include_str!("../Defaults/F_ROUTE.xml");
const RecceLeg_XML: &str = include_str!("../Defaults/RecceLeg.xml");
const RecceTgt_XML: &str = include_str!("../Defaults/RecceTgt.xml");

// Rename all the fields of this struct variant according to the given case convention.
// The possible values are "lowercase", "UPPERCASE", "PascalCase", "camelCase", "snake_case",
// "SCREAMING_SNAKE_CASE", "kebab-case", "SCREAMING-KEBAB-CASE"

trait Format {
    fn fmt(&self) -> String;
}

enum DTC {
    ADD_RINV,
    ADD,
    SINV,
    ADF,
    AIRFIELD,
    ALN_SLOT,
    AVD_AREA,
    WAYPOINT,
    COM1,
    COM2,
    POI,
    POD_RECCE,
    POD_LDP,
    SILENCE,
    SIM_INV,
    TAKEOFF,
    VOR,
    WARNING,
}

#[derive(Debug, Deserialize, Serialize)]
struct WAYPOINT {
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
    pub com1_preset: Vec<FREQ>,
}

#[derive(Debug, Deserialize, Serialize)]
struct COM2 {
    #[serde(rename = "COM2_PRESET")]
    pub com2_preset: Vec<FREQ>,
}

#[derive(Debug, Deserialize, Serialize)]
struct FREQ {
    #[serde(rename = "ID")]
    pub id: u8,
    #[serde(rename = "Freq")]
    pub freq: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct ILS {
    ILS_Rec: Vec<ILS_Rec>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
struct ILS_Rec {
    id: u8,
    #[serde(rename = "Freq")]
    freq: ILS_freq,
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Description")]
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ILS_freq {
    #[serde(rename = "MHz")]
    mhz: u16,
    #[serde(rename = "kHz")]
    khz: u16,
}

fn main() -> Result<(), quick_xml::Error> {
    let testing: COM2 = match deserialize_xml(COM2_XML) {
        Err(e) => panic!("{e}"),
        Ok(w) => w,
    };

    println!("{:#?}", testing);

    Ok(())
}

fn deserialize_xml<T: DeserializeOwned>(string_slice: &str) -> Result<T, DeError> {
    quick_xml::de::from_str(string_slice)
}
