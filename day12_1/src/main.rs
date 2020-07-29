use std::collections::HashMap;

fn getc(state:&String,i:i32)->char {
    state.chars().nth(i as usize).unwrap_or('.')
}

fn count(state:String,shift:i32)->i32 {
    let mut sum = 0;
    for i in 0..state.len() {
        if state.chars().nth(i).unwrap_or('.')=='#' {
            sum+=shift+i as i32;
        }        
    }
    sum
}

fn solve(initial_state:&str,data:Vec<&str>)->i32 {
    let mut h = HashMap::new();

    for s in data {
        let lr: Vec<_> = s.split(" => ").collect();
        h.insert(lr[0],lr[1]);
    }

    let mut state = format!("...{}............................",initial_state.to_string());
    println!("{}: {}",0,state);
    
    for i in 1..=20 {
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
    
        println!("{}: {}",i,state);
    }

    count(state,-3)  
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

//#[test]
fn test0() {
    let initial_state = "#..#.#..##......###...###";

    let data = vec![
    "...## => #",
    "..#.. => #",
    ".#... => #",
    ".#.#. => #",
    ".#.## => #",
    ".##.. => #",
    ".#### => #",
    "#.#.# => #",
    "#.### => #",
    "##.#. => #",
    "##.## => #",
    "###.. => #",
    "###.# => #",
    "####. => #",
    ];
    
    let res = solve(initial_state,data);
    assert_eq!(res,325);

}
