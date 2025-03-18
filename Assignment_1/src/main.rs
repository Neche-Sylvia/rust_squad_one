/*
-This part is for the assignment without using any functions.
-The first block is for my name.
-The second block is for my matric number.
-The third is for the division of two numbers.
-The fourth is for my favorite emoji.
*/
fn main() {
    let my_name:&str= "ILO CHINAECHEREM SYLVIA";
    println!("My name is {}", my_name);
    
    let my_matno:&str="ENG2102775";
    println!("My matric number is {}", my_matno);

    let x:f32= 65.0;
    let y:f32=4.0;
    let z:f32= x/y;
    println!("The answer when {x} divides {y} is {z}");

    let my_fav_emoji:&str="\u{1F917}";
    println!("My favourite emoji is {}.\nSending y'all lots of hugs{}{}{}", my_fav_emoji, my_fav_emoji,my_fav_emoji,my_fav_emoji);
}
