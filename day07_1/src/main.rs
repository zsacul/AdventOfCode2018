use std::collections::HashMap;
use std::collections::HashSet;

fn solve(data:&Vec<&str>)->String {
    let mut req : HashMap<char,HashSet<char>>= HashMap::new();
    let mut unused_letters : HashSet<char> = HashSet::new();

    for d in data {
        let f = d.chars().nth(5).unwrap_or(' ') as char;
        let t = d.chars().nth(36).unwrap_or(' ') as char;
        //println!("{}->{}",f,t);
        unused_letters.insert(t);
        unused_letters.insert(f);

        match req.get_mut(&t) {
            None => {
                let mut first = HashSet::new();
                first.insert(f);                
                req.insert(t, first); 
            },
            Some(s) => { s.insert(f); },
        }
    }


    //println!("{:?}",req);
    let mut res = "".to_string();
    
    while !unused_letters.is_empty() {
        for cc in b'A'..=b'Z' {
            let c = cc as char;

            if unused_letters.get(&c)!=None
            {
                if req.get(&c)==None || req.get(&c).unwrap().len()==0 {                    
                    res.push_str(&c.to_string());                   
                    unused_letters.remove(&c);

                    for se in req.values_mut() {
                        if se.get(&c)!=None {
                            se.remove(&c);
                        }
                    }
                    break;
                }
            }
            //println!("{:?}",req);
            //println!("{:?}",unused_letters);            
        }
    }

    res
}

fn main() {

    let data = vec![
    "Step P must be finished before step F can begin.",
    "Step F must be finished before step M can begin.",
    "Step Q must be finished before step S can begin.",
    "Step K must be finished before step G can begin.",
    "Step W must be finished before step X can begin.",
    "Step V must be finished before step I can begin.",
    "Step S must be finished before step Y can begin.",
    "Step U must be finished before step D can begin.",
    "Step J must be finished before step B can begin.",
    "Step Z must be finished before step C can begin.",
    "Step Y must be finished before step D can begin.",
    "Step X must be finished before step A can begin.",
    "Step E must be finished before step N can begin.",
    "Step M must be finished before step B can begin.",
    "Step N must be finished before step I can begin.",
    "Step I must be finished before step T can begin.",
    "Step H must be finished before step A can begin.",
    "Step A must be finished before step B can begin.",
    "Step O must be finished before step L can begin.",
    "Step T must be finished before step L can begin.",
    "Step D must be finished before step R can begin.",
    "Step G must be finished before step L can begin.",
    "Step C must be finished before step R can begin.",
    "Step R must be finished before step L can begin.",
    "Step L must be finished before step B can begin.",
    "Step O must be finished before step R can begin.",
    "Step Q must be finished before step I can begin.",
    "Step M must be finished before step L can begin.",
    "Step R must be finished before step B can begin.",
    "Step J must be finished before step O can begin.",
    "Step O must be finished before step B can begin.",
    "Step Y must be finished before step L can begin.",
    "Step G must be finished before step R can begin.",
    "Step P must be finished before step Z can begin.",
    "Step K must be finished before step Y can begin.",
    "Step X must be finished before step I can begin.",
    "Step E must be finished before step H can begin.",
    "Step I must be finished before step H can begin.",
    "Step P must be finished before step K can begin.",
    "Step G must be finished before step B can begin.",
    "Step H must be finished before step L can begin.",
    "Step X must be finished before step C can begin.",
    "Step P must be finished before step X can begin.",
    "Step X must be finished before step M can begin.",
    "Step Q must be finished before step H can begin.",
    "Step S must be finished before step Z can begin.",
    "Step C must be finished before step B can begin.",
    "Step N must be finished before step A can begin.",
    "Step M must be finished before step R can begin.",
    "Step X must be finished before step E can begin.",
    "Step P must be finished before step L can begin.",
    "Step H must be finished before step G can begin.",
    "Step E must be finished before step D can begin.",
    "Step D must be finished before step L can begin.",
    "Step W must be finished before step A can begin.",
    "Step S must be finished before step X can begin.",
    "Step V must be finished before step O can begin.",
    "Step H must be finished before step B can begin.",
    "Step T must be finished before step B can begin.",
    "Step Y must be finished before step C can begin.",
    "Step A must be finished before step R can begin.",
    "Step N must be finished before step L can begin.",
    "Step V must be finished before step Z can begin.",
    "Step W must be finished before step V can begin.",
    "Step S must be finished before step M can begin.",
    "Step Z must be finished before step A can begin.",
    "Step W must be finished before step S can begin.",
    "Step Q must be finished before step R can begin.",
    "Step N must be finished before step G can begin.",
    "Step Z must be finished before step L can begin.",
    "Step K must be finished before step O can begin.",
    "Step X must be finished before step R can begin.",
    "Step V must be finished before step H can begin.",
    "Step P must be finished before step R can begin.",
    "Step M must be finished before step A can begin.",
    "Step K must be finished before step L can begin.",
    "Step P must be finished before step M can begin.",
    "Step F must be finished before step N can begin.",
    "Step W must be finished before step H can begin.",
    "Step K must be finished before step B can begin.",
    "Step H must be finished before step C can begin.",
    "Step X must be finished before step H can begin.",
    "Step V must be finished before step U can begin.",
    "Step S must be finished before step H can begin.",
    "Step J must be finished before step X can begin.",
    "Step S must be finished before step N can begin.",
    "Step V must be finished before step A can begin.",
    "Step H must be finished before step O can begin.",
    "Step Y must be finished before step O can begin.",
    "Step H must be finished before step R can begin.",
    "Step X must be finished before step T can begin.",
    "Step J must be finished before step H can begin.",
    "Step G must be finished before step C can begin.",
    "Step E must be finished before step R can begin.",
    "Step W must be finished before step J can begin.",
    "Step F must be finished before step E can begin.",
    "Step P must be finished before step I can begin.",
    "Step F must be finished before step T can begin.",
    "Step J must be finished before step L can begin.",
    "Step U must be finished before step Z can begin.",
    "Step Q must be finished before step D can begin.",
    ];

    let _d = vec![
        "Step C must be finished before step A can begin.",
        "Step C must be finished before step F can begin.",
        "Step A must be finished before step B can begin.",
        "Step A must be finished before step D can begin.",
        "Step B must be finished before step E can begin.",
        "Step D must be finished before step E can begin.",
        "Step F must be finished before step E can begin.",
    ];


//    let res = solve(&_d);
    let res = solve(&data);
    println!("{}",res);

}
 


#[test]
fn test0() {

    let d = vec![
        "Step C must be finished before step A can begin.",
        "Step C must be finished before step F can begin.",
        "Step A must be finished before step B can begin.",
        "Step A must be finished before step D can begin.",
        "Step B must be finished before step E can begin.",
        "Step D must be finished before step E can begin.",
        "Step F must be finished before step E can begin.",
    ];

    let res = solve(&d);
    assert_eq!(res,"CABDFE");
}