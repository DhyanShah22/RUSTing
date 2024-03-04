fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "ALice", "Bob");

    println!("{subject} {verb} {object}",
             object="The Lazy Dog",
             subject="The quick brown fox",
             verb="jumps over");
    println!("Base 10:               {}",   69420);
    println!("Base 2 (binary):       {:b}", 69240);
    println!("Base 8 (octal):        {:o}", 69420);  
    println!("Base 16 (hexadecimal): {:x}", 69420); 
    println!("Base 16 (hexadecimal): {:X}", 69420); 
}