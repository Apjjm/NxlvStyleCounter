use std::collections::VecDeque;

pub struct TerrainPart {
    pub style: String,
    pub peice: String,
}

pub struct TerrainGroup {
    pub name: String,
    pub terrain: Vec<TerrainPart>,
}

pub struct Gadget {
    pub style: String,
    pub peice: String,
}

pub struct Level {
    pub gadgets: Vec<Gadget>,
    pub groups: Vec<TerrainGroup>,
    pub terrain: Vec<TerrainPart>,
}

pub fn parse_level(level_text: &str) -> Level {
    let mut level = Level {
        gadgets: Vec::new(),
        groups: Vec::new(),
        terrain: Vec::new(),
    };

    let mut reader: VecDeque<&str> = level_text
        .split('\n')
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .filter(|l| !l.starts_with('#'))
        .collect();

    while let Some(line) = reader.pop_front() {
        if line.starts_with("$TERRAINGROUP") {
            level.groups.push(parse_terrain_group(&mut reader));
        } else if line.starts_with("$TERRAIN") {
            level.terrain.push(parse_terrain_part(&mut reader));
        } else if line.starts_with("$GADGET") {
            level.gadgets.push(parse_gadget(&mut reader));
        }
    }
    level
}

fn parse_terrain_part(reader: &mut VecDeque<&str>) -> TerrainPart {
    let mut part = TerrainPart {
        style: String::new(),
        peice: String::new(),
    };
    while let Some(line) = reader.pop_front() {
        if line.starts_with("STYLE") {
            part.style = line[6..].to_string();
        } else if line.starts_with("PIECE") {
            part.peice = line[6..].to_string();
        } else if line.starts_with("$END") {
            break;
        }
    }
    part
}

fn parse_terrain_group(reader: &mut VecDeque<&str>) -> TerrainGroup {
    let mut group = TerrainGroup {
        name: String::new(),
        terrain: Vec::new(),
    };
    while let Some(line) = reader.pop_front() {
        if line.starts_with("NAME") {
            group.name = line[5..].to_string();
        } else if line.starts_with("$TERRAIN") {
            group.terrain.push(parse_terrain_part(reader));
        } else if line.starts_with("$END") {
            break;
        }
    }
    group
}

fn parse_gadget(reader: &mut VecDeque<&str>) -> Gadget {
    let mut gadget = Gadget {
        style: String::new(),
        peice: String::new(),
    };
    while let Some(line) = reader.pop_front() {
        if line.starts_with("STYLE") {
            gadget.style = line[6..].to_string();
        } else if line.starts_with("PIECE") {
            gadget.peice = line[6..].to_string();
        } else if line.starts_with("$END") {
            break;
        }
    }
    gadget
}
