use std::io;
const SEGUNDOS_MINUTOS: i32 = 60;

fn main() {
    println!("Digite um numero de horas :");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler linha");

    // converter String → i32
    let numero: i32 = input
        .trim()
        .parse()
        .expect("Por favor, digite um número válido!");

    println!(" Joao trabalhou {} segundos", numero * SEGUNDOS_MINUTOS);
}
