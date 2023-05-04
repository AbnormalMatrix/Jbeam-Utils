use three_d::egui::{*, self};
use crate::jbeam::{ JNode, JBeam};
use rfd::FileDialog;
use std::path::Path;

pub struct UiVariables {

    selected_group: usize,
    new_group_name: String,
    mod_name: String,
    mod_folder_path: String,

    mod_vehicles: Vec<String>,
    selected_vehicle: usize,
    new_vehicle_name: String,

    jbeams: Vec<String>,
    selected_jbeam: usize,

    group_to_remove: String,


    
}

impl UiVariables {
    pub fn new() -> Self {
        Self {

            selected_group: 0,
            new_group_name: String::new(),
            mod_name: String::new(),
            mod_folder_path: String::new(),
            mod_vehicles: vec!["None".to_string()],
            selected_vehicle: 0,
            new_vehicle_name: "untitled_vehicle".to_string(),

            jbeams: vec!["None".to_string()],
            selected_jbeam: 0,

            group_to_remove: String::new(),

            

        }
    }
}


#[derive(PartialEq)]
pub enum BigGuiMode {
    Parts,
    Nodes,
    ModManager,
}

pub fn show_parts_gui(gui_context: &egui::Context, ui_vars: &mut UiVariables, parts: &mut Vec<String>, nodes: &mut Vec<JNode>, multi_select_idxs: &mut Vec<usize>) {
    CentralPanel::default().show(gui_context, |ui| {
        ui.heading("Parts");
        ui.separator();
        for part in parts.iter() {
            ui.horizontal(|ui| {
                ui.label(part);
                if ui.button("Select").on_hover_text("Select all nodes that make up this part. This will remove your current selection!").clicked() {
                    for node in nodes.iter_mut() {
                        if &node.parent_part == part {
                            node.is_selected = true;
                        } else {
                            node.is_selected = false;
                        }
                    }
                }
            });
        }
    });
}


pub fn show_nodes_gui(gui_context: &egui::Context, ui_vars: &mut UiVariables, selected_node_id: String, selected_node_index: usize, node_selected: bool, nodes: &mut Vec<JNode>) {
    if nodes.len() > 0 {
        CentralPanel::default().show(gui_context, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Selected Node: ").on_hover_text("Changing the properties below will change them for this node.");
                ui.heading(RichText::new(selected_node_id).color(Color32::RED));
            });
            ui.separator();
    
            // show the required properties at the top
    
            ui.horizontal(|ui| {
                ui.label("ID:").on_hover_text("The id of the node. Changing this can severly break things. Only modify if you have a good reason to!");
                ui.text_edit_singleline(&mut nodes[selected_node_index].id);
                ui.separator();
    
                // xyz
    
                ui.label(format!("X: {}, Y: {}, Z:{}", nodes[selected_node_index].position.0, nodes[selected_node_index].position.1, nodes[selected_node_index].position.2)).on_hover_text("The position can be changed in the 3D editor. (TAB)");
    
            });
    
            CollapsingHeader::new("Basic").show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Weight: ").on_hover_text("The weight of the node in kg. As of game version 0.28.0.0 the default weight of a node is 25 kg.");
                    ui.add(DragValue::new(&mut nodes[selected_node_index].node_weight));
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Collision: ").on_hover_text("If the node can collide with the world.");
                    ui.checkbox(&mut nodes[selected_node_index].collision, "");
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Self Collision: ").on_hover_text("If the node can collide with the vehicle.");
                    ui.checkbox(&mut nodes[selected_node_index].self_collision, "");
                });
                ui.separator();
                CollapsingHeader::new("Groups").show(ui, |ui| {
    
                    for group in &nodes[selected_node_index].group {
                        ui.horizontal(|ui| {
                            ui.label(group);
                            if ui.button("-").clicked() {
                                ui_vars.group_to_remove = group.clone();
                            }
                        });
                    }
                    if !ui_vars.group_to_remove.is_empty() {
                        nodes[selected_node_index].group.retain(|g| *g != ui_vars.group_to_remove);
                        ui_vars.group_to_remove = String::new();
                    }
                    ui.horizontal(|ui| {
                        ui.label("New: ").on_hover_text("Create a new group.");
                        ui.text_edit_singleline(&mut ui_vars.new_group_name);
                        if ui.button("+").clicked() {
                            nodes[selected_node_index].group.push(ui_vars.new_group_name.clone());
                            ui_vars.new_group_name = String::new();
                        }
                    });
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Friction Coefficient: ").on_hover_text("Friction of the node. Default = 1.");
                    ui.add(DragValue::new(&mut nodes[selected_node_index].friction_coefficient));
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Surface Coefficient: ").on_hover_text("Makes a node have more or less drag area in ground models with depth.");
                    ui.add(DragValue::new(&mut nodes[selected_node_index].surface_coef));
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Volume Coefficient: ").on_hover_text("Makes a node more ore less buoyant in ground models with depth.");
                    ui.add(DragValue::new(&mut nodes[selected_node_index].volume_coef));
                });
    
            });
    
            CollapsingHeader::new("Advanced").show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Fixed: ").on_hover_text("If the node is fixed in 3D space (Can't move at all).");
                    ui.checkbox(&mut nodes[selected_node_index].fixed, "");
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Break Group: ").on_hover_text("Will break the coupler if the selected [breakGroup] breaks, or break the breakGroup if the coupler breaks.");
                    ui.text_edit_singleline(&mut nodes[selected_node_index].break_group);
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Paired Node: ").on_hover_text("This can be used to i.e. link double tires together.");
                    ui.text_edit_singleline(&mut nodes[selected_node_index].paired_node);
                });
                ui.separator();
    
            });
    
            CollapsingHeader::new("Undocumented - They do something... probably").show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("No Load Coefficient: ");
                    ui.add(DragValue::new(&mut nodes[selected_node_index].no_load_coef));
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Full Load Coefficient: ");
                    ui.add(DragValue::new(&mut nodes[selected_node_index].full_load_coef));
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Stribeck Vel Mult: ");
                    ui.add(DragValue::new(&mut nodes[selected_node_index].stribeck_vel_mult));
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Stribeck Exponent: ");
                    ui.add(DragValue::new(&mut nodes[selected_node_index].stribeck_exponent));
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Softness Coefficient: ");
                    ui.add(DragValue::new(&mut nodes[selected_node_index].softness_coef));
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Tread Coefficient: ");
                    ui.add(DragValue::new(&mut nodes[selected_node_index].tread_coef));
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Load Sensitivity Slope: ");
                    ui.add(DragValue::new(&mut nodes[selected_node_index].load_sensitivity_slope));
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Tag: ");
                    ui.text_edit_singleline(&mut nodes[selected_node_index].tag);
                });
    
                
    
            });
    
        });
    }
    
}

