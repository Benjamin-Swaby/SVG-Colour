

/*
Find 
this function will find the indexes/positions of a certain char in a string

args:
    - contents (String) -> the contents of the file in string form
    - query (char)      -> the char to search for

returns:
    - vec<u32> -> a vector containing all the posiotions of the chars.

*/


pub fn find(contents :&str, query :char) -> Vec<u32> {
    
    let mut result :Vec<u32> = Vec::new();
    
    for (i, c) in contents.chars().enumerate() {
        if c == query {
            result.push(i as u32);
        }
    }

    return result;
}


/*
Gen_bounds 
this function will take the positions of a character and then add the length of it to give a start index and an end index of the string

args:
    - positions (Vec<u32>)  -> the vector of indexes generated from find
    - length (u16)          -> the length of the target section (7) 

*/

pub fn gen_bounds(positions :Vec<u32>, length :u32) -> Vec<(u32, u32)>{

    let mut result :Vec<(u32, u32)> = vec![];

    for i in positions.iter() {
        result.push((i.to_owned(),i+length));
    }
    return result;

}