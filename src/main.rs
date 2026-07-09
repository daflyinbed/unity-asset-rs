use std::path::PathBuf;

use unity_asset_rs::{
    load, parse_object, Error, ParsedObject, SerializedFile, UnityObject, Value,
};

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
                let refs = sf.ref_type_table();
                for (i, obj) in sf.objects.iter().enumerate() {
                    let class_id = obj.class_id(&sf);
                    print!("\nobj[{i}] class={class_id} path_id={} ({} bytes): ", obj.path_id, obj.byte_size);
                    match parse_object(&sf, obj, &refs) {
                        Ok(ParsedObject::Typed(uo)) => {
                            println!("✓ typed ({})", typed_name(&uo));
                            dump_typed(&uo);
                        }
                        Ok(ParsedObject::Dynamic(v)) => {
                            println!("✓ dynamic (TypeTree)");
                            dump_top(&v);
                        }
                        Err(e) => println!("✗ {e}"),
                    }
                }
            }
            Err(e) => println!("  非 SerializedFile: {e}"),
        }
    }
    Ok(())
}

fn typed_name(uo: &UnityObject) -> &'static str {
    match uo {
        UnityObject::Texture2D(_) => "Texture2D",
        UnityObject::Mesh(_) => "Mesh",
        UnityObject::Material(_) => "Material",
        UnityObject::Shader(_) => "Shader",
        UnityObject::GameObject(_) => "GameObject",
        UnityObject::Transform(_) => "Transform",
        _ => "other",
    }
}

fn dump_typed(uo: &UnityObject) {
    if let UnityObject::Texture2D(tex) = uo {
        println!("    m_Name: {:?}", tex.m_name);
        println!("    m_Width={} m_Height={} m_Format={}",
            tex.m_width, tex.m_height, tex.m_format);
    }
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
