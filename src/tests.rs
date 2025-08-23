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
            Ok(_t) => {
                if self.valid {
                    return None;
                }
                "expected invalid data, but got valid".to_string()
            }
            Err(e) => {
                if !self.valid {
                    return None;
                }
                format!("expected valid data but got error: {e}",)
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
    /// Iterate over test `formerly` values, and an optional error message.
    /// None means the test passes.
    pub fn test_all<T: std::fmt::Debug + DeserializeOwned>(&self) -> SuiteReport {
        let mut pass = vec![];
        let mut fail = vec![];
        for (name, o_msg) in self
            .tests
            .iter()
            .map(|t| (t.formerly.as_str(), t.test_deser::<T>()))
        {
            if let Some(msg) = o_msg {
                fail.push((name.to_string(), msg));
            } else {
                pass.push(name.to_string());
            }
        }
        SuiteReport {
            schema: self.schema.id.to_string(),
            pass,
            fail,
        }
    }

    /// Select a test with the given `formerly` value.
    /// None means the test passes, otherwise give an error message
    /// (including if the named test doesn't exist).
    pub fn test_by_formerly<T: std::fmt::Debug + DeserializeOwned>(
        &self,
        formerly: &str,
    ) -> Option<String> {
        let Some(test) = self.tests.iter().find(|t| t.formerly.as_str() == formerly) else {
            let names = join_str(self.tests.iter().map(|t| t.formerly.as_str()), ", ", "\"");
            if names.is_empty() {
                panic!("no tests found")
            } else {
                panic!("no test called {formerly}; try {names}")
            }
        };
        test.test_deser::<T>()
    }
}

#[derive(Debug, Default)]
pub struct NamedSuites(BTreeMap<String, TestSuite>);

fn join_str<S: AsRef<str>>(strs: impl IntoIterator<Item = S>, sep: &str, surround: &str) -> String {
    let mut is_first = true;
    let mut out = String::default();
    for s in strs.into_iter() {
        if is_first {
            is_first = false;
        } else {
            out.push_str(sep);
        }
        out.push_str(surround);
        out.push_str(s.as_ref());
        out.push_str(surround);
    }
    out
}

impl NamedSuites {
    fn add_suite<S: Into<String>>(&mut self, name: S, suite: TestSuite) -> Option<TestSuite> {
        self.0.insert(name.into(), suite)
    }

    /// Yield the file stem, schema ID, and map of test "formerly" -> optional error message.
    pub fn test_all<T: std::fmt::Debug + DeserializeOwned>(&self) -> SuiteReports {
        SuiteReports(
            self.0
                .iter()
                .map(|(name, suite)| (name.to_owned(), suite.test_all::<T>()))
                .collect(),
        )
    }

    #[allow(unused)]
    pub fn test_suite<T: std::fmt::Debug + DeserializeOwned>(
        &self,
        suite_name: &str,
    ) -> SuiteReport {
        let Some(m) = self.0.get(suite_name) else {
            let suite_names = join_str(self.0.keys(), ", ", "\"");
            if suite_names.is_empty() {
                panic!("No suites found")
            } else {
                panic!("No suite with name {suite_name}; try one of {suite_names}")
            }
        };
        m.test_all::<T>()
    }

    #[allow(unused)]
    pub fn test_single<T: std::fmt::Debug + DeserializeOwned>(
        &self,
        suite: &str,
        test_formerly: &str,
    ) {
        let Some(suite) = self.0.get(suite) else {
            let suite_names = join_str(self.0.keys(), ", ", "\"");
            if suite_names.is_empty() {
                panic!("No suites found")
            } else {
                panic!("No suite with name {suite}; try one of {suite_names}")
            }
        };
        if let Some(msg) = suite.test_by_formerly::<T>(test_formerly) {
            panic!("{msg}");
        }
    }
}

/// On each line, the trailing newline is removed and prefix and suffix are added.
/// The prefix is skipped on empty lines.
/// Use suffix to re-add the newline.
///
/// Useful for indentation.
fn surround_lines(s: &str, prefix: &str, suffix: &str) -> String {
    let mut out = String::default();
    if s.is_empty() {
        return out;
    }
    for line in s.lines() {
        if !line.is_empty() {
            out.push_str(prefix);
        }
        out.push_str(line);
        out.push_str(suffix);
    }
    out
}

#[derive(Debug)]
pub struct SuiteReport {
    schema: String,
    /// Test `formerly` values which passed
    pass: Vec<String>,
    /// Test `formerly` values and error messages
    fail: Vec<(String, String)>,
}

impl SuiteReport {
    fn len(&self) -> usize {
        self.pass.len() + self.fail.len()
    }

    fn passed(&self) -> bool {
        self.fail.is_empty()
    }

    fn fail_reports(&self) -> String {
        join_str(
            self.fail
                .iter()
                .map(|(name, msg)| format!("FAILED: test {name}: {msg}")),
            "\n",
            "",
        )
    }
}

#[derive(Debug)]
pub struct SuiteReports(Vec<(String, SuiteReport)>);

impl SuiteReports {
    fn passed(&self) -> bool {
        self.0.iter().all(|(_, r)| r.passed())
    }

    fn report(&self) -> String {
        let rep_iter = self.0.iter().map(|(name, rep)| {
            if rep.passed() {
                return format!(
                    "PASSED: suite {name} (schema {}), {} tests",
                    rep.schema,
                    rep.len()
                );
            }
            let test_reports = surround_lines(
                &rep.fail_reports(),
                "\t",
                "\n",
            );
            format!(
                "FAILED: suite {name} (schema {}), {} of {} passed\n{test_reports}",
                rep.schema,
                rep.pass.len(),
                rep.len()
            )
        });
        join_str(rep_iter, "\n", "")
    }
}

static TEST_SUITES: LazyLock<BTreeMap<(u64, u64), NamedSuites>> = LazyLock::new(|| {
    VERSION_DIRS
        .iter()
        .map(|(ver, ver_path)| {
            let mut suites = NamedSuites::default();
            if ver < &(0, 5) {
                return (*ver, suites);
            }
            for (name, suite) in ver_path
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
                })
            {
                suites.add_suite(name, suite);
            }
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

pub fn get_test_suites(version: (u64, u64)) -> &'static NamedSuites {
    TEST_SUITES.get(&version).unwrap_or_else(|| {
        panic!(
            "Expected test suite for version {}.{}",
            version.0, version.1
        )
    })
}

pub fn run_test_suites_for_version<T: DeserializeOwned + std::fmt::Debug>(version: (u64, u64)) {
    let suites = get_test_suites(version);
    let results = suites.test_all::<T>();
    if !results.passed() {
        panic!("{}", results.report());
    }
}

pub fn run_examples_for_version<T: DeserializeOwned + std::fmt::Debug>(version: (u64, u64)) {
    let mut msg = String::default();
    let mut failed = 0;
    let mut total = 0;
    for (dname, map) in get_examples(version) {
        for (fname, content) in map {
            total += 1;

            let Err(e) = serde_json::from_str::<T>(content) else {
                continue;
            };
            failed += 1;
            msg.push_str(&format!(
                "dir {dname}, example {fname}: failed with error {e}\n"
            ));
        }
    }
    if failed > 0 {
        panic!("Failed {failed} of {total}:\n{}", msg.trim_end());
    }
}
