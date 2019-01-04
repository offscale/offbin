use std::fs::File;
use std::io::Write;

#[derive(Clone, Debug, Serialize)]
pub struct CargoToml {
    package: Package,
    dependencies: Dependencies,
    //bin: Vec<Bin>,
}

#[derive(Clone, Debug, Serialize)]
struct Package {
    name: String,
    version: String,
    edition: String,
}

#[derive(Clone, Debug, Serialize)]
struct Dependencies {
    toml: String,
    serde_derive: String,
    serde: String,
    indexmap: String,
    offbin: Offbin,
    #[serde(rename = "cargo-make")]
    cargo_make: CargoMake,
}

#[derive(Clone, Debug, Serialize)]
struct Offbin {
    git: String,
}

#[derive(Clone, Debug, Serialize)]
struct CargoMake {
    git: String,
    branch: String,
}

// #[derive(Clone, Debug, Serialize)]
// struct Bin {
//     name: String,
// }

// impl Bin {
//     fn new(name: &str) -> Self {
//         Bin {
//             name: name.to_string(),
//         }
//     }
// }

impl Dependencies {
    fn new() -> Self {
        Dependencies {
            toml: "0.2".to_string(),
            serde_derive: "1.0".to_string(),
            serde: "1.0".to_string(),
            indexmap: "1.0.2".to_string(),
            offbin: Offbin {
                git: "https://github.com/offscale/offbin.git".to_string(),
            },
            cargo_make: CargoMake {
                git: "https://github.com/offscale/cargo-make.git".to_string(),
                branch: "offbin".to_string(),
            },
        }
    }
}

impl Package {
    fn new(name: &str, version: &str) -> Self {
        Package {
            name: name.to_string(),
            version: version.to_string(),
            edition: "2018".to_string(),
        }
    }
}

impl CargoToml {
    pub fn new(name: &str) -> Self {
        let package = Package::new(name, "0.1.0");

        let dependencies = Dependencies::new();

        CargoToml {
            package: package,
            dependencies: dependencies,
            //bin: vec![Bin::new(name)],
        }
    }

    fn to_toml(&self) -> String {
        toml::to_string(&self).unwrap()
    }

    pub fn to_file(&self, filename: String) {
        let mut file = File::create(filename).expect("file path not valid");
        file.write_all(self.to_toml().as_bytes())
            .expect("could not write to file");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialise() {
        let cargo_toml = CargoToml::new("woop");
        println!("{}", cargo_toml.to_toml());

        assert_eq!(3, 3);
    }

}
