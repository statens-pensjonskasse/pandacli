# pandacli

## Beskrivelse
Pandacli er et kommandolinjeverktøy utviklet for Statens Pensjonskasse. Applikasjonen er laget for å forenkle og automatisere oppgaver knyttet til datahåndtering,
batch-prosessering og oppsett av testmiljøer. Med pandacli kan brukere enkelt utføre rutineoppgaver, hente ut data, og sette opp nødvendige strukturer for testing og utvikling.
Verktøyet er tilpasset interne behov og bidrar til økt effektivitet og kvalitet i arbeidsprosessene.

## Kom i gang
For å komme i gang med pandacli, følg disse trinnene:
1. **Installer pandacli**: Følg instruksjonene under "Installasjon" for å laste ned og installere verktøyet på din maskin.
2. **Kjør pandacli**: Etter installasjon kan du starte verktøyet ved å bruke kommandoen `pnd --help` for å se tilgjengelige funksjoner og kommandoer.

### Forutsetninger
- En maskin med Linux eller macOS
- En terminal med tilgang til internett
- `curl` installert for nedlasting av verktøyet

## Installasjon 
Last ned den nyeste versjonen av `pandacli` og plasser den i `/usr/local/bin/`:

### Linux
```sh
sudo curl -L https://github.com/statens-pensjonskasse/pandacli/releases/latest/download/pnd-linux-x86_64 -o /usr/local/bin/pnd
sudo chmod +x /usr/local/bin/pnd
```

#### macOS
```sh
sudo curl -L https://github.com/statens-pensjonskasse/pandacli/releases/latest/download/pnd-macos-aarch64 -o /usr/local/bin/pnd
sudo chmod +x /usr/local/bin/pnd
```

## Bruk
Etter installasjonen kan du bruke pandacli ved å skrive `pnd` i terminalen. For å se en liste over tilgjengelige kommandoer og alternativer, kjør:
```sh
pnd --help
```

## Konfigurasjon
Eventuelle miljøvariabler eller konfigurasjonsfiler som må settes opp.


## Lisens
[MIT](LICENSE).

## Kontakt / Support
- Kontaktperson eller team
- Hvordan rapportere feil eller forespørsler
