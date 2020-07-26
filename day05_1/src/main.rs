use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn simplify(data:String)->(String,bool) {
    if data=="" { return ("".to_string(),false); }
    let mut chars: Vec<char> = data.chars().collect();
    let mut changed = false;

    for i in 0..chars.len()-1 {
        let c = chars[i];
       // println!("{},",c);
        
        if chars[i]!=' ' && chars[i].is_alphabetic() && chars[i+1].is_alphabetic() {
            let cl = c.to_uppercase().nth(0).unwrap();
            let cu = c.to_lowercase().nth(0).unwrap();

            //println!("{},{},{}",c,cl,cu);
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

fn solve(data:String)->String {
    let mut d = data;
    
    loop {
        let (res,changed) = simplify(d);
        d = res;
        if !changed { break; }
    }
    
    d.to_string()
}

fn main() {

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
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
    assert_eq!(s,"dabCBAcaDA");
}

#[test]
fn test2() {
    let s = solve("DaAbBCcdDaAd".to_string());
    assert_eq!(s,"");
}

#[test]
fn test3() {
    let s = solve("aA".to_string());
    assert_eq!(s,"");
}
#[test]
fn test4() {
    let s = solve("abBA".to_string());
    assert_eq!(s,"");
}
#[test]
fn test5() {
    let s = solve("abAB".to_string());
    assert_eq!(s,"abAB");
}
#[test]
fn test6() {
    let s = solve("aabAAB".to_string());
    assert_eq!(s,"aabAAB");
}
