/*
-This assignment is for assignment using functiions.
-The first function is for my name and matric number.
-The second function is for the two numbers.
-The third function is for my favourite emoji.
*/
fn my_details(name:&str, matno:&str){
println!("My name is {name}.\nMy matric number is {matno}"); /*The '/n' character is used to move the text ahead of it to the next line*/
}

fn division(x:f32, y:f32){
let z:f32= x/y;
println!("The answer when {x} divides {y} is {z}");
}

fn my_fav_emoji(emoji:&str){
    println!("My favourite emoji is {emoji}.\nSending y'all lots of hugs {emoji} {emoji}.");
}

fn main() {
    my_details("Ilo Chinaecherem Sylvia", "ENG2102775");
    division(70.0,20.0);
    my_fav_emoji("\u{1F917}"); /*this is a unicorn way of representation an emoji. They are represented in Hexadecimal*/
}
