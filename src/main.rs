use std::path::PathBuf;

use unity_asset_rs::{load, Error, SerializedFile, Value};

fn main() -> Result<(), Error> {
    env_logger::init();

    let path = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("example/char_002_amiya.ab"));

    let bundle = load(&path)?;
    println!(
        "bundle: format v{}, engine {}, {} files",
        bundle.format_version,
        bundle.unity_revision,
        bundle.files.len()
    );

    for f in &bundle.files {
        println!("\n=== {} ({} bytes) ===", f.name, f.bytes.len());
        match SerializedFile::parse(f.bytes.clone()) {
            Ok(sf) => {
                println!(
                    "version={} unity={} endian={:?} types={} objects={}",
                    sf.version,
                    sf.unity_version,
                    sf.endian,
                    sf.types.len(),
                    sf.objects.len()
                );
                for (i, obj) in sf.objects.iter().enumerate() {
                    let class_id = obj.class_id(&sf);
                    match sf.parse_object(i) {
                        Ok(Some(v)) => {
                            println!(
                                "\nobj[{i}] class={class_id} path_id={} ({} bytes)",
                                obj.path_id,
                                obj.byte_size
                            );
                            dump_top(&v);
                        }
                        Ok(None) => println!(
                            "obj[{i}] class={class_id}: 无内嵌 typetree",
                        ),
                        Err(e) => println!("obj[{i}] class={class_id}: 解析失败: {e}"),
                    }
                }
            }
            Err(e) => println!("  非 SerializedFile: {e}"),
        }
    }
    Ok(())
}

/// 打印 struct 顶层字段；非 struct 打印单行。
fn dump_top(v: &Value) {
    match v {
        Value::Struct(fields) => {
            for (k, val) in fields {
                match val {
                    Value::Struct(inner) => {
                        println!("    {k}: struct ({} fields)", inner.len())
                    }
                    Value::Array(items) => println!("    {k}: array[{}]", items.len()),
                    Value::Bytes(b) => println!("    {k}: bytes[{}]", b.len()),
                    Value::String(s) => {
                        let s = if s.chars().count() > 80 {
                            format!("{}…", s.chars().take(80).collect::<String>())
                        } else {
                            s.clone()
                        };
                        println!("    {k}: {s:?}")
                    }
                    other => println!("    {k}: {other:?}"),
                }
            }
        }
        other => println!("    {other:?}"),
    }
}
