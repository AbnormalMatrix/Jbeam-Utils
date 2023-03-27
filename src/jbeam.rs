use std::fs;
use three_d::*;
use regex::Regex;
use rfd::FileDialog;

pub struct JNodeGroup {
    pub group_name: String,
    pub node_ids: Vec<String>,
    pub node_material: JPhysicalMaterial,


}


use pest::Parser;

#[derive(Parser)]
#[grammar = "example.pest"]
struct MyParser;


impl JNodeGroup {
    pub fn new(group_name: String) -> Self {
        Self {
            group_name,
            node_ids: Vec::new(),
            node_material: JPhysicalMaterial::Metal,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JPhysicalMaterial {
    Metal,
    Plastic,
    Rubber,
    Glass,
    Wood,
    Foliage,
    Cloth,
    Water,
    Asphalt,
    AsphaltWet,
    Slippery,
    Rock,
    DirtDusty,
    Dirt,
    Sand,
    SandyRoad,
    Mud,
    Gravel,
    Grass,
    Ice,
    Snow,
    Firesmall,
    Firemedium,
    Firelarge,
    SmokeSmallBlack,
    SmokeMediumBlack,
    Steam,
    RumbleStrip,
    Cobblestone,
    FoliageThin,
}

#[derive(Debug, Clone)]
pub struct JNode {
    pub id: String,
    pub position: (f32, f32, f32),
    pub imported: bool,
    // optional arguments
    pub node_weight: f32,
    collision: bool,
    self_collision: bool,
    pub group: Vec<String>,
    friction_coefficient: f32,
    node_material: JPhysicalMaterial,
    fixed: bool,
    coupler_strength: f32,
    coupler_tag: String,
    coupler_radius: f32,
    break_group: String,
    coupler_lock: bool,
    import_electrics: Vec<String>,
    import_inputs: Vec<String>,
    surface_coef: f32,
    volume_coef: f32,
    no_load_coef: f32,
    full_load_coef: f32,
    stribeck_exponent: f32,
    stribeck_vel_mult: f32,
    softness_coef: f32,
    tread_coef: f32,
    tag: String,
    load_sensitivity_slope: f32,
    paired_node: String,

    // editor specific thing... these should NOT get exported to BeamNG
    pub is_selected: bool,


}

impl JNode {
    pub fn new(id: String, position: (f32, f32, f32), imported: bool) -> Self {
        Self {
            id,
            position,
            imported,
            node_weight: 0.0,
            collision: false,
            self_collision: false,
            group: Vec::new(),
            friction_coefficient: 1.0,
            node_material: JPhysicalMaterial::Metal,
            fixed: false,
            // highest f32 value
            coupler_strength: 340282346638528859811704183484516925440.0,
            coupler_tag: String::new(),
            coupler_radius: 0.0,
            break_group: String::new(),
            coupler_lock: false,
            import_electrics: Vec::new(),
            import_inputs: Vec::new(),
            surface_coef: 0.1,
            volume_coef: 0.1,
            no_load_coef: 1.0,
            full_load_coef: 0.0,
            stribeck_exponent: 1.75,
            stribeck_vel_mult: 1.0,
            softness_coef: 0.5,
            tread_coef: 0.5,
            tag: String::new(),
            load_sensitivity_slope: 0.0,
            paired_node: String::new(),

            is_selected: false,
        }
    }

    pub fn write(&self, filename: String) -> String{
        let group_str = if self.group.len() > 0 {
            format!(r#""group":["{}"]"#, self.group.join("\",\""))
        } else {
            String::new()
        };
        let data = format!(r#"["{}", {}, {}, {}, {{"nodeWeight":{}, "collision":{}, "selfCollision":{}, "frictionCoef":{}, {}}}],"#, self.id, self.position.0, self.position.2, self.position.1, self.node_weight, self.collision, self.self_collision, self.friction_coefficient, group_str);
        println!("{}", data);

        data

    }

    pub fn get_3d_object(&self, context: &Context, unselected_material: &CpuMaterial, selected_material: &CpuMaterial) -> Gm<Mesh, PhysicalMaterial> {
        if self.is_selected {
            let mut sphere = Gm::new(
                Mesh::new(&context, &CpuMesh::sphere(4)),
                PhysicalMaterial::new_opaque(
                    &context,
                    selected_material,
                ),
            );
            sphere.set_transformation(Mat4::from_translation(vec3(self.position.0, self.position.1, self.position.2)) * Mat4::from_scale(0.02));
            return  sphere;
        } else {
            let mut sphere = Gm::new(
                Mesh::new(&context, &CpuMesh::sphere(4)),
                PhysicalMaterial::new_opaque(
                    &context,
                    unselected_material,
                ),
            );
            sphere.set_transformation(Mat4::from_translation(vec3(self.position.0, self.position.1, self.position.2)) * Mat4::from_scale(0.02));
            return sphere;
        }



    }

    pub fn apply_modifiers(&mut self, modifier: JNodeModifer, inline_modifiers: JNodeModifer) {
        self.node_weight = modifier.node_weight;
        self.collision = modifier.collision;
        self.self_collision = modifier.self_collision;
        self.friction_coefficient = modifier.friction_coefficient;
        self.node_material = modifier.node_material;

        self.group = modifier.group;

        // only apply inline modifiers if they are not default
        if inline_modifiers.node_weight != 0.0 {
            self.node_weight = inline_modifiers.node_weight;
        }
        if inline_modifiers.collision {
            self.collision = inline_modifiers.collision;
        }
        if inline_modifiers.self_collision {
            self.self_collision = inline_modifiers.self_collision;
        }
        if inline_modifiers.friction_coefficient != 1.0 {
            self.friction_coefficient = inline_modifiers.friction_coefficient;
        }
        if inline_modifiers.node_material != JPhysicalMaterial::Metal {
            self.node_material = inline_modifiers.node_material;
        }

        if inline_modifiers.group.len() > 0 {
            self.group = inline_modifiers.group;
        }

    }
}

#[derive(Debug, Clone)]
pub struct JNodeModifer {

    pub node_weight: f32,
    pub collision: bool,
    pub self_collision: bool,
    group: Vec<String>,
    pub friction_coefficient: f32,
    pub node_material: JPhysicalMaterial,

}

impl JNodeModifer {
    pub fn new() -> Self {
        Self {
            node_weight: 0.0,
            collision: false,
            self_collision: false,
            group: Vec::new(),
            friction_coefficient: 1.0,
            node_material: JPhysicalMaterial::Metal,
        }
    }

}


#[derive(Debug, Clone)]
pub enum BeamType {
    Normal,
    Hydro,
    ANISOTROPIC,
    BOUNDED,
    PRESSURED,
    SUPPORT,
    BROKEN,
    LBEAM,
}

#[derive(Debug, Clone)]
pub struct JBeam {
    pub id1: String,
    pub id2: String,
    imported: bool,
    // optional arguments
    beam_type: BeamType,
    beam_spring: f32,
    beam_damp: f32,
    beam_strength: f32,
    beam_deform: f32,
    beam_compression: f32,
    beam_compression_range: f32,
    beam_compression_time: f32,
    break_group: String,
    break_group_type: i8,
    name: String,
    damp_cutoff_hz: f32,
    deform_limit: f32,
    deform_limt_expansion: f32,
    optional: bool,
    deform_group: String,
    deformation_trigger_ratio: f32,
    sound_file: String,
    color_factor: f32,
    attack_factor: f32,
    volume_factor: f32,
    decay_factor: f32,
    pitch_factor: f32,
    max_stress: f32,

}

impl JBeam {
    pub fn new(id1: String, id2: String, imported: bool) -> Self {
        Self {
            id1,
            id2,
            imported,
            beam_type: BeamType::Normal,
            beam_spring: 0.0,
            beam_damp: 0.0,
            beam_strength: 0.0,
            beam_deform: 0.0,
            beam_compression: 0.0,
            beam_compression_range: 0.0,
            beam_compression_time: 0.0,
            break_group: String::new(),
            break_group_type: 0,
            name: String::new(),
            damp_cutoff_hz: 0.0,
            deform_limit: 0.0,
            deform_limt_expansion: 0.0,
            optional: false,
            deform_group: String::new(),
            deformation_trigger_ratio: 0.0,
            sound_file: String::new(),
            color_factor: 0.0,
            attack_factor: 0.0,
            volume_factor: 0.0,
            decay_factor: 0.0,
            pitch_factor: 0.0,
            max_stress: 0.0,
        }
    }

    pub fn write(&self) -> String {

        let data = format!(r#"["{}", "{}"],"#, self.id1, self.id2);
        println!("{}", data);
        data

    }

    pub fn get_3d_object(&self, context: &Context, nodes: &Vec<JNode>) -> Sprites {
        let pos1 = nodes[get_node_by_id(self.id1.clone(), nodes).unwrap()].position;
        let pos2 = nodes[get_node_by_id(self.id2.clone(), nodes).unwrap()].position;



        let (x, y, z) = get_midpoint(pos1.0, pos2.0, pos1.1, pos2.1, pos1.2, pos2.2);



        let mut sprites = Sprites::new(
            context,
            &[vec3(x, y, z)],
            None,
        );

        let new_direction = vec3(pos2.0 - pos1.0, pos2.1 - pos1.1, pos2.2 - pos1.2);

        sprites.set_direction(Some(new_direction));

        // set the scale based on the new_direction

        

        let new_transform = Mat4::from_nonuniform_scale(0.0025, 0.5, 0.0025);
        sprites.set_transformation(new_transform);

        sprites
        
        
    }

}


fn get_midpoint(x1: f32, x2: f32, y1: f32, y2: f32, z1: f32, z2: f32) -> (f32, f32, f32) {
    let x = (x1 + x2) / 2.0;
    let y = (y1 + y2) / 2.0;
    let z = (z1 + z2) / 2.0;

    (x, y, z)
}


pub fn get_node_by_id(id: String, nodes: &Vec<JNode>) -> Option<usize> {
    // get the index of the node with the given id
    for (i, node) in nodes.iter().enumerate() {
        if node.id == id {
            return Some(i);
        }
    }
    None
}



fn handle_modifier(input_line: &str, scope_modifiers: &mut JNodeModifer) {

    let parsed_modifiers = MyParser::parse(Rule::modifier_list, input_line).unwrap();

    for rule in parsed_modifiers {
        // println!("{:?}", pair.as_rule());

        match rule.as_rule() {
            Rule::group => {



                for component in rule.into_inner() {
                    let g = component.as_str().replace("\"", "");

                    if g == "" {
                        scope_modifiers.group = Vec::new();
                    } else {
                        scope_modifiers.group.push(g);
                    }
                    
                }
            },
            Rule::node_material => {
                let material_string = rule.as_str().replace("\"", "").replace(r#"""#, "");
                if material_string == r#"nodeMaterial:"""# {
                    scope_modifiers.node_material == JPhysicalMaterial::Metal;
                } else {
                    let material_string = material_string.split("_").nth(1).unwrap();
                    // match the materials
                }
            },
            _=> ()
        }
    }


}




pub fn parse_jbeam(input_string: String ) -> Vec<JNode> {

    let mut scope_modifiers = JNodeModifer::new();

    let mut nodes: Vec<JNode> = Vec::new();

    for line in input_string.split("\n") {
        if line.trim().starts_with("[") {

            let mut inline_modifiers = JNodeModifer::new();

            // handle inline modifiers
            if line.contains("{") {
                let inline_modifiers = line.split("{").nth(1).unwrap().split(",");
                
                for modifier in inline_modifiers {
                    // ignore if it starts with //
                    if !modifier.trim().starts_with("//") {
                        
                    }
                }
            }

            let inline_modifiers_str = line.clone().split("{").nth(1).unwrap_or("");

            if inline_modifiers_str != "" {
                let parsed_input = MyParser::parse(Rule::root, inline_modifiers_str).unwrap();
                for rule in parsed_input {
                    match rule.as_rule() {

                        Rule::group => {
                            for g in rule.into_inner() {
                                let g = g.as_str().replace("\"", "");
                                inline_modifiers.group.push(g);
                            }
                        },
                        Rule::node_material => {
                            println!("MATERIAL: {:?}", rule);
                        },
                        Rule::self_collision => {
                            let self_collision: bool = rule.as_str().replace(r#""selfCollision":"#, "").replace(r#""#, "").parse().unwrap();

                            inline_modifiers.self_collision = self_collision;


                        },
                        Rule::collision => {
                            let collision: bool = rule.as_str().replace(r#""collision":"#, "").replace(r#""#, "").parse().unwrap();
                            
                            inline_modifiers.collision = collision;
                        },
                        Rule::node_weight => {
                            let node_weight: f32 = rule.as_str().replace(r#""nodeWeight":"#, "").replace(r#""#, "").parse().unwrap();
                            
                            inline_modifiers.node_weight = node_weight;

                        },
                        
                        _ => ()
                    }
                }
            }

            let line = line.split("{").next().unwrap().trim();
            

            let re = Regex::new(r"[\w\.-]+").unwrap();
            let matches: Vec<regex::Match> = re.find_iter(line).collect();

            let node_id: &str = matches[0].as_str();

            let x: f32 = matches[1].as_str().parse().unwrap();
            let y: f32 = matches[2].as_str().parse().unwrap();
            let z: f32 = matches[3].as_str().parse().unwrap();

            let mut node = JNode::new(node_id.to_string(), (x, z, y), true);

            node.apply_modifiers(scope_modifiers.clone(), inline_modifiers.clone());

            //println!("{}: {}, {}, {}", node_id, x, y, z);

            nodes.push(node);
        } else if line.trim().starts_with("{") {
            // handle scope modifiers

            let line = line.trim();


            println!("{}", line);

            handle_modifier(&line, &mut scope_modifiers)



        } else if line.trim().starts_with("]") {
            // end of the nodes section so we need to reset the modifiers
            scope_modifiers = JNodeModifer::new();
        }
    }


    nodes
}

pub fn parse_beams(input_string: String, nodes: &Vec<JNode>) -> (Vec<JBeam>, Vec<String>) {
    let mut beams: Vec<JBeam> = Vec::new();

    let mut invalid_lines: Vec<String> = Vec::new();

    for line in input_string.split("\n") {
        if line.trim().starts_with("[") {
            let line = line.split("{").next().unwrap().trim();
            let re = Regex::new(r"[\w\.-]+").unwrap();
            let matches: Vec<regex::Match> = re.find_iter(line).collect();

            let id1: &str = matches[0].as_str();
            let id2: &str = matches[1].as_str();

            // check if the nodes exist
            if get_node_by_id(id1.to_string(), nodes).is_none() {
                println!("{} does not exist", id1);

                invalid_lines.push(line.to_string());

                continue;
            }

            if get_node_by_id(id2.to_string(), nodes).is_none() {
                println!("{} does not exist", id2);

                invalid_lines.push(line.to_string());

                continue;
            }

            let beam = JBeam::new(id1.to_string(), id2.to_string(), true);

            //println!("{}: {}", id1, id2);

            beams.push(beam);
            
        }
    }



    (beams, invalid_lines)
}

pub fn get_distance(a: Vec3, b: Vec3) -> f32 {
    let x = a.x - b.x;
    let y = a.y - b.y;
    let z = a.z - b.z;

    let distance = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();

    distance
}

pub fn get_closest_node_index(nodes: &Vec<JNode>, pos: Vec3) -> Option<usize> {
    let mut closest_node_index: Option<usize> = None;
    let mut closest_distance: f32 = 0.0;

    for (i, node) in nodes.iter().enumerate() {
        let distance = get_distance(pos, node.position.into());

        if closest_node_index.is_none() {
            closest_node_index = Some(i);
            closest_distance = distance;
        } else {
            if distance < closest_distance {
                closest_node_index = Some(i);
                closest_distance = distance;
            }
        }
    }

    closest_node_index
}

pub fn new_node(nodes: &Vec<JNode>, pos: (f32, f32, f32), id: String) -> Option<JNode> {
    // check if there is already a node at the given position or has the same id

    for node in nodes {
        if node.id == id {
            return None;
        }

        if node.position == pos {
            return None;
        }
    }

    let node = JNode::new(id, pos, false);

    return Some(node);

}

pub fn new_beam(nodes: &Vec<JNode>, beams: &Vec<JBeam>, id1: String, id2: String) -> Option<JBeam> {
    // check if there is already a beam going from id1 to id2

    let mut node_1_found = false;
    let mut node_2_found = false;

    for node in nodes {
        if node.id == id1 {
            node_1_found = true;
        } else if node.id == id2 {
            node_2_found = true;
        }
    }

    if !node_1_found || !node_2_found {
        println!("Invalid node ids!");
        return None;
    }

    for beam in beams {
        if beam.id1 == id1 && beam.id2 == id2 {
            println!("This beam already exists!");
            return None;
            
        }
    }

    let beam = JBeam::new(id1, id2, false);

    return Some(beam);
}




pub fn load_jbeam_file(contents: &String) -> (String, String) {

    let mut discovering_nodes = false;

    let mut discovering_beams = false;




    let mut node_string = "".to_owned();

    let mut beam_string = "".to_owned();

    for line in contents.clone().split("\n") {


        let line = line.trim();

        if discovering_nodes {

            if line.starts_with(r#"]"#) {
                discovering_nodes = false;
            } else {
                node_string = format!("{}\n{}", node_string, line.clone());
            }

            

            

        }


        if line.starts_with(r#"["id""#) {

            discovering_nodes = true;

        }


    }




    for line in contents.split("\n") {


        let line = line.trim();

        if discovering_beams {

            if line.starts_with(r#"]"#) {
                discovering_beams = false;
            } else {
                beam_string = format!("{}\n{}", beam_string, line.clone());
            }

            if line.starts_with("{") {
                println!("{}", line)
            }

            

            

        }


        if line.starts_with(r#"["id1:", "id2:"]"#) {

            discovering_beams = true;

        }


    }




    return (node_string, beam_string);

}

pub fn write_user_created_nodes(nodes: &Vec<JNode>) {

    let mut export_string = "".to_string();

    for node in nodes {
        if !node.imported {
            export_string = format!("{}{}\n", export_string, node.write("idk".to_string()));

        }
    }
    println!("{}", export_string);
    fs::write("created_nodes.txt", export_string).expect("Could not write to file!");
}

pub fn write_user_created_beams(beams: &Vec<JBeam>) {

    let mut export_string = "".to_string();

    for beam in beams {
        if !beam.imported {
            export_string = format!("{}{}\n", export_string, beam.write());
        }
    }

    println!("{}", export_string);
    fs::write("created_beams.txt", export_string).expect("Could not write to file!");


}

