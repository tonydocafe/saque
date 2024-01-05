use std::io;

fn notas(cedulas: &[i32], mut valor_saque: i32) -> i32 {
    for &cedula in cedulas.iter() {
        
        let quantidade_cedulas = valor_saque / cedula;

        if quantidade_cedulas > 0 {
            println!("Cédulas de R$ {}: {}", cedula, quantidade_cedulas);
            // atualizar o valor_saque subtraindo as cédulas já contabilizadas
            valor_saque %= cedula;
        }

        //  lógica especial para o caso de terminar em 8 ou 6
        if valor_saque  == 8 || valor_saque == 6 {
            //  cédulas de R$ 2 para atingir o número exato
            let cedulas_de_2 = valor_saque / 2;
            println!("Cédulas de R$ 2: {}", cedulas_de_2);
            break;
        }
    }
    // caso valor de saque for igual ou terminado em 1 ou 3 
    if valor_saque == 1{
        println!("Você deixou de receber um real por esse caixa só possuir notas!");
    }

    0 // return
}

fn main() {
    // solicitar ao usuário o valor a ser sacado
    println!("Digite o valor a ser sacado:");
    println!("( apenas notas! )");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a linha");

    let valor_saque: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Erro ao converter para número inteiro");
            std::process::exit(1);
        }
    };

    
    notas(&[200, 100, 50, 20, 10, 5, 2], valor_saque);
}
