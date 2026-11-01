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
}

fn ler_arquivo(caminho_arquivo: String) -> Option<String> {
    Some(String::from("Conteúdo do arquivo"))
    //None
}
