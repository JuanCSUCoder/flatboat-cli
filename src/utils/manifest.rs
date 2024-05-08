use super::constants::BASE_URL;

fn search_manifest_options(manifest_locator: Option<String>) -> Vec<String> {
    let manifest = match manifest_locator {
        Some(m) => m,
        None => "".to_owned(),
    };
    // Final result: "JuanCSUCoder/flatboat-templates/develop/templates/humble_nogpu.toml"

    let mut options = vec![
        "JuanCSUCoder/flatboat-templates/develop/templates/".to_owned() + &manifest + ".toml",
        "JuanCSUCoder/flatboat-templates/develop/".to_owned() + &manifest + ".toml",
        "JuanCSUCoder/flatboat-templates/".to_owned() + &manifest + ".toml",
        "JuanCSUCoder/".to_owned() + &manifest + ".toml",
        manifest + ".toml",
    ];

    for option in options.iter_mut() {
        *option = BASE_URL.to_owned() + &option;
    }

    return options;
}

fn search_manifest(manifest_locator: Option<String>) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_options() {
        let opts = search_manifest_options(Some("humble_nogpu".to_owned()));

        let expected_str = vec![
            "https://raw.githubusercontent.com/JuanCSUCoder/flatboat-templates/develop/templates/humble_nogpu.toml", "https://raw.githubusercontent.com/JuanCSUCoder/flatboat-templates/develop/humble_nogpu.toml", "https://raw.githubusercontent.com/JuanCSUCoder/flatboat-templates/humble_nogpu.toml", "https://raw.githubusercontent.com/JuanCSUCoder/humble_nogpu.toml", "https://raw.githubusercontent.com/humble_nogpu.toml"];
        
        let mut expected = vec![];

        for exp_str in expected_str {
            expected.push(exp_str.to_owned());
        }
        println!("{:?}", opts);

        for (i, opt) in opts.iter().enumerate() {
            assert_eq!(expected[i], *opt)
        }
    }
}