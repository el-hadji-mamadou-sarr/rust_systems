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

## Day 2

```rust
fn main() {
   let s1 = String::from("hello");
   let s2 = s1;

   // pour corriger, s2 est la nouvelle propriétaire de du string donc, on peux l'utiliser
   println!("{}", s2);
}
```