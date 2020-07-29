use std::collections::HashMap;

fn getc(state:&String,i:i32)->char {
    state.chars().nth(i as usize).unwrap_or('.')
}

fn count(state:String,shift:i64)->i64 {
    let mut sum = 0;
    for i in 0..state.len() {
        if state.chars().nth(i).unwrap_or('.')=='#' {
            sum+=shift+i as i64;
        }        
    }
    sum
}

fn solve(initial_state:&str,data:Vec<&str>)->i64 {
    let mut h = HashMap::new();

    for s in data {
        let lr: Vec<_> = s.split(" => ").collect();
        h.insert(lr[0],lr[1]);
    }

    let mut state = format!("...{}................................................................",initial_state.to_string());
    println!("{}: {}",0,state);
    let mut shift = -3i64;
    
    for i in 1..=1000 {
        let mut new_state = "".to_string();

        for i in 0..state.len() as i32 {
            let mut code = "".to_string();
            for id in i-2..=i+2 {
                code.push_str(&getc(&state,id as i32).to_string()[..]);
            }
            // println!("{}",code);
            let c = h.get(&code[..]).unwrap_or(&".");
            new_state.push_str(&c[..]);        
        }
        state = new_state;
        
        if state.find(".....").unwrap_or(888)==0 {
            state = format!("{}.",state[1..].to_string());
            shift+=1;
        }
        if state.find(".#").unwrap_or(888)==0 {
            state = format!(".{}",state);
            shift-=1;
        }
       
        println!("{}: {}={},{}",i,state,count(state.clone(),shift),shift);
    }



//2100000001168
    count(state,50000000000i64-1000i64+shift as i64)
}

fn main() {
    let initial_state = "##....#.#.#...#.#..#.#####.#.#.##.#.#.#######...#.##....#..##....#.#..##.####.#..........#..#...#";

    let data = vec![
    "..#.# => #",
    ".#### => #",
    "#.... => .",
    "####. => #",
    "...## => .",
    ".#.#. => .",
    "..#.. => .",
    "##.#. => .",
    "#.#.# => #",
    "..... => .",
    "#.#.. => .",
    "....# => .",
    ".#..# => .",
    "###.# => #",
    "#..#. => .",
    "##### => .",
    "...#. => #",
    "#.##. => #",
    ".#.## => #",
    "#..## => #",
    ".##.. => #",
    "##.## => .",
    "..### => .",
    "###.. => .",
    "##..# => #",
    ".#... => #",
    ".###. => #",
    "#.### => .",
    ".##.# => .",
    "#...# => #",
    "##... => .",
    "..##. => .",
    ];

    let res = solve(initial_state,data);
    println!("{}",res);    
}
