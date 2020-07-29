fn print(rec:&Vec<usize>,i1:usize,i2:usize) {
    for i in 0..rec.len() {
        if i==i1 as usize {
            print!("({})",rec[i]);
        }
        else if i==i2 as usize {
            print!("[{}]",rec[i]);
        }
        else {
            print!(" {} ",rec[i]);
        }
    }
    println!("");
}

fn solve(n:i32)->String {
    let mut rec = vec![3usize,7usize];
    let mut lev1 = 0;
    let mut lev2 = 1;

    //print(&rec,lev1,lev2);
    let mut res = "".to_string();

    for _time in 0..n+9 {
        let sum = rec[lev1] + rec[lev2];
        if sum>9 {
            rec.push(sum/10);            
        }
        rec.push(sum%10);

        lev1 = (lev1+rec[lev1]+1)%rec.len();
        lev2 = (lev2+rec[lev2]+1)%rec.len();
        
      //  print(&rec,lev1,lev2);
    }

    for i in n..n+10
    {
        res.push_str(&format!("{}",rec[i as usize])[..])
    }

    res
}

fn main() {
    let res = solve(580741);
    println!("{}",res);
}


#[test]
fn test0()
{
    assert_eq!(solve(9),"5158916779");
}

#[test]
fn test1()
{
    assert_eq!(solve(5) , "0124515891");
}

#[test]
fn test2()
{
    assert_eq!(solve(18) , "9251071085");
}

#[test]
fn test3()
{
    assert_eq!(solve(2018) , "5941429882");
}
