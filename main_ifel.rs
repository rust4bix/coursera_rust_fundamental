fn main() {
    let mut height = 198;
    height = height - 20;
    
    let result = if height < 180{
        "Tall"
    }else{
        "Short"
    }; 
    println!("Result: {} ", result); 
}
