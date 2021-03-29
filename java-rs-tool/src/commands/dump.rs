use std::io::Write;

use java_rs_pacific::JavaClass;

use crate::helpers::io::{create_buffered_reader, create_buffered_writer};

pub fn execute(input: &str, ugly_print: bool, output: Option<String>) {
    let class = JavaClass::read(&mut create_buffered_reader(input)).expect("cannot read class file");

    let text = if ugly_print {
        format!("{:?}", class)
    } else {
        format!("{:#?}", class)
    };

    if let Some(path) = output {
        create_buffered_writer(path).write_all(text.as_bytes()).expect("cannot write to output file");
    } else {
        println!("{}", text);
    }
}
