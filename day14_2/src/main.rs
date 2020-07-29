fn solve(n:&str)->usize {
    let mut rec = vec![3usize,7usize];
    let mut lev1 = 0;
    let mut lev2 = 1;

    let mut res = "37".to_string();
    let mut shift = 0usize;

    loop {
        let sum = rec[lev1] + rec[lev2];
        if sum>9 {
            rec.push(sum/10);            
        }
        rec.push(sum%10);

        lev1 = (lev1 + rec[lev1] + 1)%rec.len();
        lev2 = (lev2 + rec[lev2] + 1)%rec.len();
        res.push_str(&format!("{}",sum)[..]);     

        if res.find(n)!=None {
            return shift + res.find(n).unwrap();
        }    

        if res.len()>2*n.len() {
            shift+=res.len()/2;
            res = res[res.len()/2..].to_string();
        }
    }
}

fn main() {
    let res = solve("580741");
    println!("{}",res);
}

#[test]
fn test0()
{ 
    assert_eq!(solve("51589"),9);
}

#[test]
fn test1()
{
    assert_eq!(solve("01245") , 5);
}

#[test]
fn test2()
{
    assert_eq!(solve("92510") , 18);
}

#[test]
fn test3()
{     
    assert_eq!(solve("59414") , 2018);
}
