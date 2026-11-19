# pandacli

## Beskrivelse
Pandacli er et kommandolinjeverktøy utviklet for Statens Pensjonskasse. Applikasjonen er laget for å forenkle og automatisere oppgaver knyttet til datahåndtering,
batch-prosessering og oppsett av testmiljøer. Med pandacli kan brukere enkelt utføre rutineoppgaver, hente ut data, og sette opp nødvendige strukturer for testing og utvikling.
Verktøyet er tilpasset interne behov og bidrar til økt effektivitet og kvalitet i arbeidsprosessene.

## Kom i gang
For å komme i gang med pandacli, følg disse trinnene:
1. **Installer pandacli**: Følg instruksjonene under "Installasjon" for å laste ned og installere verktøyet på din maskin.
2. **Kjør pandacli**: Etter installasjon kan du starte verktøyet ved å bruke kommandoen `pcli --help` for å se tilgjengelige funksjoner og kommandoer.

### Forutsetninger
- En maskin med Linux eller macOS
- En terminal med tilgang til internett
- `curl` installert for nedlasting av verktøyet

## Installasjon 
Last ned den nyeste versjonen av `pandacli` og plasser den i `/usr/local/bin/`:

### Linux
```sh
sudo curl -L https://github.com/statens-pensjonskasse/pandacli/releases/latest/download/pcli-linux-x86_64 -o /usr/local/bin/pcli
sudo chmod +x /usr/local/bin/pcli
```

#### macOS
```sh
sudo curl -L https://github.com/statens-pensjonskasse/pandacli/releases/latest/download/pcli-macos-aarch64 -o /usr/local/bin/pcli
sudo chmod +x /usr/local/bin/pcli
```

## Bruk
Etter installasjonen kan du bruke pandacli ved å skrive `pcli` i terminalen. For å se en liste over tilgjengelige kommandoer og alternativer, kjør:
```sh
pcli --help
pcli <kommando> --help
```

### Kommandoer

