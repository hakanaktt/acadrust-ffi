use std::{env, fs};
use syn::{Fields, Item, Visibility};

// Regenerate a native view module from the pinned upstream Rust definitions.
fn main() {
    let mut args = env::args().skip(1);
    let file = args.next().expect("source file");
    let module = args.next().expect("public Rust module path");
    let filter: Vec<String> = args.collect();
    let source = fs::read_to_string(file).unwrap();
    let syntax = syn::parse_file(&source).unwrap();
    for item in syntax.items {
        match item {
            Item::Struct(item) if matches!(item.vis, Visibility::Public(_)) && item.generics.params.is_empty() => {
                let name = item.ident.to_string();
                if !filter.is_empty() && !filter.contains(&name) { continue; }
                let fields: Vec<String> = item.fields.iter().enumerate()
                    .filter(|(_, f)| matches!(f.vis, Visibility::Public(_)))
                    .map(|(i, f)| f.ident.as_ref().map(ToString::to_string).unwrap_or_else(|| i.to_string())).collect();
                println!("view_struct!({module}::{name} {{ {} }});", fields.join(", "));
                println!("const _: fn(&{module}::{name}) = |v| {{");
                for field in fields { println!("    let _ = view(&v.{field});"); }
                println!("}};");
            }
            Item::Enum(item) if matches!(item.vis, Visibility::Public(_)) && item.generics.params.is_empty() => {
                let name = item.ident.to_string();
                if !filter.is_empty() && !filter.contains(&name) { continue; }
                println!("view_enum!({module}::{name} {{");
                for variant in item.variants {
                    let suffix = match variant.fields {
                        Fields::Unit => String::new(),
                        Fields::Unnamed(fields) => format!("({})", (0..fields.unnamed.len()).map(|i| format!("v{i}")).collect::<Vec<_>>().join(", ")),
                        Fields::Named(fields) => format!(" {{ {} }}", fields.named.iter().map(|f| f.ident.as_ref().unwrap().to_string()).collect::<Vec<_>>().join(", ")),
                    };
                    println!("    {}{suffix},", variant.ident);
                }
                println!("}});");
            }
            _ => {}
        }
    }
}
