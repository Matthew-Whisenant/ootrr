use ootrr_core::byml::Node;
use ootrr_core::pickup_pack::rows::{id, POUCH_INFO};

fn show(v: &Node) -> String {
    match v {
        Node::Str(s) => s.rsplit('/').next().unwrap_or(s).to_string(),
        Node::Bool(b) => b.to_string(),
        Node::I32(i) => i.to_string(),
        o => format!("{o:?}"),
    }
}

#[test]
fn tmp_lt_rows() {
    let roots: Vec<String> = std::env::var("LR_ROOTS").unwrap().split('|').map(|s| s.to_string()).collect();
    for root in roots {
        let mut rf = ootrr_core::romfs::Romfs::open(std::path::Path::new("D:/Oot/romfs")).unwrap();
        rf.root = std::path::PathBuf::from(&root);
        let y = rf.read_byml(POUCH_INFO).unwrap();
        println!("=== {root}");
        for r in y.root.as_arr().unwrap() {
            let n = id(r).unwrap_or("");
            if !(n.contains("Arrow") || n.contains("Range_Bow") || n.contains("Quiver")) { continue }
            let Node::Map(m) = r else { continue };
            let cols: Vec<String> = m.iter().filter(|(k, _)| !["__RowId", "BuyingPrice", "SellingPrice", "LiftableGrade", "IsUsableSelectItemRiding", "IsAutoSaveAfterGet", "ShootableActor", "BundleActorNum"].contains(&k.as_str())).map(|(k, v)| format!("{k}={}", show(v))).collect();
            println!("{n}: {}", cols.join(" "));
        }
    }
}
