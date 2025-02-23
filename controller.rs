use std::io;
use crate::model;
use crate::view;

pub fn run() {
    view::display_prompt();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a linha");
    
    let valor_saque: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            view::display_error("Erro ao converter para número inteiro");
            std::process::exit(1);
        }
    };

    let result = model::calculate_bills(valor_saque);
    view::display_bills(&result.bills);
    view::display_message(&result.message);
}