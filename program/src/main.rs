mod load;
mod search;

fn main() {
    // load the data 
    let svg :String = load::load("./images/image.svg");
    println!("{:?}",svg);

    // find all the # in the data
    let pos :Vec<u32> = search::find(svg, '#');
    
    //should give the positions of the start and end of the hash 
    let indexes :Vec<(u32, u32)> = search::gen_bounds(pos, 6);
    println!("{:?}", indexes);
    
}
