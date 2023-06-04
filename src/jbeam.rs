use std::fs;
use three_d::*;
use regex::Regex;
use rfd::FileDialog;


use crate::export_beams::write_beam;

use pest::{Parser, iterators::Pairs};

#[derive(Parser)]
#[grammar = "jbeam.pest"]
pub struct JBeamParser;

fn parse_node_modifiers(rule: Pairs<Rule>, node: JNode) -> JNode {

    let mut node = node.clone();

    

    for rule in rule {
        for rule in rule.into_inner() {
            // these are the actual modifier rules
            // println!("{:?}", rule.as_rule());

            match rule.as_rule() {
                Rule::node_weight => {

                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.node_weight = rule.as_str().parse().unwrap();
                            println!("nodeWeight: {}", rule.as_str());
                        }
                    }

                },
                Rule::node_collision => {


                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::boolean {
                            node.collision = rule.as_str().parse().unwrap();
                        }
                    }


                },
                Rule::node_self_collision => {


                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::boolean {
                            node.self_collision = rule.as_str().parse().unwrap();
                        }
                    }

                },
                Rule::node_group => {


                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            // check if the trimmed string is empty
                            if rule.as_str().to_string().replace(r#"""#, "").is_empty() {
                                // clear the vector and continue

                                node.group.clear();
                                continue;
                            }
                            // add the trimmed string to the vector if it is not already in it
                            if !node.group.contains(&rule.as_str().to_string().replace(r#"""#, "")) {
                                node.group.push(rule.as_str().to_string().replace(r#"""#, ""));
                            }

                        }
                    }
                },
                Rule::node_friction_coef => {

                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.friction_coefficient = rule.as_str().parse().unwrap();
                        }
                    }

                },
                Rule::node_material => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            node.node_material = rule.as_str().to_string().replace(r#"""#, "");
                        }
                    }
                },
                Rule::node_fixed => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::boolean {
                            node.fixed = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_coupler_strength => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.coupler_strength = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_coupler_tag => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            node.coupler_tag = 0.0;
                        }
                    }
                },
                Rule::node_coupler_radius => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.coupler_radius = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_break_group => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            node.break_group = rule.as_str().to_string().replace(r#"""#, "");
                        }
                    }
                },
                Rule::node_coupler_lock => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::boolean {
                            node.coupler_lock = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_import_electrics => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            node.import_electrics.push(rule.as_str().to_string().replace(r#"""#, ""));
                        }
                    }
                },
                Rule::node_import_inputs => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            node.import_inputs.push(rule.as_str().to_string().replace(r#"""#, ""));
                        }
                    }
                },
                Rule::node_surface_coef => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.surface_coef = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_volume_coef => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.volume_coef = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_no_load_coef => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.no_load_coef = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_full_load_coef => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.full_load_coef = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_stribeck_exponent => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.stribeck_exponent = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_stribeck_vel_mult => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.stribeck_vel_mult = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_softness_coef => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.softness_coef = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_tread_coef => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.tread_coef = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_tag => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            node.tag = rule.as_str().to_string().replace(r#"""#, "");
                        }
                    }
                },
                Rule::node_load_sensitivity_slope => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            node.load_sensitivity_slope = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::node_paired_node => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            node.paired_node = rule.as_str().to_string().replace(r#"""#, "");
                        }
                    }
                },
                Rule::node_chem_energy => {

                },
                Rule::node_burn_rate => {

                },
                Rule::node_flash_point => {

                },
                Rule::node_spec_heat => {

                },
                Rule::node_smoke_point => {

                },
                Rule::node_self_ignition_coef => {

                },
                Rule::node_engine_group => {

                },
                _=> ()
            }

        }
    }

    // println!("{:#?}", node);

    return node;

    
}

