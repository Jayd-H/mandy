use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct Config {
    page: PageConfig,
    fonts: FontsConfig,
    headings: HeadingsConfig,
    spacing: SpacingConfig,
    code_blocks: CodeBlocksConfig,
    syntax_highlighting: SyntaxHighlightingConfig,
    images: ImagesConfig,
    title_page: TitlePageConfig,
}

#[derive(Debug, Deserialize)]
struct PageConfig {
    margin: String,
    first_page_top_margin: String,
}

#[derive(Debug, Deserialize)]
struct FontsConfig {
    body_family: String,
    body_size: String,
    code_family: String,
    inline_code_size: String,
    block_code_size: String,
}

#[derive(Debug, Deserialize)]
struct HeadingsConfig {
    h1_size: String,
    h1_align: String,
    h1_page_break_before: bool,
    h2_size: String,
    h2_page_break_before: bool,
    h3_size: String,
    h4_size: String,
    h5_size: String,
    h6_size: String,
}

#[derive(Debug, Deserialize)]
struct SpacingConfig {
    line_height: String,
    paragraph_margin: String,
    h1_bottom_margin: String,
    h2_bottom_margin: String,
    h3_margins: String,
    h4_margins: String,
    h5_margins: String,
    h6_margins: String,
}

#[derive(Debug, Deserialize)]
struct CodeBlocksConfig {
    background_color: String,
    border: String,
    padding: String,
    margin: String,
    word_wrap: bool,
    page_break_inside: bool,
}

#[derive(Debug, Deserialize)]
struct SyntaxHighlightingConfig {
    theme: String,
    enabled: bool,
    text_color: String,
}

#[derive(Debug, Deserialize)]
struct ImagesConfig {
    show_captions: bool,
    caption_size: String,
    caption_style: String,
    caption_align: String,
    caption_color: String,
}

#[derive(Debug, Deserialize)]
struct TitlePageConfig {
    extract_header: bool,
    first_paragraph_size: String,
}

fn load_config() -> Config {
    let exe_path = env::current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get executable directory");
    let config_path = exe_dir.join("config.json");

    if config_path.exists() {
        let config_content = fs::read_to_string(&config_path).expect("Failed to read config.json");
        match serde_json::from_str(&config_content) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Error parsing config.json: {}", e);
                eprintln!("Using default configuration instead.");
                get_default_config()
            }
        }
    } else {
        eprintln!("Warning: config.json not found, using default configuration");
        get_default_config()
    }
}

fn get_default_config() -> Config {
    serde_json::from_str(
        r##"{
        "page": {
            "margin": "1in",
            "first_page_top_margin": "2in"
        },
        "fonts": {
            "body_family": "Times New Roman",
            "body_size": "12pt",
            "code_family": "Courier New",
            "inline_code_size": "12pt",
            "block_code_size": "9pt"
        },
        "headings": {
            "h1_size": "24pt",
            "h1_align": "center",
            "h1_page_break_before": true,
            "h2_size": "16pt",
            "h2_page_break_before": true,
            "h3_size": "14pt",
            "h4_size": "13pt",
            "h5_size": "12pt",
            "h6_size": "12pt"
        },
        "spacing": {
            "line_height": "1.25",
            "paragraph_margin": "12pt",
            "h1_bottom_margin": "12pt",
            "h2_bottom_margin": "16pt",
            "h3_margins": "24pt 0 12pt 0",
            "h4_margins": "20pt 0 10pt 0",
            "h5_margins": "16pt 0 8pt 0",
            "h6_margins": "16pt 0 8pt 0"
        },
        "code_blocks": {
            "background_color": "transparent",
            "border": "none",
            "padding": "0",
            "margin": "6pt 0",
            "word_wrap": true,
            "page_break_inside": false
        },
        "syntax_highlighting": {
            "theme": "monokai",
            "enabled": true,
            "text_color": "#333"
        },
        "images": {
            "show_captions": true,
            "caption_size": "10pt",
            "caption_style": "italic",
            "caption_align": "center",
            "caption_color": "#666"
        },
        "title_page": {
            "extract_header": true,
            "first_paragraph_size": "16pt"
        }
    }"##,
    )
    .expect("Failed to parse default config")
}

