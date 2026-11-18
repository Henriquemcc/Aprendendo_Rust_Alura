fn main() {
    conta_corrente();
}

struct Titular {
    nome: String,
    sobrenome: String,
}

struct Conta {
    titular: Titular,
    saldo: f64,
}

impl Conta {
    fn imprimir_dados(&self) {
        println!(
            "Dados da conta: Titular = {} {}, Saldo = {}",
            self.titular.nome, self.titular.sobrenome, self.saldo
        );
    }
}

fn conta_corrente() {
    let conta: Conta = Conta {
        titular: Titular {
            nome: String::from("João"),
            sobrenome: String::from("Silva"),
        },
        saldo: 100.0,
    };

    conta.imprimir_dados();
}
