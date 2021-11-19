use std::io;
use rand::Rng;

fn main() {
    let word_strings = "ant baboon badger bat bear beaver camel cat clam cobra cougar coyote crow deer dog donkey duck eagle ferret fox frog goat goose hawk lion lizard llama mole monkey moose mouse mule newt otter owl panda parrot pigeon python rabbit ram rat raven rhino salmon seal shark sheep skunk sloth snake spider stork swan tiger toad trout turkey turtle weasel whale wolf wombat zebra".split(" ");
    let words: Vec<&str> = word_strings.collect();
    let mut rng = rand::thread_rng();
    let mut won = false;
    let word: &str = words[rng.gen_range(0..words.len())];
    println!("Welcome to hangman!\nWill only get the first char of your input.");
    let mut guesses: usize = 0;
    let mut correct = String::new();
    while guesses < 6 {
        println!("{}", draw_game(guesses));
        let mut hidden = String::new();
        for c in word.chars() {
            if correct.contains(c){
                hidden.push(c);
            } else {
                hidden.push('_');
            }
        }
        println!("Your word: {}", hidden);
        if !hidden.contains('_'){
            won = true;
            break;
        }
        let input = get_input();
        if word.contains(input){
            correct.push(input);
            continue;
        }
        guesses += 1;
    }

    println!("{}", draw_game(guesses));
    println!("Your word: {}", word);
    if won {
        println!("Congrats you won!!!!!!");
    } else {
        println!("Better luck next time! :)");
    }
}

fn draw_game(guesses: usize) -> String {
    print!("{}[2J", 27 as char);
    let mut outstr = String::new();
    outstr.push_str(get_ascii(guesses).as_str());
    outstr.push_str(format!("\nWrong guesses: {}\n", guesses).as_str());

    return outstr;
}

fn get_ascii(i: usize) -> String {
    let hangmanpics = vec![r#"
    +---+
    |   |
        |
        |
        |
        |
  =========
    "#,
    r#"
    +---+
    |   |
    O   |
        |
        |
        |
  =========
    "#,
    r#"
    +---+
    |   |
    O   |
    |   |
        |
        |
  =========
    "#,
    r#"
    +---+
    |   |
    O   |
   /|   |
        |
        |
  =========
    "#,
    r#"
    +---+
    |   |
    O   |
   /|\  |
        |
        |
  =========
    "#,
    r#"
    +---+
    |   |
    O   |
   /|\  |
   /    |
        |
  =========
  "#,
  r#"
    +---+
    |   |
    O   |
   /|\  |
   / \  |
        |
  =========
"#];
    return hangmanpics[i].to_string();
}

fn get_input() -> char {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).expect("Did not enter a correct string");

    return buffer.chars().next().unwrap();
}