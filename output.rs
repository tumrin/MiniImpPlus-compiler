fn main(){
println!("please insert a number");
let mut ANSWER = String::new();
std::io::stdin().read_line(&mut ANSWER).unwrap();
println!("{ANSWER}");
}
