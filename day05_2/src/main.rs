use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn simplify(data:String)->(String,bool) {
    if data=="" { return ("".to_string(),false); }
    let mut chars: Vec<char> = data.chars().collect();
    let mut changed = false;

    for i in 0..chars.len()-1 {
        let c = chars[i];
        
        if chars[i]!=' ' && chars[i].is_alphabetic() && chars[i+1].is_alphabetic() {
            let cl = c.to_uppercase().nth(0).unwrap();
            let cu = c.to_lowercase().nth(0).unwrap();

            if (chars[i]==cu && chars[i+1]==cl) ||
               (chars[i]==cl && chars[i+1]==cu)
            {
                chars[i  ] = ' ';
                chars[i+1] = ' ';
                changed = true;
            }
        }
    }

    (chars.iter().filter(|c| *c!=&' ').collect(),changed)
}

fn filter(data:String,c:char)->String {
    let cl = c.to_uppercase().nth(0).unwrap();
    let cu = c.to_lowercase().nth(0).unwrap();

    data.chars().filter(|c| *c!=cl && *c!=cu).collect()
}

fn solve_one(data:String)->String {
    let mut d = data;
    
    loop {
        let (res,changed) = simplify(d);
        d = res;
        if !changed { break; }
    }
    
    d.to_string()
}

fn solve(data:String)->String {
    let mut res = data.clone();

    for c in b'a'..=b'z' {
        let d = filter(data.clone(),c as char);
        let r = solve_one(d);

        if r.len()<res.len() {
            res = r.clone();
        }
    }
    res
}

fn main() {
    if let Ok(lines) = read_lines("./data.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let res = solve(ip.to_string()).len();
                println!("{}",res);
                return;
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


#[test]
fn test1() {
    let s = solve("dabAcCaCBAcCcaDA".to_string());
    assert_eq!(s,"daDA");
}
