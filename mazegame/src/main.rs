use termion::{clear, cursor, input::TermRead, raw::IntoRawMode, terminal_size};
use termion::event::{Key, Event};
use std::{self, io::{Write, stdout, stdin}};
use rand::Rng;

fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    let (term_x, term_y): (u16, u16) = terminal_size().unwrap();
    let (mut player_x, mut player_y): (u16, u16) = (2, 2);
    let maze: String = gen_maze(12,40);

    write!(stdout, "{}", clear::All).expect("msg");

    let mut maze_vec = Vec::new();
    for (i, s) in maze.split("\n").enumerate(){
        writeln!(stdout, "{}{}", cursor::Goto(1, i as u16 + 1), s).unwrap();
        let mut maze_chars = Vec::new();
        for c in s.chars(){
            maze_chars.push(c);
        }
        maze_vec.push(maze_chars);
    }

    write!(stdout, "{}@{}", cursor::Goto(player_x, player_y), cursor::Goto(term_x, term_y)).unwrap();
    stdout.flush().unwrap();

    for c in stdin.events() {
        let (last_player_x, last_player_y) = (player_x, player_y);

        let evt = c.unwrap();
        match evt {
            Event::Key(key) => {
                match key {
                    Key::Char('q') => {
                        break;
                    },
                    Key::Char('s') => {
                        player_y += 1;
                    },
                    Key::Char('w') => {
                        player_y -= 1;
                    },
                    Key::Char('a') => {
                        player_x -= 1;
                    },
                    Key::Char('d') => {
                        player_x += 1;
                    },
                    _ => {}
                }
            }
            _ => {}
        }
        
        //Check if player is out of bounds
        if player_x <= 0 || player_y <= 0 {
            player_x = last_player_x;
            player_y = last_player_y;
        }

        //Check if player is colliding (vector x/y is reverse of terminal x/y)
        match maze_vec.get((player_y - 1) as usize){
            Some(x) =>  {
                match x.get((player_x - 1) as usize) {
                    Some(c) => {
                        if *c == '#' {
                            player_x = last_player_x;
                            player_y = last_player_y;
                        }
                    },
                    None => {}
                }
            },
            None => {}
        }
        //Replace old position with whitespace
        if last_player_x != player_x || last_player_y != player_y {
            write!(stdout, "{} ", cursor::Goto(last_player_x, last_player_y)).unwrap();
        }

        write!(stdout, "{}@{}", cursor::Goto(player_x, player_y), cursor::Goto(term_x, term_y)).unwrap();
        stdout.flush().unwrap();
    }
}

/*
    This function is really fucking bad, need to refactor it and implement propper maze algo.
    Works for testing purposes tho.
*/
fn gen_maze(size_x: u16, size_y: u16) -> String {
    let mut maze: String = String::new();
    let mut maze_vec = Vec::new();
    for _ in 0..size_x {
        let mut line_vec = Vec::new();
        for _ in 0..size_y {
            line_vec.push('#');
        }
        maze_vec.push(line_vec);
    }
    let mut rng = rand::thread_rng();

    //Shitty bsp
    for x in 0 .. maze_vec.len() {
        for y in 0 .. maze_vec[x].len() {
            let mut ycheck = 1;
            let mut xcheck = 0;
            if rng.gen_bool(1.0/2.0) {
                ycheck = 0;
                xcheck = 1;
            }
            if y > 0 && x + xcheck < maze_vec.len() {
                maze_vec[x + xcheck][y - ycheck] = ' ';
            }
        }
    }
    //Padding for roof
    for _ in 0..size_y+2 {
        maze.push('#');
    }
    maze.push('\n');
    //Convert char 2d vector to string
    for line in &maze_vec {
        maze.push('#');
        for c in line {
            maze.push(*c);
        }
        maze.push('#');
        maze.push('\n');
    }

    for _ in 0..size_y+2 {
        maze.push('#');
    }

    return maze;
}