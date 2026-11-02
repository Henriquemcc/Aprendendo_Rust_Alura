fn main() {
    conteudo_opcional();
}

fn conteudo_opcional() {
    let conteudo_arquivo = ler_arquivo(String::from(""));
    match &conteudo_arquivo {
        Some(valor) => println!("{}", valor),
        None => println!("Arquivo não existe"),
    };

    println!("{:?}", conteudo_arquivo);

    if let Some(valor) = conteudo_arquivo {
        println!("Agora tenho certeza de que ha o valor: {}", valor);
    }
}

fn ler_arquivo(caminho_arquivo: String) -> Option<String> {
    Some(String::from("Conteúdo do arquivo"))
    //None
}
