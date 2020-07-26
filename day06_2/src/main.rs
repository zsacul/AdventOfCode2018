use std::collections::HashMap;

fn dist(a:(i32,i32),b:(i32,i32))->i32 {
    return (a.0-b.0).abs() + (a.1-b.1).abs()
}

fn sum_dist(p:(i32,i32),points:&Vec<(i32,i32)>)->i32 {
    let mut res = 0;

    for i in 0..points.len() {
        res+= dist(p,points[i]);
    }
    res
}

fn print(field:HashMap<(i32,i32),i32>,f:i32,t:i32)
{
    for y in f..t {
        for x in f..t {
            let c = field.get(&(x as i32,y as i32)).unwrap_or(&0);
            print!("{}",c);
        }        
        println!("");
    }    
}

fn solve(d:Vec<i32>,less:i32)->i32 {
    let mut points = vec![];
    for i in 0..d.len()/2 {        
        let p:(i32,i32) = (d[2*i+0],d[2*i+1]);
        points.push(p);
    }

    let f = 0;
    let t = 450;
    let mut sum_all=0;
    
    for y in f..t {
        for x in f..t {
            let sum = sum_dist((x,y), &points);
            if sum<less {
                sum_all+=1;
            }
        }        
    }

    sum_all
}

fn main() {


let data = vec![
194, 200,
299, 244,
269, 329,
292, 55,
211, 63,
123, 311,
212, 90,
292, 169,
359, 177,
354, 95,
101, 47,
95, 79,
95, 287,
294, 126,
81, 267,
330, 78,
202, 165,
225, 178,
266, 272,
351, 326,
180, 62,
102, 178,
151, 101,
343, 145,
205, 312,
74, 193,
221, 56,
89, 89,
242, 172,
59, 138,
83, 179,
223, 88,
297, 234,
147, 351,
226, 320,
358, 338,
321, 172,
54, 122,
263, 165,
126, 341,
64, 132,
264, 306,
72, 202,
98, 49,
238, 67,
310, 303,
277, 281,
222, 318,
357, 169,
123, 225];

    let res = solve(data,10000);
    println!("{}",res);

}

#[test]
fn test0() {
    let d = vec![
    1, 1,
    1, 6,
    8, 3,
    3, 4,
    5, 5,
    8, 9];
    let res = solve(d,32);
    assert_eq!(res,16);
}