pub fn parse_nodes(unparsed_file: String) -> (Vec<JNode>, Vec<String>) {
    
    
    let file = JBeamParser::parse(Rule::parts, &unparsed_file).expect("Failed to parse JBeam!");

    let mut nodes: Vec<JNode> = Vec::new();
    let mut part_name = String::new();

    // list of part names:
    let mut part_names: Vec<String> = Vec::new();

    let mut node = JNode::new();
    for rule in file {


        if let Some(name) = rule.clone().into_inner().next() {
            part_name = name.as_str().to_string().replace(r#"""#, "");
            // if the part name is not in the list, add it
            if !part_names.contains(&part_name) {
                println!("{}", part_name);
                part_names.push(part_name.clone());
            }
        }

        for rule in rule.into_inner() {
            if rule.as_rule() == Rule::nodes {


                

                for rule in rule.into_inner() {
                    match rule.as_rule() {
                        Rule::node => {
                            

                            let mut coord_counter = 0;
                            for rule in rule.into_inner() {
                                println!("Node Found!");
                                match rule.as_rule() {
                                    Rule::string => {
                                        node.id = rule.as_str().to_string().replace(r#"""#, "");

                                    },
                                    Rule::number => {
                                        let value: f32 = rule.as_str().parse().unwrap();
                                        println!("{}", value);
                                        match coord_counter {
                                            0 => {
                                                node.position.0 = -1.0 * value;
                                            },
                                            1 => {
                                                node.position.2 = value;
                                            },
                                            2 => {
                                                node.position.1 = value;
                                            },
                                            _=> ()
                                        }
                                        coord_counter += 1;
                                    },
                                    Rule::node_modifiers => {
                                        node = parse_node_modifiers(rule.into_inner(), node)
                                    }
                                    _=> ()
                                }
                            }

                            println!("{:?}", node);
                            node.parent_part = part_name.clone();
                            nodes.push(node.clone());
                            // node = JNode::new();

                        },
                        Rule::node_modifiers => {
                            node = parse_node_modifiers(rule.into_inner(), node);
                        },
                        _ => ()
                    }
                }
            }
        }
    }

    (nodes, part_names)
}







#[derive(Debug, Clone)]
pub struct JNode {
    pub id: String,
    pub position: (f32, f32, f32),
    pub imported: bool,

    pub parent_part: String,

    // optional arguments
    pub node_weight: f64,
    pub collision: bool,
    pub self_collision: bool,
    pub group: Vec<String>,
    pub friction_coefficient: f32,
    pub node_material: String,
    pub fixed: bool,
    pub coupler_strength: f32,
    pub coupler_tag: f32,
    pub coupler_radius: f32,
    pub break_group: String,
    pub coupler_lock: bool,
    pub import_electrics: Vec<String>,
    pub import_inputs: Vec<String>,
    pub surface_coef: f32,
    pub volume_coef: f32,
    pub no_load_coef: f32,
    pub full_load_coef: f32,
    pub stribeck_exponent: f32,
    pub stribeck_vel_mult: f32,
    pub softness_coef: f32,
    pub tread_coef: f32,
    pub tag: String,
    pub load_sensitivity_slope: f32,
    pub paired_node: String,

    // editor specific thing... these should NOT get exported to BeamNG
    pub is_selected: bool,


}
impl JNode {
    pub fn new() -> Self {
        Self {
            id: "".to_owned(),
            position: (0.0, 0.0, 0.0),
            imported: true,
            parent_part: "".to_owned(),
            node_weight: 25.0,
            collision: false,
            self_collision: false,
            group: Vec::new(),
            friction_coefficient: 1.0,
            node_material: "|NM_METAL".to_owned(),
            fixed: false,
            // highest f32 value
            coupler_strength: 340282346638528859811704183484516925440.0,
            coupler_tag: 1.0,
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

        let mut import_electrics_str = "[".to_string();

        for ie in &self.import_electrics {
            import_electrics_str = format!(r#""{}", "#, ie);
        }
        import_electrics_str = format!("{}]", import_electrics_str);

        let mut import_inputs_str = "[".to_string();

        for ii in &self.import_inputs {
            import_inputs_str = format!(r#""{}", "#, ii);
        }
        import_inputs_str = format!("{}]", import_inputs_str);

        let data = format!(r#"["{}", {}, {}, {}, {{"nodeWeight":{}, "collision":{}, "selfCollision":{}, "frictionCoef":{}, {}, "fixed":{}, "breakGroup":"{}", "importElectrics":{}, "importInputs":{}, "surfaceCoef":{}, "volumeCoef":{}, "noLoadCoef":{}, "fullLoadCoef":{}, "stribeckExponent":{}, "stribeckVelMult":{}, "softnessCoef":{}, "treadCoef":{}, "loadSensitivitySlope":{} }}],"#,
            self.id,
            -self.position.0,
            self.position.2,
            self.position.1,
            self.node_weight,
            self.collision,
            self.self_collision,
            self.friction_coefficient,
            group_str,
            // self.node_material,
            self.fixed,

            self.break_group,

            import_electrics_str,
            import_inputs_str,
            self.surface_coef,
            self.volume_coef,
            self.no_load_coef,
            self.full_load_coef,
            self.stribeck_exponent,
            self.stribeck_vel_mult,
            self.softness_coef,
            self.tread_coef,

            self.load_sensitivity_slope,

        );
        // println!("{}", data);

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
    pub imported: bool,
    pub node1_idx: usize,
    pub node2_idx: usize,
    // optional arguments
    pub beam_type: String,
    pub beam_spring: f32,
    pub beam_damp: f32,
    pub beam_strength: String,
    pub beam_deform: f32,
    pub beam_compression: f32,
    pub beam_compression_range: f32,
    pub beam_compression_time: f32,
    pub break_group: String,
    pub break_group_type: String,
    pub name: String,
    pub damp_cutoff_hz: f32,
    pub deform_limit: f32,
    pub deform_limt_expansion: f32,
    pub optional: bool,
    pub deform_group: String,
    pub deformation_trigger_ratio: f32,
    pub sound_file: String,
    pub color_factor: f32,
    pub attack_factor: f32,
    pub volume_factor: f32,
    pub decay_factor: f32,
    pub pitch_factor: f32,
    pub max_stress: f32,

}

impl JBeam {
    pub fn new() -> Self {
        Self {
            id1: String::new(),
            id2: String::new(),
            imported: true,
            node1_idx: 0,
            node2_idx: 0,
            beam_type: "|NORMAL".to_owned(),
            beam_spring: 4300000.0,
            beam_damp: 580.0,
            beam_strength: "FLT_MAX".to_owned(),
            beam_deform: 220000.0,
            beam_compression: 1.0,
            beam_compression_range: 0.0,
            beam_compression_time: 0.0,
            break_group: String::new(),
            break_group_type: "0".to_owned(),
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

        // println!("{:#?}", self);

        write_beam(self)

    }

    pub fn get_3d_object(&self, context: &Context, nodes: &Vec<JNode>) -> Sprites {
        let pos1 = nodes[get_node_by_id(self.id1.clone(), nodes).unwrap()].position;
        let pos2 = nodes[get_node_by_id(self.id2.clone(), nodes).unwrap()].position;

        // let pos1 = nodes[self.node1_idx].position;
        // let pos2 = nodes[self.node2_idx].position;


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








fn parse_beam_modifiers(rule: Pairs<Rule>, beam: JBeam) -> JBeam {

    let mut beam = beam.clone();

    for rule in rule {
        for rule in rule.into_inner() {
            // these are the actual beam modifiers
            match rule.as_rule() {
                Rule::beam_beam_type => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            beam.beam_type = rule.as_str().to_string().replace(r#"""#, "");
                            println!("{}", beam.beam_type);
                        }
                    }
                },
                Rule::beam_beam_spring => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.beam_spring = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_beam_damp => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.beam_damp = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_beam_strength => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.beam_strength = rule.as_str().to_string().replace(r#"""#, "");
                        } else if rule.as_rule() == Rule::string {
                            beam.beam_strength = rule.as_str().to_string().replace(r#"""#, "");
                        }
                    }
                },
                Rule::beam_beam_deform => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.beam_deform = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_beam_precompression => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.beam_compression = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_beam_precompression_range => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.beam_compression_range = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_beam_precompression_time => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.beam_compression_time = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_break_group => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            beam.break_group = rule.as_str().to_string().replace(r#"""#, "");
                        }
                    }
                },
                Rule::beam_break_group_type => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            beam.break_group_type = rule.as_str().to_string().replace(r#"""#, "");
                        }
                    }
                },
                Rule::beam_name => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            beam.name = rule.as_str().to_string().replace(r#"""#, "");
                        }
                    }
                },
                Rule::beam_damp_cuttoff_hz => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.damp_cutoff_hz = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_deform_limit => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.deform_limit = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_deform_limit_expansion => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.deform_limt_expansion = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_optional => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::boolean {
                            beam.optional = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_deform_group => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            beam.deform_group = rule.as_str().to_string().replace(r#"""#, "");
                        }
                    }
                },
                Rule::beam_deformation_trigger_ratio => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.deformation_trigger_ratio = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_sound_file => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::string {
                            beam.sound_file = rule.as_str().to_string().replace(r#"""#, "");
                        }
                    }
                },
                Rule::beam_color_factor => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.color_factor = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_attack_factor => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.attack_factor = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_volume_factor => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.volume_factor = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_decay_factor => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.decay_factor = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_pitch_factor => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.pitch_factor = rule.as_str().parse().unwrap();
                        }
                    }
                },
                Rule::beam_max_stress => {
                    for rule in rule.into_inner() {
                        if rule.as_rule() == Rule::number {
                            beam.max_stress = rule.as_str().parse().unwrap();
                        }
                    }
                },
                _=> ()
            }
        }
    }

    beam

}

