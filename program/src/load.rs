use std::fs;
use std::fs::File;
use std::io::{Write, Result};
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



/*
write 
this function will write an svg string to a file of a specified filename

args:
    - filename (&str)   -> the name of the output file
    - svg (String)      -> the svg string to be written

returns:
    

//TODO find out why it can't create the file
*/
pub fn write(filename :&str, svg :String) {
    //build the target dir 
    let name :String = format!("{}{}{}","./images/",filename,".svg");
    println!("{}",name);
    //create the file
    let mut f = File::create(name).expect("Unable to create file");
    //write the svg string to the file
    writeln!(f, "{}", svg);
    
}