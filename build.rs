//! Split out the JSON Schema test suites into individual files
//! for use with rstest.
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
struct Output {
    schema: Value,
    test_case_description: String,
    test: Test,
}

#[derive(Debug, Deserialize, Serialize)]
struct Test {
    formerly: Option<String>,
    description: Option<String>,
    data: Value,
    valid: bool,
}

impl Test {
    fn full_description(&self) -> String {
        match (self.formerly.as_deref(), self.description.as_deref()) {
            (None, None) => String::default(),
            (None, Some(s)) => s.to_string(),
            (Some(s), None) => s.to_string(),
            (Some(form), Some(desc)) => format!("{desc} [formerly: {form}]"),
        }
    }
}

#[derive(Debug, Deserialize)]
struct TestCase {
    description: String,
    schema: Value,
    tests: Vec<Test>,
}

#[derive(Debug)]
struct Processor {
    src_dir: PathBuf,
    tgt_dir: PathBuf,
}

impl Processor {
    fn new(src_dir: impl Into<PathBuf>, tgt_dir: impl Into<PathBuf>) -> Self {
        let src_dir: PathBuf = src_dir.into();
        let tgt_dir: PathBuf = tgt_dir.into();

        let s = Self { src_dir, tgt_dir };
        println!("Processing with {s:?}");
        s
    }

    fn process_all(&self) {
        for res in std::fs::read_dir(&self.src_dir).expect("list src") {
            let entry = res.expect("read dir entry");
            println!("Processing test case {:?}", entry.path());
            let fname = entry.file_name();
            let name = fname.to_str().expect("stringify name");
            if !name.ends_with(".json") {
                continue;
            }
            let s = std::fs::read_to_string(entry.path()).expect("read file");
            let tc: TestCase = serde_json::from_str(&s).expect("deser test case");

            let out_dir = self.tgt_dir.join(
                entry
                    .path()
                    .strip_prefix(&self.src_dir)
                    .unwrap()
                    .with_extension(""),
            );

            for (idx, test) in tc.tests.iter().enumerate() {
                let output = Output {
                    schema: tc.schema.clone(),
                    test_case_description: tc.description.clone(),
                    test: Test {
                        formerly: None,
                        description: Some(test.full_description()),
                        data: test.data.clone(),
                        valid: test.valid,
                    },
                };
                let s = serde_json::to_string_pretty(&output).unwrap();
                let out_path = out_dir.join(format!("{}.json", infer_stem(test, idx)));
                std::fs::create_dir_all(out_path.parent().unwrap()).unwrap();
                std::fs::write(out_path, s).expect("write test");
            }
        }
    }
}

fn infer_stem(t: &Test, idx: usize) -> String {
    let m_desc = t
        .description
        .as_deref()
        .filter(|d| *d != "TBD")
        .or(t.formerly.as_deref())
        .map(|s| s.strip_suffix(".json").unwrap_or(s));
    let Some(desc) = m_desc else {
        return format!("{idx:0>3}");
    };
    let mut out = String::with_capacity(desc.len());
    for c in desc.chars() {
        if c.is_alphanumeric() || ['-', '_', '/'].contains(&c) {
            out.push(c);
        } else {
            out.push('_');
        }
    }
    out
}

fn main() {
    let project_root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let src_root = project_root.join("ome-zarr");
    let tgt_root = project_root.join("tests/fixtures/generated");

    // DANGER DANGER
    std::fs::remove_dir_all(&tgt_root).unwrap_or(());

    for ver in ["0.4", "0.5"] {
        let src_dir = src_root.join(format!("{ver}/tests"));
        let tgt_dir = tgt_root.join(ver);
        let proc = Processor::new(src_dir, tgt_dir);
        proc.process_all();
    }
}
