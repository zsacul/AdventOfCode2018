use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug,Clone,Copy)]
struct Worker {
    letter: char,
    time_left: i32,
}

impl Worker {
    fn new(l:char,time:i32)->Self {
        Worker {
            letter:l,
            time_left:time
        }
    }

    fn empty()->Self {
        Worker::new('.',0)
    }

    fn get_time(l:char)->i32 {
        l as i32 - b'A' as i32 + 1 + 60
    }

    fn set_letter(&mut self,l:char){
        self.letter = l;
        self.time_left = Worker::get_time(l);
    }

    fn tick(&mut self)->bool {
        if self.time_left==0 {
            return false;
        }

        self.time_left-=1;
        self.time_left==0
    }
}

fn get_free_worker(tab:&Vec<Worker>)->i32 {
    for i in 0..tab.len() {
        if tab[i].letter=='.' {
            return i as i32;
        }
    }
    -1
}

fn all_done(tab:&Vec<Worker>)->bool {
    for i in 0..tab.len() {
        if tab[i].letter!='.' {
            return false;
        }
    }
    true
}

fn print_workers(time:i32,tab:&Vec<Worker>){
    print!("{}  = ",time);
    for w in tab {
        print!("l:{}({})  ",w.letter,w.time_left);
    }
    println!("");
}


fn solve(data:&Vec<&str>,workers_count:usize)->String {
    let mut req : HashMap<char,HashSet<char>>= HashMap::new();
    let mut unused_letters : HashSet<char> = HashSet::new();

    for d in data {
        let f = d.chars().nth(5).unwrap_or(' ') as char;
        let t = d.chars().nth(36).unwrap_or(' ') as char;

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

    let mut res = "".to_string();
    let mut time = 0i32;

    let mut workers = vec![Worker::empty();workers_count];
    
    loop {

        for cc in b'A'..=b'Z' {
            let c = cc as char;

            if unused_letters.get(&c)!=None
            {
                if req.get(&c)==None || req.get(&c).unwrap().len()==0 {                    

                    let fw = get_free_worker(&workers);
                    if fw==-1 {
                        break;
                    }
                    else
                    {                        
                        workers[fw as usize].set_letter(c);                        
                        unused_letters.remove(&c);                                            
                    }
                }
            }         
        }


        print_workers(time,&workers);
        if all_done(&workers) { break; }
        time+=1;

        for w in workers.iter_mut() {
            if w.tick() {
                let cc = w.letter;
                res.push_str(&cc.to_string());
                for se in req.values_mut() {
                    if se.get(&cc)!=None {
                        se.remove(&cc);
                    }
                }
                *w = Worker::empty();
            }
        }     

        
    }

    println!("result: {}",time);

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


//    let res = solve(&_d,2);
    let res = solve(&data,5);
    println!("{}",res);
    //1709

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

    let res = solve(&d,2);
    assert_eq!(res,"CABFDE");
}