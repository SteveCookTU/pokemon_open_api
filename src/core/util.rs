pub fn get_species_list_from_str(str: &str) -> Vec<u16> {
    str.split(",")
        .filter_map(|s| {
            if let Ok(species) = s.parse::<u16>() {
                Some(species)
            } else {
                None
            }
        })
        .collect()
}

pub fn get_location_list_from_str(str: &str) -> Vec<u8> {
    str.split(",")
        .filter_map(|s| {
            if let Ok(species) = s.parse::<u8>() {
                Some(species)
            } else {
                None
            }
        })
        .collect()
}
