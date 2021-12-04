use std::collections::HashMap;
use std::io;
use std::fs;

fn create_hash_map(letterstring: String) -> HashMap<char, i32> {
    let mut i: i32 = 1;
    let mut hashmap: HashMap<char, i32> = HashMap::new();
    for c in letterstring.chars() {
        hashmap.insert(c, i);
        i += 1;
    }
    return hashmap;
}

fn generate_letter_sum(letterstring: String, letterdict: HashMap<char, i32>) -> i32 {
    let mut letter_value: i32 = 0;
    for c in letterstring.to_lowercase().chars() {
        let value = letterdict.get(&c);
        if value == None {
            letter_value += 0;
        } else {
            letter_value += value.unwrap();
        }
    }
    letter_value
}

fn get_input(prompt: &str) -> String{
    println!("{}",prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}

fn menu() {
    println!("---------MENU---------");
    println!("1) Calculate word letter value");
    println!("2) Find all words with a specific value");
    println!("3) Exit");
    println!("----------------------");
}

fn calc_word_value_loop() {
    loop {
        let letterstring= String::from("abcdefghijklmnopqrstuvwxyz");
        let hashmap = create_hash_map(letterstring);
        let input: String = get_input("Please input a letter value");
        
        match input.as_str() {
            "exit()" => break,
            &_ => {
                let value = generate_letter_sum(input, hashmap);
                println!("The value of your string word is: {}", value);
            }
        }
    }
}


fn generate_list_of_words() -> HashMap<String, i32> {
    //Generate hashmap
    let mut returnhash: HashMap<String, i32> = HashMap::new();
    
    //read file named words.txt
    let contents = fs::read_to_string("./words.txt").expect("Something went wrong with reading the wordlist file!");
    println!("Generating word list hashmap, this may take some time!");
    for s in contents.lines() {
        let real_str: String = String::from(s);
        let hashmap = create_hash_map(String::from("abcdefghijklmnopqrstuvwxyz"));
        let value = generate_letter_sum(real_str, hashmap);
        returnhash.insert(String::from(s), value);
        //println!("Key {}, Value {}", s, value);
    }
    
    return returnhash;
}

fn all_words_with_value_loop() {
    let hashmap = generate_list_of_words();
    loop {
        let input = get_input("Please input a letter value you would like to search for.");
        
        match input.as_str() {
            "exit()" => break,
            &_ => ()
        }

        if let Err(_e) = input.parse::<i32>() {
            println!("Invalid input, unable to convert to i32!");
        }

        for (key, value) in &hashmap {
            if let Err(_e) = input.parse::<i32>() {
                println!("Invalid input, unable to convert to i32!");
            }
            let searchval = input.parse::<i32>().unwrap();
            if value == &searchval {
                println!("Found {} which matches input {}", key, input);
            }
        }
    }
}

fn main() { 
    loop {
        menu();
        let input: String = get_input("Please input an option: 1-3");
        
        

        match input.as_str() {
            "1" => calc_word_value_loop(),
            "2" => all_words_with_value_loop(),
            "3" => {
                println!("Closing....");
                break;
            }
            &_ => println!("not an available value!")
        }
    }  
}

