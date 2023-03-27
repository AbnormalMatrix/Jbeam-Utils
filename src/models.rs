use dae_parser::*;
use rfd::FileDialog;

pub fn load_dae() {
    let path = FileDialog::new()
        .add_filter("Collada (DAE)", &["dae"])
        .set_directory(".")
        .pick_file()
        .unwrap();


    let document = Document::from_file(path).expect("Failed to load DAE!");


    let sources_map = document.local_map::<Source>().unwrap();
    let vertices_map = document.local_map::<Vertices>().unwrap();



    
}