use three_d::egui::{*, self};
use std::{fs, path::PathBuf};
use rfd::FileDialog;

use pest::Parser;

use crate::jbeam;



#[derive(Parser)]
#[grammar = "jbeam.pest"]
pub struct JParser;

struct JPart {
    name: String,
    contents: String,
    selected: bool,
}



fn get_parts(path: &PathBuf) -> Result<Vec<JPart>, &str> {

    // use rust file dialog to prompt the user to open a jbeam file



    let contents = fs::read_to_string(path).expect("Could not read file!");


    if let Ok(jbeam) = JParser::parse(Rule::jbeam, &contents) {
        if let Some(jbeam) = JParser::parse(Rule::jbeam, &contents).unwrap().next() {
            // jbeam.as_rule() is the main "json" object that would hold the parts
    
            let mut parts: Vec<JPart> = Vec::new();
    
            
    
            for part in jbeam.into_inner() {
                // each part is a pair
    
                if part.as_rule() == Rule::pair {
                    let part_name = part.clone().into_inner().nth(0).unwrap().as_str().to_string();
                    let part_contents = part.into_inner().nth(1).unwrap().as_str().to_string();
    
                    parts.push(JPart { name: part_name, contents: part_contents, selected: false })
                }
            }
    
            Ok(parts)
        } else {
            Err("Could not parse file!")
        }
    } else {
        Err("Could not parse file!")
    }




    

}

// this is probably a super janky way of doiong this but whatever


// the old code: 

// let (node_string, beam_string) = jbeam::load_jbeam_file();

// let mut new_nodes = jbeam::parse_jbeam(node_string);

// let mut new_beams = jbeam::parse_beams(beam_string, &new_nodes);

// nodes.append(&mut new_nodes);
// beams.append(&mut new_beams.0);
// invalid_beams.append(&mut new_beams.1);


pub struct ImportVars {
    path: PathBuf,
    part_list: Vec<JPart>,
}

impl Default for ImportVars {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            part_list: Vec::new(),
        }
    }
}
    


pub fn show_import_gui(gui_context: &egui::Context, import_vars: &mut ImportVars, nodes: &mut Vec<jbeam::JNode>, beams: &mut Vec<jbeam::JBeam>, invalid_beams: &mut Vec<String>) {
    Window::new("JBeam Importer").anchor(Align2::CENTER_TOP, [0.0, 0.0]).show(&gui_context, |ui| {

        if ui.button("browse...").clicked() {

            let file_dialog = FileDialog::new().add_filter("JBeam", &["jbeam"]);
            let result = file_dialog.pick_file();

            if let Some(path) = result {
                import_vars.path = path;

                if let Ok(parts) = get_parts(&import_vars.path) {
                    import_vars.part_list = parts;
                } else {
                    import_vars.part_list = Vec::new();
                    ui.label("Error parsing file! (probably not a jbeam file)");
                }
            }
            
        }
        
        ui.label(format!("Path: {}", import_vars.path.display()));



        ui.separator();

        for part in import_vars.part_list.iter_mut() {

            ui.horizontal(|ui| {
                ui.checkbox(&mut part.selected, "");
                ui.label(&part.name);
            });


        }

        if import_vars.part_list.len() > 0 {
            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("all").clicked() {
                    for part in import_vars.part_list.iter_mut() {
                        part.selected = true;
                    }
                }
                if ui.button("none").clicked() {
                    for part in import_vars.part_list.iter_mut() {
                        part.selected = false;
                    }
                }
            });

            ui.separator();

            if ui.button("import").clicked() {
                for part in import_vars.part_list.iter() {
                    if part.selected {
                        println!("importing part: {}", part.name);
                        let (node_string, beam_string) = jbeam::load_jbeam_file(&part.contents);
                        let mut new_nodes = jbeam::parse_jbeam(node_string);
                        let mut new_beams = jbeam::parse_beams(beam_string, &new_nodes);
                        nodes.append(&mut new_nodes);
                        beams.append(&mut new_beams.0);
                        invalid_beams.append(&mut new_beams.1);
                    }
                }
            }

        }

    });
}