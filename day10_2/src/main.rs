//nexplrxk
//10459

fn get_from_string(s:String,start:&str,end:&str)->String
{
    let st = s.find(start);
    let en = s.find(end);

    if st!=None && en!=None {
        return s[st.unwrap()+start.len()..en.unwrap()].to_string();
    }

    "".to_string()
}

fn get_point(s:String)->(i32,i32) {
    let lr: Vec<_> = s.split(",").collect();

    let x = lr[0].trim().parse::<i32>().unwrap();
    let y = lr[1].trim().parse::<i32>().unwrap();
    (x,y)
}

fn get_position(s:String)->(i32,i32) {
    let s = get_from_string(s,"n=<","> ");
    get_point(s)
}

fn get_velocity(s:String)->(i32,i32) {
    let st = s.find("y=<");
    let lr = &s[st.unwrap()+3..s.len()-1];
    get_point(lr.to_string())
}

fn move_points(points:&mut Vec<(i32,i32)>,velocity:&Vec<(i32,i32)>)
{
    for i in 0..points.len() {
        points[i].0+=velocity[i].0;
        points[i].1+=velocity[i].1;
    }
}

fn print(d:&Vec<Vec<char>>)
{
    for l in d {
        for c in l {
            print!("{}",c);
        }
        println!("");
    }
}

fn solve(data:&Vec<&str>,n:i32) {
    let mut points = vec![];
    let mut velocity = vec![];

    for s in data {
        points.push(get_position(s.to_string()));
        velocity.push(get_velocity(s.to_string()));
    }

    let s = 64usize;

    for i in 0..n {        
        
        let mut min_x = 9999999;
        let mut min_y = 9999999;

        for p in &points {
            if p.0<min_x { min_x = p.0; }
            if p.1<min_y { min_y = p.1; }
        }

        let mut vis = 0;

        for p in &points
        {
            let x = p.0 - min_x;
            let y = p.1 - min_y;

            if x>=0 && y>=0 && x<s as i32 && y<s as i32 {   
                vis+=1;
            }
        }
       
        if vis==points.len() { 
            let mut field = vec![vec!['.';s];s];
            for p in &points
            {
                let x = p.0 - min_x;
                let y = p.1 - min_y;

                if x>=0 && y>=0 && x<s as i32 && y<s as i32 {   
                  field[y as usize][x as usize] = 'X';
                }
            }

            println!("t:{}",i);       
            print(&field); 
        }
        move_points(&mut points,&velocity);
    }
}

