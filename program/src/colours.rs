use rand::thread_rng;
use rand::Rng;
//use std::fmt;

/*
Random Value 
this function will generate a random int between 0 and 255 for the RGB values

args:
    - max (i32) -> maximum value
    - min (i32) -> minimum value

returns:
    - i32 -> a positive int between 0 and 255

*/
fn random_value(max :i32 , min :i32) -> i16 {

    let mut rng = thread_rng();

    //the smallest value will always be used as the min. 
    if max < min {
        let y :i16 = rng.gen_range(max, min) as i16;
        return y;

    }


    
    let y :i16 = rng.gen_range(min,max) as i16;
    println!("{}",y);
    return y;
}


/*
Random Red 
this function will generate a random red colour using some random thresholds that are within the red colour band.


args:
    
returns:
    String -> a hex code for the colour that has been generated


Bounds for red:
    120<R<255
    0  <G<70
    G == B
*/

pub fn random_red() -> String {

    let Red :String = format!("{:X}",random_value(255, 120));
    let Green :String = format!("{:X}", random_value(70,17));

    println!("red : {}", Red);
    println!(" green: {}", Green);

    let end :String = format!("{}{}{}",Red, Green , Green);
    println!("{}", end);
    return end;
}



pub fn random_green() -> String {

    let Green :String = format!("{:X}",random_value(255, 120));
    let Red :String = format!("{:X}", random_value(70,17));

    println!("red : {}", Red);
    println!(" green: {}", Green);

    let end :String = format!("{}{}{}",Red, Green , Red);
    println!("{}", end);
    return end;
}


pub fn random_blue() -> String {

    let Blue :String = format!("{:X}",random_value(255, 120));
    let Red :String = format!("{:X}", random_value(70,17));

    println!("red : {}", Red);
    println!(" blue: {}", Blue);

    let end :String = format!("{}{}{}",Red, Red , Blue);
    println!("{}", end);
    return end;
}

pub fn random_cyan() -> String {

    let Blue :String = format!("{:X}",random_value(255, 120));
    let Red :String = format!("{:X}", random_value(70,17));

    println!("red : {}", Red);
    println!(" blue: {}", Blue);

    let end :String = format!("{}{}{}",Red, Blue , Blue);
    println!("{}", end);
    return end;
}

pub fn random_yellow() -> String {

    let Green :String = format!("{:X}",random_value(255, 120));
    let Blue :String = format!("{:X}", random_value(70,17));

    println!("green : {}", Green);
    println!(" blue: {}", Blue);

    let end :String = format!("{}{}{}",Green, Green , Blue);
    println!("{}", end);
    return end;
}


pub fn random_pink() -> String {

    let Blue :String = format!("{:X}",random_value(255, 120));
    let Green :String = format!("{:X}", random_value(70,17));

    println!("green : {}", Green);
    println!(" blue: {}", Blue);

    let end :String = format!("{}{}{}",Blue, Green , Blue);
    println!("{}", end);
    return end;
}