use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Unknown {
    time: i32,
    position: (f32, f32),
}

#[derive(Debug)]
pub enum OsuFileInfo {
    Info(String),
    Unknown(Unknown),
}

fn hitobject_processing(string: &str) -> OsuFileInfo
{
    let mut information = vec![];
    let mut getter = String::new();
    for char in string.chars() {
        
        if char == ',' {
            information.push(getter.clone());
            getter = String::new();
            continue
        }

        getter += &char.to_string();

    }
    information.push(getter.clone());
    
    // Screw it, I am going to return all of them as unknown
    return OsuFileInfo::Unknown(
        Unknown {
            time: information[2].parse().unwrap(),
            position: (information[0].parse().unwrap(), information[1].parse().unwrap()),
        }
    )

}

fn get_names_from_vec(vec: &Vec<String>, name: &str) -> usize {
    match vec.iter().position(|x| *x == name) {
        Some(s) => s,
        None => panic!("{} was not found in the file!", name)
    }
}

fn json_like_key_value_get(to_get: &str) -> Option<(String, String)> {
    let to_get = to_get.replace(" ", "");

    if !to_get.contains(":") {
        return None
    }

    let to_get = to_get.split(":").collect::<Vec<&str>>();
    Some((to_get[0].to_string() ,to_get[1].to_string()))
}

pub fn open_osu(path: &str) {
    let mut source = String::new();

    File::open(path)
        .expect("Path does not exist!")
        .read_to_string(&mut source)
        .unwrap();

    source.retain(|x| x != '\r');

    let source = source.split("\n").collect::<Vec<&str>>();

    let mut values_map: HashMap<&str, HashMap<String, OsuFileInfo>>  = HashMap::new();
    let mut indexes = vec![];

    let mut index = 0;
    for value in source.iter() {
        if value.contains('[') {
            values_map.insert(value, HashMap::new());
            values_map.get_mut(value).unwrap().insert("SourceLine - 1".to_string(), OsuFileInfo::Info(index.to_string()));
            indexes.push(index.to_string());
        }
        index += 1;
    }

    for map in values_map.values_mut() {
        let index_struct  = map.get("SourceLine - 1").unwrap().clone();

        let mut index = String::new();
        if let OsuFileInfo::Info(s) = index_struct {
            index = s.clone();
        }

        let index_vec_pos = get_names_from_vec(&indexes, &index);

        let mut last_line = source.len()-1;

        if index_vec_pos != indexes.len()-1 {
            last_line = indexes.get(index_vec_pos+1).unwrap().parse().unwrap();
        }

        let mut hitcount = 0;
        for line in &source[index.parse::<usize>().unwrap()+1..last_line] {
            if indexes[indexes.len()-1] == index {
                map.insert(hitcount.to_string(), hitobject_processing(line));
                hitcount += 1;
            } else {
                match json_like_key_value_get(line) {
                    Some(s) => map.insert(s.0, OsuFileInfo::Info(s.1)),
                    None => None
                };
            }
        }
    }

}