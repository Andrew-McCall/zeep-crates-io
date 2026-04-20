use std::{
    fs,
    io::{BufWriter, Write},
    path::Path,
};
use zeep_lib::{reader::{WriteXml, XmlReader}, utils::read_input_file_and_xsd_files_at_path};

fn main() {
    let wsdl_dir = Path::new("./wsdl");
    let out_dir = Path::new("./src/zeep");

    fs::create_dir_all(out_dir).expect("failed to create src/zeep");

    let mut modules: Vec<String> = vec![];

    for entry in fs::read_dir(wsdl_dir).expect("failed to read wsdl dir") {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some("wsdl") {
            continue;
        }

        let files_to_read =
            read_input_file_and_xsd_files_at_path(&path).expect("failed to read WSDL");

        let document = XmlReader::read_xml(&files_to_read).expect("failed to parse WSDL");

        let stem = path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .chars()
            .enumerate()
            .flat_map(|(i, c)| {
                if c.is_uppercase() && i > 0 {
                    vec!['_', c.to_lowercase().next().unwrap()]
                } else {
                    vec![c.to_lowercase().next().unwrap()]
                }
            })
            .collect::<String>();

        let out_path = out_dir.join(format!("{stem}.rs"));
        let file = fs::File::create(&out_path).expect("failed to create output file");
        let mut writer = BufWriter::new(file);

        document.write_xml(&mut writer).expect("failed to write generated code");

        modules.push(stem);
    }

    let mod_path = out_dir.join("mod.rs");
    let mut mod_file = fs::File::create(mod_path).expect("failed to create mod.rs");
    for m in &modules {
        writeln!(mod_file, "pub mod {m};").unwrap();
    }
}
