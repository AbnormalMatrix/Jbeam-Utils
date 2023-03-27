use three_d::*;


pub enum GuiType {
    Viewport,
    PartManager,
}



pub struct UserInterface {
    pub gui: GUI,
    pub gui_type: GuiType,
}

pub fn update_gui(ui: &mut UserInterface) {
    gui.update(
        &mut frame_input.events,
        frame_input.accumulated_time,
        frame_input.viewport,
        frame_input.device_pixel_ratio,
        |gui_context| {
            use three_d::egui::*;
            SidePanel::left("side_panel").show(gui_context, |ui| {
                use three_d::egui::*;

                ui.horizontal(|ui| {
                    ui.heading("Properties");
                    if ui.button("Theme").clicked() {
                        if ui_dark {
                            ui.ctx().set_visuals(Visuals::light());
                            ui_dark = false;
                        } else {
                            ui.ctx().set_visuals(Visuals::dark());
                            ui_dark = true;
                        }

                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Editor Mode: ");
                    if editor_mode == EditorMode::Normal {
                        ui.label("Normal");
                    } else if editor_mode == EditorMode::Move {
                        ui.label("Move");
                    }
                });

                if ui.button("Open JBeam").clicked() {

                    let (node_string, beam_string) = jbeam::load_jbeam_file();

                    let mut new_nodes = jbeam::parse_jbeam(node_string);

                    let mut new_beams = jbeam::parse_beams(beam_string, &new_nodes);

                    nodes.append(&mut new_nodes);
                    beams.append(&mut new_beams.0);
                    invalid_beams.append(&mut new_beams.1);

                }

                ui.separator();
                ui.add(Slider::new(&mut camera_speed, 0.01..=1.0).text("Camera Speed"));
                ui.separator();
                ui.heading("Nodes");
                ui.separator();
                ui.add(TextEdit::singleline(&mut node_to_get).hint_text("Node ID"));
                if ui.button("Select Node").clicked() {
                    let selected_node = jbeam::get_node_by_id(node_to_get.clone(), &nodes);
                    if selected_node.is_some() {
                        node_selected_index = selected_node.unwrap();
                        println!("Node found");
                        node_selected = true;
                        nodes[node_selected_index].is_selected = true;
                                                
                    } else {
                        println!("Node not found");
                        node_selected = false;
                        nodes[node_selected_index].is_selected = false;
                    }
                }

                if node_selected {
                    ui.separator();
                    ui.heading("Node Properties");
                    ui.separator();
                    ui.add(TextEdit::singleline(&mut nodes[node_selected_index].id).hint_text("Node ID"));

                    ui.add(DragValue::new(&mut nodes[node_selected_index].node_weight).speed(0.1).prefix("Weight (kg): "));

                    ui.label("position:");

                    ui.add(DragValue::new(&mut nodes[node_selected_index].position.0).speed(0.01).prefix("X: "));
                    ui.add(DragValue::new(&mut nodes[node_selected_index].position.1).speed(0.01).prefix("Y: "));
                    ui.add(DragValue::new(&mut nodes[node_selected_index].position.2).speed(0.01).prefix("Z: "));
                    
                    if ui.button("Generate JBeam").clicked() {
                        selected_node_jbeam_data = nodes[node_selected_index].write("bruh".to_string());
                    }

                    ui.text_edit_multiline(&mut selected_node_jbeam_data);
                    


                } else {
                    ui.separator();
                    ui.heading("Node Properties");
                    ui.separator();
                    ui.label("No node selected");
                }

                ui.separator();
                ui.heading("New Node");

                ui.horizontal(|ui| {
                    ui.add(DragValue::new(&mut new_node_pos.0).speed(0.01).prefix("X: "));
                    ui.add(DragValue::new(&mut new_node_pos.1).speed(0.01).prefix("Y: "));
                    ui.add(DragValue::new(&mut new_node_pos.2).speed(0.01).prefix("Z: "));
                });

                ui.text_edit_singleline(&mut new_node_id);

                if ui.button("Add Node").clicked() {
                    
                    let new_node = jbeam::new_node(&nodes, new_node_pos, new_node_id.clone());
                    
                    if new_node.is_some() {
                        nodes.push(new_node.unwrap());

                        println!("Node added");

                        for node in &mut nodes {
                            node.is_selected = false;
                        }

                        // select the new node
                        node_selected_index = nodes.len() - 1;
                        node_selected = true;
                        nodes[node_selected_index].is_selected = true;

                    } else {
                        println!("Node with that ID already exists");
                    }



                }

                ui.separator();
                ui.heading("Beams");

                ui.horizontal(|ui| {
                    ui.label("id1: ");
                    ui.text_edit_singleline(&mut new_beam_id1);

                });
                ui.horizontal(|ui| {

                    ui.label("id2: ");
                    ui.text_edit_singleline(&mut new_beam_id2);
                });

                if ui.button("Add Beam").clicked() {
                    
                    let new_beam = jbeam::new_beam(&nodes, &beams, new_beam_id1.clone(), new_beam_id2.clone());

                    if new_beam.is_some() {
                        beams.push(new_beam.unwrap());
                    } else {
                        println!("There was an error creating a new beam!");
                    }

                }


            });
            panel_width = gui_context.used_rect().width() as f64;
        },
    );
}