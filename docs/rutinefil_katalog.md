# Rutinefil-katalog (avansert `valider`)

`pcli valider` gjør avansert validering av rutinefiler ved å sammenligne dem mot
en **versjonert katalog** over hva konsumentsystemet
[`panda-orkestrering`](https://github.com/statens-pensjonskasse/panda-orkestrering)
faktisk støtter. Katalogen ligger som konstanter i
[`src/kommandoer/rutinefil_katalog.rs`](../src/kommandoer/rutinefil_katalog.rs).

Dette dokumentet beskriver **hva katalogen inneholder** og **hvordan du
oppdaterer den** når `panda-orkestrering` får nye instruksjoner. Nye
instruksjoner dukker sjelden opp, så katalogen vedlikeholdes manuelt («approach
A») framfor å genereres automatisk.

> **Sist synket mot `panda-orkestrering`-versjon:** `0.1.8`
> (se `KATALOG_SYNKET_MOT_ORKESTRERING_VERSJON` i `rutinefil_katalog.rs`).

---

## Hva valideres

| # | Sjekk | Kilde i `panda-orkestrering` |
|---|-------|------------------------------|
| 1 | **Manglende variabler** – `${x}` brukt, men ikke definert i `variabler` | `VariabelflettingJson.flettVariabler` |
| 2 | **Uutfylte variabler** – verdi er fortsatt en `<placeholder>` | konvensjon i maler |
| 3 | **Ubrukte variabler** – definert i `variabler`, men aldri brukt | – |
| 4 | **Ukjent `handling`** – matcher ingen `Oversetter.navn()` eller spesial­handling | `DelegerOversetting`, `UtledHandlinger` |
| 5 | **Ukjent `#{...}`-funksjon** | `TekstTilFunksjonHjelper.eksekver` |
| 6 | **Strukturfeil** – manglende `operasjoner`/`handling`, `gruppe` uten `gruppeAv`, `forHver` uten `elementer`/`utfør_handlinger`, `deploy_batch` uten `batcher[].navn/versjon` | `BatchOrkestreringApp`, `UtledHandlinger`, `VariabelflettingJson`, `DeployBatchInstruksjonOversetter` |
| 7 | **Feil bruk av listevariabel** – array-variabel må brukes som egen streng `"${x}"` | `VariabelflettingJson.flettVariabler` (array-grenen matcher `"${x}"`) |
| 8 | **Ugyldig versjonsformat** på `gyldigVersjon` / `metainfo.støttetAvPaOrkBa01FraVersjon` | `Versjon` (regex `\d+(\.\d+)*(-SNAPSHOT)?`) |
| 9 | **Feil `metainfo.mal`** – filnavnet i `mal` må matche filas eget navn (fanger kopier-lim-feil) | konvensjon i maler |

---

## Katalogens innhold

### Spesielle handlinger (`SPESIELLE_HANDLINGER`)
Tolkes direkte av orkestreringa, ikke via en `Oversetter`:

- `gruppe` – grupperer underhandlinger (`gruppeAv`), pakkes ut av `UtledHandlinger`.
- `forHver` – løkke over `elementer` som kjører `utfør_handlinger`, ekspanderes av `VariabelflettingJson`.

`deploy_batch` er en vanlig `Oversetter`, men har egen strukturvalidering.

### Kjente handlinger (`KJENTE_HANDLINGER`)
Alle verdier som en `Oversetter` i `DelegerOversetting` svarer på via
`navn()`. Merk at noen navn kommer fra konstanter (`NAVN` / `BATCH_NAVN`) i
oversetterne, f.eks. `pa_res_ba_01`, `pa_res_ba_02`, `pa_res_ba_03`,
`pa_pre_ba_01`.

### Kjente funksjoner
`#{...}`-syntaks kjøres av `TekstTilFunksjonHjelper.eksekver`:

- **Eksakt match** (`KJENTE_FUNKSJONER_EKSAKT`): `grunnlagsdatamappeNå`,
  `genererUUID`, `gyldigTermininfo`.
- **Prefiks-match** (`KJENTE_FUNKSJONER_PREFIKS`, tar argumenter):
  `finnArkivertGrunnlagsdataMappeFraPaResBa02`,
  `finnLevendeGrunnlagsdataMappeFraPaResBa02`.

---

## Slik oppdaterer du katalogen

Når `panda-orkestrering` får en ny handling eller funksjon:

1. **Nye handlinger** – finn alle `Oversetter.navn()`-verdier:
   ```bash
   cd panda-orkestrering
   # Litterale navn
   grep -rh -A2 'public String navn()' \
     panda-orkestrering-batch/src/main/java/.../instruksjoner/oversetting \
     | grep 'return' | sed -E 's/.*return //; s/;//' | sort -u
   # Navn som kommer fra konstanter (NAVN / BATCH_NAVN)
   grep -rhE 'String (NAVN|BATCH_NAVN)\s*=' --include=*.java . | sort -u
   ```
   Legg nye verdier inn i `KJENTE_HANDLINGER`.

2. **Nye funksjoner** – se `TekstTilFunksjonHjelper.eksekver`. Bruk
   `KJENTE_FUNKSJONER_EKSAKT` for funksjoner uten argumenter og
   `KJENTE_FUNKSJONER_PREFIKS` for funksjoner som tar argumenter
   (`startsWith`-sjekk i Java).

3. **Versjonsformat** – hvis `Versjon`-regexen endres, oppdater
   `er_gyldig_versjonsformat`.

4. Oppdater `KATALOG_SYNKET_MOT_ORKESTRERING_VERSJON` og
   «Sist synket»-linja øverst i dette dokumentet.

5. Kjør `cargo test` og valider alle eksisterende maler for å sikre at ingen
   gyldige filer blir feilaktig flagget:
   ```bash
   cd panda-orkestrering-rutiner
   for f in $(find maler -name '*.json'); do pcli valider "$f"; done
   ```

---

## Kjent avvik oppdaget ved innføring

Flere maler bruker `#{sisteBatchVersjon(...)}`, men denne funksjonen finnes
**ikke** i `TekstTilFunksjonHjelper`. Slike kall vil kaste
`UnsupportedOperationException` ved kjøring. `valider` flagger dette som en
ukjent funksjon. Dette må enten implementeres i `panda-orkestrering` eller
fjernes fra malene – det rettes ikke av `pandacli`.
