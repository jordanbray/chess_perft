use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
struct PIN {
    id: String,
    fen: String,
    depth: String,
    expected: String,
}

#[derive(Deserialize, Clone)]
struct Func {
    perft_func: String,
    perft_name: String,
}

const FN_STR: &'static str = "\
\t\tpaste! {
            #[allow(dead_code)]
            fn [<perft_{id}_ $perft_name>](bench: &mut Bencher) {
                $perft_func(
                    bench,
                    \"{fen}\".to_string(),
                    {depth},
                    {expected},
                )
            }
        }
";

const N_STR: &'static str = "\t\t\tperft_{id}_{perft_name},\n";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut f = File::create(Path::new(&out_dir).join("bench_macros.rs")).unwrap();

    let mut buf = Vec::new();
    File::open(Path::new("build/perft_inputs.json"))
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();

    let s: String = buf.into_iter().map(|x| x as char).collect();
    let inputs: Vec<PIN> = serde_json::from_str(s.as_str()).unwrap();

    let mut buf = Vec::new();
    File::open(Path::new("build/perft_funcs.json"))
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    let s: String = buf.into_iter().map(char::from).collect();
    let funcs: Vec<Func> = serde_json::from_str(s.as_str()).unwrap();

    writeln!(
        f,
        r"use paste::paste;
"
    )
    .unwrap();
    let mut gpi_internal =
        "macro_rules! gen_perft_inputs_internal {\n\t($perft_func:ident, $perft_name:tt) => {\n"
            .to_string();

    let mut gn_internal = "macro_rules! get_names_internal {\n\t($group_name:tt) => {\n\t\tbenchmark_group!(\n\t\t\t$group_name,\n".to_string();

    gpi_internal.push_str(
        inputs
            .iter()
            .cloned()
            .map(|pin| {
                FN_STR
                    .to_string()
                    .replace("{id}", &pin.id)
                    .replace("{fen}", &pin.fen)
                    .replace("{depth}", &pin.depth)
                    .replace("{expected}", &pin.expected)
            })
            .collect::<String>()
            .as_str(),
    );

    gn_internal.push_str(
        std::iter::repeat(
            inputs
                .into_iter()
                .map(|pin| N_STR.to_string().replace("{id}", &pin.id))
                .collect::<String>(),
        )
        .zip(funcs)
        .map(|(s, f)| s.replace("{perft_name}", &f.perft_name))
        .collect::<String>()
        .as_str(),
    );

    gpi_internal.push_str("\t}\n}");

    gn_internal.push_str("\t\t);\n\t}\n}");

    writeln!(f, "{gpi_internal}").unwrap();
    writeln!(f, "{gn_internal}").unwrap();

    write!(
        f,
        "

pub(crate) use gen_perft_inputs_internal;
pub(crate) use get_names_internal;"
    )
    .unwrap();
}
