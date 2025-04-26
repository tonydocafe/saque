# MVC ATM in Rust

A simulation program for ATM withdrawals that clearly separates responsibilities using the MVC (Model-View-Controller) pattern.

## Features

- Calculation of banknotes for bank withdrawals
- Support for R$ 200, 100, 50, 20, 10, 5 and 2 banknotes
- Handling of special cases (values ​​ending in 1, 3, 6 and 8)
- Informational messages for residual values
- Command line interface (CLI)

## Prerequisites

- Rust 1.50 or higher

## Installation and Executio
```bash
git clone https://github.com/seu-usuario/caixa-eletronico-mvc.git
rustc main.rs -o caixa
./caixa
