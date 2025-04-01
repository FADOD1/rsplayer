# RSPLAYER

Uma breve descrição sobre o objetivo e funcionalidades principais deste projeto Rust.

## Índice
- [Descrição Geral](#descrição-geral)
- [Tecnologias Utilizadas](#tecnologias-utilizadas)
- [Pré-requisitos](#pré-requisitos)
- [Configuração](#configuração)
- [Rodando o Projeto](#rodando-o-projeto)
- [Estrutura do Projeto](#estrutura-do-projeto)
- [Como Contribuir](#como-contribuir)
- [Licença](#licença)

## Descrição Geral

Este projeto é um aplicativo escrito em Rust , com funcionalidades avançadas de mídia e manipulação de arquivos. Utiliza diversas bibliotecas para garantir robustez e eficiência, incluindo integração multimídia com GStreamer e serialização/desserialização com Serde.

## Tecnologias Utilizadas

Este projeto utiliza principalmente:

- **Rust 1.84.1**
- [Serde](https://serde.rs/) (1.0.219)
- [TOML](https://github.com/toml-rs/toml) (0.8.20)
- [Dialoguer](https://github.com/mitsuhiko/dialoguer) (0.11.0)
- [GStreamer](https://gstreamer.freedesktop.org/) (0.23.5)
- [Percent-Encoding](https://github.com/servo/rust-url) (2.3.1)
- [Cairo-rs](https://github.com/gtk-rs/cairo) (0.20.7)
- [Shellexpand](https://github.com/netvl/shellexpand) (3.1.0)
- [Glib](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/) (0.20.9)
- [GTK4](https://gtk-rs.org/) (0.9.6)

## Pré-requisitos

- Rust e Cargo instalados. Você pode baixá-los em [rustup.rs](https://rustup.rs/).
- Bibliotecas e dependências adicionais necessárias pelo GTK4 e GStreamer. (Verifique a documentação oficial para instalação específica no seu sistema operacional).

Para Linux (Debian/Ubuntu/Pop!_OS):
```bash
sudo apt update && sudo apt install libgtk-4-dev gstreamer1.0-tools libgstreamer-plugins-base1.0-dev librust-cairo-dev
```

## Configuração

Clone este repositório e acesse seu diretório:
```bash
git clone <URL_DO_REPOSITÓRIO>
cd <NOME_DO_DIRETÓRIO_CLONADO>
```

Instale as dependências usando Cargo:
```bash
cargo build --release
```

## Rodando o Projeto

Após a configuração, execute o projeto com Cargo:
```bash
cargo run
```

## Como Contribuir

1. Faça um Fork do projeto.
2. Crie uma nova branch a partir da branch principal (`main`):
   ```bash
   git checkout -b minha-alteracao
   ```
3. Faça suas alterações e envie commits com mensagens claras.
4. Envie suas alterações para o seu repositório remoto:
   ```bash
   git push origin minha-alteracao
   ```
5. Abra um Pull Request a partir do seu fork na branch principal deste repositório explicando as alterações efetuadas.

## Licença

Este projeto está licenciado sob a licença MIT - veja o arquivo [LICENSE](LICENSE) para mais detalhes.
