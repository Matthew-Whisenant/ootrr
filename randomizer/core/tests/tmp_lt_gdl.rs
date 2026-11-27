use ootrr_core::byml::Node;
fn walk(n: &Node, path: &str, out: &mut Vec<String>) {
    match n {
        Node::Map(m) => for (k, v) in m { walk(v, &format!("{path}.{k}"), out) },
        Node::Arr(a) => {
            let s = format!("{:?}", n);
            if s.contains("LightArrow") && a.iter().all(|x| !matches!(x, Node::Map(_) | Node::Arr(_))) { out.push(format!("{path} = {s}")); return; }
            for (i, v) in a.iter().enumerate() {
                let s = format!("{:?}", v);
                if s.contains("LightArrow") { if let Node::Map(_) = v { out.push(format!("{path}[{i}] = {}", &s[..s.len().min(1500)])); continue } }
                walk(v, &format!("{path}[{i}]"), out)
            }
        }
        Node::Str(s) if s.contains("LightArrow") => out.push(format!("{path} = {s}")),
        _ => {}
    }
}
#[test]
fn tmp_lt_gdl() {
    let mut rf = ootrr_core::romfs::Romfs::open(std::path::Path::new("D:/Oot/romfs")).unwrap();
    let y = rf.read_byml("GameData/GameDataList.Product.110.byml.zs").unwrap();
    let mut out = Vec::new();
    walk(&y.root, "", &mut out);
    for o in out { println!("GDL {o}"); }
}
