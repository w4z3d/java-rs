use std::io::BufReader;
use std::io;

use zip::{ZipArchive, ZipWriter};
use zip::write::FileOptions;

use java_rs_pacific::JavaClass;

use crate::helpers::io::{create_buffered_reader, create_buffered_writer};

pub fn execute(input: &str, output: String) {
    if input.ends_with(".class") {
        JavaClass::read(&mut create_buffered_reader(input)).expect("cannot read class file").write(&mut create_buffered_writer(output)).expect("cannot write class file");
    } else if input.ends_with(".jar") {
        let mut archive = ZipArchive::new(create_buffered_reader(input)).expect("cannot read jar file");
        let mut writer = ZipWriter::new(create_buffered_writer(output));

        for i in 0..archive.len() {
            let file = archive.by_index(i).expect("cannot read jar file entry");
            let name = file.name();

            let options = {
                let options = FileOptions::default()
                    .compression_method(file.compression())
                    .last_modified_time(file.last_modified());

                if let Some(mode) = file.unix_mode() {
                    options.unix_permissions(mode)
                } else {
                    options
                }
            };

            writer.start_file(name, options).expect("cannot start new jar file entry");

            if name.ends_with(".class") {
                println!("Replicate {}...", name);
                JavaClass::read(&mut BufReader::new(file)).expect("cannot read class file").write(&mut writer).expect("cannot write class file");
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
