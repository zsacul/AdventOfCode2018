//todo replace brute force vector solution with cyclic list
//3180929875

fn print(time:i32,data:&Vec<i32>,act:usize) {
    print!("[{}] ",time);
    for i in 0..data.len() {
        if i==act {
            print!("({}) ",data[i]);
        }
        else {
            print!(" {}  ",data[i]);
        }
    }
    println!("");
}

fn solve(players:i32,last_points:i64)->i64 {
    let mut act = 0;
    let mut data = vec![0i64];
    let mut act_player = 1;
    let mut scores = vec![0i64;players as usize];

    //print(0,&data,act);

    for i in 1..=last_points
    {
        if i%23==0 {
            act = (act+data.len()-8)%data.len() + 1;
            scores[act_player]+=data[act];
            scores[act_player]+=i;
            data.remove(act);
        }
          else 
        {
            act = (act+1)%data.len() + 1;

            if act==data.len() {
                data.push(i);
            }
            else
            {   
                data.insert(act,i)
            }
        }

       // print(act_player as i32,&data,act);
        act_player = (act_player+1)%(players as usize);      

        if i%72170==0 {
            println!("{}/{}",i,last_points);
        }
    }

    *scores.iter().max().unwrap()
}

fn main() {
    let res = solve(470,72170*100);
    //let res = solve(9,25);
    println!("{}",res);
}

#[test]
fn test00(){
    let res = solve(9,25);
    assert_eq!(res,32);
}

#[test]
fn test0(){
    let res = solve(10,1618);
    assert_eq!(res,8317);
}

#[test]
fn test1(){
    let res = solve(13,7999);
    assert_eq!(res,146373);
}

#[test]
fn test2(){
    let res = solve(17,1104);
    assert_eq!(res,2764);
}

#[test]
fn test3(){
    let res = solve(21,6111);
    assert_eq!(res,54718);
}

#[test]
fn test4(){
    let res = solve(30,5807);
    assert_eq!(res,37305);
}


//470 players; last marble is worth 72170 points