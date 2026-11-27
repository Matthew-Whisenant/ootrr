use ootrr_core::flags;
use ootrr_mem::game::Game;

const GDM_BSS: u64 = 0x04e9_0ce8;
const GDM_VT: u64 = 0x04c5_8270;
const TABLES: [(u64, usize, u64); 4] = [(0x230, 0x28, 0x04c2_0718), (0x2f0, 0x28, 0x04c2_07d8), (0x3b0, 0x38, 0x04c2_0898), (0x8f0, 0x28, 0x04c2_0dd8)];

fn r32(b: &[u8], o: usize) -> u32 { u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]) }
fn r64(b: &[u8], o: usize) -> u64 { u64::from_le_bytes(b[o..o + 8].try_into().unwrap()) }

fn find(g: &Game, base: u64, hash: u32) -> Option<(usize, u64)> {
    for (ti, (off, stride, vt)) in TABLES.iter().enumerate() {
        let hd = g.read(base + off, 0x28)?;
        let cnt = r32(&hd, 8);
        let ents = r64(&hd, 0x10);
        let ht = r64(&hd, 0x18);
        let cap = r32(&hd, 0x24);
        let hb = g.read(ht, cap as usize * 8)?;
        for i in 0..cap as usize {
            if r32(&hb, i * 8) == hash {
                let idx = r32(&hb, i * 8 + 4);
                if idx >= cnt { continue }
                let at = ents + idx as u64 * *stride as u64;
                if g.u64_at(at) == Some(g.main + vt) { return Some((ti, at + 0x1c)); }
            }
        }
    }
    None
}

#[test]
fn tmp_lt_write() {
    let Ok(spec) = std::env::var("LW_SET") else { return };
    let g = Game::attach().unwrap();
    let p = g.u64_at(g.main + GDM_BSS).unwrap();
    assert_eq!(g.u64_at(p), Some(g.main + GDM_VT));
    for one in spec.split(',') {
        let (name, val) = one.split_once('=').unwrap();
        let h = flags::hash_name(name);
        let (t, at) = find(&g, p, h).expect("not found");
        let before = r32(&g.read(at, 4).unwrap(), 0);
        if val != "?" {
            let v: u32 = val.parse().unwrap();
            assert!(g.write(at, &v.to_le_bytes()));
        }
        let after = r32(&g.read(at, 4).unwrap(), 0);
        println!("{name} table{t} {before} -> {after}");
    }
}
