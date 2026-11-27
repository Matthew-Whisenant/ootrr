#[test]
fn tmp_lt_banc() {
    let Some(m) = std::env::var_os("LT_MOD") else { return };
    let mut rf = ootrr_core::romfs::Romfs::open(std::path::Path::new("D:/Oot/romfs")).unwrap();
    rf.root = std::path::PathBuf::from(m);
    for f in rf.list("Banc").unwrap() {
        if !f.ends_with(".byml.zs") { continue }
        let Ok(y) = rf.read_byml(&f) else { continue };
        let Some(list) = y.root.get("Actors").and_then(|a| a.as_arr()) else { continue };
        for a in list {
            let d = a.get("Dynamic");
            let s = format!("{:?}", d);
            if ootrr_core::placement::hash_of(a) == Some(14060099092170010696) || s.contains("Arrow_Light") {
                println!("{f} {:?} {:?} {}", ootrr_core::placement::hash_of(a), a.get("Gyaml"), s);
            }
        }
    }
}

#[test]
fn tmp_lt_fake() {
    let Some(m) = std::env::var_os("LT_MOD") else { return };
    let mut rf = ootrr_core::romfs::Romfs::open(std::path::Path::new("D:/Oot/romfs")).unwrap();
    rf.root = std::path::PathBuf::from(m);
    for n in ["OOTRR_Item_Arrow_Light_Cold", "OOTRR_Item_Arrow_Light_Cold_Obj"] {
        let raw = rf.read_zs(&format!("Pack/Actor/{n}.pack.zs")).unwrap();
        for e in ootrr_core::sarc::parse(&raw).unwrap() {
            if e.name.contains(n) || e.name.contains("ActorName") {
                let b = ootrr_core::byml::Byml::parse(&e.data).map(|b| format!("{:?}", b.root)).unwrap_or_default();
                println!("{n} :: {} :: {}", e.name, b);
            }
        }
    }
    for t in [ootrr_core::pickup_pack::rows::ACTOR_INFO, ootrr_core::pickup_pack::rows::POUCH_INFO] {
        let y = rf.read_byml(t).unwrap();
        for n in ["OOTRR_Item_Arrow_Light_Cold", "OOTRR_Item_Arrow_Light_Cold_Obj", "Item_Arrow_Light"] {
            println!("ROW {t} {n} {:?}", ootrr_core::pickup_pack::rows::find(&y, n));
        }
    }
}

#[test]
fn tmp_lt_find() {
    let Some(m) = std::env::var_os("LT_MOD") else { return };
    let mut rf = ootrr_core::romfs::Romfs::open(std::path::Path::new("D:/Oot/romfs")).unwrap();
    rf.root = std::path::PathBuf::from(m);
    let pat: &[u8] = b"Arrow_Light_Cold";
    let mut all = Vec::new(); for d in ["Banc","Event","Pack","RSDB","GameData","Logic","Sequence","System","Mals","Component"] { if let Ok(v) = rf.list(d) { all.extend(v); } }
    for f in all {
        if f.starts_with("Pack/Actor/OOTRR_Item_Arrow_Light_Cold") { continue }
        let raw = if f.ends_with(".zs") { match rf.read_zs(&f) { Ok(b) => b, Err(_) => continue } } else { rf.read(&f).unwrap() };
        let mut hits: Vec<String> = Vec::new();
        if f.ends_with(".pack.zs") || f.ends_with(".sarc.zs") || f.ends_with(".bfarc.zs") {
            if let Ok(es) = ootrr_core::sarc::parse(&raw) {
                for e in es {
                    if e.data.windows(pat.len()).any(|w| w == pat) { hits.push(e.name.clone()); }
                }
            }
        }
        if hits.is_empty() && raw.windows(pat.len()).any(|w| w == pat) { hits.push("(file)".to_string()); }
        if !hits.is_empty() { println!("HIT {f} {:?}", hits); }
    }
}

#[test]
fn tmp_lt_grave() {
    let Some(m) = std::env::var_os("LT_MOD") else { return };
    let mut rf = ootrr_core::romfs::Romfs::open(std::path::Path::new("D:/Oot/romfs")).unwrap();
    rf.root = std::path::PathBuf::from(m);
    for f in ["Banc/Graveyard/Graveyard_Adult_Static.bcett.byml.zs", "Banc/Graveyard/Graveyard_Child_Static.bcett.byml.zs"] {
        let y = rf.read_byml(f).unwrap();
        for a in y.root.get("Actors").unwrap().as_arr().unwrap() {
            let s = format!("{:?}", a);
            if s.contains("Arrow_Light_Cold") {
                println!("{f} hash={:?} gyaml={:?} tr={:?} dyn={:?}", ootrr_core::placement::hash_of(a), a.get("Gyaml"), a.get("Translate"), a.get("Dynamic"));
            }
        }
    }
}
