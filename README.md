# Eco

![GitHub top language](https://img.shields.io/github/languages/top/kauefraga/eco)
![echo eco](https://img.shields.io/badge/echo-eco-8A2BE2)
![GitHub's license](https://img.shields.io/github/license/kauefraga/eco)
![GitHub last commit (branch)](https://img.shields.io/github/last-commit/kauefraga/eco/main)

> 🔊 Uma implementação em Rust do comando echo.

> [!IMPORTANT]
> Escolhi esse nome porque eu sempre li o comando `echo` como /ɛko/. Além disso, o comando `echo` pega a entrada e devolve a mesma coisa, como uma reverberação, um **eco**.

## 🎯 Objetivo e funcionalidades

O objetivo é que ao usar o programa `eco` seguido de argumentos (ex.: `eco "bom dia" boa tarde`), tenha a saída esperada: `bom dia boa tarde`. Ou seja, meu objetivo é tornar esse programa funcional e compatível com o `echo`.

Sobre as funcionalidades:

- [x] Compatível com o `echo`. Nesse caso, implementa grande parte das funcionalidades do programa/comando.
- [x] Veloz e portável.
- [x] Palavra "rust" é colorida em vermelho.

## ⬇ Como instalar e usar

Se você tem o acesso à ferramenta Cargo, então execute: `cargo install eco-rs`. Caso você não tenha o Cargo instalado, sinto muito, ainda não estou distribuindo os executáveis.

Veja abaixo um exemplo de uso:

```bash
eco-rs "Olá, mundo!" # Saída: Olá, mundo!
eco-rs Olá, mundo!   # Olá, mundo!
eco-rs "Olá," mundo! # Olá, mundo!
```

## ✨ O que melhorar e contribuições

Esta seção é dedicada para descrever o que pode ser melhorado atualmente no projeto como um todo. O que for feito, será removido da lista.

Fique à vontade para abrir um Pull Request adicionando melhorias na lista acima ou alterando o código-fonte diretamente. Outra opção é abrir uma issue solicitando melhorias ou correções de bugs.

## 🌳 Interações

- [Idealização do projeto](https://twitter.com/rkauefraga/status/1760814049065832789)
- [Sobre escrever uma thread](https://twitter.com/rkauefraga/status/1760826297175953810)
- [Distribuição no crates.io](https://twitter.com/rkauefraga/status/1761207277674012702)
- [Thread sobre o projeto](https://twitter.com/rkauefraga/status/1761391229986599342)
- [Verificação para colorir a palavra "rust"](https://twitter.com/rkauefraga/status/1761550454880633278)

## 📝 Licença

Este projeto está sob licença do MIT - Veja a [LICENÇA](https://github.com/kauefraga/eco/blob/main/LICENSE) para mais informações.

---

Feito com ❤ por Kauê Fraga Rodrigues.
