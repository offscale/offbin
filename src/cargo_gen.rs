use toml::*;

#[derive(Clone, Debug, Serialize)]
struct CargoToml {
    package: Package,
    dependencies: Dependencies,
    bin: Vec<Bin>,
}

#[derive(Clone, Debug, Serialize)]
struct Package {
    name: String,
    version: String,
    edition: String,
    build: String,
}

#[derive(Clone, Debug, Serialize)]
struct Dependencies {
    toml: String,
    serde_derive: String,
    serde: String,
    clap: String,
}

#[derive(Clone, Debug, Serialize)]
struct Bin {
    name: String,
}


impl Bin {
    fn new(name: &str) -> Self {
        Bin {
            name: name.to_string(),
        }
    }
}

impl Dependencies {
    fn new() -> Self {
        Dependencies {
            toml: "0.2".to_string(),
            serde_derive: "1.0".to_string(),
            serde: "1.0".to_string(),
            clap: "*".to_string()
        }
    }
}

impl Package {
    fn new(name: &str, version: &str) -> Self {
        Package {
            name: name.to_string(),
            version: version.to_string(),
            edition: "2018".to_string(),
            build: "build.rs".to_string(),
        }
    }
}

impl CargoToml {
    fn new(name: &str) -> Self {

        let package = Package::new(
            name,
            "0.1.0");

        let dependencies =  Dependencies::new();

        CargoToml {
            package: package,
            dependencies: dependencies,
            bin: vec![Bin::new(name)],
        }
    }

    fn to_toml(&self) -> String {
        toml::to_string( &self).unwrap()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialise() {
        let cargo_toml = CargoToml::new("woop");
        println!("{}", cargo_toml.to_toml());

        assert_eq!(4, 3);
    }
}
