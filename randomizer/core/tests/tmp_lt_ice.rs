use ootrr_core::byml::Node;
use ootrr_core::pickup_pack::rows::{find, ACTOR_INFO, POUCH_INFO};

fn cols(n: &Node) -> Vec<(String, String)> {
    let Node::Map(m) = n else { return Vec::new() };
    m.iter().filter(|(k, _)| k != "__RowId").map(|(k, v)| (k.clone(), match v { Node::Str(s) => s.rsplit('/').next().unwrap_or(s).to_string(), Node::Bool(b) => b.to_string(), Node::I32(i) => i.to_string(), o => format!("{o:?}") })).collect()
}

#[test]
fn tmp_lt_ice() {
    let mut rf = ootrr_core::romfs::Romfs::open(std::path::Path::new("D:/Oot/romfs")).unwrap();
    let y = rf.read_byml(POUCH_INFO).unwrap();
    let info = rf.read_byml(ACTOR_INFO).unwrap();
    let names = ["Pouch_Range_BowAndArrow_Normal", "Pouch_Range_BowAndArrow_Ice", "Pouch_Range_BowAndArrow_Fire", "Pouch_Arrow_Normal", "Weapon_Range_Bow"];
    let all: Vec<Vec<(String, String)>> = names.iter().map(|n| cols(find(&y, n).unwrap())).collect();
    let mut keys: Vec<String> = all.iter().flatten().map(|e| e.0.clone()).collect();
    keys.sort();
    keys.dedup();
    println!("COL | {}", names.join(" | "));
    for k in keys {
        let v: Vec<String> = all.iter().map(|c| c.iter().find(|e| e.0 == k).map(|e| e.1.clone()).unwrap_or("-".to_string())).collect();
        if v.iter().all(|x| x == &v[0]) { continue }
        println!("{k} | {}", v.join(" | "));
    }
    for n in names {
        let a = find(&info, n).unwrap();
        println!("ACTOR {n}: ActorName={:?} Fmdb={:?}", a.get("ActorName").and_then(|v| v.as_str()), a.get("FmdbName").and_then(|v| v.as_str()));
    }
}
