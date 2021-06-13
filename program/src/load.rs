use std::fs;


/*
Load 

This function will load a file into a string and then return the contents of the file

args:
    - filename (string) -> the directory of the file to be read

returns:
    str -> the contents of the file

*/
pub fn load(filename :&str) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Couldn't read file");
    
    return contents;
}