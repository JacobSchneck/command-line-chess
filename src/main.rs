use ansi_term::Colour;

fn main() {
    println!("This is {} in color, {} in color and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green")
    );
    
    let s = Colour::Red.paint("red").to_string();
    println!("{}", s);
}
