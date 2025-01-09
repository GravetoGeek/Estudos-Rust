fn main() {
    // Em geral, o `{}` será automaticamente substituído por qualquer
    // argumento. Esses serão convertidos em texto.
    println!("{} dias", 31);

    // Argumentos posicionais podem ser usados. Especificar um número inteiro
    // dentro de `{}` determina qual argumento adicional será substituído.
    // Os argumentos começam no índice 0 logo após a string de formato.
    println!("{0}, este é {1}. {1}, este é {0}", "Alice", "Bob");

    // Também é possível usar argumentos nomeados.
    println!(
        "{subject} {verb} {object}",
        object = "o cachorro preguiçoso",
        subject = "a rápida raposa marrom",
        verb = "pula sobre"
    );

    // Diferentes formatações podem ser usadas especificando o caractere
    // de formato após `:`.
    println!("Base 10:               {}", 69420); // 69420
    println!("Base 2 (binária):      {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    // Você pode alinhar à direita especificando uma largura. Isso imprimirá
    // "    1". (Quatro espaços em branco e um "1", para um total de 5 caracteres.)
    println!("{number:>5}", number = 1);

    // É possível preencher números com zeros extras,
    println!("{number:0>5}", number = 1); // 00001
                                          // e alinhar à esquerda invertendo o sinal. Isso imprimirá "10000".
    println!("{number:0<5}", number = 1); // 10000

    // Você pode usar argumentos nomeados no especificador de formato
    // adicionando um `$`.
    println!("{number:0>width$}", number = 1, width = 5);

    // Rust até verifica para garantir que o número correto de argumentos seja usado.
    println!("Meu nome é {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Adicione o argumento que está faltando: "James"

    // Apenas tipos que implementam fmt::Display podem ser formatados com `{}`.
    // Tipos definidos pelo usuário não implementam fmt::Display por padrão.

    #[allow(dead_code)] // desabilita o aviso de `dead_code` para módulos não usados
    struct Structure(i32);

    // Isso não irá compilar porque `Structure` não implementa fmt::Display.
    // println!("Esta struct `{}` não será impressa...", Structure(3));
    // TODO ^ Tente descomentar esta linha

    // Para Rust 1.58 e acima, você pode capturar diretamente o argumento
    // de uma variável circundante. Assim como o exemplo acima, isso imprimirá
    // "    1", 4 espaços em branco e um "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
