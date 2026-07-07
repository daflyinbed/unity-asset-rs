//! 验证：用 codegen 生成的代码解析 example/char_002_amiya.ab 里的全部对象。
//! MonoBehaviour (classID=114) 走 TypeTree 解释器，其余走 generated typed parse。

use unity_asset_rs::{load, parse_object, Bundle, ClassIDType, SerializedFile};

fn main() {
    env_logger::init();
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "example/char_002_amiya.ab".into());

    let bundle: Bundle = load(&path).expect("加载 bundle 失败");
    eprintln!("bundle: {} 个内部文件", bundle.files.len());

    let mut typed_ok = 0usize;
    let mut typed_fail = 0usize;
    let mut mono_ok = 0usize;
    let mut total = 0usize;
    let mut failures: Vec<(i32, String, String)> = Vec::new();

    for bf in &bundle.files {
        eprintln!("\n=== 内部文件: {} ({} bytes) ===", bf.name, bf.bytes.len());
        let sf = SerializedFile::parse(bf.bytes.clone()).expect("解析 SerializedFile 失败");
        eprintln!(
            "Unity {}, format v{}, endian {:?}, {} 个对象",
            sf.unity_version, sf.version, sf.endian, sf.objects.len()
        );

        let refs = sf.ref_type_table();
        for (i, obj) in sf.objects.iter().enumerate() {
            total += 1;
            let class_id = obj.class_id(&sf);
            let type_name = ClassIDType::from_i32(class_id)
                .map(|c| format!("{c:?}"))
                .unwrap_or_else(|| format!("ClassID({class_id})"));
            match parse_object(&sf, obj, &refs) {
                Ok(unity_asset_rs::ParsedObject::Typed(_)) => {
                    typed_ok += 1;
                    eprintln!("  [{i}] classID={class_id:>4} {type_name:<28} ✓ typed");
                }
                Ok(unity_asset_rs::ParsedObject::Dynamic(_)) => {
                    mono_ok += 1;
                    eprintln!("  [{i}] classID={class_id:>4} {type_name:<28} ✓ dynamic (TypeTree)");
                }
                Err(e) => {
                    typed_fail += 1;
                    eprintln!("  [{i}] classID={class_id:>4} {type_name:<28} ✗ {e}");
                    failures.push((class_id, type_name.to_string(), format!("{e}")));
                }
            }
        }
    }

    eprintln!("\n========== 汇总 ==========");
    eprintln!("总对象数:   {total}");
    eprintln!("typed 成功: {typed_ok}");
    eprintln!("dynamic:    {mono_ok}");
    eprintln!("失败:       {typed_fail}");
    if !failures.is_empty() {
        eprintln!("\n失败明细:");
        for (cid, name, err) in &failures {
            eprintln!("  classID={cid} {name}: {err}");
        }
        std::process::exit(1);
    }
}
