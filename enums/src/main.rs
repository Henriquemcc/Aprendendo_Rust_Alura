#[allow(dead_code)]
enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
}

fn eh_fim_de_semana(dia_da_semana: DiaDaSemana) -> bool {
    match dia_da_semana {
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ => false,
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CymkColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

fn cores() {
    // let cor = Color::CymkColor { cyan: 100, magenta: 50, yellow: 70, black: 255 };
    let cor = Color::RgbColor(12, 5, 32);

    println!(
        "Cor = {}",
        match cor {
            Color::Red => "vermelho",
            Color::Green => "verde",
            Color::Blue => "azul",
            Color::RgbColor(0, 0, 0)
            | Color::CymkColor {
                cyan: _,
                magenta: _,
                yellow: _,
                black: 255,
            } => "preta",
            Color::RgbColor(_, green, _) => {
                println!("{}", green);
                "RGB Desconhecida"
            }
            Color::CymkColor {
                cyan: _,
                magenta: _,
                yellow: _,
                black: _,
            } => "CYMK desconhecida",
        }
    );
}

fn main() {
    println!("É fim de semana? {}", eh_fim_de_semana(DiaDaSemana::Quarta));
    cores();
}
