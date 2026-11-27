use ootrr_core::byml::Node;
use ootrr_core::pickup_pack::rows::{id, find, ACTOR_INFO, POUCH_INFO};

#[test]
fn tmp_lt_order() {
    let roots: Vec<String> = std::env::var("LR_ROOTS").unwrap().split('|').map(|s| s.to_string()).collect();
    for root in roots {
        let mut rf = ootrr_core::romfs::Romfs::open(std::path::Path::new("D:/Oot/romfs")).unwrap();
        rf.root = std::path::PathBuf::from(&root);
        let y = rf.read_byml(POUCH_INFO).unwrap();
        let info = rf.read_byml(ACTOR_INFO).unwrap();
        println!("=== {root}");
        let rows = y.root.as_arr().unwrap();
        let mut by: std::collections::BTreeMap<String, Vec<String>> = Default::default();
        for (i, r) in rows.iter().enumerate() {
            let n = id(r).unwrap_or("");
            let Some(Node::Str(slot)) = r.get("AssignableItem") else { continue };
            let ident = find(&info, n).and_then(|a| a.get("ActorName")).and_then(|v| v.as_str()).unwrap_or("?").to_string();
            let der = matches!(r.get("IsEquipDerivationActor"), Some(Node::Bool(true)));
            by.entry(slot.clone()).or_default().push(format!("#{i} {n} (name={ident}{})", if der { ", derived" } else { "" }));
        }
        for (slot, v) in by {
            let idents: std::collections::BTreeSet<&str> = v.iter().filter(|x| !x.contains("derived")).filter_map(|x| x.split("name=").nth(1)).map(|x| x.trim_end_matches(')')).collect();
            if idents.len() > 1 || slot.contains("Arrow") { println!("{slot}: {}", v.join(" | ")); }
        }
    }
}
