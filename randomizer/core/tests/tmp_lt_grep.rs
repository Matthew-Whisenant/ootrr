#[test]
fn tmp_lt_grep() {
    let Some(root) = std::env::var_os("LG_ROOT") else { return };
    let dirs: Vec<String> = std::env::var("LG_DIRS").unwrap().split(',').map(|s| s.to_string()).collect();
    let pats: Vec<Vec<u8>> = std::env::var("LG_PAT").unwrap().split(',').map(|s| s.as_bytes().to_vec()).collect();
    let mut rf = ootrr_core::romfs::Romfs::open(std::path::Path::new("D:/Oot/romfs")).unwrap();
    rf.root = std::path::PathBuf::from(root);
    let has = |d: &[u8]| pats.iter().filter(|p| d.windows(p.len()).any(|w| w == p.as_slice())).map(|p| String::from_utf8_lossy(p).to_string()).collect::<Vec<_>>();
    for d in &dirs {
        let Ok(list) = rf.list(d) else { continue };
        for f in list {
            let raw = if f.ends_with(".zs") { match rf.read_zs(&f) { Ok(b) => b, Err(_) => continue } } else { match rf.read(&f) { Ok(b) => b, Err(_) => continue } };
            let mut inner: Vec<String> = Vec::new();
            if let Ok(es) = ootrr_core::sarc::parse(&raw) {
                for e in es {
                    let h = has(&e.data);
                    if !h.is_empty() { inner.push(format!("{} {:?}", e.name, h)); }
                }
            }
            if !inner.is_empty() {
                for i in inner { println!("HIT {f} :: {i}"); }
            } else {
                let h = has(&raw);
                if !h.is_empty() { println!("HIT {f} {:?}", h); }
            }
        }
    }
}
