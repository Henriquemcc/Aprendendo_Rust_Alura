fn condicionais() {
    let idade: u8 = 19;
    let responsavel_autorizou: bool = true;
    let eh_maior = idade >= 18;

    if eh_maior {
        println!("Pode entrar na balada");
    } else if idade >= 16 && responsavel_autorizou {
        println!("Pode entrar na balada com assinatura do responsável");
    } else {
        println!("Não pode entrar na balada");
    }

    let condicao = if eh_maior { "maior" } else { "menor" };

    println!("É {} de idade", condicao);

    let linguagem = "C#";
    let proposito = match linguagem {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Desconhecido",
    };

    println!("O proposito da linguagem {} é {}", linguagem, proposito);
}

fn main() {
    condicionais();
}