fn main() {

    let data = vec![
    "position=<-31138, -10302> velocity=< 3,  1>",
    "position=< 10703,  41994> velocity=<-1, -4>",
    "position=< 52503,  21082> velocity=<-5, -2>",
    "position=<-31135,  10618> velocity=< 3, -1>",
    "position=< 42051, -41680> velocity=<-4,  4>",
    "position=<-41605, -10302> velocity=< 4,  1>",
    "position=<-10212, -52139> velocity=< 1,  5>",
    "position=< 52534, -31215> velocity=<-5,  3>",
    "position=< 52528, -52131> velocity=<-5,  5>",
    "position=< 21125, -10298> velocity=<-2,  1>",
    "position=<-41584,  10614> velocity=< 4, -1>",
    "position=<-52096,  21082> velocity=< 5, -2>",
    "position=<-20711,  42000> velocity=< 2, -4>",
    "position=<-10248,  21073> velocity=< 1, -2>",
    "position=< 31619,  10619> velocity=<-3, -1>",
    "position=<-52080, -31222> velocity=< 5,  3>",
    "position=< 52494, -31214> velocity=<-5,  3>",
    "position=< 21133,  52459> velocity=<-2, -5>",
    "position=<-20695, -10304> velocity=< 2,  1>",
    "position=< 52554,  31540> velocity=<-5, -3>",
    "position=<-10208, -41674> velocity=< 1,  4>",
    "position=< 21157,  52457> velocity=<-2, -5>",
    "position=< 31628,  52456> velocity=<-3, -5>",
    "position=<-41605, -31214> velocity=< 4,  3>",
    "position=< 52523, -31220> velocity=<-5,  3>",
    "position=< 10714,  41991> velocity=<-1, -4>",
    "position=<-10248, -10300> velocity=< 1,  1>",
    "position=<-31170,  31540> velocity=< 3, -3>",
    "position=<-31166, -20759> velocity=< 3,  2>",
    "position=<-41621,  21074> velocity=< 4, -2>",
    "position=< 21125,  52451> velocity=<-2, -5>",
    "position=< 10663,  21075> velocity=<-1, -2>",
    "position=< 10658, -31220> velocity=<-1,  3>",
    "position=<-31158,  52457> velocity=< 3, -5>",
    "position=< 31616, -20759> velocity=<-3,  2>",
    "position=<-31177, -20761> velocity=< 3,  2>",
    "position=<-20703, -20763> velocity=< 2,  2>",
    "position=<-20711,  52458> velocity=< 2, -5>",
    "position=<-52091, -52131> velocity=< 5,  5>",
    "position=<-52052, -52134> velocity=< 5,  5>",
    "position=<-20714,  10617> velocity=< 2, -1>",
    "position=<-10235,  41995> velocity=< 1, -4>",
    "position=< 52518,  10616> velocity=<-5, -1>",
    "position=< 42084, -31216> velocity=<-4,  3>",
    "position=< 52527, -10295> velocity=<-5,  1>",
    "position=<-31120, -10301> velocity=< 3,  1>",
    "position=<-20719,  52450> velocity=< 2, -5>",
    "position=< 10661, -31216> velocity=<-1,  3>",
    "position=<-52040,  31540> velocity=< 5, -3>",
    "position=< 52550,  52457> velocity=<-5, -5>",
    "position=<-10210,  10619> velocity=< 1, -1>",
    "position=< 10675, -41675> velocity=<-1,  4>",
    "position=< 10714, -31220> velocity=<-1,  3>",
    "position=<-10217,  41991> velocity=< 1, -4>",
    "position=<-41579,  21079> velocity=< 4, -2>",
    "position=<-52084,  42000> velocity=< 5, -4>",
    "position=<-10236, -31219> velocity=< 1,  3>",
    "position=<-41587,  10618> velocity=< 4, -1>",
    "position=<-31143, -41672> velocity=< 3,  4>",
    "position=<-41637,  21080> velocity=< 4, -2>",
    "position=<-41597,  52453> velocity=< 4, -5>",
    "position=<-41588,  41993> velocity=< 4, -4>",
    "position=<-10240,  31534> velocity=< 1, -3>",
    "position=< 10716, -20760> velocity=<-1,  2>",
    "position=< 52518,  31533> velocity=<-5, -3>",
    "position=< 21128,  10618> velocity=<-2, -1>",
    "position=<-20658,  21073> velocity=< 2, -2>",
    "position=<-41637, -52135> velocity=< 4,  5>",
    "position=<-20659,  52451> velocity=< 2, -5>",
    "position=<-10257,  21079> velocity=< 1, -2>",
    "position=<-31176,  21076> velocity=< 3, -2>",
    "position=<-20695, -20760> velocity=< 2,  2>",
    "position=< 31616, -52140> velocity=<-3,  5>",
    "position=<-10251,  42000> velocity=< 1, -4>",
    "position=<-20674,  31533> velocity=< 2, -3>",
    "position=<-10252,  31536> velocity=< 1, -3>",
    "position=<-10215, -52137> velocity=< 1,  5>",
    "position=<-20679, -31220> velocity=< 2,  3>",
    "position=< 21141, -41678> velocity=<-2,  4>",
    "position=< 42080,  31533> velocity=<-4, -3>",
    "position=<-31146, -52140> velocity=< 3,  5>",
    "position=<-31169, -41677> velocity=< 3,  4>",
    "position=<-52096, -31216> velocity=< 5,  3>",
    "position=<-41592,  52453> velocity=< 4, -5>",
    "position=< 10686,  10614> velocity=<-1, -1>",
    "position=< 31625,  41998> velocity=<-3, -4>",
    "position=< 10658, -10296> velocity=<-1,  1>",
    "position=<-52056,  21073> velocity=< 5, -2>",
    "position=<-41618,  52455> velocity=< 4, -5>",
    "position=< 21117,  31532> velocity=<-2, -3>",
    "position=< 10667, -20759> velocity=<-1,  2>",
    "position=<-20717,  52453> velocity=< 2, -5>",
    "position=< 10658, -20762> velocity=<-1,  2>",
    "position=<-52064, -31219> velocity=< 5,  3>",
    "position=<-52047, -10297> velocity=< 5,  1>",
    "position=<-20663, -31213> velocity=< 2,  3>",
    "position=<-41586, -10299> velocity=< 4,  1>",
    "position=< 31632,  21079> velocity=<-3, -2>",
    "position=<-20715,  41998> velocity=< 2, -4>",
    "position=<-20714, -52139> velocity=< 2,  5>",
    "position=< 21137,  41994> velocity=<-2, -4>",
    "position=<-52038,  21076> velocity=< 5, -2>",
    "position=<-52051,  52458> velocity=< 5, -5>",
    "position=< 10690, -41680> velocity=<-1,  4>",
    "position=< 21149,  41998> velocity=<-2, -4>",
    "position=<-10257, -41676> velocity=< 1,  4>",
    "position=< 52550, -31218> velocity=<-5,  3>",
    "position=< 31616, -20754> velocity=<-3,  2>",
    "position=<-52056,  21078> velocity=< 5, -2>",
    "position=< 21166, -31215> velocity=<-2,  3>",
    "position=<-31135,  10614> velocity=< 3, -1>",
    "position=<-52088,  52456> velocity=< 5, -5>",
    "position=<-10216, -41677> velocity=< 1,  4>",
    "position=< 21117,  31535> velocity=<-2, -3>",
    "position=< 10658,  41993> velocity=<-1, -4>",
    "position=<-20714,  52459> velocity=< 2, -5>",
    "position=< 42091,  21074> velocity=<-4, -2>",
    "position=< 31600, -52133> velocity=<-3,  5>",
    "position=< 21119,  10618> velocity=<-2, -1>",
    "position=< 21170,  31532> velocity=<-2, -3>",
    "position=< 42048, -10295> velocity=<-4,  1>",
    "position=< 52535,  41991> velocity=<-5, -4>",
    "position=< 42067, -52138> velocity=<-4,  5>",
    "position=< 52530,  31541> velocity=<-5, -3>",
    "position=< 10714, -41676> velocity=<-1,  4>",
    "position=< 42087, -52138> velocity=<-4,  5>",
    "position=< 21117,  52458> velocity=<-2, -5>",
    "position=<-41576,  31532> velocity=< 4, -3>",
    "position=< 42052,  21075> velocity=<-4, -2>",
    "position=< 10663,  52455> velocity=<-1, -5>",
    "position=< 42091, -20754> velocity=<-4,  2>",
    "position=<-20700, -10299> velocity=< 2,  1>",
    "position=< 52538, -20757> velocity=<-5,  2>",
    "position=<-52088, -41674> velocity=< 5,  4>",
    "position=< 31608, -31217> velocity=<-3,  3>",
    "position=< 10711, -41672> velocity=<-1,  4>",
    "position=< 52502,  52452> velocity=<-5, -5>",
    "position=< 10663,  21077> velocity=<-1, -2>",
    "position=<-41629, -10298> velocity=< 4,  1>",
    "position=<-20687, -10296> velocity=< 2,  1>",
    "position=<-41581, -10299> velocity=< 4,  1>",
    "position=< 10693, -10295> velocity=<-1,  1>",
    "position=< 42075,  42000> velocity=<-4, -4>",
    "position=< 42051,  41999> velocity=<-4, -4>",
    "position=< 52518,  21082> velocity=<-5, -2>",
    "position=< 21122, -10303> velocity=<-2,  1>",
    "position=< 42068,  31541> velocity=<-4, -3>",
    "position=<-20676, -10299> velocity=< 2,  1>",
    "position=<-20666, -10295> velocity=< 2,  1>",
    "position=< 31608,  41996> velocity=<-3, -4>",
    "position=< 10709,  52455> velocity=<-1, -5>",
    "position=<-31129,  52452> velocity=< 3, -5>",
    "position=<-20698, -52131> velocity=< 2,  5>",
    "position=< 31608,  52453> velocity=<-3, -5>",
    "position=<-31168, -31213> velocity=< 3,  3>",
    "position=< 52523, -52138> velocity=<-5,  5>",
    "position=< 10663,  41999> velocity=<-1, -4>",
    "position=< 52515, -31221> velocity=<-5,  3>",
    "position=< 10695, -31213> velocity=<-1,  3>",
    "position=<-10243, -10297> velocity=< 1,  1>",
    "position=< 42067, -10300> velocity=<-4,  1>",
    "position=<-20668,  21077> velocity=< 2, -2>",
    "position=< 42051,  41999> velocity=<-4, -4>",
    "position=< 42063, -10300> velocity=<-4,  1>",
    "position=<-10217, -20758> velocity=< 1,  2>",
    "position=<-10252, -41678> velocity=< 1,  4>",
    "position=<-41600,  42000> velocity=< 4, -4>",
    "position=<-20679,  41994> velocity=< 2, -4>",
    "position=<-52051, -20754> velocity=< 5,  2>",
    "position=< 42048, -31213> velocity=<-4,  3>",
    "position=< 10682, -20759> velocity=<-1,  2>",
    "position=< 10678,  21075> velocity=<-1, -2>",
    "position=< 42060, -20759> velocity=<-4,  2>",
    "position=<-52068,  31536> velocity=< 5, -3>",
    "position=<-31174,  31540> velocity=< 3, -3>",
    "position=<-10248,  52459> velocity=< 1, -5>",
    "position=<-10236, -20756> velocity=< 1,  2>",
    "position=< 52503, -52131> velocity=<-5,  5>",
    "position=<-31161,  41998> velocity=< 3, -4>",
    "position=< 31624, -20763> velocity=<-3,  2>",
    "position=< 31596, -41674> velocity=<-3,  4>",
    "position=<-10228,  31538> velocity=< 1, -3>",
    "position=< 10682, -52139> velocity=<-1,  5>",
    "position=< 10658, -20763> velocity=<-1,  2>",
    "position=<-52091, -20758> velocity=< 5,  2>",
    "position=<-41605, -10303> velocity=< 4,  1>",
    "position=<-20668, -52135> velocity=< 2,  5>",
    "position=<-41610,  31536> velocity=< 4, -3>",
    "position=<-52060, -52131> velocity=< 5,  5>",
    "position=< 42084,  52456> velocity=<-4, -5>",
    "position=< 10671, -20763> velocity=<-1,  2>",
    "position=< 21174,  52455> velocity=<-2, -5>",
    "position=<-20695, -41676> velocity=< 2,  4>",
    "position=< 52499,  52454> velocity=<-5, -5>",
    "position=<-31170,  10614> velocity=< 3, -1>",
    "position=< 10658,  31539> velocity=<-1, -3>",
    "position=< 52543, -41678> velocity=<-5,  4>",
    "position=< 31621, -31220> velocity=<-3,  3>",
    "position=< 10690,  31536> velocity=<-1, -3>",
    "position=< 10658,  52452> velocity=<-1, -5>",
    "position=< 10679,  31533> velocity=<-1, -3>",
    "position=< 42083, -41673> velocity=<-4,  4>",
    "position=<-10252,  31532> velocity=< 1, -3>",
    "position=< 52528,  42000> velocity=<-5, -4>",
    "position=<-10231, -10303> velocity=< 1,  1>",
    "position=<-41588,  31535> velocity=< 4, -3>",
    "position=<-10234, -41677> velocity=< 1,  4>",
    "position=< 31632, -52138> velocity=<-3,  5>",
    "position=<-31158, -41675> velocity=< 3,  4>",
    "position=< 31629, -10296> velocity=<-3,  1>",
    "position=<-20695,  10618> velocity=< 2, -1>",
    "position=<-10250,  10614> velocity=< 1, -1>",
    "position=<-10204, -10296> velocity=< 1,  1>",
    "position=<-31169,  52450> velocity=< 3, -5>",
    "position=< 31601, -52136> velocity=<-3,  5>",
    "position=< 42053, -31218> velocity=<-4,  3>",
    "position=< 42087, -41679> velocity=<-4,  4>",
    "position=<-20718, -10303> velocity=< 2,  1>",
    "position=< 42045, -31218> velocity=<-4,  3>",
    "position=<-10243, -41674> velocity=< 1,  4>",
    "position=<-31141, -20754> velocity=< 3,  2>",
    "position=<-41629,  21074> velocity=< 4, -2>",
    "position=<-10217, -31218> velocity=< 1,  3>",
    "position=<-31159, -41677> velocity=< 3,  4>",
    "position=< 21173,  42000> velocity=<-2, -4>",
    "position=<-52056, -41681> velocity=< 5,  4>",
    "position=< 21161,  21080> velocity=<-2, -2>",
    "position=< 42079, -20756> velocity=<-4,  2>",
    "position=< 10699,  31532> velocity=<-1, -3>",
    "position=<-52064, -10300> velocity=< 5,  1>",
    "position=< 52503, -10300> velocity=<-5,  1>",
    "position=< 21138,  10615> velocity=<-2, -1>",
    "position=< 52499,  31540> velocity=<-5, -3>",
    "position=< 42059, -41677> velocity=<-4,  4>",
    "position=<-10250, -20754> velocity=< 1,  2>",
    "position=<-10204,  21075> velocity=< 1, -2>",
    "position=< 52499, -20758> velocity=<-5,  2>",
    "position=< 52510,  52451> velocity=<-5, -5>",
    "position=< 42061, -31222> velocity=<-4,  3>",
    "position=<-41629, -41676> velocity=< 4,  4>",
    "position=<-52078,  52455> velocity=< 5, -5>",
    "position=<-10260,  21077> velocity=< 1, -2>",
    "position=<-20698,  52459> velocity=< 2, -5>",
    "position=<-52052,  21073> velocity=< 5, -2>",
    "position=<-20711, -41681> velocity=< 2,  4>",
    "position=< 42075, -20760> velocity=<-4,  2>",
    "position=<-10228, -41672> velocity=< 1,  4>",
    "position=< 10700,  21077> velocity=<-1, -2>",
    "position=< 21146,  41992> velocity=<-2, -4>",
    "position=<-41593, -41674> velocity=< 4,  4>",
    "position=<-52069,  31532> velocity=< 5, -3>",
    "position=<-41580, -31217> velocity=< 4,  3>",
    "position=<-10252, -41677> velocity=< 1,  4>",
    "position=< 21138,  21081> velocity=<-2, -2>",
    "position=< 10682,  41997> velocity=<-1, -4>",
    "position=< 10659,  52452> velocity=<-1, -5>",
    "position=< 42076,  21077> velocity=<-4, -2>",
    "position=<-20671, -10303> velocity=< 2,  1>",
    "position=< 21157, -10299> velocity=<-2,  1>",
    "position=< 52514, -20761> velocity=<-5,  2>",
    "position=<-10207,  31532> velocity=< 1, -3>",
    "position=<-10260, -41676> velocity=< 1,  4>",
    "position=<-52043, -52139> velocity=< 5,  5>",
    "position=< 21178, -41672> velocity=<-2,  4>",
    "position=< 21149,  52452> velocity=<-2, -5>",
    "position=< 10706, -31222> velocity=<-1,  3>",
    "position=<-31134, -52136> velocity=< 3,  5>",
    "position=< 10659, -52139> velocity=<-1,  5>",
    "position=<-10232, -31218> velocity=< 1,  3>",
    "position=<-20663,  10615> velocity=< 2, -1>",
    "position=< 21145,  52450> velocity=<-2, -5>",
    "position=< 42091,  41991> velocity=<-4, -4>",
    "position=< 21136,  10619> velocity=<-2, -1>",
    "position=< 10683, -10304> velocity=<-1,  1>",
    "position=< 42040, -52131> velocity=<-4,  5>",
    "position=< 42056, -10304> velocity=<-4,  1>",
    "position=< 31634,  52456> velocity=<-3, -5>",
    "position=< 42067, -31222> velocity=<-4,  3>",
    "position=< 10699, -41681> velocity=<-1,  4>",
    "position=<-31154, -20754> velocity=< 3,  2>",
    "position=<-10248,  31536> velocity=< 1, -3>",
    "position=< 52521, -31222> velocity=<-5,  3>",
    "position=<-41617,  31539> velocity=< 4, -3>",
    "position=<-20679,  31534> velocity=< 2, -3>",
    "position=< 52542,  21082> velocity=<-5, -2>",
    "position=< 42046,  41991> velocity=<-4, -4>",
    "position=<-20719,  31541> velocity=< 2, -3>",
    "position=<-41581,  41997> velocity=< 4, -4>",
    "position=<-10260, -52137> velocity=< 1,  5>",
    "position=< 52518,  10622> velocity=<-5, -1>",
    "position=< 21170, -52131> velocity=<-2,  5>",
    "position=< 42040, -20763> velocity=<-4,  2>",
    "position=<-10215, -41673> velocity=< 1,  4>",
    "position=<-31135,  10618> velocity=< 3, -1>",
    "position=<-20703, -52132> velocity=< 2,  5>",
    "position=<-31126,  41994> velocity=< 3, -4>",
    "position=<-20659,  52458> velocity=< 2, -5>",
    "position=< 52550,  41995> velocity=<-5, -4>",
    "position=<-31162,  21073> velocity=< 3, -2>",
    "position=< 21134,  21079> velocity=<-2, -2>",
    "position=<-41637,  31538> velocity=< 4, -3>",
    "position=< 10701, -52140> velocity=<-1,  5>",
    "position=< 42080, -20761> velocity=<-4,  2>",
    "position=< 31576,  10621> velocity=<-3, -1>",
    "position=<-10243, -41675> velocity=< 1,  4>",
    "position=<-10203,  10618> velocity=< 1, -1>",
    "position=<-20718, -41680> velocity=< 2,  4>",
    "position=< 42051,  52459> velocity=<-4, -5>",
    "position=< 10707, -52137> velocity=<-1,  5>",
    "position=< 42067,  10620> velocity=<-4, -1>",
    "position=< 52534, -31214> velocity=<-5,  3>",
    "position=< 10667, -20763> velocity=<-1,  2>",
    "position=<-41629,  31536> velocity=< 4, -3>",
    "position=<-31154, -20758> velocity=< 3,  2>",
    "position=< 31608, -52140> velocity=<-3,  5>",
    "position=<-20692,  21073> velocity=< 2, -2>",
    "position=< 21167,  52455> velocity=<-2, -5>",
    "position=<-31154, -10297> velocity=< 3,  1>",
    "position=< 52546,  31539> velocity=<-5, -3>",
    "position=< 42056,  10614> velocity=<-4, -1>",
    "position=<-31121, -31217> velocity=< 3,  3>",
    "position=< 52505,  10618> velocity=<-5, -1>",
    "position=<-41581,  10617> velocity=< 4, -1>",
    "position=< 42075,  31538> velocity=<-4, -3>",
    "position=< 10669,  31532> velocity=<-1, -3>",
    "position=<-52044,  31534> velocity=< 5, -3>",
    "position=< 42052,  31534> velocity=<-4, -3>",
    "position=< 10682,  41999> velocity=<-1, -4>",
    "position=<-52052,  31532> velocity=< 5, -3>",
    "position=< 10661, -20757> velocity=<-1,  2>",
    "position=<-41597,  52451> velocity=< 4, -5>",
    "position=<-41578, -20761> velocity=< 4,  2>",
    "position=< 42039,  52457> velocity=<-4, -5>",
    "position=< 52511,  21076> velocity=<-5, -2>",
    "position=< 21165, -20763> velocity=<-2,  2>",
    "position=<-10260, -31216> velocity=< 1,  3>",
    "position=<-10249,  10623> velocity=< 1, -1>",
    "position=<-10255, -41675> velocity=< 1,  4>",
    "position=< 42068,  10623> velocity=<-4, -1>",
    "position=<-41608, -31219> velocity=< 4,  3>",
    "position=<-10255,  10621> velocity=< 1, -1>",
    "position=<-10239, -20763> velocity=< 1,  2>",
    "position=<-31170,  31540> velocity=< 3, -3>",
    "position=<-20710, -20763> velocity=< 2,  2>",
    "position=< 52526,  21076> velocity=<-5, -2>",
    "position=< 52550,  41994> velocity=<-5, -4>",
    "position=<-20700,  31536> velocity=< 2, -3>",
    "position=< 31616, -52136> velocity=<-3,  5>",
    "position=<-10199,  10623> velocity=< 1, -1>",
    "position=< 52522,  10614> velocity=<-5, -1>",
    "position=<-52091, -10296> velocity=< 5,  1>",
    "position=<-10258,  31536> velocity=< 1, -3>",
    "position=<-20670, -31220> velocity=< 2,  3>",
    "position=<-10200, -52132> velocity=< 1,  5>",
    "position=<-52054,  41991> velocity=< 5, -4>",
    "position=<-41589, -31221> velocity=< 4,  3>",
    "position=<-41633,  31540> velocity=< 4, -3>",
    "position=<-31126,  10617> velocity=< 3, -1>",
    "position=< 10669,  10618> velocity=<-1, -1>",
    "position=<-41578, -20756> velocity=< 4,  2>",
    "position=< 21157,  21079> velocity=<-2, -2>",
    "position=< 10690, -10303> velocity=<-1,  1>",
    "position=< 42045,  10623> velocity=<-4, -1>",
    ];
    solve(&data,20000);

    //test0();
    //println!("{}",res);
}

