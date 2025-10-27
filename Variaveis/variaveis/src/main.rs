fn main() {
    let total: i32 = 30;
    let trabalhador: &str = "João Batista";
    println!("O trabalhador {} Trabalhou {} horas", trabalhador, total);
    let mut total_mutavel: i32 = 40;
    println!(
        "O trabalhador {} Trabalhou {} horas",
        trabalhador, total_mutavel
    );
    total_mutavel = 100;
    println!(
        "O trabalhador {} Trabalhou {} horas",
        trabalhador, total_mutavel
    );

    //shadowing

    let x = 42;
    let ptr = &x; // referência
    println!("Endereço de x: {:p}", ptr);

    let x: &'static str = "quarenta e dois";

    let ptr: &&'static str = &x;
    println!("Endereço de x: {:p}", ptr);

    //escopo

    {
        let variavel1: i32 = 10;

        {
            let variavel1: i32 = 20;
            println!("Valor da variavel no escopo menor: {}", variavel1);
        }
        println!("Valor da variavel no escopo maior: {}", variavel1);
    }
}
