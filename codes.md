## Day 1

```rust
fn main() {
   let mut age = 20;
   println!("{}", age);
   
   age = 25;
   
   println!("{}", age);
   
   let age = age + 5;
   age = 5
   println!("{}", age);
}
```

```rust
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
```