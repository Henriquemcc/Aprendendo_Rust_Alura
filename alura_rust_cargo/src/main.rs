use crate::DiaDaSemana::Domingo;

fn main() {
    // Criando um array de notas com 4 elementos do tipo f32
    // let notas: [f32; 4] = [10.0, 8.0, 9.5, 6.0];

    // Criando um array de notas com 4 elementos repetidos
    let notas: [f32; 4] = [6.5; 4];

    // Acessando o primeiro elemento do array usando um índice
    let inteiro: usize = 0;
    println!("{}", notas[inteiro]);

    // Acessando os elementos individualmente e imprimindo-os
    println!(
        "Nota1 = {}, Nota2 = {}, Nota3 = {}, Nota4 = {}",
        notas[0], notas[1], notas[2], notas[3]
    );

    // Percorrendo o array usando um loop for e imprimindo cada nota
    for nota in notas {
        println!("Nota = {}", nota);
    }

    // Percorrendo o array usando um loop for com índice e imprimindo cada nota
    for indice in 0..notas.len() {
        println!("A nota {} é {}", indice + 1, notas[indice]);
    }

    matriz();

    println!("É fim de semana? {}", eh_fim_de_semana(DiaDaSemana::Quarta));

    let dia: DiaDaSemana = DiaDaSemana::Sexta;
}

enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
}

fn eh_fim_de_semana(dia_da_semana: DiaDaSemana ) -> bool {
    match dia_da_semana {
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ => false
    }
}

fn matriz() {

    // Criando uma matriz com 2 linhas e 3 colunas
    let matriz: [[f32; 3]; 2] = [
        [0.0, 1.2, 0.1],
        [1.3, 0.3, 1.4],
    ];

    // Interando sobre a matriz usando dois loops for aninhados
    for linha in matriz {
        for item in linha {
            println!("Item = {}", item);
        }
    }
}