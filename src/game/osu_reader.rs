use std::collections::HashMap;
use std::fs::{write, File};
use std::io::Read;

use bevy::prelude::Vec2;

#[derive(Debug)]
pub enum OsuFileSection {
    KeyValueMap(HashMap<String, String>),
    HitObjects(Vec<HitObject>),
    None,
}

#[derive(Debug)]
pub struct HitObject {
    pub hit_type: usize,
    pub time: u32,
    pub position: Vec2,
}

fn hitobject_processing(line: &str) -> HitObject {
    // Split line into its parts
    let information: Vec<&str> = line.split(',').collect();

    // Potential alternative: creating new variants of the OsuFileInfo Enum for each HitObject type?
    return HitObject {
        position: Vec2 {
            x: information[0].parse().unwrap(),
            y: information[1].parse().unwrap(),
        },
        time: information[2].parse().unwrap(),
        hit_type: information[3].parse().unwrap(),
    };
}

fn json_like_key_value_get(line: &str) -> Option<(String, String)> {
    // Ignore lines that do not follow the 'Json-like' key:value pattern
    if !line.contains(':') {
        return None;
    }

    // Remove potential trailing whitespace after ':'
    let line = line.replace(": ", ":");
    // Then split line into the parts before and after the ':'
    let parts: Vec<&str> = line.split(':').collect();
    Some((parts[0].to_string(), parts[1].to_string()))
}

pub fn open_osu(path: &str) -> HashMap<String, OsuFileSection> {
    let mut source = String::new();

    File::open(path)
        .expect("Path does not exist!")
        .read_to_string(&mut source)
        .unwrap();

    source.retain(|x| x != '\r');
    let source_lines: Vec<&str> = source.split('\n').collect();

    let mut sections: HashMap<String, OsuFileSection> = HashMap::new();
    // Each time a new [Section] line is met, a new HashMap will be assigned to current_section_map
    let mut current_section = &mut OsuFileSection::None;

    for line in source_lines.iter() {
        if line.is_empty() {
            continue;
        }

        // If line starts with '[', this is a Section Title
        // Otherwise, this line contains data for the current Section
        if line.starts_with('[') {
            // Create a new entry in the section map for that Section Title, with the type in the OsuFileSection Enum determined by the title name
            current_section = sections.entry(line.to_string()).or_insert(match *line {
                "[HitObjects]" => OsuFileSection::HitObjects(vec![]),
                _ => OsuFileSection::KeyValueMap(HashMap::new()),
            });
        } else {
            match current_section {
                OsuFileSection::HitObjects(section_data) => {
                    section_data.push(hitobject_processing(line))
                }
                OsuFileSection::KeyValueMap(section_map) => {
                    match json_like_key_value_get(line) {
                        Some(s) => section_map.insert(s.0, s.1),
                        None => None,
                    };
                }
                _ => {}
            }
        }
    }

    // DEBUG: Write all the parsed data into an output file
    let output_path = path.replace(".osu", "_output.txt");
    write(output_path, format!("{sections:#?}")).unwrap();

    sections
}