//#[test]
fn test0(){


    let data = vec![
        "position=< 9,  1> velocity=< 0,  2>",
        "position=< 7,  0> velocity=<-1,  0>",
        "position=< 3, -2> velocity=<-1,  1>",
        "position=< 6, 10> velocity=<-2, -1>",
        "position=< 2, -4> velocity=< 2,  2>",
        "position=<-6, 10> velocity=< 2, -2>",
        "position=< 1,  8> velocity=< 1, -1>",
        "position=< 1,  7> velocity=< 1,  0>",
        "position=<-3, 11> velocity=< 1, -2>",
        "position=< 7,  6> velocity=<-1, -1>",
        "position=<-2,  3> velocity=< 1,  0>",
        "position=<-4,  3> velocity=< 2,  0>",
        "position=<10, -3> velocity=<-1,  1>",
        "position=< 5, 11> velocity=< 1, -2>",
        "position=< 4,  7> velocity=< 0, -1>",
        "position=< 8, -2> velocity=< 0,  1>",
        "position=<15,  0> velocity=<-2,  0>",
        "position=< 1,  6> velocity=< 1,  0>",
        "position=< 8,  9> velocity=< 0, -1>",
        "position=< 3,  3> velocity=<-1,  1>",
        "position=< 0,  5> velocity=< 0, -1>",
        "position=<-2,  2> velocity=< 2,  0>",
        "position=< 5, -2> velocity=< 1,  2>",
        "position=< 1,  4> velocity=< 2,  1>",
        "position=<-2,  7> velocity=< 2, -2>",
        "position=< 3,  6> velocity=<-1, -1>",
        "position=< 5,  0> velocity=< 1,  0>",
        "position=<-6,  0> velocity=< 2,  0>",
        "position=< 5,  9> velocity=< 1, -2>",
        "position=<14,  7> velocity=<-2,  0>",
        "position=<-3,  6> velocity=< 2, -1>",
    ];

    let res = solve(&data,9);
    //assert_eq!(res,32);
}
