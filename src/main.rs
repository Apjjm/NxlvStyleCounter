mod level;
use level::*;
use std::{collections::HashMap, fs, path::Path};

fn main() {
    println!("Enter in the full path to the level pack below.");
    println!("Note that this tool will recursively search for levels.");
    let mut folder_path = String::new();
    std::io::stdin().read_line(&mut folder_path).expect("error reading line");
    let levels = get_input_levels(folder_path.trim()).unwrap();

    let mut per_style_counts: HashMap<String, (i32, i32)> = HashMap::new();
    let mut per_level_counts: HashMap<String, (i32, i32)> = HashMap::new();
    let mut total_objects = 0;
    let mut total_terrain = 0;

    for level_path in levels {
        let level_name = get_file_name(&level_path);
        let level = parse_level(&fs::read_to_string(level_path).unwrap());

        visit_gadgets(&level, &mut |g| {
            count_object(&mut per_style_counts, &g.style);
            count_object(&mut per_level_counts, &level_name);
            total_objects += 1;
        });

        visit_terrain_recursive(&level, &mut |t| {
            count_terrain(&mut per_style_counts, &t.style);
            count_terrain(&mut per_level_counts, &level_name);
            total_terrain += 1;
        });
    }

    println!("");
    println!("----------------------------------");
    println!("PER STYLE COUNTS");
    println!("----------------------------------");
    println!("");
    println!("{:<80}|{:<10}|{:<10}", "style", "terrain", "objects");
    for style in get_sorted_keys(&per_style_counts) {
        let counts = per_style_counts.get(style).unwrap();
        println!("{:<80}|{:<10}|{:<10}", style, counts.0, counts.1);
    }

    println!("");
    println!("----------------------------------");
    println!("PER LEVEL COUNTS");
    println!("----------------------------------");
    println!("");
    println!("{:<80}|{:<10}|{:<10}", "level", "terrain", "objects");
    for level in get_sorted_keys(&per_level_counts) {
        let counts = per_level_counts.get(level).unwrap();
        println!("{:<80}|{:<10}|{:<10}", level, counts.0, counts.1);
    }

    println!("");
    println!("----------------------------------");
    println!("TOTAL COUNTS");
    println!("----------------------------------");
    println!("");
    println!("Total objects: {}", total_objects);
    println!("Total terrain: {}", total_terrain);
}

fn get_input_levels(path: &str) -> std::io::Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    let mut to_visit: Vec<String> = vec![path.to_string()];
    while let Some(path_to_visit) = to_visit.pop() {
        for entry in fs::read_dir(path_to_visit)? {
            let entry_path = entry?.path();
            let path_string = entry_path.to_str().unwrap().to_string();
            if entry_path.is_file() && path_string.ends_with(".nxlv") {
                result.push(path_string);
            } else if entry_path.is_dir() {
                to_visit.push(path_string);
            }
        }
    }
    Ok(result)
}

fn get_file_name(path: &str) -> String {
    let name_part = Path::new(path).file_name().unwrap();
    name_part.to_str().unwrap().to_string()
}

fn get_sorted_keys(map: &HashMap<String, (i32, i32)>) -> Vec<&String> {
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort();
    keys
}

fn count_object(map: &mut HashMap<String, (i32, i32)>, key: &String) {
    if let Some((_, objects)) = map.get_mut(key) {
        *objects += 1
    } else {
        map.insert(key.clone(), (0, 1));
    }
}

fn count_terrain(map: &mut HashMap<String, (i32, i32)>, key: &String) {
    if let Some((terrain, _)) = map.get_mut(key) {
        *terrain += 1
    } else {
        map.insert(key.clone(), (1, 0));
    }
}

fn visit_gadgets(level: &Level, visitor: &mut dyn FnMut(&Gadget)) {
    for gadget in &level.gadgets {
        visitor(gadget)
    }
}

fn visit_terrain_recursive(level: &Level, visitor: &mut dyn FnMut(&TerrainPart)) {
    let mut to_visit: Vec<&TerrainPart> = level.terrain.iter().collect();
    while let Some(terrain) = to_visit.pop() {
        if terrain.style.eq_ignore_ascii_case("*GROUP") {
            let group = level.groups.iter().find(|g| g.name == terrain.peice);
            for other_terrain in &group.unwrap().terrain {
                to_visit.push(other_terrain)
            }
        } else {
            visitor(terrain)
        }
    }
}
