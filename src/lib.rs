use lazy_static::lazy_static;

pub mod encounter;
pub mod encounter_area;
pub mod gen3;
pub mod gen4;
pub mod slot;

pub const LOCATIONS_E_RAW: &str = include_str!("resources/text/e_en.txt");
pub const LOCATIONS_FRLG_RAW: &str = include_str!("resources/text/frlg_en.txt");
pub const LOCATIONS_RS_RAW: &str = include_str!("resources/text/rs_en.txt");
pub const LOCATIONS_DPPT_RAW: &str = include_str!("resources/text/dppt_en.txt");
pub const LOCATIONS_HGSS_RAW: &str = include_str!("resources/text/hgss_en.txt");

lazy_static! {
    pub static ref LOCATIONS_EMERALD: Vec<&'static str> = {
        LOCATIONS_E_RAW
            .split("\n")
            .map(|s| s.split(",").skip(1).next().unwrap().trim())
            .collect()
    };
    pub static ref LOCATIONS_FRLG: Vec<&'static str> = {
        LOCATIONS_FRLG_RAW
            .split("\n")
            .map(|s| s.split(",").skip(1).next().unwrap().trim())
            .collect()
    };
    pub static ref LOCATIONS_RS: Vec<&'static str> = {
        LOCATIONS_RS_RAW
            .split("\n")
            .map(|s| s.split(",").skip(1).next().unwrap().trim())
            .collect()
    };
    pub static ref LOCATIONS_DPPT: Vec<&'static str> = {
        LOCATIONS_DPPT_RAW
            .split("\n")
            .map(|s| s.split(",").skip(1).next().unwrap().trim())
            .collect()
    };
    pub static ref LOCATIONS_HGSS: Vec<&'static str> = {
        LOCATIONS_HGSS_RAW
            .split("\n")
            .map(|s| s.split(",").skip(1).next().unwrap().trim())
            .collect()
    };
}
