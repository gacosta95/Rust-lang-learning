use std::io;
use std::io::prelude::*;
//O comando use é usado para fazer a chamada dos arquivos dos pacotes(Bibliotecas de auxilio da linguagem)


fn main() {
    println!("Programação na linguagem Rust.\n");
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap(); // Limpa o Buffer
    io::stdin().read(&mut [0u8]).unwrap();
//A função io (input/output) é usada para auxiliar as operações de entrada e saída
//pelo usuário na linha de comando, sendo uma das funçoes da bibliotéca-padrão std
    
}