fn main() {
    let config = load_config();

    let args: Vec<String> = env::args().collect();

    let md_path = if args.len() < 2 {
        println!("Enter the path to the markdown file:");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input.trim().to_string()
    } else {
        args[1].clone()
    };

    if !Path::new(&md_path).exists() {
        eprintln!("Error: File '{}' does not exist", md_path);
        std::process::exit(1);
    }

    let markdown_content = fs::read_to_string(&md_path).expect("Failed to read markdown file");

    let (header_text, processed_markdown) = if config.title_page.extract_header {
        extract_header(&markdown_content)
    } else {
        (String::new(), markdown_content)
    };

    let html = markdown_to_html(&processed_markdown);

    let full_html = generate_html(&config, &header_text, &html);

    let temp_html_path = Path::new(&md_path).with_extension("temp.html");
    fs::write(&temp_html_path, full_html).expect("Failed to write temporary HTML file");

    let output_pdf_path = Path::new(&md_path).with_extension("pdf");

    let edge_paths = vec![
        r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
        r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
    ];

    let chrome_paths = vec![
        r"C:\Program Files\Google\Chrome\Application\chrome.exe",
        r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
    ];

    let mut browser_path = None;
    for path in edge_paths.iter().chain(chrome_paths.iter()) {
        if Path::new(path).exists() {
            browser_path = Some(path);
            break;
        }
    }

    let browser_path = browser_path
        .expect("Could not find Edge or Chrome. Please install Microsoft Edge or Chrome.");

    let temp_html_absolute =
        fs::canonicalize(&temp_html_path).expect("Failed to get absolute path for HTML file");

    let output_pdf_absolute = fs::canonicalize(Path::new(&md_path).parent().unwrap())
        .expect("Failed to get absolute path for output directory")
        .join(output_pdf_path.file_name().unwrap());

    println!("Using browser: {}", browser_path);
    println!(
        "Converting: {} -> {}",
        md_path,
        output_pdf_absolute.display()
    );

    let temp_dir = env::temp_dir();
    let user_data_dir = temp_dir.join("mandy_browser_data");

    let temp_html_str = temp_html_absolute.to_string_lossy().to_string();
    let cleaned_path = if temp_html_str.starts_with(r"\\?\") {
        temp_html_str[4..].to_string()
    } else {
        temp_html_str
    };

    let file_url = format!("file:///{}", cleaned_path.replace("\\", "/"));

    println!("File URL: {}", file_url);

    let output = Command::new(browser_path)
        .arg("--headless=new")
        .arg("--disable-gpu")
        .arg("--no-first-run")
        .arg("--no-default-browser-check")
        .arg("--disable-extensions")
        .arg("--disable-background-networking")
        .arg("--no-pdf-header-footer")
        .arg(format!("--user-data-dir={}", user_data_dir.display()))
        .arg(format!("--print-to-pdf={}", output_pdf_absolute.display()))
        .arg(&file_url)
        .output()
        .expect("Failed to execute browser");

    fs::remove_file(&temp_html_path).expect("Failed to remove temporary HTML file");

    if output.status.success() {
        println!(
            "PDF created successfully: {}",
            output_pdf_absolute.display()
        );
    } else {
        eprintln!("Error: Browser failed to generate PDF");
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
}

fn generate_html(config: &Config, header_text: &str, html_content: &str) -> String {
    let h1_page_break = if config.headings.h1_page_break_before {
        "page-break-before: always;"
    } else {
        ""
    };

    let h2_page_break = if config.headings.h2_page_break_before {
        "page-break-before: always;"
    } else {
        ""
    };

    let pre_page_break = if !config.code_blocks.page_break_inside {
        "page-break-inside: avoid;"
    } else {
        ""
    };

    let word_wrap_styles = if config.code_blocks.word_wrap {
        "white-space: pre-wrap; word-wrap: break-word;"
    } else {
        ""
    };

    let syntax_theme = &config.syntax_highlighting.theme;
    let syntax_link = if config.syntax_highlighting.enabled {
        format!(
            r#"<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/{}.min.css">"#,
            syntax_theme
        )
    } else {
        String::new()
    };

    let image_caption_script = if config.images.show_captions {
        format!(
            r#"
            document.querySelectorAll('img').forEach((img) => {{
                if (img.alt) {{
                    const figure = document.createElement('figure');
                    figure.style.margin = '12pt 0';
                    figure.style.pageBreakInside = 'avoid';
                    
                    img.parentNode.insertBefore(figure, img);
                    figure.appendChild(img);
                    
                    const caption = document.createElement('figcaption');
                    caption.textContent = img.alt;
                    caption.style.fontSize = '{}';
                    caption.style.fontStyle = '{}';
                    caption.style.textAlign = '{}';
                    caption.style.marginTop = '6pt';
                    caption.style.color = '{}';
                    figure.appendChild(caption);
                }}
            }});"#,
            config.images.caption_size,
            config.images.caption_style,
            config.images.caption_align,
            config.images.caption_color
        )
    } else {
        String::new()
    };

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    {}
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"></script>
    <script>
        document.addEventListener('DOMContentLoaded', (event) => {{
            document.querySelectorAll('pre code').forEach((block) => {{
                hljs.highlightBlock(block);
            }});
            {}
        }});
    </script>
    <style>
        @page {{
            margin: {};
        }}
        
        @page :first {{
            margin-top: {};
            margin-bottom: 1in;
            margin-left: 1in;
            margin-right: 1in;
            @top-center {{
                content: "{}";
                font-family: '{}', serif;
                font-size: {};
            }}
            @bottom-right {{
                content: none;
            }}
        }}
        
        @page {{
            @bottom-right {{
                content: counter(page);
                font-family: '{}', serif;
                font-size: {};
            }}
        }}
        
        body {{
            font-family: '{}', serif;
            line-height: {};
            font-size: {};
            counter-reset: page 1;
        }}
        
        h1 {{
            font-size: {};
            font-weight: bold;
            margin: 0 0 {} 0;
            padding: 0;
            {}
            text-align: {};
        }}
        
        h1:first-of-type {{
            page-break-before: avoid;
        }}
        
        h1:first-of-type + p {{
            font-size: {};
        }}
        
        .h1-page {{
            margin-top: {};
        }}
        
        h2 {{
            font-size: {};
            font-weight: bold;
            margin: 0 0 {} 0;
            padding: 0;
            {}
        }}
        
        h3 {{
            font-size: {};
            font-weight: bold;
            margin: {};
        }}
        
        h4 {{
            font-size: {};
            font-weight: bold;
            margin: {};
        }}
        
        h5 {{
            font-size: {};
            font-weight: bold;
            margin: {};
        }}
        
        h6 {{
            font-size: {};
            font-weight: bold;
            font-style: italic;
            margin: {};
        }}
        
        p {{
            margin: {};
        }}
        
        code {{
            font-family: '{}', monospace;
            font-size: {};
            background-color: {} !important;
            padding: {};
            color: {} !important;
        }}
        
        pre {{
            font-family: '{}', monospace;
            font-size: {};
            background-color: {} !important;
            padding: {};
            margin: {};
            border: {};
            {}
            {}
        }}
        
        pre code {{
            font-size: {};
            background-color: {} !important;
            padding: 0;
            {}
            color: {} !important;
        }}
        
        .hljs {{
            background-color: {} !important;
            color: {} !important;
        }}
        
        .hljs-keyword, .hljs-selector-tag, .hljs-literal, .hljs-section, .hljs-link {{
            color: #0000ff !important;
        }}
        
        .hljs-string, .hljs-title, .hljs-name, .hljs-type, .hljs-attribute, .hljs-symbol, .hljs-bullet, .hljs-built_in, .hljs-addition, .hljs-variable, .hljs-template-tag, .hljs-template-variable {{
            color: #d73a49 !important;
        }}
        
        .hljs-comment, .hljs-quote, .hljs-deletion, .hljs-meta {{
            color: #6a737d !important;
        }}
        
        .hljs-number {{
            color: #005cc5 !important;
        }}
        
        blockquote {{
            border-left: 3px solid #ccc;
            padding-left: 12pt;
            margin-left: 0;
            margin: 12pt 0;
            color: #666;
        }}
        
        ul, ol {{
            margin: 12pt 0;
            padding-left: 24pt;
        }}
        
        li {{
            margin: 6pt 0;
        }}
        
        table {{
            border-collapse: collapse;
            width: 100%;
            margin: 12pt 0;
        }}
        
        th, td {{
            border: 1px solid #000;
            padding: 6pt;
            text-align: left;
        }}
        
        th {{
            font-weight: bold;
            background-color: #f5f5f5;
        }}
        
        img {{
            max-width: 100%;
            height: auto;
            display: block;
            margin: 0 auto;
        }}
        
        figure {{
            margin: 12pt 0;
            page-break-inside: avoid;
        }}
    </style>
</head>
<body>
{}
</body>
</html>"#,
        syntax_link,
        image_caption_script,
        config.page.margin,
        config.page.first_page_top_margin,
        header_text,
        config.fonts.body_family,
        config.fonts.body_size,
        config.fonts.body_family,
        config.fonts.body_size,
        config.fonts.body_family,
        config.spacing.line_height,
        config.fonts.body_size,
        config.headings.h1_size,
        config.spacing.h1_bottom_margin,
        h1_page_break,
        config.headings.h1_align,
        config.title_page.first_paragraph_size,
        config.page.first_page_top_margin,
        config.headings.h2_size,
        config.spacing.h2_bottom_margin,
        h2_page_break,
        config.headings.h3_size,
        config.spacing.h3_margins,
        config.headings.h4_size,
        config.spacing.h4_margins,
        config.headings.h5_size,
        config.spacing.h5_margins,
        config.headings.h6_size,
        config.spacing.h6_margins,
        config.spacing.paragraph_margin,
        config.fonts.code_family,
        config.fonts.inline_code_size,
        config.code_blocks.background_color,
        config.code_blocks.padding,
        config.syntax_highlighting.text_color,
        config.fonts.code_family,
        config.fonts.block_code_size,
        config.code_blocks.background_color,
        config.code_blocks.padding,
        config.code_blocks.margin,
        config.code_blocks.border,
        word_wrap_styles,
        pre_page_break,
        config.fonts.block_code_size,
        config.code_blocks.background_color,
        word_wrap_styles,
        config.syntax_highlighting.text_color,
        config.code_blocks.background_color,
        config.syntax_highlighting.text_color,
        html_content
    )
}

fn extract_header(markdown: &str) -> (String, String) {
    let lines: Vec<&str> = markdown.lines().collect();

    if lines.len() >= 2 {
        let first_line = lines[0].trim();
        let second_line = lines[1].trim();

        if second_line.chars().all(|c| c == '-') && second_line.len() >= 3 {
            let header_text = first_line.to_string();
            let remaining_markdown = lines[2..].join("\n");
            return (header_text, remaining_markdown);
        }
    }

    (String::new(), markdown.to_string())
}

fn markdown_to_html(markdown: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}