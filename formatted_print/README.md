Impressão formatada
================

A impressão é gerenciada por uma série de macros definidas em `std::fmt`, algumas das quais são:

`format!`: escreve texto formatado em uma String.

`print!`: igual a format!, mas o texto é impresso no console (io::stdout).

`println!`: igual a print!, mas uma nova linha é adicionada ao final.

`eprint!`: igual a print!, mas o texto é impresso no erro padrão (io::stderr).

`eprintln!`: igual a eprint!, mas uma nova linha é adicionada ao final.

Todas processam o texto da mesma maneira. Como vantagem, o Rust verifica a correção da formatação durante o tempo de compilação.

O módulo `std::fmt` contém muitos traits que governam a exibição de texto. Abaixo estão as formas básicas de dois traits importantes:

`fmt::Debug`: Usa o marcador `{:?}`. Formata texto para fins de depuração.

`fmt::Display`: Usa o marcador `{}`. Formata texto de uma maneira mais elegante e amigável ao usuário.

Aqui, usamos `fmt::Display` porque a biblioteca padrão fornece implementações para esses tipos. Para imprimir texto de tipos personalizados, mais etapas são necessárias.

A implementação do trait `fmt::Display` implementa automaticamente o trait ToString, permitindo converter o tipo em uma String.

Na linha 43, o atributo #[allow(dead_code)] se aplica apenas ao módulo subsequente.

### Atividades

1. Corrija o problema no código acima (veja FIXME) para que ele seja executado sem erros.
2. Tente descomentar a linha que tenta formatar a estrutura Structure (veja TODO).
3. Adicione uma chamada à macro println! que imprime:
"Pi é aproximadamente 3.142"
controlando o número de casas decimais exibidas. Para fins deste exercício, use let pi = 3.141592 como estimativa para o valor de pi.

(Dica: você pode precisar verificar a documentação de std::fmt para configurar o número de casas decimais a serem exibidas.)