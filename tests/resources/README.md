# Testfiksturer for `pcli valider`

Denne mappa inneholder rutinefiler som brukes som testfiksturer for
valideringa (`pcli valider`). Testriggen ligger i
`src/kommandoer/rutinefil_fixtures.rs` og kjøres med `cargo test`.

Her hører både **ekte rutiner fra virkeligheten** (rutiner som gikk galt, eller
kjent gode) og **konstruerte eksempler** som demonstrerer én bestemt feiltype
hjemme. Ekte eksempler er spesielt verdifulle for å fange obskure feil, mens de
konstruerte dekker hver feilkategori systematisk.

## Mappestruktur

| Mappe | Betydning |
|-------|-----------|
| `rutinefiler_gyldige/` | Rutiner som skal validere **uten feil**. Verifiserer at vi ikke gir falske positive. |
| `rutinefiler_med_feil/` | Rutiner med **reelle problemer**. Hver fil har en sidecar som beskriver forventet resultat. |

Begge mappene kan være tomme – riggen bygges opp med ekte eksempler over tid.

## Sidecar-filer (`<navn>.forventet.json`)

Hver fil i `rutinefiler_med_feil/` skal ha en tilhørende forventningsfil med
samme navn pluss `.forventet.json`. Eksempel:


```
ugyldig_versjon.json            <- selve rutinefila
ugyldig_versjon.forventet.json  <- forventet resultat
```

Format:

```json
{
  "beskrivelse": "Kort forklaring på hva som er feil (til mennesker).",
  "forventede_kategorier": ["versjonsfeil"],
  "kjent_mangel": false
}
```

### Felter

- **`beskrivelse`** (valgfri): Fri tekst som forklarer feilen. Brukes ikke av
  testen, men gjør korpuset lesbart.
- **`forventede_kategorier`**: Liste med kategorinavn valideringa skal flagge.
  Gyldige verdier er feltnavnene i `ValidationResult`:
  - `manglende_variabler`
  - `udefinerte_variabler`
  - `ubrukte_variabler`
  - `ukjente_handlinger`
  - `ukjente_funksjoner`
  - `strukturfeil`
  - `metainfo_feil`
  - `versjonsfeil`
  - `listevariabel_feil`
  - `grunnlagsdatamappe_feil`
- **`kjent_mangel`** (standard `false`):
  - `false`: Valideringa **skal** fange feilen i dag. Testen krever at hver
    kategori i `forventede_kategorier` faktisk flagges.
  - `true`: Valideringa fanger **ikke** feilen ennå (semantisk feil vi ikke har
    en sjekk for). Testen krever at kategoriene fortsatt er tomme, og
    **feiler** den dagen en ny sjekk begynner å fange feilen – da er det på
    tide å sette `kjent_mangel` til `false`.

## Legge til en ny fikstur

Både ekte rutiner (gjerne anonymisert ved behov) og konstruerte eksempler er
velkomne.

### Gyldig fil
1. Legg en gyldig `.json`-rutine i `rutinefiler_gyldige/`.
2. Kjør `cargo test` – fila valideres automatisk.

### Fil med feil
1. Legg `.json`-rutina i `rutinefiler_med_feil/`.
2. Lag en `<navn>.forventet.json`-sidecar (se formatet over).
3. Kjør `cargo test`.

## Kjøre testene

```sh
cargo test
```

Riggen plukker opp nye filer automatisk, så det er ikke nødvendig å endre
testkoden når man legger til en fikstur.