pub fn show_mod_manager(gui_context: &egui::Context, ui_vars: &mut UiVariables) {
    CentralPanel::default().show(gui_context, |ui| {
        ui.heading("Mod Manager");

        ui.separator();

        ui.horizontal(|ui| {

            ui.label("Mod folder path: ").on_hover_text("The location of your unpacked mod folder. Usually this will be somewhere under your documents folder.");
            ui.text_edit_singleline(&mut ui_vars.mod_folder_path);
            if ui.button("browse").clicked() {
                let folder = FileDialog::new()
                    .set_directory(".")
                    .pick_folder()
                    .unwrap()
                    .as_path()
                    .display()
                    .to_string();

                ui_vars.mod_folder_path = folder;

            }

        });




        ui.separator();

        ui.horizontal(|ui| {

            ui.label("Mod name: ");
            ui.text_edit_singleline(&mut ui_vars.mod_name);

            if ui.button("setup/open mod").clicked() {

                let mut pass_checks = true;

                // check if the mod name is empty
                if ui_vars.mod_name == "".to_string() {
                    pass_checks = false;
                }

                // check if the folder exists already

                let new_path = Path::new(&ui_vars.mod_folder_path).join(&ui_vars.mod_name);
                if pass_checks {
                    if new_path.exists() {
                        println!("Mod {} already exists!", ui_vars.mod_name);
                    } else {
                        // create the directory
                        std::fs::create_dir(new_path).unwrap();
                        println!("Created new mod folder {}", ui_vars.mod_name);
                        
                    }
                } else {
                    println!("Mod name can not be blank!");
                }

            }

        });

        ui.separator();
        ui.heading("Vehicles");

        ui.horizontal(|ui| {

            ui.label("select vehicle: ").on_hover_text("The vehicle that is currently being edited.");
            
            egui::ComboBox::from_label("").show_index(
                ui,
                &mut ui_vars.selected_vehicle,
                ui_vars.mod_vehicles.len(),
                |i| ui_vars.mod_vehicles[i].to_owned(),
            );

            ui.label(" or create a new one: ");

            ui.text_edit_singleline(&mut ui_vars.new_vehicle_name);

            if ui.button("create").clicked() {

                // check if the folder exists already

                let new_path = Path::new(&ui_vars.mod_folder_path).join(&ui_vars.mod_name).join(&ui_vars.new_vehicle_name);
                if new_path.exists() {
                    println!("Vehicle {} already exists!", ui_vars.new_vehicle_name);
                    ui_vars.mod_vehicles.push(ui_vars.new_vehicle_name.clone());
                } else {
                    println!("Creating vehicle {}!", ui_vars.new_vehicle_name);
                    std::fs::create_dir(new_path).unwrap();
                    ui_vars.mod_vehicles.push(ui_vars.new_vehicle_name.clone());
                }

            }
            

        });

        ui.separator();

        ui.heading("JBeams").on_hover_text("The jbeams that are a part of the selected vehicle will appear here");

        ui.horizontal(|ui| {

            ui.label("Select JBeam file: ");
            if ui.button("load jbeams").clicked() {
                if Path::new(&ui_vars.mod_folder_path).join(&ui_vars.mod_name).join(&ui_vars.mod_vehicles[ui_vars.selected_vehicle]).exists() {
                    println!("loading");
                    let paths = Path::new(&ui_vars.mod_folder_path).join(&ui_vars.mod_name).join(&ui_vars.mod_vehicles[ui_vars.selected_vehicle]).read_dir().unwrap();
                    let extension = "jbeam";



                    for path in paths {
                        let path = path.unwrap().path();
                        if path.extension().unwrap() == extension {
                            ui_vars.jbeams.push(path.file_name().unwrap().to_str().unwrap().to_string());

                            println!("Found jbeam file: {}", path.file_name().unwrap().to_str().unwrap().to_string());
                        }
                    }
                }
            }

            egui::ComboBox::from_label(" ").show_index(
                ui,
                &mut ui_vars.selected_jbeam,
                ui_vars.jbeams.len(),
                |i| ui_vars.jbeams[i].to_owned(),
            );

            

        });



    });
}

pub fn show_beams_gui(gui_context: &egui::Context, ui_vars: &mut UiVariables, node1_id: String, node2_id: String, beams: &mut JBeam) {
    CentralPanel::default().show(gui_context, |ui| {

        // check if at node1_id and node2_id exist
        if !node1_id.is_empty() && !node2_id.is_empty() {

            // check if there is a beam between the nodes and get the index of the beam

            let mut beam_exists = false;
            

            

        }

    });
}