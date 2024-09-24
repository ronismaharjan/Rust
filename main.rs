// This is the comment in rust
fn main(){
//Varaibles 
//Number
//signed integer , unsigned integer
// how to declare variable 
let var1:u8=23;
let var2:i8 = -34;

//How to print in rust 
println!("{} {}",var1, var2);

// To add variable in string in above we use {} to add the variables

//String and &str
//&str it is static data type means store in stack;
//How to declare
let my_str:&str="hello world";

//String it is dynamic datatype means we can push edit the value 
//How to declare 
let my_string:String=String::from("hello world");
print_helloworld();


};
//functions
//functions are defined using "fn" key word;
fn print_helloworld(){
  println!("hello world");
};
