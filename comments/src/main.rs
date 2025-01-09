fn main() {
    // Este é um exemplo de um comentário de linha.
    // Há duas barras no início da linha.
    // E nada escrito após essas barras será lido pelo compilador.

    //println!("Olá mundo!");

    //Execute-o Viu? Agora tente remover as duas barras e execute novamente.

    /*
    * Este é outro tipo de comentário, um comentário de bloco. Em geral,
    * os comentário de linha são o estilo recomendado. Mas os comentários
    * de bloco são extremamente úteis para desativar temporariamente
    * partes do código.
    * /* Comentários de bloco podem ser /* aninhados, */*/ então é preciso
    * apenas algumas teclas para comentar tudo nesta função `main()`.
    * /*/*/* Experimente você mesmo! */*/*/
    */

    /*
    Nota: A coluna anterior de `*` foi inteiramente para estilo. não há
    necessidade real disso.
    */

    // Você pode manipular expressões mais facilmente com comentários de bloco
    // do que com comentários de linha. Tente remover os delimitadores
    // de comentário para alterar o resultado:
    let x = 5 + /* 90 + */ 5;
    println!("`x`é 10 ou 100? x = {}", x);




}
