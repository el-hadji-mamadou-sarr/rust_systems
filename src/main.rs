fn is_even(n: i32)-> bool{
   n%2 == 0
}

fn main() {
   for n in 1..11{
      if is_even(n) {
         println!("Pair");
      }else{
         println!("Impair");
      }
   }
}