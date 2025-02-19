# codigo simples em rust
Explorarando um pouco a linguagem de programação Rust por curiosidade criei esse script chamado sacar.rs. 
O objetivo deste script é simular o funcionamento de um caixa eletrônico, calculando a quantidade de cédulas necessárias para um determinado valor de saque solicitado pelo usuário.


### biblioteca 
- std:io

## funções 



- notas

Esta função recebe um vetor de cédulas (cedulas) e o valor a ser sacado (valor_saque).
A função percorre cada denominação de cédula e calcula quantas de cada são necessárias.
Se o valor do saque for 8 ou 6, ele utiliza cédulas de R$ 2 para ajustar o valor exato.
Se o valor restante for 1 ou 3, uma mensagem é exibida informando que não é possível sacar o valor exato com as cédulas disponíveis.
O resultado é impresso no console.

(foi adicionado um tratamento para casos de 8 e 6 e numeros maiores que 13 para que as notas sejam exatas, 
no caso do 3 e do 1 a logica não foi adcionada pois não temos notas brasileiras e o loop percorre o vetor 
do maior para o menor )

- main
Solicita ao usuário que digite o valor a ser sacado.
Lê a entrada do usuário e tenta convertê-la para um número inteiro.
Chama a função notas com o valor solicitado pelo usuário.

### execução

rustc sacar.rs
./sacar
