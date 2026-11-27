use ootrr_core::byml::{Byml, Node};
use ootrr_core::pickup_pack::rows::{id, POUCH_INFO};

#[test]
fn tmp_lt_ab() {
    let Some(m) = std::env::var_os("LT_MOD") else { return };
    let out = std::env::var("LT_OUT").unwrap();
    let mut rf = ootrr_core::romfs::Romfs::open(std::path::Path::new("D:/Oot/romfs")).unwrap();
    rf.root = std::path::PathBuf::from(m);
    let z = rf.read(POUCH_INFO).unwrap();
    let dict = ootrr_core::zs::dict_of(&z);
    let mut y: Byml = rf.read_byml(POUCH_INFO).unwrap();
    let mut n = 0;
    for r in y.root.as_arr_mut().unwrap().iter_mut() {
        let name = id(r).unwrap_or("").to_string();
        if name.starts_with("OOTRR_") && name.contains("_Cold") && r.get("AssignableItem").is_some() {
            println!("CUT {name} slot={:?}", r.get("AssignableItem"));
            r.remove("AssignableItem");
            n += 1;
        }
    }
    if std::env::var_os("LT_BOW").is_some() {
        let ch = ootrr_core::dungeon_credit::bow_slot(&mut y).unwrap();
        println!("BOW_SLOT changed={ch}");
    }
    let raw = y.write().unwrap();
    let back = Byml::parse(&raw).unwrap();
    assert_eq!(back.root.as_arr().unwrap().len(), y.root.as_arr().unwrap().len());
    let packed = ootrr_core::zs::compress_raw(&raw, dict).unwrap();
    let mut chk = ootrr_core::romfs::Romfs::open(std::path::Path::new("D:/Oot/romfs")).unwrap();
    assert_eq!(chk.zs.decompress(&packed).unwrap(), raw);
    std::fs::write(&out, &packed).unwrap();
    println!("WROTE {out} rows_cut={n} dict={dict:?} size {} -> {}", z.len(), packed.len());
    let _ = Node::Bool(true);
}
