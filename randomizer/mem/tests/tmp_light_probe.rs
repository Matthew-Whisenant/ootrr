use std::collections::HashMap as HM;
use std::time::Instant;

use ootrr_core::flags;
use ootrr_core::json;
use ootrr_mem::reader::Reader;

fn names() -> HM<u32, (String, String)> {
    let t = std::fs::read_to_string("../../data/flags.json").unwrap();
    let l = flags::from_json(&json::parse_with(&t, &json::Limits { depth: 64, bytes: 64 << 20 }).unwrap()).unwrap();
    let mut out = HM::new();
    for f in &l.flags {
        if let Some(n) = &f.name {
            out.insert(f.hash as u32, (n.clone(), f.ty.clone()));
        }
    }
    for n in ["OOTRR.ColdPending", "OOTRR.AP.Pending"] {
        out.insert(flags::hash_name(n), (n.to_string(), "mod".to_string()));
    }
    for base in ["OOTRR_Item_Arrow_Light_Cold", "OOTRR_Item_Arrow_Light_Cold_Obj", "OOTRR_Item_Arrow_Fire_Cold", "OOTRR_Item_DinFire_Cold"] {
        for pre in ["PouchData.", "IsGet.", "HavePlayedGetEvent."] {
            let n = if pre == "PouchData." { format!("{pre}{base}.StockNum") } else { format!("{pre}{base}") };
            out.insert(flags::hash_name(&n), (n, "mod".to_string()));
        }
    }
    out
}

#[test]
fn tmp_light_probe() {
    let pat: Vec<String> = std::env::var("LP_PAT").unwrap_or_default().split(',').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
    let nm = names();
    let mut r = Reader::new();
    let s = r.poll(Instant::now()).unwrap();
    println!("pid {:?} rows {} keys {}", r.attached(), s.rows.len(), s.keys.len());
    let mut lines: Vec<String> = Vec::new();
    let mut unknown: usize = 0;
    for (h, v) in &s.rows {
        if *h == 0x6bee602d || *h == 0x578d41e1 { lines.push(format!("0x{h:08x} = {v}")); }
        match nm.get(h) {
            Some((n, ty)) => {
                if pat.iter().any(|p| n.contains(p.as_str())) {
                    lines.push(format!("{n} [{ty}] = {v}"));
                }
            }
            None => unknown += 1,
        }
    }
    lines.sort();
    for l in lines {
        println!("{l}");
    }
    println!("unnamed rows: {unknown}");
}
