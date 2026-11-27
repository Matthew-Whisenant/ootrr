#[test]
fn tmp_lt_blarc() {
    let Some(m) = std::env::var_os("LT_MOD") else { return };
    let p = "UI/Layout2Archive/Common.Product.100.Nin_Ounce_NVN2.blarc.zs";
    let mut v = ootrr_core::romfs::Romfs::open(std::path::Path::new("D:/Oot/romfs")).unwrap();
    let a = v.read_zs(p).unwrap();
    let mut rf = ootrr_core::romfs::Romfs::open(std::path::Path::new("D:/Oot/romfs")).unwrap();
    rf.root = std::path::PathBuf::from(m);
    let b = rf.read_zs(p).unwrap();
    println!("SIZES vanilla={} mod={} same={}", a.len(), b.len(), a == b);
    let ea = ootrr_core::sarc::parse(&a).unwrap();
    let eb = ootrr_core::sarc::parse(&b).unwrap();
    println!("ENTRIES vanilla={} mod={}", ea.len(), eb.len());
    for e in &eb {
        match ea.iter().find(|x| x.name == e.name) {
            None => println!("NEW {} {}", e.name, e.data.len()),
            Some(x) if x.data != e.data => println!("CHANGED {} {} -> {}", e.name, x.data.len(), e.data.len()),
            _ => {}
        }
    }
    for e in &ea { if !eb.iter().any(|x| x.name == e.name) { println!("GONE {}", e.name); } }
}
