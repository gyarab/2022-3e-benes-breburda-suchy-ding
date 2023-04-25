<h3 align="center">Ding - audiosociální síť</h3>

<p align="center"><b><a href="https://gyarab.github.io/2022-3e-benes-breburda-suchy-ding/main.pdf">Online PDF dokumentace</a></b></p>

[![Build documentation status](https://github.com/gyarab/2022-3e-benes-breburda-suchy-ding/actions/workflows/build-docs.yml/badge.svg)](https://github.com/gyarab/2022-3e-benes-breburda-suchy-ding/actions/workflows/build-docs.yml)

Zadání:
> Cílem by bylo vytvořit sociální síť, na kterou by se nedalo 
> přidat nic jiného než zvukové soubory, které by nesměly obsahovat
> mluvené slova (nebo by množství bylo omezené). Mluvená slova by 
> se automaticky filtrovala. Aplikace by byla webová stránka.
> 
> UI by bylo ve stylu Twitteru, ale upravené pro audio formát. Uživatel 
> by mohl nahrávat audio hned na webové stránce, nebo nahrát už vytvořený 
> soubor. Měl by také feed audia od dalších uživatelů.
>
> Později bychom mohli pomocí elektronu udělat android / IOS aplikaci. 
> Primárně by ale aplikace žila na webu.

Autoři: Rory Beneš, Kryštof Breburda a Adam Suchý

Repozitář je dělen na frontend (`/front`) a backend (`/back`).

## Instalace

1. Stáhněte si build z github release, nebo spusťte `deploy-prep.sh`
   (závislosti: git, cargo (rust), cross (rust), node, npm a standardní
   linuxové utility).

2. Archív nahrajte na server a rozbalte.

3. Nainstalujte `whisper.cpp`. Hlavní program `main` a používaný model musí být
   ve stejné složce (`whisper_cpp_root` v configu). Soubor s modelem se musí
   jmenovat `model.bin`.

4. Nastavte správně `config.toml` (doporučujeme upravit
   `back/config.toml.sample`). Server hledá config na cestě `config.toml`, nebo
   podle ENV proměnné `CONFIG_FILE`.

5. vytvořte složku `upload_dir`

6. (nastavte systemd službu a) spusťte server
