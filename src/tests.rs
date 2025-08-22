use std::io::Read;
use std::sync::LazyLock;
use std::{collections::BTreeMap, path::PathBuf};

use serde::de::DeserializeOwned;
use serde::Deserialize;

fn strip_comments(jsonc: &str) -> String {
    let mut s = String::with_capacity(jsonc.len());
    let mut rd = json_comments::StripComments::new(jsonc.as_bytes());
    rd.read_to_string(&mut s).unwrap();
    s
}

type VersionMap<T> = LazyLock<BTreeMap<(u64, u64), T>>;

static VERSION_DIRS: VersionMap<PathBuf> = LazyLock::new(|| {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ome-zarr");
    let Ok(rddir) = std::fs::read_dir(&root) else {
        return Default::default();
    };
    rddir
        .filter_map(|r| {
            let entry = r.ok()?;
            if !entry.file_type().ok()?.is_dir() {
                return None;
            };
            let fname = entry.file_name().into_string().ok()?;
            let (first, second) = fname.split_once('.')?;
            let ver = (first.parse::<u64>().ok()?, second.parse::<u64>().ok()?);
            if ver < (0, 4) {
                return None;
            }
            Some((ver, entry.path()))
        })
        .collect()
});

/// Map from version tuple to directory name to example file stem to JSON content, e.g.
/// `(0, 5) -> "multiscales_strict" -> "multiscales_example" -> '{"key": "value", ...}'`
static EXAMPLE_JSON: VersionMap<BTreeMap<String, BTreeMap<String, String>>> = LazyLock::new(|| {
    let mut out: BTreeMap<(u64, u64), BTreeMap<String, BTreeMap<String, String>>> =
        BTreeMap::default();
    for (ver, ver_path_ref) in VERSION_DIRS.iter() {
        let ver_path = ver_path_ref.join("examples");
        let Ok(rddir) = std::fs::read_dir(&ver_path) else {
            continue;
        };

        for (dir, dir_path) in rddir.filter_map(|r| {
            let entry = r.ok()?;
            if !entry.file_type().ok()?.is_dir() {
                return None;
            };
            let fname = entry.file_name().into_string().ok()?;

            let mut path = entry.path();
            path.push("valid");
            if !path.is_dir() {
                path.pop();
            }
            Some((fname, path))
        }) {
            let Ok(rddir) = std::fs::read_dir(&dir_path) else {
                continue;
            };
            for (fname, content) in rddir.filter_map(|r| {
                let entry = r.ok()?;
                if !entry.file_type().ok()?.is_file() {
                    return None;
                };
                let fname = entry.file_name().into_string().ok()?;
                if fname.starts_with(".config") || !fname.ends_with(".json") {
                    return None;
                };
                let p = entry.path();
                let jsonc = std::fs::read_to_string(&p).ok()?;
                let json = strip_comments(&jsonc);
                let stem = p.file_stem()?.to_str()?.to_string();
                Some((stem, json))
            }) {
                out.entry(*ver)
                    .or_default()
                    .entry(dir.clone())
                    .or_default()
                    .insert(fname, content);
            }
        }
    }
    out
});

#[derive(Debug, Deserialize)]
pub struct TestSuiteSchema {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Test {
    pub formerly: String,
    // pub description: Option<String>,
    pub data: serde_json::Value,
    pub valid: bool,
}

impl Test {
    /// Return None if the test passes; an error message otherwise.
    pub fn test_deser<T: DeserializeOwned + std::fmt::Debug>(&self) -> Option<String> {
        let result: serde_json::Result<T> = serde_json::from_value(self.data.clone());
        let msg = match result {
            Ok(t) => {
                if self.valid {
                    return None;
                }
                format!(
                    "test {}: expected invalid data, but got valid {t:?}",
                    self.formerly
                )
            }
            Err(e) => {
                if !self.valid {
                    return None;
                }
                format!(
                    "test {}: expected valid data but got error {e}",
                    self.formerly
                )
            }
        };
        Some(msg)
    }
}

#[derive(Debug, Deserialize)]
pub struct TestSuite {
    // pub description: Option<String>,
    pub schema: TestSuiteSchema,
    pub tests: Vec<Test>,
}

impl TestSuite {
    /// Iterate over error messages.
    pub fn test_deser_all<'a, T: std::fmt::Debug + DeserializeOwned>(
        &'a self,
    ) -> impl Iterator<Item = String> + 'a {
        self.tests.iter().filter_map(|t| {
            let msg = t.test_deser::<T>()?;
            Some(format!("schema {}: {msg}", self.schema.id))
        })
    }
}

static TEST_SUITES: LazyLock<BTreeMap<(u64, u64), BTreeMap<String, TestSuite>>> =
    LazyLock::new(|| {
        VERSION_DIRS
            .iter()
            .map(|(ver, ver_path)| {
                let mut suites = BTreeMap::default();
                if ver < &(0, 5) {
                    return (*ver, suites);
                }
                let suites_iter =
                    ver_path
                        .join("tests")
                        .read_dir()
                        .unwrap()
                        .filter_map(|r_entry| {
                            let entry = r_entry.ok()?;
                            let fname_os = entry.file_name();
                            let fname = fname_os.to_string_lossy();
                            let mut stem = fname.strip_suffix(".json")?;
                            if let Some(s) = stem.strip_suffix("_suite") {
                                stem = s;
                            }
                            let contents = std::fs::read(entry.path()).unwrap();
                            let suite: TestSuite = serde_json::from_slice(&contents).unwrap();
                            Some((stem.to_string(), suite))
                        });
                suites.extend(suites_iter);
                (*ver, suites)
            })
            .collect()
    });

/// Get example JSON for a given version, in a map from directory name to file stem to JSON content (comments stripped).
pub fn get_examples(version: (u64, u64)) -> &'static BTreeMap<String, BTreeMap<String, String>> {
    EXAMPLE_JSON
        .get(&version)
        .unwrap_or_else(|| panic!("Expected examples of version {}.{}", version.0, version.1))
}

pub fn get_test_suites(version: (u64, u64)) -> &'static BTreeMap<String, TestSuite> {
    TEST_SUITES.get(&version).unwrap_or_else(|| {
        panic!(
            "Expected test suite for version {}.{}",
            version.0, version.1
        )
    })
}
