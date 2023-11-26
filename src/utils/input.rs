use std::fs;

#[cfg(test)]
const TYPE: &str = "example";
#[cfg(not(test))]
const TYPE: &str = "actual";

pub fn read_input(modpath: &str) -> String {
    let i = modpath.len()-2;
    let fname = format!("inputs/{}_{}.txt", TYPE, &modpath[i..]);
    fs::read_to_string(fname).unwrap()
}
