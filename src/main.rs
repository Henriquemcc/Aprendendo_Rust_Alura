fn soma(a: i32, b: i32) -> i32 {
    println!("{} + {} = {}", a, b, a + b);
    a + b
}

fn ownership() {
    let mut uma_string = String::from("Olá, mundo!");
    rouba(&mut uma_string);

    println!("{}", uma_string);
}

fn rouba(string: &mut String) {
    // string = String::from("Oi"); // Não compila, pois string é uma referência imutável
    string.push_str(" Como vai?");
    println!("{}", string);
}

fn pattern_matching() {
    for x in 1..=20 {
        println!(
            "{}: {}",
            x,
            match x {
                1 => "Pouco",
                2 | 3 => "Um pouquinho",
                4..=10 => "Um bocado",
                _ if x % 2 == 0 => "Uma boa quantidade",
                _ => "Muito",
            }
        );
    }
}

fn erros() {
    // Exemplo de erro ao acessar um índice inexistente em um vetor
    // let v = vec![1, 2, 3];
    // v[4]; // Panics, pois o índice 4 não existe no vetor

    // Exemplo de erro proposital usando a macro panic!
    // panic!("Erro proposital"); // Panics, pois a função panic!() é chamada

    // Exemplo de erro recuperável usando Result
    match resultado() {
        Ok(mensagem) => println!("String de sucesso: {}", mensagem),
        Err(codigo) => println!("Erro com código: {}", codigo),
    };
}

fn resultado() -> Result<String, u8> {
    //Ok(String::from("Tudo deu certo"))
    Err(42)
}

fn main() {
    println!("Soma = {}", soma(2, 2));

    ownership();

    pattern_matching();
    erros();
}
