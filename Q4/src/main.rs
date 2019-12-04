 #[derive(Debug)]
struct Children {
   name:String,
}

pub trait Primary_passed {
    fn Pp(&self) -> i32 {
        1
    }
}
pub trait Bilingual {
    fn bil(&self) -> i32 {
        1
    }
}

impl Primary_passed for Children {}
impl Bilingual for Children {}

fn main () {
    let my_children = Children {
        name :String::from("Orphan Children"),
    };
    let x = my_children.Pp();
    let y = my_children.bil();

 if x == 1 && y == 1 {
     println!("Mr Asim should Adopt these children");
 }
 else {println!("Mr Asim shouldnt Adopt these children");}
}
