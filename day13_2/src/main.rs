use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Cart {
    x:i32,
    y:i32,
    dir:char,
    rotation_id:i32,
}

impl Cart {
    fn new(x:i32,y:i32,d:char)->Self {
        Cart {
            x:x,
            y:y,
            dir:d,
            rotation_id: 0,           
        }
    }

    fn move_cart(&mut self){
        match self.dir {
            '^' => { self.y-=1; },
            'v' => { self.y+=1; },
            '>' => { self.x+=1; },
            '<' => { self.x-=1; },
            _ => panic!("unknown dir")
        }
    }

    fn right(&mut self){
        match self.dir {
            '^' => { self.dir='>'; },
            'v' => { self.dir='<'; },
            '>' => { self.dir='v'; },
            '<' => { self.dir='^'; },
            _ => panic!("unknown dir")
        }
    }

    fn look(&mut self,see:char)
    {
        match see {
            '+'  => { self.rotate(); },
            '\\' => { 
                    match self.dir {
                        '^' => { self.dir='<'; },
                        'v' => { self.dir='>'; },
                        '>' => { self.dir='v'; },
                        '<' => { self.dir='^'; },
                        _ => panic!("unknown dir")
                    }        
            },
            '/'  => { 
                        match self.dir {
                            '^' => { self.dir='>'; },
                            'v' => { self.dir='<'; },
                            '>' => { self.dir='^'; },
                            '<' => { self.dir='v'; },
                            _ => panic!("unknown dir")
                        }        
                    },
            _    => {}
        }
    }

    fn rotate(&mut self)
    {
        match self.rotation_id {
            0 => { self.right(); self.right(); self.right(); },
            1 => {                                           },
            2 => { self.right();                             },
            _ => panic!("wrond rotation dir")
        } 
        self.rotation_id = (self.rotation_id+1)%3;
    }

    fn is_cart(c:char)->bool {        
        c=='^' || c=='v' || c=='>' || c=='<'
    }
}

fn print(f:&Vec<Vec<char>>) {
    for l in f {
        for c in l {
            print!("{}",c);
        }
        println!("");
    }
}

fn crash_ids(c:&Vec<Cart>)->Option<(usize,usize)> {
    for a in 0..c.len() {
        for b in a+1..c.len() {
            if c[a].x==c[b].x && c[a].y==c[b].y {
                return Some((a,b));
            }
        }
    }
    None
}


fn solve(filename:&str)->(i32,i32) {

    let mut field = vec![];
    let mut carts = vec![];
    
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let mut field_line = vec![];
            if let Ok(ip) = line {
                for c in ip.chars() {
                    field_line.push(c);
                }               
            }
            field.push(field_line.clone());
        }
    }
    
    for y in 0..field.len()
    {
        for x in 0..field[0].len()
        {
            let c = field[y][x];
            if Cart::is_cart(c) {
                carts.push(Cart::new(x as i32,y as i32,c));
            }
        }
    }

    //println!("{:?}",carts);

    while carts.len()>1 {
    
        if crash_ids(&carts)!=None {
            let (i1,i2) = crash_ids(&carts).unwrap();
            carts.remove(i2);
            carts.remove(i1);
            //println!("{:?}",carts);
        }
        else
        {
            for c in carts.iter_mut() {
                c.move_cart();
                c.look(field[c.y as usize][c.x as usize]);
            }
        }
    }

    (carts[0].x,carts[0].y)    
}

fn main() {
    let res = solve("./data.txt");
    println!("{:?}",res);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
} 

#[test]
fn test0() {
    let res = solve("./test.txt");
    assert_eq!(res,(6,4));
}
