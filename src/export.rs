use std::{fs, io::Write};
// use serde_json;
use crate::*;


fn write_log(f: String) {
    let mut file = fs::OpenOptions::new().read(true).create(true).append(true).open("log.jsonc").unwrap();
    writeln!(file, "{}", f).unwrap();
}

pub fn export(place: &Place) {
    let log = serde_json::to_string(&place).unwrap();
    write_log(log);
    // println!("{}", log);
}

#[test]
fn test_export() {
    let test_p = Place::new(11, CurrentPoint { x: 1, y: 1 });
    export(&test_p);
}