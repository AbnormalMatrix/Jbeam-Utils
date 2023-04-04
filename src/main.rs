mod jbeam;

mod main_ui;
mod import_wizard;


use three_d::*;
#[macro_use]
extern crate pest_derive;



#[derive(PartialEq)]
enum EditorMode {
    Normal,
    Move
}

#[derive(PartialEq)]
pub enum MoveAxis {
    Undefined,
    X,
    Y,
    Z,
}

#[derive(PartialEq)]
enum OrthoMode {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom
}



#[derive(PartialEq)]
enum SnappingMode {
    Off,
    Grid,
    Increment,
}


fn main() {




    let mut nodes: Vec<jbeam::JNode> = Vec::new();

    let mut beams: Vec<jbeam::JBeam> = Vec::new();

    // any beams that are loaded but the nodes do not exist will be added to a separate vector so they can be added later if more nodes are loaded

    let mut invalid_beams: Vec<jbeam::JBeam> = Vec::new();

    


    
    
    // settings

    let mut camera_speed = 0.1;

    let mut ortho_camera_height = 2.0;

    let mut node_to_get = "".to_string();

    let mut node_selected = false;

    let mut node_selected_index = 0;

    let mut ui_dark = true;

    let mut snapping_mode = SnappingMode::Increment;

    let mut snap_increment = 0.1;

    let mut snap_speed = 500.0;

    let mut translate_dist = 0.0;
    
    let mut mirror_axis = (-1.0, 1.0, 1.0);


    let mut selected_node_jbeam_data = String::from("");

    let mut new_node_number = 0;

    let mut new_node_pos = (0.0, 0.0, 0.0);

    let mut new_node_id = String::from("");

    let mut new_beam_id1 = String::from("");
    let mut new_beam_id2 = String::from("");


    // the editor mode determines what happens when you move the mouse

    let mut editor_mode = EditorMode::Normal;

    let mut move_axis = MoveAxis::Undefined;

    let mut move_dist = 0.0;

    // record the mouse position when changing to move mode

    let mut mouse_pos = (0.0, 0.0);

    // variable to store the position of the node before moving it

    let mut node_pos_before_move = (0.0, 0.0, 0.0);

    let mut ortho_mode = OrthoMode::Front;

    let mut show_big_gui = false;

    let mut big_gui_mode =  main_ui::BigGuiMode::Parts;

    let mut big_gui_vars = main_ui::UiVariables::new();


    let window = Window::new(WindowSettings {
        title: "JBeam Editor".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut gui = three_d::GUI::new(&context);

    let mut gui_floating = three_d::GUI::new(&context);

    let mut show_floating_gui = false;

    let mut import_vars = import_wizard::ImportVars::default();


    let mut big_gui = three_d::GUI::new(&context);


    // let mut camera = Camera::new_perspective(
    //     window.viewport(),
    //     vec3(5.0, 2.0, 2.5),
    //     vec3(0.0, 0.0, -0.5),
    //     vec3(0.0, 1.0, 0.0),
    //     degrees(45.0),
    //     0.1,
    //     1000.0,
    // );

    let mut camera = Camera::new_orthographic(
        window.viewport(),
        vec3(10.0, 0.0, 0.0),
        vec3(-1.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        ortho_camera_height,
        0.001,
        10000.0,
    );



    
    
    //let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let mut control = FlyControl::new(0.1);

    //let mut control = OrbitControl::new(*camera.target(), 0.1, 100.0);


    // the axis indicator
    let mut axes = Axes::new(&context, 0.01, 0.5);


    let mut loaded = three_d_asset::io::load(&["assets/arrow.obj"]).unwrap();
    let model = loaded.deserialize("arrow.obj").unwrap();
    

    // load the arrows

    let mut arrow = Model::<PhysicalMaterial>::new(&context, &model).unwrap();
    

    arrow[0].material = PhysicalMaterial::new_opaque(&context, &CpuMaterial {
        albedo: Color {
            r: 0,
            g: 0,
            b: 255,
            a: 255
        },
        ..Default::default()
    });

    arrow[1].material = PhysicalMaterial::new_opaque(&context, &CpuMaterial {
        albedo: Color {
            r: 0,
            g: 255,
            b: 0,
            a: 255
        },
        ..Default::default()
    });

    arrow[2].material = PhysicalMaterial::new_opaque(&context, &CpuMaterial {
        albedo: Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255
        },
        ..Default::default()
    });
    

    let mut loaded = three_d_asset::io::load(&["assets/fender.obj"]).unwrap();
    let model = loaded.deserialize("fender.obj").unwrap();

    // load the fenders

    let mut fender = Model::<PhysicalMaterial>::new(&context, &model).unwrap();

    let mut sphere = Gm::new(
        Mesh::new(&context, &CpuMesh::sphere(16)),
        PhysicalMaterial::new_transparent(
            &context,
            &CpuMaterial {
                albedo: Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0,
                },
                ..Default::default()
            },
        ),
    );
    sphere.set_transformation(Mat4::from_translation(vec3(0.0, 1.3, 0.0)) * Mat4::from_scale(0.2));


    // define the materials

    // unselected node material
    let node_material = CpuMaterial {
        albedo: Color {
            r: 255,
            g: 136,
            b: 0,
            a: 255,
        },
        ..Default::default()
    };

    // selected node material
    let selected_node_material = CpuMaterial {
        albedo: Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        },
        ..Default::default()
    };

    let img = three_d_asset::io::load(&["assets/beam_image.jpg"]).unwrap().deserialize("").unwrap();

    let beam_material = ColorMaterial {
        color: Color::WHITE,
        texture: Some(std::sync::Arc::new(Texture2D::new(&context, &img))).into(),
        ..Default::default()
    };


    let light0 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, -0.5, -0.5));
    let light1 = DirectionalLight::new(&context, 1.0, Color::WHITE, &vec3(0.0, 0.5, 0.5));


    window.render_loop(move |mut frame_input| {

        if show_big_gui {


            // the big gui is for everything that does not require the 3d viewport

            big_gui.update(&mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;

                TopBottomPanel::top("tab_selector_left_panel").show(gui_context, |ui| {
                    ui.horizontal(|ui| {
                        // define the categories
                        if ui.button("Parts").clicked() {
                            big_gui_mode = main_ui::BigGuiMode::Parts;
                        };
                        if ui.button("Nodes").clicked() {
                            big_gui_mode = main_ui::BigGuiMode::Nodes;
                        };
                        if ui.button("Mod Manager").clicked() {
                            big_gui_mode = main_ui::BigGuiMode::ModManager;
                        }
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


                });

                let mut selected_node_id = "".to_string();

                if node_selected {
                    selected_node_id = nodes[node_selected_index].id.clone();
                }

                match big_gui_mode {
                    main_ui::BigGuiMode::Parts => {
                        main_ui::show_parts_gui(gui_context, &mut big_gui_vars);
                    },
                    main_ui::BigGuiMode::Nodes => {
                        main_ui::show_nodes_gui(gui_context, &mut big_gui_vars, selected_node_id, node_selected_index, node_selected, nodes.clone());
                    }
                    main_ui::BigGuiMode::ModManager => {
                        main_ui::show_mod_manager(gui_context, &mut big_gui_vars);
                    }
                    _=> {}
                }

            });

            // handle events
            for event in frame_input.events.iter() {
                match event {
                    // w key pressed
                        Event::KeyPress { kind, modifiers, handled } => {
                            if *kind == Key::Tab {
                                show_big_gui = false;
                            }
                        },
                        _ => {}
                    }
            }

            frame_input
                .screen()
                .write(|| big_gui.render());

            FrameOutput::default()

        } else {

        

            // define gui
            let mut panel_width = 0.0;
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

                            show_floating_gui = true;

                            // let (node_string, beam_string) = jbeam::load_jbeam_file();

                            // let mut new_nodes = jbeam::parse_jbeam(node_string);

                            // let mut new_beams = jbeam::parse_beams(beam_string, &new_nodes);

                            // nodes.append(&mut new_nodes);
                            // beams.append(&mut new_beams.0);
                            // invalid_beams.append(&mut new_beams.1);

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
                            
                            if ui.button("Generate Single Node").clicked() {
                                selected_node_jbeam_data = nodes[node_selected_index].write("bruh".to_string());
                            }

                            ui.text_edit_multiline(&mut selected_node_jbeam_data);

                            


                        } else {
                            ui.separator();
                            ui.heading("Node Properties");
                            ui.separator();
                            ui.label("No node selected");
                        }

                        if ui.button("Delete Node").clicked() {
                            if node_selected {

                                // remove beams connected to node
                                let mut beams_to_remove = Vec::new();
                                for (i, beam) in beams.iter().enumerate() {
                                    if beam.id1 == nodes[node_selected_index].id || beam.id2 == nodes[node_selected_index].id {
                                        beams_to_remove.push(i);
                                    }
                                }

                                for beam in beams_to_remove.iter().rev() {
                                    beams.remove(*beam);
                                }
                                

                                nodes.remove(node_selected_index);
                                node_selected = false;
                            }
                        }

                        if ui.button("Mark node for export").clicked() {
                            if node_selected {
                                nodes[node_selected_index].imported = false;
                            }
                        }
                        if ui.button("Mark all nodes for export").clicked() {
                            for node in nodes.iter_mut() {
                                node.imported = false;
                            }

                        }

                        if ui.button("Generate Created Nodes").clicked() {
                            jbeam::write_user_created_nodes(&nodes);
                        }

                        ui.separator();
                        ui.heading("New Node");

                        ui.horizontal(|ui| {
                            ui.add(DragValue::new(&mut new_node_pos.0).speed(0.01).prefix("X: "));
                            ui.add(DragValue::new(&mut new_node_pos.1).speed(0.01).prefix("Y: "));
                            ui.add(DragValue::new(&mut new_node_pos.2).speed(0.01).prefix("Z: "));
                        });

                        ui.text_edit_singleline(&mut new_node_id);
                        ui.label(format!("{}{}",new_node_id, new_node_number));

                        if ui.button("Add Node").clicked() {
                            
                            let new_node = jbeam::new_node(&nodes, new_node_pos, format!("{}{}", new_node_id, new_node_number));
                            
                            new_node_number += 1;

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

                        if ui.button("Mirror").clicked() {
                            if node_selected {
                                let new_id = format!("{}_mirror", nodes[node_selected_index].id.clone());
                                let mut new_pos = nodes[node_selected_index].position.clone();
                                new_pos.0 *= mirror_axis.0;
                                new_pos.1 *= mirror_axis.1;
                                new_pos.2 *= mirror_axis.2;

                                let new_node = jbeam::new_node(&nodes, new_pos, new_id);
                            
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

                        if ui.button("Delete Beam").clicked() {
                            // check if there are any beams with new_beam_id1 and new_beam_id2
                            let mut beam_to_remove = None;
                            for (i, beam) in beams.iter().enumerate() {
                                if (beam.id1 == new_beam_id1 && beam.id2 == new_beam_id2) || (beam.id1 == new_beam_id2 && beam.id2 == new_beam_id1) {
                                    beam_to_remove = Some(i);
                                }
                            }

                            if beam_to_remove.is_some() {
                                beams.remove(beam_to_remove.unwrap());
                            } else {
                                println!("No beam found with those IDs");
                            }
                        }

                        if ui.button("Delete connected beams").clicked() {
                            if node_selected {

                                let flag_id = &nodes[node_selected_index].id;

                                // remove beams connected to node
                                beams.retain(|beam| beam.id1 != *flag_id && beam.id2 != *flag_id);


                            }
                        }


                        if ui.button("Generate Created Beams").clicked() {
                            jbeam::write_user_created_beams(&beams);
                        }


                    });



                    panel_width = gui_context.used_rect().width() as f64;
                },
            );



            let viewport = Viewport {
                x: (panel_width * frame_input.device_pixel_ratio) as i32,
                y: 0,
                width: frame_input.viewport.width
                    - (panel_width * frame_input.device_pixel_ratio) as u32,
                height: frame_input.viewport.height,
            };


            camera.set_viewport(viewport);
            control.handle_events(&mut camera, &mut frame_input.events);


            let mut node_objects = Vec::new();


            let mut beam_objects = Vec::new();

            for beam in &beams {
                let beam_object = Gm{
                    geometry: beam.get_3d_object(&context, &nodes),
                    material: &beam_material,
                };
                beam_objects.push(beam_object);
            }

            for node in &nodes {
                let node_object = node.get_3d_object(&context, &node_material, &selected_node_material);
                node_objects.push(node_object);
            }

            // handle input
            for event in frame_input.events.iter() {
                
                match event {
                // w key pressed
                    Event::KeyPress { kind, modifiers, handled } => {

                        if *kind == Key::Tab {
                            show_big_gui = true;
                        }

                        if *kind == Key::W {

                            camera.translate(&(&camera.view_direction() * camera_speed))
                        }
                        if *kind == Key::S {
                            camera.translate(&(&camera.view_direction() * -camera_speed))
                        }
                        if *kind == Key::D {
                            camera.translate(&(&camera.right_direction() * camera_speed))
                        }
                        if *kind == Key::A {
                            camera.translate(&(&camera.right_direction() * -camera_speed))
                        }
                        if *kind == Key::Q {
                            camera.translate(&(camera.up() * -camera_speed))
                        }
                        if *kind == Key::E {
                            camera.translate(&(camera.up() * camera_speed))
                        }
                        
                        // camera orthographic view

                        if *kind == Key::Num1 {
                            if modifiers.ctrl {
                                camera.set_view(
                                    vec3(0.0, 0.0, -100.0),
                                    vec3(0.0, 0.0, 1.0),
                                    vec3(0.0, 1.0, 0.0),
                                );

                                // set the ortho mode
                                ortho_mode = OrthoMode::Back;

                            } else {
                                camera.set_view(
                                    vec3(0.0, 0.0, 100.0),
                                    vec3(0.0, 0.0, -1.0),
                                    vec3(0.0, 1.0, 0.0),
                                );

                                // set the ortho mode
                                ortho_mode = OrthoMode::Front;
                            }

                        }

                        if *kind == Key::Num3 {
                            if modifiers.ctrl {
                                camera.set_view(
                                    vec3(-100.0, 0.0, 0.0),
                                    vec3(1.0, 0.0, 0.0),
                                    vec3(0.0, 1.0, 0.0),
                                );

                                // set the ortho mode
                                ortho_mode = OrthoMode::Left;

                            } else {
                                camera.set_view(
                                    vec3(100.0, 0.0, 0.0),
                                    vec3(-1.0, 0.0, 0.0),
                                    vec3(0.0, 1.0, 0.0),
                                );

                                // set the ortho mode
                                ortho_mode = OrthoMode::Right;

                            }

                        }

                        if *kind == Key::Num7 {
                            if modifiers.ctrl {
                                camera.set_view(
                                    vec3(0.0, 100.0, 0.0),
                                    vec3(0.0, -1.0, 0.0),
                                    vec3(1.0, 0.0, 0.0),
                                );

                                // set the ortho mode
                                ortho_mode = OrthoMode::Bottom;
                            } else {
                                camera.set_view(
                                    vec3(0.0, -100.0, 0.0),
                                    vec3(0.0, 1.0, 0.0),
                                    vec3(-1.0, 0.0, 0.0),
                                );

                                // set the ortho mode
                                ortho_mode = OrthoMode::Top;
                            }

                        }
                        
                        // toggle between orthographic and perspective cameras
                        if *kind == Key::Num5 {

                            match camera.projection_type() {
                                three_d_asset::ProjectionType::Perspective {..} => {
                                    camera.set_orthographic_projection(ortho_camera_height, 0.01, 100.0);
                                },
                                _ => {
                                    camera.set_perspective_projection(degrees(45.0), 0.01, 100.0);
                                }
                            }

                            
                        }




                        if *kind == Key::G {
                            if editor_mode == EditorMode::Normal {
                                println!("Switching to move mode");
                                editor_mode = EditorMode::Move;




                            } else if editor_mode == EditorMode::Move {
                                println!("Switching to normal mode");
                                editor_mode = EditorMode::Normal;
                                move_axis = MoveAxis::Undefined;
                            }
                        }
                        // choose the axis when in move mode
                        if *kind == Key::X {
                            if editor_mode == EditorMode::Move {
                                println!("Moving on the X axis");
                                move_axis = MoveAxis::X;
                            }
                        }
                        if *kind == Key::Y {
                            if editor_mode == EditorMode::Move {
                                println!("Moving on the Y axis");
                                move_axis = MoveAxis::Y;
                            }
                        }
                        if *kind == Key::Z {
                            if editor_mode == EditorMode::Move {
                                println!("Moving on the Z axis");
                                move_axis = MoveAxis::Z;
                            }
                        }
                        if *kind == Key::F {
                            let new_beam = jbeam::new_beam(&nodes, &beams, new_beam_id1.clone(), new_beam_id2.clone());

                            if new_beam.is_some() {
                                beams.push(new_beam.unwrap());
                            } else {
                                println!("There was an error creating a new beam!");
                            }
                        }

                    },

                    Event::MousePress {
                        button, position, ..
                    } => {
                        if *button == MouseButton::Left {

                            if editor_mode == EditorMode::Normal {

                                let pixel = (
                                    (frame_input.device_pixel_ratio * position.0) as f32,
                                    (frame_input.viewport.height as f64
                                        - frame_input.device_pixel_ratio * position.1)
                                        as f32,
                                );
                                
                                // check if we are in orthographic mode
                                match camera.projection_type() {
                                    three_d_asset::ProjectionType::Orthographic { .. } => {

                                        // we know the screen width

                                // get the frustum width from the ortho_camera_height and the aspect ratio

                                let aspect_ratio = (frame_input.viewport.width - (panel_width as u32)) as f32 / frame_input.viewport.height as f32;

                                println!("{}, {}", frame_input.viewport.width - panel_width as u32, frame_input.viewport.height);

                                let frustum_width = ortho_camera_height * aspect_ratio;
                                



                                // find the distance in world space between the camera position and the mouse position on the screen space x axis
                                // we know how wide the screen is and the how wide that is in world space



                                // scale the 0-1 range to be between -1 and 1
                                let scaled_uv_x = camera.uv_coordinates_at_pixel(pixel).0 * 2.0 - 1.0;

                                println!("UV X: {}", scaled_uv_x);

                                let move_dist = scaled_uv_x * (frustum_width / 2.0);

                                println!("Move dist: {}", move_dist);

                                // move on the x axis by multiplying the move distance by the right vector



                                let translated_x  = camera.right_direction().map(|f| f * move_dist);

                                



                                // do the same for the y axis

                                let frustum_height = ortho_camera_height;

                                let scaled_uv_y = camera.uv_coordinates_at_pixel(pixel).1 * 2.0 - 1.0;

                                let move_dist_y = scaled_uv_y * (frustum_height / 2.0);

                                let translated_y  = camera.up().map(|f| f * move_dist_y);



                                // add the two vectors together to get the final translation vector

                                let translation = translated_x + translated_y;

                                let ray_start = camera.position() + translation;

                                println!("Ray start: {:?}", ray_start);

                                // let mut debug_node1 = jbeam::new_node(&nodes, (ray_start.x, ray_start.y, ray_start.z), "debug_node1".to_string()).unwrap();

                                // nodes.push(debug_node1);

                                if let Some(ray) = ray_intersect(&context, ray_start, camera.view_direction(), 1000.0, &node_objects) {
                                    println!("{:?}", ray);
                                    new_beam_id2 = nodes[node_selected_index].id.clone();

                                    nodes[node_selected_index].is_selected = false;

                                    node_selected_index = jbeam::get_closest_node_index(&nodes, ray).unwrap();

                                    new_node_pos = nodes[node_selected_index].position.clone();
                                    new_node_pos.2 += 0.2;


                                    println!("Closest node: {}", nodes[node_selected_index].id);
                                    
                                    new_beam_id1 = nodes[node_selected_index].id.clone();

                                    node_selected = true;
                                    nodes[node_selected_index].is_selected = true;

                                    // save the position of the node
                                    node_pos_before_move = nodes[node_selected_index].position.clone();


                                }

                                    }
                                _ => {
                                    if let Some(pick) = pick(&context, &camera, pixel, &node_objects) {
                                        println!("{:?}", pick);
                                        new_beam_id2 = nodes[node_selected_index].id.clone();

                                        nodes[node_selected_index].is_selected = false;

                                        node_selected_index = jbeam::get_closest_node_index(&nodes, pick).unwrap();

                                        new_node_pos = nodes[node_selected_index].position.clone();
                                        new_node_pos.2 += 0.2;

                                        println!("Closest node: {}", nodes[node_selected_index].id);
                                        
                                        new_beam_id1 = nodes[node_selected_index].id.clone();

                                        node_selected = true;
                                        nodes[node_selected_index].is_selected = true;

                                        // save the position of the node
                                        node_pos_before_move = nodes[node_selected_index].position.clone();
                                    }
                                }

                                }

                                
                            } else if editor_mode == EditorMode::Move {

                                editor_mode = EditorMode::Normal;
                                move_axis = MoveAxis::Undefined;
                            }
                        } else if *button == MouseButton::Right {
                            if editor_mode == EditorMode::Move {
                                // reset the position of the node
                                if node_selected {
                                    nodes[node_selected_index].position = node_pos_before_move.clone();
                                }
                                editor_mode = EditorMode::Normal;
                                move_axis = MoveAxis::Undefined;

                            }
                        }
                    },
                    Event::MouseWheel {
                        delta, ..
                    } => {
                        ortho_camera_height += delta.1 as f32 * 0.01;
                        println!("{}", ortho_camera_height);
                        camera.set_orthographic_projection(ortho_camera_height, 0.00001, 100000.0)
                    },

                    Event::MouseMotion {
                        delta, position, ..
                    } => {
                        

                        

                        

                        // if the editor is not in move mode, update the mouse position variable
                        if editor_mode != EditorMode::Move {
                            mouse_pos = position.clone();
                        }

                        // check if a node is selected, the editor is in move mode and an axis is chosen
                        if node_selected && editor_mode == EditorMode::Move && move_axis != MoveAxis::Undefined {



                            // find the difference between mouse_pos and position regardless of the direction of the mouse movement


    
                            translate_dist += delta.0 as f32;
                            let dir = if delta.0 > 0.0 { 1.0 } else { -1.0 };

                            // translate_dist = translate_dist.abs();



                            


                            

                            if snapping_mode == SnappingMode::Increment {
                                match move_axis {
                                    MoveAxis::X => {



                                        // find the next point to snap to

                                        let next_snap =  nodes[node_selected_index].position.0 + snap_increment * dir;

                                        // find the position that it would be at if you were not snapping

                                        let no_snap = nodes[node_selected_index].position.0 + translate_dist;

                                        let snap_dif = no_snap - next_snap;
                                        


                                        if snap_dif.abs() > (snap_increment*snap_speed) {
                                            nodes[node_selected_index].position.0 += snap_increment * dir;
                                            translate_dist = 0.0;
                                        }

                                    },
                                    MoveAxis::Y => {
                                        // move the node on the Y axis
                                        // find the next point to snap to

                                        let next_snap =  nodes[node_selected_index].position.1 + snap_increment * dir;

                                        // find the position that it would be at if you were not snapping

                                        let no_snap = nodes[node_selected_index].position.1 + translate_dist;

                                        let snap_dif = no_snap - next_snap;
                                        


                                        if snap_dif.abs() > (snap_increment*snap_speed) {
                                            nodes[node_selected_index].position.1 += snap_increment * dir;
                                            translate_dist = 0.0;
                                        }
                                    },
                                    MoveAxis::Z => {
                                        // move the node on the Z axis
                                        // find the next point to snap to

                                        let next_snap =  nodes[node_selected_index].position.2 + snap_increment * dir;

                                        // find the position that it would be at if you were not snapping

                                        let no_snap = nodes[node_selected_index].position.2 + translate_dist;

                                        let snap_dif = no_snap - next_snap;
                                        


                                        if snap_dif.abs() > (snap_increment*snap_speed) {
                                            nodes[node_selected_index].position.2 += snap_increment * dir;
                                            translate_dist = 0.0;
                                        }
                                    },
                                    MoveAxis::Undefined => {
                                        // do nothing
                                    }
                                }
                            } else {

                                // match the axis chosen
                                match move_axis {
                                    MoveAxis::X => {
                                        // move the node on the X axis
                                        
                                        nodes[node_selected_index].position.0 += translate_dist as f32 * 0.01;
                                    },
                                    MoveAxis::Y => {
                                        // move the node on the Y axis
                                        nodes[node_selected_index].position.1 += translate_dist as f32 * 0.01;
                                    },
                                    MoveAxis::Z => {
                                        // move the node on the Z axis
                                        nodes[node_selected_index].position.2 += translate_dist as f32 * 0.01;
                                    },
                                    MoveAxis::Undefined => {
                                        // do nothing
                                    }
                                }
                            }

                        }
                    }

                    _ => (),
                }
            }



            // if a node is selected, move the axes to the node, otherwise move the axes to 0,0,0
            if node_selected {
                let axes_pos = Mat4::from_translation(Vector3 { x: nodes[node_selected_index].position.0, y: nodes[node_selected_index].position.1, z: nodes[node_selected_index].position.2 });
                axes.set_transformation(axes_pos);

            } else {
                let axes_pos = Mat4::from_translation(Vector3 { x: 0.0, y: 0.0, z: 0.0 });
                axes.set_transformation(axes_pos);
            }
            
            // render everything
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                .write(|| gui.render())
                

                .render(
                    &camera,
                    fender.into_iter(),
                    &[&light0, &light1],
                )

                .render(
                    &camera,

                    // get an iterator over all the objects to render including the sprites
                    
                    
                    node_objects.iter().chain(std::iter::once(&sphere)),

                    //beam_objects.iter(),

                    &[&light0, &light1],
                ).render(
                    &camera,
                    beam_objects.iter(),
                    &[&light0, &light1],
                )
                .render(&camera, axes.into_iter(), &[&light0, &light1]);



            // only show the floating window when required because it is annoying

            if show_floating_gui {


                gui_floating.update(
                    &mut frame_input.events,
                    frame_input.accumulated_time,
                    frame_input.viewport,
                    frame_input.device_pixel_ratio,
                    |gui_context| {

                        import_wizard::show_import_gui(gui_context, &mut import_vars, &mut nodes, &mut beams, &mut invalid_beams);
                
                });

                frame_input
                .screen()
                .write(|| gui_floating.render());

                FrameOutput::default();

            }


                

            FrameOutput::default()

            
        }
    });

    
}
