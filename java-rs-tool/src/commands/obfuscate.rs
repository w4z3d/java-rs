use std::io::BufReader;
use std::io;

use zip::{ZipArchive, ZipWriter};
use zip::write::FileOptions;

use java_rs_pacific::JavaClass;

use crate::helpers::io::{create_buffered_reader, create_buffered_writer};
use crate::helpers::obfuscation::{process_class, Mode, Options};

pub fn execute(input: &str, output: String, no_remap_instructions: bool, no_correct_strings: bool) {
    let obfuscation_options = Options {
        mode: Mode::Obfuscate,
        correct_strings: !no_correct_strings,
        remap_instructions: !no_remap_instructions
    };

    if input.ends_with(".class") {
        process_class(&obfuscation_options, JavaClass::read(&mut create_buffered_reader(input)).expect("cannot read class file")).expect("cannot deobfuscate class file").write(&mut create_buffered_writer(output)).expect("cannot write class file");
    } else if input.ends_with(".jar") {
        let mut archive = ZipArchive::new(create_buffered_reader(input)).expect("cannot read jar file");
        let mut writer = ZipWriter::new(create_buffered_writer(output));

        for i in 0..archive.len() {
            let file = archive.by_index(i).expect("cannot read jar file entry");
            let name = file.name();

            let options = FileOptions::default()
                .compression_method(file.compression())
                .last_modified_time(file.last_modified());

            writer.start_file(name, options).expect("cannot start new jar file entry");

            if name.ends_with(".class") {
                println!("Obfuscate {}...", name);
                process_class(&obfuscation_options, JavaClass::read(&mut BufReader::new(file)).expect("cannot read class file")).expect("cannot deobfuscate class file").write(&mut writer).expect("cannot write class file");
            } else {
                println!("Copy {}...", name);
                io::copy(&mut BufReader::new(file), &mut writer).expect("cannot copy resource");
            }
        }

        writer.finish().expect("cannot finish new jar file");
    } else {
        eprintln!("invalid input file extension");
        std::process::exit(1);
    }
}
