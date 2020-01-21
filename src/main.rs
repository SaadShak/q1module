mod principal{
    pub mod headmaster{
        pub fn teacher(){
            println!("You are here to learn and study");
        }
    }
}

fn main() {
    crate::principal::headmaster::teacher(); // :: is delimiter 
}
