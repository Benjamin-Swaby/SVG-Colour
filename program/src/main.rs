mod load;
mod search;
mod colours;



/*
Create Image
this function will take the indexes and the string and generate an image for a colour. 

args:
    - indexes (Vec<(u32,  u32)>) -> the indexes of the colours
    - svg (&str)               -> the svg string 
    - colour (i32)               -> the colour. 

returns:
    - String -> a new svg string with the colours replaced 

*/
fn create_image(indexes :&Vec<(u32, u32)>, svg :&str, colour :i32) -> String{


    // super cool match statements mapping the correct function to be called a bunch. 
    let new_colour :&dyn Fn() -> String;
    //converting to a vector will make it easier to map the new colour to a specific index
    let mut svg_string_vec :Vec<char> = svg.chars().collect();



    match colour {
        0 => new_colour = &colours::random_red,
        1 => new_colour = &colours::random_blue,
        2 => new_colour = &colours::random_green,
        3 => new_colour = &colours::random_cyan,
        4 => new_colour = &colours::random_yellow,
        5 => new_colour = &colours::random_pink,
        _ => panic!("colour not in range"),

    };

    // for every index generate a new colour and replace the previous colour with the new one
    for i in indexes.iter() {

        let colour_replacement :String = new_colour();
        let colour_replacement_vec :Vec<char> = colour_replacement.chars().collect();

        for index in i.0 .. i.1 {
            // set the chars between the bounds to the new colour.
            // so new colour [0] will replace svg [i.0]
            svg_string_vec[index as usize] = colour_replacement_vec[(index - i.0) as usize];
        }

    }

    //converting back to a String after making the changes 
    let out :String = svg_string_vec.into_iter().collect();    

    return out;
}


/*
Create
this function will create all 6 of the necessary colours for the image. 

args:
    - indexes   (Vec<u32, u32>)     -> the index array
    - svg       (String)            -> the svg data to be changed

returns:
    - Vec<String> -> an array of 6 svg datas with new colours.

*/
fn create(indexes :Vec<(u32, u32)>, svg :String) -> Vec<String> {

    let mut out :Vec<String> = vec![];

    for i in 0..6{
        out.push(create_image(&indexes, &svg, i as i32));

    }

    return out;
}


fn main() {
    // load the data 
    let svg :String = load::load("./images/image.svg");
    println!("{:?}",svg);

    // find all the # in the data
    let pos :Vec<u32> = search::find(&svg, '#');
    
    //should give the positions of the start and end of the hash 
    let indexes :Vec<(u32, u32)> = search::gen_bounds(pos, 6);
    println!("{:?}", indexes);
    

    let images :Vec<String> = create(indexes, svg);


    
    for (i, image) in images.iter().enumerate() {
        load::write(&i.to_string(), image.to_string());    
    }


}

