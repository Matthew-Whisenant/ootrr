#[test]
fn tmp_lt_table() {
    let root = std::env::var("LT_ROOT").unwrap_or("D:/Oot/romfs".to_string());
    let mut rf = ootrr_core::romfs::Romfs::open(std::path::Path::new("D:/Oot/romfs")).unwrap();
    rf.root = std::path::PathBuf::from(root);
    let y = rf.read_byml("UI/Pouch/PouchTable/PouchTable.game__ui__PouchTable.bgyml").unwrap();
    let s = format!("{:#?}", y.root);
    std::fs::write(std::env::var("LT_OUT").unwrap(), s).unwrap();
}
