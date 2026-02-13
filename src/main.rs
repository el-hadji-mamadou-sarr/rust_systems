fn main(){
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &mut s;

    // le code ne compile pas parce qu'on viole la régle:
    // sur les 2 références immuables et mutables,
    // on devrait avoir plusieurs références immuables oubien une seule référence mutable
    println!("{}", r1);
}