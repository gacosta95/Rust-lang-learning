use std::io;
use std::io::prelude::*;


fn main() {
	let mut nome = String::new();

	print!("Informe seu nome: ");
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut nome).expect("Entrada incorreta"); //Entrada dos caracteres informados no teclado

	if let Some('\n') = nome.chars().next_back() {
		nome.pop();
	//Some é uma função que serve para "pegar" algo ou alguma coisa que se relaciona há próxima iteração (Pega o proximo caractere após localizar o caractere indicado como código) 
	}
    if let Some('\r') = nome.chars().next_back() {
    	nome.pop();
    }
    println!("Olá, {}\n", nome);
    print!("Pressione <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}

