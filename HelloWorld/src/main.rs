fn main() {
   // Converting a string to a number
   let num = "25";
   let num: u32 = num.parse()
       .expect("Please provide a valid number!");
   
   println!("Parsed number: {}", num);
   
   // Further transformations
   let num = num + 25;
   let num = num * 2;
   
   println!("Final value of num: {}", num);
}
