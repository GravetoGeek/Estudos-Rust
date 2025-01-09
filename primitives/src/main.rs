fn main() {
    let logico: bool = true;
    println!("Valor lógico: {}",logico);

    let um_flutuante:f64 = 1.0; //Anotação regular de tipo
    let um_inteiro = 5i32; //Anotação de tipo sufixada

    println!("Um número em ponto flutuante: {}", um_flutuante);
    println!("Um número inteiro: {}", um_inteiro);

    // Ou uma padrão será usada
    let padrao_flutuante = 3.0; // `f64`
    let padrao_inteiro = 7; // `i32`

    println!("Um número em ponto flutuante: {}", padrao_flutuante);
    println!("Um número inteiro: {}", padrao_inteiro);

    // Um tipo pode ser inferido a partir de um contexto
    let mut tipo_inferido = 12; // O tipo i64 é inferido a partir do contexto
    tipo_inferido = 4294967296i64;

    println!("Um número inteiro inferido a partir do contexto: {}", tipo_inferido);

    // Um valor de variável mutável pode ser reatribuído.
    let mut mutavel = 12; // Mutável `i32`
    mutavel = 21;

    println!("Um número inteiro mutável: {}", mutavel);

    // Error! O tipo da variável não pode ser alterado.
    //mutavel = true;

    // Variáveis poder ser sobrescritas com sombras(Shadowing), o que pode ser útil para converter um valor de um tipo para outro.
    let mutavel = true;

    println!("Um valor booleano mutável: {}", mutavel);

    // Tipos compostos - Arrays e Tuplas

    // A assinatura de um array é [T; N], onde T é o tipo dos elementos e N é o número de elementos.
    // Arrays são alocados na pilha em vez de na heap.
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Um array: {:?}", array);

    // Arrays podem ser inicializados com o mesmo valor
    let mesmo_valor = [3; 5]; // Isso é equivalente a [3, 3, 3, 3, 3]

    println!("Um array com o mesmo valor: {:?}", mesmo_valor);

    // Acesso a elementos de um array

    println!("O primeiro elemento do array: {}", array[0]);
    println!("O segundo elemento do array: {}", array[1]);

    // O acesso a um índice fora dos limites do array resulta em um erro de compilação.
    //println!("O sexto elemento do array: {}", array[5]);

    // Tupla é uma coleção de valores de tipos diferentes e é definido por parênteses `()`.
    // Cada posição em uma tupla tem um tipo específico.
    let tupla: (i32, f64, u8) = (500, 6.4, 1);

    // Tuplas podem ser desestruturadas para vincular seus elementos a variáveis.
    let (x, y, z) = tupla;

    println!("O valor de y é: {}", y);

    // Os elementos de uma tupla também podem ser acessados usando índices.
    let quinhentos = tupla.0;
    let seis_ponto_quatro = tupla.1;
    let um = tupla.2;

    println!("O valor de seis_ponto_quatro é: {}", seis_ponto_quatro);



}
