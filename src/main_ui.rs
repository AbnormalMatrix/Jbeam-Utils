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
    Beams,
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


pub fn show_nodes_gui(gui_context: &egui::Context, ui_vars: &mut UiVariables, selected_nodes: &Vec<usize>, node_selected: bool, nodes: &mut Vec<JNode>) {
    if nodes.len() > 0 {
        CentralPanel::default().show(gui_context, |ui| {
            ui.horizontal(|ui| {
                ui.heading(RichText::new(format!("{} nodes selected", selected_nodes.len())).color(Color32::RED));
            });
            ui.separator();
    
            // show the required properties at the top
    
            // ui.horizontal(|ui| {
            //     ui.label("ID:").on_hover_text("The id of the node. Changing this can severly break things. Only modify if you have a good reason to!");
            //     ui.text_edit_singleline(&mut nodes[selected_node_index].id);
            //     ui.separator();
    
            //     // xyz
    
            //     ui.label(format!("X: {}, Y: {}, Z:{}", nodes[selected_node_index].position.0, nodes[selected_node_index].position.1, nodes[selected_node_index].position.2)).on_hover_text("The position can be changed in the 3D editor. (TAB)");
    
            // });
    
            CollapsingHeader::new("Basic").show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Weight: ").on_hover_text("The weight of the node in kg. As of game version 0.28.0.0 the default weight of a node is 25 kg.");
                    if ui.add(DragValue::new(&mut nodes[*selected_nodes.last().unwrap_or(&0)].node_weight)).changed(){
                        let new_value = nodes[*selected_nodes.last().unwrap_or(&0)].node_weight.clone();
                        for node in nodes.iter_mut() {
                            if node.is_selected {
                                node.node_weight = new_value;
                            }
                        }
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Collision: ").on_hover_text("If the node can collide with the world.");
                    if ui.checkbox(&mut nodes[*selected_nodes.last().unwrap_or(&0)].collision, "").changed() {
                        let new_value = nodes[*selected_nodes.last().unwrap_or(&0)].collision.clone();
                        for node in nodes.iter_mut() {
                            if node.is_selected {
                                node.collision = new_value;
                            }
                        }
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Self Collision: ").on_hover_text("If the node can collide with the vehicle.");
                    if ui.checkbox(&mut nodes[*selected_nodes.last().unwrap_or(&0)].self_collision, "").changed() {
                        let new_value = nodes[*selected_nodes.last().unwrap_or(&0)].self_collision.clone();
                        for node in nodes.iter_mut() {
                            if node.is_selected {
                                node.self_collision = new_value;
                            }
                        }
                    }
                });
                ui.separator();
                CollapsingHeader::new("Groups").show(ui, |ui| {
    
                    for group in &nodes[*selected_nodes.last().unwrap_or(&0)].group {
                        ui.horizontal(|ui| {
                            ui.label(group);
                            if ui.button("-").clicked() {
                                ui_vars.group_to_remove = group.clone();
                            }
                        });
                    }
                    if !ui_vars.group_to_remove.is_empty() {
                        nodes[*selected_nodes.last().unwrap_or(&0)].group.retain(|g| *g != ui_vars.group_to_remove);
                        ui_vars.group_to_remove = String::new();
                    }
                    ui.horizontal(|ui| {
                        ui.label("New: ").on_hover_text("Create a new group.");
                        ui.text_edit_singleline(&mut ui_vars.new_group_name);
                        if ui.button("+").clicked() {
                            nodes[*selected_nodes.last().unwrap_or(&0)].group.push(ui_vars.new_group_name.clone());
                            ui_vars.new_group_name = String::new();
                        }
                    });
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Friction Coefficient: ").on_hover_text("Friction of the node. Default = 1.");
                    if ui.add(DragValue::new(&mut nodes[*selected_nodes.last().unwrap_or(&0)].friction_coefficient)).changed() {
                        let new_value = nodes[*selected_nodes.last().unwrap_or(&0)].friction_coefficient.clone();
                        for node in nodes.iter_mut() {
                            if node.is_selected {
                                node.friction_coefficient = new_value;
                            }
                        }
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Surface Coefficient: ").on_hover_text("Makes a node have more or less drag area in ground models with depth.");
                    if ui.add(DragValue::new(&mut nodes[*selected_nodes.last().unwrap_or(&0)].surface_coef)).changed() {
                        let new_value = nodes[*selected_nodes.last().unwrap_or(&0)].surface_coef.clone();
                        for node in nodes.iter_mut() {
                            if node.is_selected {
                                node.surface_coef = new_value;
                            }
                        }
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Volume Coefficient: ").on_hover_text("Makes a node more ore less buoyant in ground models with depth.");
                    if ui.add(DragValue::new(&mut nodes[*selected_nodes.last().unwrap_or(&0)].volume_coef)).changed() {
                        let new_value = nodes[*selected_nodes.last().unwrap_or(&0)].volume_coef.clone();
                        for node in nodes.iter_mut() {
                            if node.is_selected {
                                node.volume_coef = new_value;
                            }
                        }
                    }
                });
    
            });
    
            CollapsingHeader::new("Advanced").show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Fixed: ").on_hover_text("If the node is fixed in 3D space (Can't move at all).");
                    if ui.checkbox(&mut nodes[*selected_nodes.last().unwrap_or(&0)].fixed, "").changed() {
                        let new_value = nodes[*selected_nodes.last().unwrap_or(&0)].fixed.clone();
                        for node in nodes.iter_mut() {
                            if node.is_selected {
                                node.fixed = new_value;
                            }
                        }
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Break Group: ").on_hover_text("Will break the coupler if the selected [breakGroup] breaks, or break the breakGroup if the coupler breaks.");
                    if ui.text_edit_singleline(&mut nodes[*selected_nodes.last().unwrap_or(&0)].break_group).changed() {
                        let new_value = nodes[*selected_nodes.last().unwrap_or(&0)].break_group.clone();
                        for node in nodes.iter_mut() {
                            if node.is_selected {
                                node.break_group = new_value.clone();
                            }
                        }
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Paired Node: ").on_hover_text("This can be used to i.e. link double tires together.");
                    if ui.text_edit_singleline(&mut nodes[*selected_nodes.last().unwrap_or(&0)].paired_node).changed() {
                        let new_value = nodes[*selected_nodes.last().unwrap_or(&0)].paired_node.clone();
                        for node in nodes.iter_mut() {
                            if node.is_selected {
                                node.paired_node = new_value.clone();
                            }
                        }
                    }
                });
                ui.separator();
    
            });
    
            CollapsingHeader::new("Undocumented - They do something... probably").show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("No Load Coefficient: ");
                    if ui.add(DragValue::new(&mut nodes[*selected_nodes.last().unwrap_or(&0)].no_load_coef)).changed() {
                        let new_value = nodes[*selected_nodes.last().unwrap_or(&0)].no_load_coef.clone();
                        for node in nodes.iter_mut() {
                            if node.is_selected {
                                node.no_load_coef = new_value;
                            }
                        }
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Full Load Coefficient: ");
                    if ui.add(DragValue::new(&mut nodes[*selected_nodes.last().unwrap_or(&0)].full_load_coef)).changed() {
                        let new_value = nodes[*selected_nodes.last().unwrap_or(&0)].full_load_coef.clone();
                        for node in nodes.iter_mut() {
                            if node.is_selected {
                                node.full_load_coef = new_value;
                            }
                        }
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Stribeck Vel Mult: ");
                    if ui.add(DragValue::new(&mut nodes[*selected_nodes.last().unwrap_or(&0)].stribeck_vel_mult)).changed() {
                        let new_value = nodes[*selected_nodes.last().unwrap_or(&0)].stribeck_vel_mult.clone();
                        for node in nodes.iter_mut() {
                            if node.is_selected {
                                node.stribeck_vel_mult = new_value;
                            }
                        }
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Stribeck Exponent: ");
                    if ui.add(DragValue::new(&mut nodes[*selected_nodes.last().unwrap_or(&0)].stribeck_exponent)).changed() {
                        let new_value = nodes[*selected_nodes.last().unwrap_or(&0)].stribeck_exponent.clone();
                        for node in nodes.iter_mut() {
                            if node.is_selected {
                                node.stribeck_exponent = new_value;
                            }
                        }
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Softness Coefficient: ");
                    if ui.add(DragValue::new(&mut nodes[*selected_nodes.last().unwrap_or(&0)].softness_coef)).changed() {
                        let new_value = nodes[*selected_nodes.last().unwrap_or(&0)].softness_coef.clone();
                        for node in nodes.iter_mut() {
                            if node.is_selected {
                                node.softness_coef = new_value;
                            }
                        }
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Tread Coefficient: ");
                    if ui.add(DragValue::new(&mut nodes[*selected_nodes.last().unwrap_or(&0)].tread_coef)).changed() {
                        let new_value = nodes[*selected_nodes.last().unwrap_or(&0)].tread_coef.clone();
                        for node in nodes.iter_mut() {
                            if node.is_selected {
                                node.tread_coef = new_value;
                            }
                        }
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Load Sensitivity Slope: ");
                    if ui.add(DragValue::new(&mut nodes[*selected_nodes.last().unwrap_or(&0)].load_sensitivity_slope)).changed() {
                        let new_value = nodes[*selected_nodes.last().unwrap_or(&0)].load_sensitivity_slope.clone();
                        for node in nodes.iter_mut() {
                            if node.is_selected {
                                node.load_sensitivity_slope = new_value;
                            }
                        }
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Tag: ");
                    if ui.text_edit_singleline(&mut nodes[*selected_nodes.last().unwrap_or(&0)].tag).changed() {
                        let new_value = nodes[*selected_nodes.last().unwrap_or(&0)].tag.clone();
                        for node in nodes.iter_mut() {
                            if node.is_selected {
                                node.tag = new_value.clone();
                            }
                        }
                    }
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

pub fn show_beams_gui(gui_context: &egui::Context, ui_vars: &mut UiVariables, beam_selected: bool, beam_idx: usize, beams: &mut Vec<JBeam>) {
    CentralPanel::default().show(gui_context, |ui| {

        // does a beam exist?
        if beam_selected {

            ui.horizontal(|ui| {
                ui.heading("Id1:");
                ui.heading(RichText::new(format!("{}", beams[beam_idx].id1)).color(Color32::RED));
                ui.heading("Id2:");
                ui.heading(RichText::new(format!("{}", beams[beam_idx].id2)).color(Color32::RED));
            });

            ui.separator();

            ui.collapsing("Basic", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Beam Spring:").on_hover_text("Rigidity of the beam (N/m). Force required to change the length of the beam by a set amount. Excessively high stiffness compared to node weight can cause the beam to vibrate and cause instability issues.");
                    ui.add(DragValue::new(&mut beams[beam_idx].beam_spring));
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Beam Damp:").on_hover_text("Damping of the beam (N/m/s). Damping causes oscillations to reduce over time. Excessively high damping compared to the node weight might cause pulsing stresses and instability.");
                    ui.add(DragValue::new(&mut beams[beam_idx].beam_damp));
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Beam Strength:").on_hover_text("Strength of the beam. (N). How much force the beam can resist before breaking. A value of “FLT_MAX” will result in an unbreakable beam.");
                    ui.text_edit_singleline(&mut beams[beam_idx].beam_strength);
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Beam Deform:").on_hover_text("How much force (N) is required to deform the beam permanently. A value of “FLT_MAX” will result in a beam that can’t be broken.");
                    ui.add(DragValue::new(&mut beams[beam_idx].beam_deform));            
                });

                ui.separator();



            });


        } else {
            ui.heading(RichText::new("No beam selected, go back to the viewport and select one.").color(Color32::RED));
        }

    });
}