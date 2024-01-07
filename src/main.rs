use rand::Rng;
use std::io;

fn main() -> io::Result<()>{
   let mut rng = rand::thread_rng();
   let ans = rng.gen_range(1..11);
   let mut inp = String::new();

   println!("Guess a number between 1 and 10: ");
   let mut p;
   loop{
    io::stdin().read_line(&mut inp)?;
    p = inp.trim_end().parse::<i32>().unwrap();
    if p != ans{
     println!("Nope. Choose another: ");
     inp.clear();
    }else {
        println!("Correct!");
        break;
    }
   }

   Ok(())
}