pub fn parse_beams(unparsed_file: String, nodes: &Vec<JNode>) -> (Vec<JBeam>, Vec<JBeam>) {
    let file = JBeamParser::parse(Rule::parts, &unparsed_file).expect("Failed to parse JBeam!");

    let mut beams: Vec<JBeam> = Vec::new();

    let mut invalid_beams: Vec<JBeam> = Vec::new();

    let mut beam = JBeam::new();

    for rule in file {

        for rule in rule.into_inner() {
            if rule.as_rule() == Rule::beams {


                

                for rule in rule.into_inner() {
                    match rule.as_rule() {
                        Rule::beam => {


                            for rule in rule.into_inner() {
                                match rule.as_rule() {
                                    Rule::string => {
                                        if beam.id1.is_empty() {
                                            beam.id1 = rule.as_str().to_string().replace(r#"""#, "");
                                        } else {
                                            beam.id2 = rule.as_str().to_string().replace(r#"""#, "");
                                        }
                                    },
                                    Rule::beam_modifiers => {
                                        beam = parse_beam_modifiers(rule.into_inner(), beam);
                                    }
                                    _=> ()
                                }
                            }

                            // check if nodes exist

                            let mut node1_exists = false;
                            let mut node2_exists = false;



                            for (i, node) in nodes.iter().enumerate() {
                                if node.id == beam.id1 {
                                    node1_exists = true;
                                    beam.node1_idx = i;
                                }
                                if node.id == beam.id2 {
                                    node2_exists = true;
                                    beam.node2_idx = i;
                                }
                            }

                            if node1_exists && node2_exists {
                                beams.push(beam.clone());
                                // println!("Valid beam");
                            } else {
                                invalid_beams.push(beam.clone());
                                // println!("Invalid beam");
                            }



                            // beam = JBeam::new();

                            beam.id1 = String::new();
                            beam.id2 = String::new();

                        },
                        Rule::beam_modifiers => {
                            beam = parse_beam_modifiers(rule.into_inner(), beam);
                        },
                        _ => ()
                    }
                }
            }
        }
    }



    (beams, invalid_beams)
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

    let mut node = JNode::new();

    node.id = id;
    node.position = pos;

    return Some(node);

}

pub fn new_beam(nodes: &Vec<JNode>, beams: &Vec<JBeam>, id1: String, id2: String) -> Option<JBeam> {
    // check if there is already a beam going from id1 to id2

    let mut node_1_found = false;
    let mut node_2_found = false;

    let mut node_1_idx: usize = 0;
    let mut node_2_idx: usize = 0;

    for (i, node) in nodes.iter().enumerate() {
        if node.id == id1 {
            node_1_found = true;
            node_1_idx = i;

        } else if node.id == id2 {
            node_2_found = true;
            node_2_idx = i;
        }
    }

    if !node_1_found || !node_2_found {
        // println!("Invalid node ids!");
        return None;
    }

    for beam in beams {
        if beam.id1 == id1 && beam.id2 == id2 {
            // println!("This beam already exists!");
            return None;
            
        }
    }

    let mut beam = JBeam::new();
    beam.id1 = id1;
    beam.id2 = id2;
    beam.node1_idx = node_1_idx;
    beam.node2_idx = node_2_idx;
    beam.imported = false;

    return Some(beam);
}

pub fn new_tri(nodes: &Vec<JNode>, tris: &Vec<JTri>, n1: &usize, n2: &usize, n3: &usize) -> Result<JTri, String> {
    
    // check if the triangle already exists

    let mut tri_valid = true;

    // check if the triangle is valid by checking if the nodes are the same regardless of order

    let id1 = nodes[*n1].id.clone();
    let id2 = nodes[*n2].id.clone();
    let id3 = nodes[*n3].id.clone();


    

    for tri in tris {
        let tri_ids = vec![nodes[tri.id1].id.clone(), nodes[tri.id2].id.clone(), nodes[tri.id3].id.clone()];

        if tri_ids.contains(&id1) && tri_ids.contains(&id2) && tri_ids.contains(&id3) {
            tri_valid = false;
        }
    }

    if tri_valid {
        let mut tri = JTri::new();
        tri.id1 = *n1;
        tri.id2 = *n2;
        tri.id3 = *n3;

        return Ok(tri);
    } else {
        return Err("This triangle already exists!".to_string());
    }

}




pub fn write_user_created_nodes(nodes: &Vec<JNode>) {

    let mut export_string = "".to_string();

    for node in nodes {
        if !node.imported {
            export_string = format!("{}{}\n", export_string, node.write("idk".to_string()));

        }
    }
    // println!("{}", export_string);
    fs::write("created_nodes.txt", export_string).expect("Could not write to file!");
}

pub fn write_user_created_beams(beams: &Vec<JBeam>) {

    let mut export_string = "".to_string();

    for beam in beams {
        if !beam.imported {
            export_string = format!("{}{}\n", export_string, beam.write());
        }
    }

    // println!("{}", export_string);
    fs::write("created_beams.txt", export_string).expect("Could not write to file!");


}



#[derive(Debug, Clone)]
pub struct JTri {
    pub id1: usize,
    pub id2: usize,
    pub id3: usize,
}

impl JTri {
    pub fn new() -> JTri {
        JTri {
            id1: 0,
            id2: 0,
            id3: 0,
        }
    }
    pub fn get_3d_object(&self, context: &Context, nodes: &Vec<JNode>) -> Gm<Mesh, ColorMaterial> {
        let positions = vec![
            vec3(nodes[self.id1].position.0, nodes[self.id1].position.1, nodes[self.id1].position.2),
            vec3(nodes[self.id2].position.0, nodes[self.id2].position.1, nodes[self.id2].position.2),
            vec3(nodes[self.id3].position.0, nodes[self.id3].position.1, nodes[self.id3].position.2),
        ];
        let colors = vec![
            Color::new(255, 0, 0, 255), // bottom right
            Color::new(0, 255, 0, 255), // bottom left
            Color::new(0, 0, 255, 255), // top
        ];

        let mesh = CpuMesh {
            positions: Positions::F32(positions),
            colors: Some(colors),
            ..Default::default()
        };

        Gm::new(Mesh::new(&context, &mesh), ColorMaterial::default())

    }

    pub fn write(&self) -> String {
        format!("[{}, {}, {}]", self.id1, self.id2, self.id3)
    }
}


pub fn parse_tris(unparsed_file: String, nodes: &Vec<JNode>) -> Vec<JTri> {
    let file = JBeamParser::parse(Rule::parts, &unparsed_file).expect("Failed to parse JBeam!");

    let mut tris: Vec<JTri> = Vec::new();



    let mut tri = JTri::new();

    for rule in file {

        for rule in rule.into_inner() {
            if rule.as_rule() == Rule::triangles {

                
                

                for rule in rule.into_inner() {
                    if rule.as_rule() == Rule::triangle {
                        // println!("{:?}", rule.as_rule());
                        let mut nodes_found = (false, false, false);
                        for rule in rule.into_inner() {
                            match rule.as_rule() {

                                Rule::string => {
                                    if nodes_found.0 == false {
                                        let idx = get_node_by_id(rule.as_str().to_string().replace(r#"""#, ""), nodes);
                                        // println!("{:?}", idx);
                                        if let Some(idx) = idx {
                                            tri.id1 = idx;
                                            nodes_found.0 = true;
                                        }
                                    } else if nodes_found.1 == false {
                                        let idx = get_node_by_id(rule.as_str().to_string().replace(r#"""#, ""), nodes);
                                        if let Some(idx) = idx {
                                            tri.id2 = idx;
                                            nodes_found.1 = true;
                                        }
                                    } else if nodes_found.2 == false {
                                        let idx = get_node_by_id(rule.as_str().to_string().replace(r#"""#, ""), nodes);
                                        if let Some(idx) = idx {
                                            tri.id3 = idx;
                                            nodes_found.2 = true;
                                        }
                                    }
                                },
        
                                _=> ()
        
                            }
                        }
                        if nodes_found.0 && nodes_found.1 && nodes_found.2 {
                            tris.push(tri.clone());
                            // println!("{:#?}", tri);
                        }
                        tri = JTri::new();
                    }

                }


            }
        }
    }

    tris
    
}

#[derive(Debug)]
pub enum SelectError {
    NoBeam
}

// try to select a beam based on two input nodes
pub fn try_select_beam(id1: &String, id2: &String, beams: &Vec<JBeam>) -> Result<usize, SelectError> {

    for (i, beam) in beams.iter().enumerate() {
        if (&beam.id1 == id1 && &beam.id2 == id2) || (&beam.id2 == id1 && &beam.id1 == id2) {
            return Ok(i);
        }
    }

    Err(SelectError::NoBeam)

}

pub fn subdivide_beam(beams: &mut Vec<JBeam>, selected_beam: &usize, nodes: &mut Vec<JNode>) {

    // get the positions of the nodes

    let node_1_pos = nodes[get_node_by_id(beams[*selected_beam].id1.clone(), nodes).unwrap()].position;
    let node_2_pos = nodes[get_node_by_id(beams[*selected_beam].id2.clone(), nodes).unwrap()].position;

    // get the node ids

    let node_1_id = nodes[get_node_by_id(beams[*selected_beam].id1.clone(), nodes).unwrap()].id.clone();
    let node_2_id = nodes[get_node_by_id(beams[*selected_beam].id2.clone(), nodes).unwrap()].id.clone();

    // get the name of node_1 and add subd to the end

    let new_node_name = format!("{}_subd", nodes[get_node_by_id(beams[*selected_beam].id1.clone(), nodes).unwrap()].id);

    // get the midpoint

    let midpoint_x = (node_1_pos.0 + node_2_pos.0) / 2.0;
    let midpoint_y = (node_1_pos.1 + node_2_pos.1) / 2.0;
    let midpoint_z = (node_1_pos.2 + node_2_pos.2) / 2.0;

    let midpoint = (midpoint_x, midpoint_y, midpoint_z);

    let new_node = new_node(nodes, midpoint, new_node_name.clone()).unwrap();

    nodes.push(new_node);

    beams.remove(*selected_beam);

    // create a beam connecting node_1 to new_node

    let new_beam1 = new_beam(nodes, beams, node_1_id.clone(), new_node_name.clone()).unwrap();
    let new_beam2 = new_beam(nodes, beams, node_2_id.clone(), new_node_name.clone()).unwrap();

    beams.push(new_beam1);
    beams.push(new_beam2);

}