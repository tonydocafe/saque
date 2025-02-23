# Caixa Eletrônico MVC em Rust

Um programa de simulação de saque de caixa eletrônico que separa claramente as responsabilidades usando o padrão MVC (Model-View-Controller).

## Funcionalidades

- Cálculo de cédulas para saques bancários
- Suporte para cédulas de R$ 200, 100, 50, 20, 10, 5 e 2
- Tratamento de casos especiais (valores terminados em 1, 3, 6 e 8)
- Mensagens informativas para valores residuais
- Interface de linha de comando (CLI)

## Pré-requisitos

- Rust 1.50 ou superior


## Instalação e Execução

```bash
git clone https://github.com/seu-usuario/caixa-eletronico-mvc.git
rustc main.rs -o caixa
./caixa
