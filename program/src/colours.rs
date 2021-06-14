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
    return y;
}


/*
Random *
this function will generate a random * colour using some random thresholds that are within the * colour band.


args:
    
returns:
    String -> a hex code for the colour that has been generated

//TODO maybe see if i can make it deal with single hex values. rather than starting at 17
*/

pub fn random_red() -> String {

    let red :String = format!("{:X}",random_value(255, 120));
    let green :String = format!("{:X}", random_value(70,17));

    //println!("red : {}", red);
    //println!(" green: {}", green);

    let end :String = format!("{}{}{}",red, green , green);
    println!("{}", end);
    return end;
}



pub fn random_green() -> String {

    let green :String = format!("{:X}",random_value(255, 120));
    let red :String = format!("{:X}", random_value(70,17));

    //println!("red : {}", red);
    //println!(" green: {}", green);

    let end :String = format!("{}{}{}",red, green , red);
    println!("{}", end);
    return end;
}


pub fn random_blue() -> String {

    let blue :String = format!("{:X}",random_value(255, 120));
    let red :String = format!("{:X}", random_value(70,17));

    //println!("red : {}", red);
    //println!(" blue: {}", blue);

    let end :String = format!("{}{}{}",red, red , blue);
    println!("{}", end);
    return end;
}

pub fn random_cyan() -> String {

    let blue :String = format!("{:X}",random_value(255, 120));
    let red :String = format!("{:X}", random_value(70,17));

    //println!("red : {}", red);
    //println!(" blue: {}", blue);

    let end :String = format!("{}{}{}",red, blue , blue);
    println!("{}", end);
    return end;
}

pub fn random_yellow() -> String {

    let green :String = format!("{:X}",random_value(255, 120));
    let blue :String = format!("{:X}", random_value(70,17));

    //println!("green : {}", green);
    //println!(" blue: {}", blue);

    let end :String = format!("{}{}{}",green, green , blue);
    println!("{}", end);
    return end;
}


pub fn random_pink() -> String {

    let blue :String = format!("{:X}",random_value(255, 120));
    let green :String = format!("{:X}", random_value(70,17));

    //println!("green : {}", green);
    //println!(" blue: {}", blue);

    let end :String = format!("{}{}{}",blue, green , blue);
    println!("{}", end);
    return end;
}