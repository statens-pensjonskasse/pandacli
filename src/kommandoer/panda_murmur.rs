// Port av Java pandaMurmur for plassering av brukere i partisjoner.
// https://github.com/statens-pensjonskasse/panda-verifikator/blob/main/bin/utils/splitt-medlemsdata#L111
//
// Etterlikner java på følgende punkter:
// - Intern matematikk bruker u64 med innpakningsaritmetikk for å etterligne java 64-bit two's-complement overflow.
// - Den returnerer i64 som bevarer Javas signed long bit pattern
// - "Math.abs"-virkemåten fra Java emuleres

pub fn panda_murmur(id: &str) -> Result<i64, String> {
    let antall_partisjoner = 271;
    let bytes = id.as_bytes();
    let hash = hash64(bytes, bytes.len(), DEFAULT_SEED);

    let abs_hash = java_abs_i64(hash);

    let index = abs_hash % antall_partisjoner;

    Ok(1 + index)
}

const M_I64: i64 = -4132994306676758123i64;
const M_U64: u64 = M_I64 as u64;
const DEFAULT_SEED: i32 = -512093083;


fn java_abs_i64(x: i64) -> i64 {
    if x == i64::MIN { i64::MIN } else { x.abs() }
}

pub fn hash64(data: &[u8], length: usize, seed: i32) -> i64 {
    let len_u64 = length as u64;
    let seed_u64 = (seed as i64 as u64) & 0xffff_ffff_u64;
    let mut h: u64 = seed_u64 ^ len_u64.wrapping_mul(M_U64);

    let length8 = length / 8;
    for i in 0..length8 {
        let i8 = i * 8;
        let k = (data[i8 + 0] as u64)
            | ((data[i8 + 1] as u64) << 8)
            | ((data[i8 + 2] as u64) << 16)
            | ((data[i8 + 3] as u64) << 24)
            | ((data[i8 + 4] as u64) << 32)
            | ((data[i8 + 5] as u64) << 40)
            | ((data[i8 + 6] as u64) << 48)
            | ((data[i8 + 7] as u64) << 56);

        let mut k = k.wrapping_mul(M_U64);
        k ^= k >> 47;
        k = k.wrapping_mul(M_U64);
        h ^= k;
        h = h.wrapping_mul(M_U64);
    }

    // håndterer tail bytes (length % 8) med Java's fall-through switch oppførsel
    let rem = length & 7;
    let base = length & !7; // (length & -8) i Java
    match rem {
        7 => {
            h ^= (data[base + 6] as u64) << 48;
            // fall through
            h ^= (data[base + 5] as u64) << 40;
            h ^= (data[base + 4] as u64) << 32;
            h ^= (data[base + 3] as u64) << 24;
            h ^= (data[base + 2] as u64) << 16;
            h ^= (data[base + 1] as u64) << 8;
            h ^= data[base + 0] as u64;
            h = h.wrapping_mul(M_U64);
        }
        6 => {
            h ^= (data[base + 5] as u64) << 40;
            h ^= (data[base + 4] as u64) << 32;
            h ^= (data[base + 3] as u64) << 24;
            h ^= (data[base + 2] as u64) << 16;
            h ^= (data[base + 1] as u64) << 8;
            h ^= data[base + 0] as u64;
            h = h.wrapping_mul(M_U64);
        }
        5 => {
            h ^= (data[base + 4] as u64) << 32;
            h ^= (data[base + 3] as u64) << 24;
            h ^= (data[base + 2] as u64) << 16;
            h ^= (data[base + 1] as u64) << 8;
            h ^= data[base + 0] as u64;
            h = h.wrapping_mul(M_U64);
        }
        4 => {
            h ^= (data[base + 3] as u64) << 24;
            h ^= (data[base + 2] as u64) << 16;
            h ^= (data[base + 1] as u64) << 8;
            h ^= data[base + 0] as u64;
            h = h.wrapping_mul(M_U64);
        }
        3 => {
            h ^= (data[base + 2] as u64) << 16;
            h ^= (data[base + 1] as u64) << 8;
            h ^= data[base + 0] as u64;
            h = h.wrapping_mul(M_U64);
        }
        2 => {
            h ^= (data[base + 1] as u64) << 8;
            h ^= data[base + 0] as u64;
            h = h.wrapping_mul(M_U64);
        }
        1 => {
            h ^= data[base + 0] as u64;
            h = h.wrapping_mul(M_U64);
        }
        0 => {
        }
        _ => unreachable!(),
    }

    // (bruker unsigned shifts for å etterlikne Java's >>>)
    h ^= h >> 47;
    h = h.wrapping_mul(M_U64);
    h ^= h >> 47;

    // cast u64 -> i64 bevarer bit-pattern som Java long
    h as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_partitions() {
        let id_liste = ["19060101002", "19060122222", "19060144444", "204918726617","D9F47A69-E85E-42A0-9686-151835F38ACB", "FDF65490-3376-403E-A8FA-08B1BF727788", "10449213-0612-4B84-843E-054673D4AB0D"];
        let resultat = [150, 109, 79, 35, 29, 123, 93];

        for(i, &p) in id_liste.iter().enumerate() {
            assert_eq!(
                panda_murmur(p).unwrap(),
                resultat[i],
                "Element[{}]: {}", i, p
            );
        };
    }
}
