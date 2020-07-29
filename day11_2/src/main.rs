#[inline]
fn comp(x:i32,y:i32,serial:i32)->i32 {
    let rack_id = x+10;
    let mut level = rack_id*y;
    level+=serial;
    level*=rack_id;
    level = (level/100)%10;
    level-5    
}

fn solve(serial:i32)->(usize,usize,usize) {

    let mut res = (0,0,0);
    let mut max = 0i64;

    let mut prec = vec![vec![0i64;301];301];

    for y in 1..=300 {
        for x in 1..=300 {
            prec[y][x] = prec[y][x-1] + comp(x as i32,y as i32,serial) as i64;
        }
    }
    
    for y in 1..=300 {
        for x in 1..=300 {
            for size in 1..=300 {
                if x+size<=300 && y+size<=300 {
                    let mut s = 0;
                    for yy in y..y+size {
                        s+= prec[yy][x+size-1] - prec[yy][x-1];
                    }

                    if s>max
                    { 
                        max = s;
                        res = (x,y,size);
                    }
                }
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
    assert_eq!(res,(90,269,16));
}

#[test]
fn test5() {
    let res = solve(42);
    assert_eq!(res,(232,251,12));
}
