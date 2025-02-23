pub struct CalculationResult {
    pub bills: Vec<(i32, i32)>,
    pub message: Option<String>,
}

pub fn calculate_bills(valor_saque: i32) -> CalculationResult {
    let cedulas = [200, 100, 50, 20, 10, 5, 2];
    let mut valor_restante = valor_saque;
    let mut bills = Vec::new();
    let mut message = None;

    for &cedula in cedulas.iter() {
        let quantidade = valor_restante / cedula;
        if quantidade > 0 {
            bills.push((cedula, quantidade));
            valor_restante %= cedula;
        }

        if valor_restante == 8 || valor_restante == 6 {
            let qtd_2 = valor_restante / 2;
            bills.push((2, qtd_2));
            valor_restante %= 2;
            break;
        }

        let novo_valor = valor_restante - 3;
        if novo_valor % 10 == 0 {
            bills.push((5, 1));
            bills.push((2, 4));
            if valor_restante != 13 {
                valor_restante -= 3;
            }
        }
    }

    if valor_restante == 1 {
        message = Some("Você deixou de receber um real por esse caixa só possuir notas!".to_string());
    }

    CalculationResult { bills, message }
}