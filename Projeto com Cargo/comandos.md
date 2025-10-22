# Trabalhando com Cargo

## Criando um Projeto com Cargo
```
cargo new hello_cargo
````

## Construindo e Executando um Projeto de Carga
```
cargo build
```

## Executar o codigo
```
./target/debug/hello_cargo
```

## Compilar o código e então executar o executável resultante, tudo em um comando
```
cargo run
```
## Este comando verifica rapidamente o seu código para garantir que ele seja compilado, mas não produza um executável
```
cargo check
```

##  Quando seu projeto estiver finalmente pronto para lançamento, você poderá usá-lo para compilá-lo com otimizações. 
```
cargo build --release
```