| Kommando | Beskrivelse | Eksempel |
|----------|-------------|----------|
| `valider` | Validerer rutinefiler mot det `panda-orkestrering` støtter. Se [Validering av rutinefiler](#validering-av-rutinefiler). | `pcli valider *.json` |
| `variabler` | Lister variablene som er definert i en rutinefil. | `pcli variabler fil.json` |
| `summer` | Summerer en kolonne i én eller flere CSV-filer (`.csv` eller `.csv.gz`). Se [Summering av CSV-filer](#summering-av-csv-filer). | `pcli summer 2 *.csv.gz` |
| `header` | Viser headerne i en CSV-fil med kolonneindeks. Nyttig for å finne riktig kolonne til `summer`. | `pcli header fil.csv` |
| `operasjoner` | Viser hvor langt en kjøring har kommet, ut fra `operasjoner.log`. Med `-s` skrives kun `OK` eller `FEILET`. | `pcli operasjoner operasjoner.log -s` |
| `filpartisjon` | Viser hvilken filpartisjon (1–271) et fødselsnummer eller en ID havner i. Bruker samme murmur-hash som tidsserie-lib. | `pcli filpartisjon 19060101002` |
| `rnd` | Velger en tilfeldig verdi fra listen. | `pcli rnd a b c` |

### Exit-koder
Alle kommandoer returnerer exit-kode som kan brukes i skript og CI:

| Kode | Betydning |
|------|-----------|
| `0` | Kommandoen lyktes. |
| `1` | Kommandoen feilet, f.eks. valideringsfeil, en fil som ikke kunne leses, eller en CSV-fil som ikke kunne summeres. |
| `2` | Ugyldige argumenter (f.eks. kolonnenummer som ikke er et tall). |

```sh
# Stopp hvis noen av rutinefilene har feil
pcli valider rutiner/*.json || exit 1

# Sjekk om en kjøring har feilet
if ! pcli operasjoner operasjoner.log --status; then
  echo "Kjøringen har feilet"
fi
```

## Konfigurasjon
pandacli trenger ingen konfigurasjon. Følgende miljøvariabel kan brukes ved feilsøking:

| Variabel | Beskrivelse |
|----------|-------------|
| `PCLI_MEM_LOG=1` | Logger minnebruk (RSS) til stderr under `summer`. Virker kun på Linux. |

## Validering av rutinefiler
`pcli valider <fil.json>` gjør avansert validering av rutinefiler mot en
versjonert katalog over hva `panda-orkestrering` støtter (kjente handlinger,
`#{...}`-funksjoner, struktur, versjonsformat m.m.). Se
[`docs/rutinefil_katalog.md`](docs/rutinefil_katalog.md) for hva som valideres
og hvordan katalogen oppdateres når det kommer nye instruksjoner.

Resultatet deles i feil og advarsler:

- **❌ Feil** – fila vil ikke fungere. Gir exit-kode 1.
- **❓ Uutfylte variabler** og **⚠️ ubrukte variabler** – advarsler. Gir *ikke*
  exit-kode 1, siden maler naturlig har uutfylte `<plassholdere>`.

Når flere filer valideres, sjekkes alle filene før kommandoen avslutter, og
exit-koden er 1 hvis minst én av dem har feil.

Sjekken av `metainfo.mal` (at filnavnet matcher malen) er **av som standard**,
fordi filer ofte blir omdøpt før de kjøres. Slå den på med `--streng-metainfo`:

```sh
pcli valider --streng-metainfo mal.json
```

## Summering av CSV-filer
`pcli summer <kolonne> <filer...>` summerer én kolonne i semikolonseparerte
CSV-filer. Filene leses parallelt og strømmes linje for linje, så store
datamengder går fint.

- **Kolonnenummer er 0-indeksert.** Bruk `pcli header <fil>` for å se
  kolonnene med indeks.
- **Header oppdages automatisk:** hvis verdien i første linje ikke er et tall,
  regnes linja som header.
- **Tomme felt** regnes som 0.
- **Ulike headere:** hvis filene har ulik header for kolonnen, listes hvilke
  filer som har hvilken header, slik at du kan oppdage at du summerer feil
  kolonne i noen av filene.
- **Feil i en fil** (ugyldig tall, manglende kolonne, ukjent filtype) rapporteres
  med filnavn og linjenummer. Fila er da ikke med i summen, og exit-koden blir 1.

```sh
$ pcli summer 1 a.csv bad.csv
❌ Kunne ikke parse verdi 'x' som et tall i 'bad.csv' linje 2.
Summerte 1 filer med header 'beløp', sum: 15.50
❌ 1 av 2 filer kunne ikke summeres og er ikke med i summen.
```

## Utvikling

### Forutsetninger
- [Rust](https://rustup.rs/) (stable). CI bruker siste stable-versjon, så kjør
  `rustup update` jevnlig for å få de samme clippy-sjekkene lokalt.

### Bygg og test
```sh
cargo build                 # bygger target/debug/pcli
cargo run -- valider fil.json
cargo test
```

### Før du lager PR
CI (`.github/workflows/rust.yml`) kjører følgende, og alle må passere:

```sh
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo build --locked
cargo test --locked
```

Kjør `cargo fmt --all` for å formatere koden. Clippy-advarsler behandles som
feil i CI.

### Tester
- Enhetstester ligger i `#[cfg(test)]`-moduler i samme fil som koden.
- Valideringa testes i tillegg mot ekte og konstruerte rutinefiler i
  [`tests/resources/`](tests/resources/README.md). Nye fiksturer plukkes opp
  automatisk.

### Release
Versjonering og release skjer automatisk ved merge til `main`, basert på
[Conventional Commits](https://www.conventionalcommits.org/) siden forrige tag:

| Commit-melding | Versjonsbump |
|----------------|--------------|
| `feat!: ...` eller `BREAKING CHANGE` i meldingen | major |
| `feat: ...` | minor |
| Alt annet (`fix:`, `chore:`, ...) | patch |

`release.yml` lager en `chore(release): vX.Y.Z`-commit på `main`, og
`publish.yml` bygger binærer for Linux (x86_64) og macOS (aarch64) og legger
dem ut som GitHub Release.

## Lisens
[MIT](LICENSE).

## Kontakt / Support
- Vedlikeholdes av team premieleveranse (`@statens-pensjonskasse/team-premieleveranse`).
- Feil og ønsker meldes som [issue på GitHub](https://github.com/statens-pensjonskasse/pandacli/issues).
