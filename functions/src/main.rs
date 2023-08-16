fn main() {
    hello();
    println!("I am {} years old", age());

    let mut no:i32 = 5;
   mutate_no_to_zero(&mut no);
   println!("The value of no is:{}",no);

   let mut n:i32 = 10;
   ref_mut(&mut n);
   println!("The value of n is:{}",n);
}

fn hello() {
    println!("Hello, world!");
}

fn age() -> usize {
    32
}

fn mutate_no_to_zero(param_no:&mut i32){
    *param_no = 0; //de reference
 }

 fn ref_mut(num:&mut i32) {
    *num = 12;
 }