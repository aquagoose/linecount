use std::{fmt::Display, collections::HashMap, fs, ffi::OsString, path::Path};

use clap::Parser;
use toml::Value;

#[derive(Parser)]
struct CliArgs {
    directory: OsString,

    #[arg(long = "ignore-dir", short = 'i')]
    ignored_dirs: Option<Vec<String>>,

    #[arg(long, default_value_t = false)]
    count_comments: bool,

    #[arg(long, default_value_t = false)]
    count_whitespace: bool,

    #[arg(long, default_value_t = false)]
    count_unknown: bool,

    #[arg(long)]
    config: Option<OsString>
}

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
enum Language
{
    Unknown(String),

    C,
    CPlusPlus,
    CSharp,

    FSharp,

    Rust,

    Python,

    JavaScript,
    TypeScript,
    Html,
    Css,
    Sass,
    Cshtml,

    Java,

    Swift,

    Json,
    Xml,
    Ini,
    Toml,
    Yaml,

    Sln,
    Csproj,

    Text,
    Markdown,
    Gitignore,

    Glsl,
    Hlsl,
    Wgsl,
    Msl,
    Cg,

    Powershell,
    Batch,
    Cmd,
    Sh,

    Assembly
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Language::Unknown(language) => language,
            Language::C => "C",
            Language::CPlusPlus => "C++",
            Language::CSharp => "C#",
            Language::FSharp => "F#",
            Language::Rust => "Rust",
            Language::Sln => "Visual Studio Solution",
            Language::Csproj => "C# Project",
            Language::Python => "Python",
            Language::JavaScript => "JavaScript",
            Language::TypeScript => "TypeScript",
            Language::Json => "JSON",
            Language::Xml => "XML",
            Language::Ini => "INI",
            Language::Toml => "TOML",
            Language::Yaml => "YAML",
            Language::Text => "Text",
            Language::Html => "HTML",
            Language::Css => "CSS",
            Language::Sass => "Sass",
            Language::Markdown => "Markdown",
            Language::Gitignore => ".gitignore",
            Language::Cshtml => "Razor page",
            Language::Glsl => "GLSL",
            Language::Hlsl => "HLSL",
            Language::Java => "Java",
            Language::Wgsl => "WebGPU Shading Language",
            Language::Msl => "Metak Shading Language",
            Language::Cg => "CG",
            Language::Powershell => "Powershell Script",
            Language::Batch => "Batch File",
            Language::Cmd => "CMD Script",
            Language::Sh => "Bash Script",
            Language::Assembly => "Assembly",
            Language::Swift => "Swift"
        };

        write!(f, "{text}")
    }
}

fn extension_to_language(extension: &str) -> Language
{
    match extension.to_lowercase().as_str() {
        "c" => Language::C,
        "h" => Language::C, // TODO: Probably should make header files a bit clever to check if its a C/CPP header

        "cpp" => Language::CPlusPlus,
        "hpp" => Language::CPlusPlus,
        "tcc" => Language::CPlusPlus,
        "cxx" => Language::CPlusPlus,
        "hxx" => Language::CPlusPlus,

        "cs" => Language::CSharp,

        "fs" => Language::FSharp,

        "rs" => Language::Rust,

        "py" => Language::Python,

        "js" => Language::JavaScript,
        "ts" => Language::TypeScript,

        "html" => Language::Html,
        "htm" => Language::Html,

        "css" => Language::Css,
        "sass" => Language::Sass,

        "cshtml" => Language::Cshtml,

        "java" => Language::Java,

        "swift" => Language::Swift,

        "json" => Language::Json,
        "xml" => Language::Xml,
        "ini" => Language::Ini,
        "toml" => Language::Toml,

        "yaml" => Language::Yaml,
        "yml" => Language::Yaml,

        "sln" => Language::Sln,
        "csproj" => Language::Csproj,

        "txt" => Language::Text,
        "md" => Language::Markdown,
        "gitignore" => Language::Gitignore,

        "glsl" => Language::Glsl,
        "vert" => Language::Glsl,
        "frag" => Language::Glsl,
        "tesc" => Language::Glsl,
        "tese" => Language::Glsl,
        "geom" => Language::Glsl,
        "comp" => Language::Glsl,

        "hlsl" => Language::Hlsl,
        "fx" => Language::Hlsl,

        "wgsl" => Language::Wgsl,
        "msl" => Language::Msl,
        "cg" => Language::Cg,

        "ps1" => Language::Powershell,
        "bat" => Language::Batch,
        "cmd" => Language::Cmd,
        "sh" => Language::Sh,

        "asm" => Language::Assembly,
        
        ext => Language::Unknown(ext.to_string())
    }
}

