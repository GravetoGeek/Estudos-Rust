Comentários
===========

Qualquer programa requer comentários, e Rust suporta alguns tipos diferentes:

- Comentários regulares, que são ignorados pelo compilador:
```rust
// Comentários de linha que vão até o final da linha.
/* Comentários de bloco que vão até o delimitador de fechamento. */
```

- Comentários de documentação, que são analisados para gerar a documentação HTML da biblioteca:
```rust
/// Gera a documentação da biblioteca para o item seguinte.
//! Gera a documentação da biblioteca para o escopo atual (item pai).
```
