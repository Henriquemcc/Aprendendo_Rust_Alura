fn sombra() {
    let a = 123;

    {
        let b = 456;
        println!("dentro, b = {}", b);

        let a = 777; // Sombra a variável a declarada no escopo externo
        println!("dentro, a = {}", a);
    }

    // println!("fora, b = {}", b); // Não compila, pois b está fora do escopo no qual foi declarado
    println!("fora, a = {}", a);
}

fn main() {
    sombra();
}
