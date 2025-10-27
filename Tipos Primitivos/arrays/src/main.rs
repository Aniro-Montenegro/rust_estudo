fn main() {
    println!("Arrays");
    print!("\n");
    print!("
Uma matriz de tamanho fixo, denotada [T; N],\n para o tipo de elemento, T, \ne o tamanho da constante de tempo de compilação não negativa, N.");
    let mut array: [i32; 3] = [1; 3];
    print!("\n");
    array[1] = 10;
    array[2] = 20;

    for x in array {
        print!("{x} ");
    }
}
