// This is my first attempt at coding in rust do not askh ow long this took.
// 5/4/2024  5:36am
use std::io;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static IS_X: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(true));

fn main() {
    //My main funciton!
    //Board array init
    let mut array: [[&str; 3]; 3] = [
    [" ", " ", " "],
    [" ", " ", " "],
    [" ", " ", " "],
    ];
    //This loop plays the game until it hits a tie or a win.
    //I think loop is really funny
    loop{
    print_turn();
    print_board(&array);
    if is_game_won(&mut array) == true{
        break;
    }else{
        if is_game_tied(&mut array) == true{
            println!("Game is tied!");
            break;
        }else{
        prompt_player(&mut array);
        }
    }
    }
}

fn print_board(array: &[[&str; 3]; 3]) {
    //Prints out the board array to look like a tic tak toe board.
    //Again this took me a hot minute longer than it should have
    for (row_index, row) in array.iter().enumerate() {
        if row_index > 0 {
            println!("   -----------");
        }
        for (col_index, &val) in row.iter().enumerate() {
            if col_index > 0 {
                print!("|")
            }else{
                print!("{}",2-row_index);
            }
            print!(" {}  ", val); 
        }
        println!(); 
    }
    println!("   0   1   2");
}

fn prompt_player(array: &mut [[&str; 3]; 3]) {
    //Prompts the player and takes user input
    //Fun fact I forgot about trim for a few minutes and got really frustrated when my line was never length 1 
    println!("Input the x,y of the space you would like to go");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let input = input.trim();
    let parts: Vec<&str> = input.split(',').collect();

    if parts.len() == 2 {
        match (parts[0].trim().parse::<usize>(), parts[1].trim().parse::<usize>()) {
            (Ok(x), Ok(y)) if x <= 2 && y <= 2 => {
                if is_valid_space(x, y, array) {
                    let mut guard = IS_X.lock().unwrap();
                    if *guard {
                        array[2-y][x] = "X";
                        *guard = false;
                    } else {
                        array[2-y][x] = "O";
                        *guard = true;
                    }
                } else {
                    println!("The chosen space is already occupied. Please choose another.");
                }
            },
            _ => println!("Invalid input. Both coordinates must be 0, 1, or 2 and must be valid integers."),
        }
    } else {
        println!("Invalid format. Please input as x,y");
    }
}
fn print_turn() {
    //Prints out the turn depending on a global IS_X
    let guard = IS_X.lock().unwrap();
    if *guard {
        println!("It is X's turn.");
    } else {
        println!("It is O's turn.");
    }
}
fn print_winner() {
    //Print out whoever has won the game by seeing who went last
    let guard = IS_X.lock().unwrap();
    if *guard {
        println!("Game has been won by O!");
    } else {
        println!("Game has been won by X!");
    }
}

fn is_valid_space(x:usize,y:usize,array: &[[&str; 3]; 3]) -> bool{
    //Makes sure there is not already a character in that space
    return array[2-y][x] == " ";
}

fn is_game_tied(array: &[[&str; 3]; 3]) -> bool{
    //Checks if the game is tied by seeing if there is still open spaces.
    for row in array.iter() {
        for &space in row.iter() {
            if space == " " {
                return false;
            }
        }
    }
    true
}

fn is_game_won(array: &[[&str; 3]; 3]) -> bool{
    //I felt like a genuis righting this function i dont know why it made me giggle
        //Checks if the game has been won horizonally or vertically
        for i in 0..3 {
            if (array[i][0] == array[i][1] && array[i][1] == array[i][2] && array[i][0] != " ") ||
               (array[0][i] == array[1][i] && array[1][i] == array[2][i] && array[0][i] != " ") {
                print_winner();
                return true;
            }
        }
        //Checks if the game has been won diagonally
        if (array[0][0] == array[1][1] && array[1][1] == array[2][2] && array[0][0] != " ") ||
           (array[0][2] == array[1][1] && array[1][1] == array[2][0] && array[0][2] != " ") {
            print_winner();
            return true;
        }
    
        false
}



