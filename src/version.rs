/// Macro to define a wrapper type for constrained versions.
///
/// Arguments are the type name, version specifier string, and default version string.
#[macro_export]
macro_rules! constrained_version {
    ($name:ident, $spec:expr, $default:expr) => {
        #[doc = "PEP-440 version with constraint `'"]
        #[doc = $spec]
        // #[doc = stringify!($spec)]
        #[doc = "'` and default `'"]
        #[doc = $default]
        // #[doc = stringify!($default)]
        #[doc = "'`.\n Instantiate with its [`std::str::FromStr`] or [`std::convert::TryFrom<pep440_rs::Version>`] implementations.\n\n"]
        #[doc = "Defined by the `constrained_version!` macro."]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(pep440_rs::Version);

        impl Default for $name {
            fn default() -> Self {
                static VERSION: std::sync::LazyLock<$name> = std::sync::LazyLock::new(|| {
                    // this potential panic is OK
                    // because this macro also generates a unit test
                    // for this default version
                    let ver: pep440_rs::Version =
                        $default.parse().expect("Failed to parse version");
                    $name::try_from(ver).expect("Default version does not satisfy constraints")
                });
                VERSION.clone()
            }
        }

        impl TryFrom<pep440_rs::Version> for $name {
            type Error = $crate::Error;
            fn try_from(version: pep440_rs::Version) -> Result<Self, Self::Error> {
                static SPECIFIERS: std::sync::LazyLock<pep440_rs::VersionSpecifiers> =
                    std::sync::LazyLock::new(|| {
                        // this potential panic is OK
                        // because this macro also generates a unit test
                        // which checks the validity of the specifier
                        $spec.parse().expect("Failed to parse version constraints")
                    });
                if SPECIFIERS.contains(&version) {
                    Ok(Self(version))
                } else {
                    Err($crate::Error::VersionConstraint {
                        constraint: $spec,
                        version,
                    })
                }
            }
        }

        impl From<$name> for pep440_rs::Version {
            fn from(cv: $name) -> Self {
                cv.0
            }
        }

        impl std::ops::Deref for $name {
            type Target = pep440_rs::Version;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl std::str::FromStr for $name {
            type Err = $crate::Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let v: pep440_rs::Version = s.parse()?;
                Self::try_from(v)
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                String::deserialize(deserializer)?
                    .parse()
                    .map_err(serde::de::Error::custom)
            }
        }

        paste::paste! {
            #[cfg(test)]
            #[allow(non_snake_case)]
            mod [<test_ $name>] {
                use super::$name;

                #[test]
                fn test_valid_specifier() {
                    let _spec: pep440_rs::VersionSpecifiers =
                        $spec.parse().expect("Failed to parse version specifiers");
                }

                #[test]
                fn test_default() {
                    let _default: $name = Default::default();
                }
            }
        }
    };
}
