use three_d::egui::{*, self};
use crate::jbeam::{JNodeGroup, JPhysicalMaterial, JNode};
use rfd::FileDialog;
use std::path::Path;

pub struct UiVariables {
    pub groups: Vec<JNodeGroup>,
    selected_group: usize,
    new_group_name: String,
    mod_name: String,
    mod_folder_path: String,

    mod_vehicles: Vec<String>,
    selected_vehicle: usize,
    new_vehicle_name: String,

    jbeams: Vec<String>,
    selected_jbeam: usize,


    
}

impl UiVariables {
    pub fn new() -> Self {
        Self {
            groups: vec![JNodeGroup::new("Default".to_string())],
            selected_group: 0,
            new_group_name: String::new(),
            mod_name: String::new(),
            mod_folder_path: String::new(),
            mod_vehicles: vec!["None".to_string()],
            selected_vehicle: 0,
            new_vehicle_name: "untitled_vehicle".to_string(),

            jbeams: vec!["None".to_string()],
            selected_jbeam: 0,



        }
    }
}


#[derive(PartialEq)]
pub enum BigGuiMode {
    Parts,
    Nodes,
    ModManager,
}

pub fn show_parts_gui(gui_context: &egui::Context, ui_vars: &mut UiVariables) {
    CentralPanel::default().show(gui_context, |ui| {
        ui.heading("Parts");
    });
}


pub fn show_nodes_gui(gui_context: &egui::Context, ui_vars: &mut UiVariables, selected_node_id: String, selected_node_index: usize, node_selected: bool, nodes: Vec<JNode>) {
    CentralPanel::default().show(gui_context, |ui| {
        ui.heading("Nodes");
        ui.separator();

        ui.horizontal(|ui| {

            ui.label("Select Group:");

            egui::ComboBox::new("group_selector", "").show_index(
                ui,
                &mut ui_vars.selected_group,
                ui_vars.groups.len(),
                |i| ui_vars.groups[i].group_name.clone(),
            );
            
            ui.label("Or create new group:");

            ui.add(egui::TextEdit::singleline(&mut ui_vars.new_group_name).hint_text("New Group Name"));

            if ui.button("Create").clicked() {
                ui_vars.groups.push(JNodeGroup::new(ui_vars.new_group_name.clone()));
                ui_vars.new_group_name.clear();
            }

            ui.separator();

            if ui.button("Add selected node to group").clicked() {

                // check if node is already in group
                if ui_vars.groups[ui_vars.selected_group].node_ids.contains(&selected_node_id) {
                    println!("Node is already in group!");
                    return;
                }

                // check if node is already in another group
                for group in ui_vars.groups.iter() {
                    if group.node_ids.contains(&selected_node_id) {
                        println!("Node is already in another group!");
                        return;
                    }
                }

                // check if the selected node is an empty string
                if selected_node_id == "".to_string() {
                    println!("No node selected!");
                    return;
                }

                ui_vars.groups[ui_vars.selected_group].node_ids.push(selected_node_id.clone());
            }

        });

        ui.separator();

        ui.label("Node Physical Material:");


        
        // physical material

        // Metal,
        // Plastic,
        // Rubber,
        // Glass,
        // Wood,
        // Foliage,
        // Cloth,
        // Water,
        // Asphalt,
        // AsphaltWet,
        // Slippery,
        // Rock,
        // DirtDusty,
        // Dirt,
        // Sand,
        // SandyRoad,
        // Mud,
        // Gravel,
        // Grass,
        // Ice,
        // Snow,
        // Firesmall,
        // Firemedium,
        // Firelarge,
        // SmokeSmallBlack,
        // SmokeMediumBlack,
        // Steam,
        // RumbleStrip,
        // Cobblestone,
        // FoliageThin,

        ui.horizontal(|ui| {
            ComboBox::from_label("")
                .selected_text(format!("{:?}", ui_vars.groups[ui_vars.selected_group].node_material))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Metal, "Metal");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Plastic, "Plastic");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Rubber, "Rubber");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Glass, "Glass");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Wood, "Wood");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Foliage, "Foliage");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Cloth, "Cloth");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Water, "Water");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Asphalt, "Asphalt");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::AsphaltWet, "Wet Asphalt");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Slippery, "Slippery");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Rock, "Rock");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::DirtDusty, "Dusty Dirt");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Dirt, "Dirt");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Sand, "Sand");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::SandyRoad, "Sandy Road");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Mud, "Mud");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Gravel, "Gravel");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Grass, "Grass");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Ice, "Ice");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Snow, "Snow");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Firesmall, "Small Fire");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Firemedium, "Medium Fire");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Firelarge, "Large Fire");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::SmokeSmallBlack, "Small Black Smoke");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::SmokeMediumBlack, "Medium Black Smoke");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Steam, "Steam");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::RumbleStrip, "Rumble Strip");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::Cobblestone, "Cobblestone");
                    ui.selectable_value(&mut ui_vars.groups[ui_vars.selected_group].node_material, JPhysicalMaterial::FoliageThin, "Thin Foliage");

                });
        });

        ui.horizontal(|ui| {
            ui.label("This group contains: ");
            ui.label(format!("{} nodes", ui_vars.groups[ui_vars.selected_group].node_ids.len()));
            ui.label("node(s).");
        });
        
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("The slelected node is a member of the following groups: ");
            ui.vertical(|ui| {

                if node_selected {

                    for group in nodes[selected_node_index].group.iter() {
                        ui.label(group);
                    }

                } else {
                    ui.label("No node selected.");
                }

            });
        });

    });
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