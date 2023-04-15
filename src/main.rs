pub mod grabby;

use grabby::*;


fn main() {
    
 

    let c = "llo"; //type &str
    let text = "Hello World";

    let result = get_substring_to_end(c, text);

    match result {
        Ok(v) => println!("Resulting match = : {v:?}"),
        Err(e) => println!("Error no match found: {e:?}"),
    }

    get_every_part_of_filepath("/home/logan/test.txt");
    
}
