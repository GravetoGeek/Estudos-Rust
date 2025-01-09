Tipos Primitivos
================

Rust provê uma série de tipos primitivos, que são implementados diretamente pela linguagem e não dependem de bibliotecas externas. Os tipos primitivos mais comuns são:

### Tipos Escalares

- [**Inteiros com sinal**](inteiros.md): `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize` (Tamanho do ponteiro)

- [**Inteiros sem sinal**](inteiros.md): `u8`, `u16`, `u32`, `u64`, `u128`, `usize` (Tamanho do ponteiro)

- [**Ponto flutuante**](ponto-flutuante.md): `f32`, `f64`

- [**Booleano**](booleano.md): `true`, `false`

- [**Caractere**](caractere.md): Valores escalares Unicode individuais (`char`) como `'a'`, `'α'`, `'∞'` (4 bytes cada)

- [**Tipo unitário**](unitario.md): `()`, cujo único valor possível é uma tupla vazia: `()`. Apesar do valor do tipo unitário ser uma tupla, ele não é considerado um tipo composto porque não contém múltiplos valores.

### Tipos Compostos

- [**Tuplas**](tuplas.md): Sequências de valores de tipos arbitrários, de tamanho fixo. Por exemplo, `(i32, f64, u8)`

- [**Arrays**](arrays.md): Sequências de valores de tipos arbitrários, de tamanho fixo e conhecido em tempo de compilação. Por exemplo, `[i32; 5]`


### Notas
-As variáveis pode sempre ser anotadas com tipos. Números podem ser anotados adicionalmente com um sufixo ou por padrão. Inteiros têm como padrão o tipo i32, e números de ponto flutuante têm como padrão o tipo f64. Vale notar que o Rust também pode inferir tipos a partir do contexto.

- Rust não possui um tipo primitivo para representar números complexos, mas bibliotecas externas como `num` podem ser usadas para esse propósito.