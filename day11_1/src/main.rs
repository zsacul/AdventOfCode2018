
fn comp(x:i32,y:i32,serial:i32)->i32 {
    let rack_id = x+10;
    let mut level = rack_id*y;
    level+=serial;
    level*=rack_id;
    level = (level/100)%10;
    level-5    
}

fn solve(serial:i32)->(i32,i32) {

    let mut res = (0,0);
    let mut max = 0;
    
    for y in 1..=300-3 {
        for x in 1..=300-3 {
            let mut s = 0;
            for yy in y..y+3 {
                for xx in x..x+3 {
                    s+= comp(xx,yy,serial);
                }
            }

            if s>max
            { 
                max = s;
                res = (x,y);
            }
        }
    }
    res
}

fn main() {
    let res = solve(6392);
    println!("{:?}",res);
}

#[test]
fn test0() {
    let res = comp(3,5,8);
    assert_eq!(res,4);
}

#[test]
fn test1() {
    let res = comp(122,79,57);
    assert_eq!(res,-5);
}

#[test]
fn test2() {
    let res = comp(217,196,39);
    assert_eq!(res,0);
}

#[test]
fn test3() {
    let res = comp(101,153,71);
    assert_eq!(res,4);
}

#[test]
fn test4() {
    let res = solve(18);
    assert_eq!(res,(33,45));
}

#[test]
fn test5() {
    let res = solve(42);
    assert_eq!(res,(21,61));
}
