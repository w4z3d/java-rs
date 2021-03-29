use std::fs::{File, OpenOptions};
use std::io::{BufWriter, BufReader};
use std::path::Path;

pub fn create_buffered_writer<P: AsRef<Path>>(path: P) -> BufWriter<File> {
    BufWriter::new(
        OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .expect("cannot create output file"),
    )
}

pub fn create_buffered_reader<P: AsRef<Path>>(path: P) -> BufReader<File> {
    BufReader::new(
        OpenOptions::new()
            .read(true)
            .write(false)
            .open(path)
            .expect("input file not found")
    )
}
