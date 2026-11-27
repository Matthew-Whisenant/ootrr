use std::collections::HashMap as HM;
use std::time::Instant;
use ootrr_core::flags;
use ootrr_core::json;
use ootrr_mem::reader::Reader;

#[test]
fn tmp_lt_owned() {
    let t = std::fs::read_to_string("../../data/flags.json").unwrap();
    let l = flags::from_json(&json::parse_with(&t, &json::Limits { depth: 64, bytes: 64 << 20 }).unwrap()).unwrap();
    let mut nm: HM<u32, String> = HM::new();
    for f in &l.flags { if let Some(n) = &f.name { nm.insert(f.hash as u32, n.clone()); } }
    let mut r = Reader::new();
    let s = r.poll(Instant::now()).unwrap();
    let mut out: Vec<String> = Vec::new();
    for (h, v) in &s.rows {
        let Some(n) = nm.get(h) else { continue };
        if *v == 0 { continue }
        if (n.starts_with("PouchData.") && n.ends_with(".StockNum")) || n.starts_with("IsGet.") || n.starts_with("HavePlayedGetEvent.") || n.starts_with("UI_GetAssignable") || n.starts_with("PlayerStatus.Equip") || n.starts_with("PlayerStatus.AssignableItem") {
            out.push(format!("{n} = {}", *v as i32));
        }
    }
    out.sort();
    for o in out { println!("OWN {o}"); }
}
