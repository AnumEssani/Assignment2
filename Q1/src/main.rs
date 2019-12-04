//Q1 Calclator
use std::io;
fn main() {
    println!("The Calculator is ready (Please input each term with spaces)");
    loop {
        let mut a = String ::new();
     io::stdin().read_line(& mut a);
     let mut vec = vec![];
     for i in a.split_whitespace() {
        
         vec.push(i);
     }
     let x :f32 = vec[0].parse().unwrap();
     let y :f32 = vec[2].parse().unwrap();
     if x == 0.0 { println!("Bye Now,Calculator ending");
         break;}
     
     
     if vec[1] == "+" { println!("> {}",x+y);}
     else if vec[1] == "-" { println!("> {}",x-y);}
     else  if vec[1] == "*" { println!("> {}",x*y);}
      else  if vec[1] == "/" { println!("> {}",x/y);}
      else if vec[1] == "^" {println!("> {}",x.powf(y as f32));}
      else { println!("> Incorrect Operator");}
     }
  
}
