use crate::utils::constants::BASE_URL;

pub fn manifest_locations(manifest: String) -> Vec<String> {
    // Final result: "JuanCSUCoder/flatboat-templates/develop/templates/humble_nogpu.toml"

    let mut options = vec![
        "JuanCSUCoder/flatboat-templates/main/templates/".to_owned() + &manifest + ".toml",
        "JuanCSUCoder/flatboat-templates/main/".to_owned() + &manifest + ".toml",
        "JuanCSUCoder/flatboat-templates/".to_owned() + &manifest + ".toml",
        "JuanCSUCoder/".to_owned() + &manifest + ".toml",
        manifest.clone() + ".toml",
        manifest.clone(),
    ];

    for option in options.iter_mut() {
        *option = BASE_URL.to_owned() + &option;
    }

    options.push(manifest);

    return options;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_options() {
        let opts = manifest_locations("humble_nogpu".to_owned());

        let expected_str = vec![
            "https://raw.githubusercontent.com/JuanCSUCoder/flatboat-templates/main/templates/humble_nogpu.toml", "https://raw.githubusercontent.com/JuanCSUCoder/flatboat-templates/main/humble_nogpu.toml", "https://raw.githubusercontent.com/JuanCSUCoder/flatboat-templates/humble_nogpu.toml", "https://raw.githubusercontent.com/JuanCSUCoder/humble_nogpu.toml", "https://raw.githubusercontent.com/humble_nogpu.toml", "https://raw.githubusercontent.com/humble_nogpu", "humble_nogpu"];
        
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