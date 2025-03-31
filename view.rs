pub fn display_prompt() {
    println!("Digite o valor a ser sacado:");
    println!("( apenas notas! )");
}


pub fn display_bills(bills: &[(i32, i32)]) {
    for (cedula, quantidade) in bills {
        println!("Cédulas de R$ {}: {}", cedula, quantidade);
    }
}

pub fn display_message(message: &Option<String>) {
    if let Some(msg) = message {
        println!("{}", msg);
    }
}

pub fn display_error(error: &str) {
    eprintln!("{}", error);
}