fn recurse_directory(dir: OsString, paths: &mut Vec<OsString>, ignored_dirs: &Option<Vec<String>>) {
    let files = fs::read_dir(&dir).unwrap();

    for file in files {
        let file = file.unwrap();

        let file_type = file.file_type().unwrap();

        if file_type.is_dir() {
            if let Some(ignored_dirs) = ignored_dirs {
                if ignored_dirs.contains(&file.file_name().into_string().unwrap()) {
                    continue;
                }
            }
            recurse_directory(file.path().into_os_string(), paths, ignored_dirs);
            continue;
        }

        paths.push(file.path().into_os_string())
    }
}

fn main() {
    let args = CliArgs::parse();
    let mut paths = Vec::new();

    let mut custom_langs: HashMap<String, String> = HashMap::new();

    if let Some(config) = args.config {
        if let Ok(cust) = fs::read_to_string(config) {
            let cust = cust.parse::<Value>().unwrap();
            let cust = cust.as_table().unwrap();

            for (key, value) in cust {
                let lang_name = key;
                let extensions = value["extensions"].as_array().unwrap();
                for extension in extensions.into_iter() {
                    custom_langs.insert(extension.as_str().unwrap().to_string(), lang_name.clone());
                }
            }
        }
    }

    recurse_directory(args.directory, &mut paths, &args.ignored_dirs);

    let mut languages = HashMap::new();

    for path in paths {
        let path = path.as_os_str();
        let extension = Path::new(path).extension();
        if extension.is_none() {
            continue;
        }
        let extension = extension.unwrap();

        let mut language = extension_to_language(extension.to_str().unwrap());
        if let Language::Unknown(ext) = &language {
            if !custom_langs.contains_key(ext) && !args.count_unknown {
                continue;
            } else if let Some(custom_lang) = custom_langs.get(ext) {
                language = Language::Unknown(custom_lang.to_string());
            } else {
                language = Language::Unknown(format!("Unknown (.{ext})"));
            }
        }

        println!("Counting {}...", path.to_str().unwrap());

        let mut count = 0;

        let file = fs::read_to_string(path);
        if file.is_err() {
            continue;
        }
        let file = file.unwrap();

        for line in file.lines() {
            let line = line.trim();

            if line == "" && !args.count_whitespace {
                continue;
            } else if !args.count_comments && (line.starts_with("//") || line.starts_with("#")) {
                continue;
            }

            count += 1;
        }

        languages.entry(language).and_modify(|total| *total += count).or_insert(count);
    }

    let mut languages: Vec<(Language, i32)> = languages.iter().map(|value| (value.0.clone(), *value.1)).collect();
    languages.sort_by_key(|value| value.0.to_string());

    const LANGUAGES_NAME: &str = "Languages";
    const LINES_NAME: &str = "Lines";

    let mut length_names = LANGUAGES_NAME.len();
    let mut length_lines = LINES_NAME.len();

    for (language, total) in languages.iter() {
        let lang_length = language.to_string().len();
        if lang_length > length_names {
            length_names = lang_length;
        }

        let lines_length = f32::log10((total + 1) as f32).ceil() as usize;
        if lines_length > length_lines {
            length_lines = lines_length;
        }
    }
    
    const PADDING: usize = 2;

    println!();

    print!("| {LANGUAGES_NAME}");

    for _ in LANGUAGES_NAME.len()..length_names {
        print!(" ");
    }

    print!(" | {LINES_NAME}");
    for _ in LINES_NAME.len()..length_lines {
        print!(" ");
    }

    println!(" |");

    print!("|");
    for _ in 0..length_names + PADDING {
        print!("-");
    }
    print!("|");
    for _ in 0..length_lines + PADDING {
        print!("-");
    }
    println!("|");

    for (language, total) in languages.iter() {
        let language = language.to_string();
        let total = total.to_string();

        print!("| {language}");

        for _ in language.len()..length_names {
            print!(" ");
        }

        print!(" | {total}");
        for _ in total.len()..length_lines {
            print!(" ");
        }

        println!(" |");
    }

    println!();
}
