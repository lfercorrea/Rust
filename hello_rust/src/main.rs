use std::io;

fn main() {
    let text = "Hello, World!! By nandinho";
    println!("{text}");

    let other = String::from("nandinho");
    println!("{:=^40} ", other);

    let mut input = String::new();
    println!("Digite um texto: ");
    io::stdin().read_line(&mut input).expect("Erro lendo texto");

    println!("Você escreveu: {input}")
}
