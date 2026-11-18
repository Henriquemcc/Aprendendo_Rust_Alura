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

    fn sacar(&mut self, valor: f64) {
        self.saldo -= valor;
    }
}

fn conta_corrente() {
    let mut conta: Conta = Conta {
        titular: Titular {
            nome: String::from("João"),
            sobrenome: String::from("Silva"),
        },
        saldo: 100.0,
    };

    conta.sacar(50.0);

    conta.imprimir_dados();
}
