use std::env;
use rustDatabook;

fn main() {
    let send: Vec<String> = env::args().collect();
    
    let x = rustDatabook::rdbData(&send[1]);
    if send[2] == "-m"{
        print!("{}", x.M[rustDatabook::findValue(&send[3], &x, "_M")])
    }
    if send[2] == "-d"{
        print!("{}", x.D[rustDatabook::findValue(&send[3], &x, "_D")].to_string())
    }
    if send[2] == "-a"{
        for xss in 0..x.A[rustDatabook::findValue(&send[3], &x, "_D")].len(){
            println!("{}",x.A[rustDatabook::findValue(&send[3], &x, "_D")][xss])
        }
    }
    if send[2] == "-nd"{
        let mut x = send[3].replace("="," ");
        x = x.replace("/","|");
        //x = x.replace(">", "|");
        rustDatabook::addData(&send[1], &x);
    }
    if send[2] == "-cd"{
        let mut f = send[4].replace("="," ");
        f = f.replace("/","|");
        //x = x.replace(">", "|");
        rustDatabook::changeData(&send[1],&f,&send[3]);
       
    }
    if send[2] == "-rd"{
    
        //x = x.replace(">", "|");
        rustDatabook::removeData(&send[1],&send[3]);
        //print!("{}",send[3]);
    }
    if send[2] == "-cn"{
    
        //x = x.replace(">", "|");
        rustDatabook::copyValueToNew(&send[1],&send[3],&send[4], x, &send[5]);
        //print!("{}",send[3]);
    }
   

}   