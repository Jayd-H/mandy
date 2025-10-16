use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
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

    let (header_text, processed_markdown) = extract_header(&markdown_content);

    let html = markdown_to_html(&processed_markdown);

    let full_html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/monokai.min.css">
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"></script>
    <script>
        document.addEventListener('DOMContentLoaded', (event) => {{
            document.querySelectorAll('pre code').forEach((block) => {{
                hljs.highlightBlock(block);
            }});
            
            document.querySelectorAll('img').forEach((img) => {{
                if (img.alt) {{
                    const figure = document.createElement('figure');
                    figure.style.margin = '12pt 0';
                    figure.style.pageBreakInside = 'avoid';
                    
                    img.parentNode.insertBefore(figure, img);
                    figure.appendChild(img);
                    
                    const caption = document.createElement('figcaption');
                    caption.textContent = img.alt;
                    caption.style.fontSize = '10pt';
                    caption.style.fontStyle = 'italic';
                    caption.style.textAlign = 'center';
                    caption.style.marginTop = '6pt';
                    caption.style.color = '#666';
                    figure.appendChild(caption);
                }}
            }});
        }});
    </script>
    <style>
        @page {{
            margin: 1in;
        }}
        
        @page :first {{
            margin-top: 2in;
            margin-bottom: 1in;
            margin-left: 1in;
            margin-right: 1in;
            @top-center {{
                content: "{}";
                font-family: 'Times New Roman', serif;
                font-size: 12pt;
            }}
            @bottom-right {{
                content: none;
            }}
        }}
        
        @page {{
            @bottom-right {{
                content: counter(page);
                font-family: 'Times New Roman', serif;
                font-size: 12pt;
            }}
        }}
        
        body {{
            font-family: 'Times New Roman', serif;
            line-height: 1.25;
            font-size: 12pt;
            counter-reset: page 1;
        }}
        
        h1 {{
            font-size: 24pt;
            font-weight: bold;
            margin: 0 0 12pt 0;
            padding: 0;
            page-break-before: always;
            text-align: center;
        }}
        
        h1:first-of-type {{
            page-break-before: avoid;
        }}
        
        h1:first-of-type + p {{
            font-size: 16pt;
        }}
        
        .h1-page {{
            margin-top: 2in;
        }}
        
        h2 {{
            font-size: 16pt;
            font-weight: bold;
            margin: 0 0 16pt 0;
            padding: 0;
            page-break-before: always;
        }}
        
        h3 {{
            font-size: 14pt;
            font-weight: bold;
            margin: 24pt 0 12pt 0;
        }}
        
        h4 {{
            font-size: 13pt;
            font-weight: bold;
            margin: 20pt 0 10pt 0;
        }}
        
        h5 {{
            font-size: 12pt;
            font-weight: bold;
            margin: 16pt 0 8pt 0;
        }}
        
        h6 {{
            font-size: 12pt;
            font-weight: bold;
            font-style: italic;
            margin: 16pt 0 8pt 0;
        }}
        
        p {{
            margin: 0 0 12pt 0;
        }}
        
        code {{
            font-family: 'Courier New', monospace;
            font-size: 12pt;
            background-color: transparent !important;
            padding: 0;
            color: #333 !important;
        }}
        
        pre {{
            font-family: 'Courier New', monospace;
            font-size: 9pt;
            background-color: transparent !important;
            padding: 0;
            margin: 6pt 0;
            border: none;
            white-space: pre-wrap;
            word-wrap: break-word;
            page-break-inside: avoid;
        }}
        
        pre code {{
            font-size: 9pt;
            background-color: transparent !important;
            padding: 0;
            white-space: pre-wrap;
            word-wrap: break-word;
            color: #333 !important;
        }}
        
        .hljs {{
            background-color: transparent !important;
            color: #333 !important;
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
        header_text,
        html
    );

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