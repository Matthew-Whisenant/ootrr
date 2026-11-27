use ootrr_core::flags;

#[test]
fn tmp_lt_saves() {
    let want = ["PlayerStatus.MaxLife", "PouchData.Weapon_Range_Bow.StockNum", "IsGet.Weapon_Range_Bow", "PouchData.Item_Arrow_Fire.StockNum", "PouchData.Item_Arrow_Ice.StockNum", "PouchData.Item_Arrow_Light.StockNum", "IsGet.Item_Arrow_Light", "HavePlayedGetEvent.Item_Arrow_Light", "PouchData.Item_Hammer.StockNum", "PlayerStatus.MPMax", "IsTreasureBoxOpen.14060099092170010696", "PouchData.Item_Quiver_Medium.StockNum", "PouchData.Item_Arrow_Normal.StockNum"];
    for root in ["C:/Users/Oscar Ruiz/AppData/Roaming/Longshot/save/5a3c9e1f0b7d24680000000000000001", "C:/Users/Oscar Ruiz/Games/Longshot/stable/v3.1.1/user/save-work/5a3c9e1f0b7d24680000000000000001"] {
        for i in 0..6 {
            let p = format!("{root}/slot_{i:02}/progress.sav");
            let Ok(b) = std::fs::read(&p) else { continue };
            let m = std::fs::metadata(&p).unwrap().modified().unwrap();
            let w = |o: usize| u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]);
            let mut at = 0x28usize;
            let mut map = std::collections::HashMap::new();
            while at + 8 <= b.len() {
                let (h, v) = (w(at), w(at + 4));
                at += 8;
                if h == 0xa3db_7114 { break }
                map.entry(h).or_insert(v);
            }
            let mut out = Vec::new();
            for n in want {
                out.push(format!("{}={}", n, map.get(&flags::hash_name(n)).map(|v| *v as i64).unwrap_or(-99)));
            }
            println!("{p} {:?} rows={}
   {}", m, map.len(), out.join("
   "));
        }
    }
}
