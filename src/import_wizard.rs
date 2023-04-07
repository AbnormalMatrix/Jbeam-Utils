use three_d::egui::{*, self};
use std::{fs, path::PathBuf};
use rfd::FileDialog;

use crate::jbeam;





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
    rename_dupes: bool,
}

impl Default for ImportVars {
    fn default() -> Self {
        Self {
            path: PathBuf::new(),
            rename_dupes: false,
        }
    }
}
    


pub fn show_import_gui(gui_context: &egui::Context, import_vars: &mut ImportVars, nodes: &mut Vec<jbeam::JNode>, beams: &mut Vec<jbeam::JBeam>, invalid_beams: &mut Vec<jbeam::JBeam>) {
    Window::new("JBeam Importer").anchor(Align2::CENTER_TOP, [0.0, 0.0]).show(&gui_context, |ui| {

        if ui.button("browse...").clicked() {

            let file_dialog = FileDialog::new().add_filter("JBeam", &["jbeam"]);
            let result = file_dialog.pick_file();

            if let Some(path) = result {
                import_vars.path = path;

            }
            
        }
        
        ui.label(format!("Path: {}", import_vars.path.display()));



        ui.separator();

        ui.checkbox(&mut import_vars.rename_dupes, "Rename Duplicates");

        if ui.button("import").clicked() {
            // load the file
            let file_contents = fs::read_to_string(&import_vars.path).unwrap();

            let mut new_nodes = jbeam::parse_nodes(file_contents.clone());

            

            // handle duplicate nodes

            for new_node in new_nodes.iter_mut() {

                let mut duplicate_flag = false;

                for existing_node in nodes.iter() {
                    
                    if new_node.id == existing_node.id {
                        duplicate_flag = true;
                        break;
                    }
                }

                if duplicate_flag {
                    if import_vars.rename_dupes {
                        // just stick a d on the end for now
                        new_node.id = format!("{}{}", new_node.id, "d");
                        nodes.push(new_node.clone());
                    }
                } else {
                    nodes.push(new_node.clone());
                }
            }
            
            let (mut new_valid_beams, mut new_invalid_beams) = jbeam::parse_beams(file_contents, nodes);

            beams.append(&mut new_valid_beams);
            invalid_beams.append(&mut new_invalid_beams);

        }


    });
}