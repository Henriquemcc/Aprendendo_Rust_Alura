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
    erros();
}
