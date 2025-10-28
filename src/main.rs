use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

const INSTALL_DIR: &str = r"C:\Program Files\Mandy";
const CONVERTER_EXE: &str = "mandy-converter.exe";
const CONFIG_FILE: &str = "config.json";

const DEFAULT_CONFIG: &str = r##"{
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
}"##;

fn main() {
    if !is_admin() {
        eprintln!("Error: This installer requires administrator privileges.");
        eprintln!("Please run this program as Administrator.");
        pause();
        std::process::exit(1);
    }

    println!("╔══════════════════════════════════════╗");
    println!("║   Mandy MD to PDF Converter Setup   ║");
    println!("╚══════════════════════════════════════╝");
    println!();
    println!("1. Install Mandy");
    println!("2. Uninstall Mandy");
    println!("3. Exit");
    println!();
    print!("Choose an option (1-3): ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => install(),
        "2" => uninstall(),
        "3" => {
            println!("Exiting...");
            std::process::exit(0);
        }
        _ => {
            eprintln!("Invalid choice.");
            pause();
            std::process::exit(1);
        }
    }
}

fn install() {
    println!("\n=== Installing Mandy ===\n");

    let install_path = Path::new(INSTALL_DIR);

    if install_path.exists() {
        println!("Mandy is already installed. Updating...");
        if let Err(e) = fs::remove_dir_all(install_path) {
            eprintln!("Error removing old installation: {}", e);
            pause();
            std::process::exit(1);
        }
    }

    if let Err(e) = fs::create_dir_all(install_path) {
        eprintln!("Error creating installation directory: {}", e);
        pause();
        std::process::exit(1);
    }

    let current_exe = env::current_exe().expect("Failed to get current executable path");
    let current_dir = current_exe
        .parent()
        .expect("Failed to get executable directory");

    let converter_source = current_dir.join(CONVERTER_EXE);
    let converter_dest = install_path.join(CONVERTER_EXE);

    if !converter_source.exists() {
        eprintln!(
            "Error: {} not found in the same directory as the installer.",
            CONVERTER_EXE
        );
        eprintln!("Please ensure both files are in the same folder.");
        pause();
        std::process::exit(1);
    }

    if let Err(e) = fs::copy(&converter_source, &converter_dest) {
        eprintln!("Error copying converter executable: {}", e);
        pause();
        std::process::exit(1);
    }
    println!("✓ Copied converter to {}", converter_dest.display());

    let config_dest = install_path.join(CONFIG_FILE);
    if let Err(e) = fs::write(&config_dest, DEFAULT_CONFIG) {
        eprintln!("Error creating config file: {}", e);
        pause();
        std::process::exit(1);
    }
    println!("✓ Created default config.json");

    if let Err(e) = add_context_menu(&converter_dest) {
        eprintln!("Error adding context menu: {}", e);
        pause();
        std::process::exit(1);
    }
    println!("✓ Added right-click context menu");

    println!("\n✓ Installation complete!");
    println!("\nYou can now right-click on any .md file and select");
    println!("'Convert to PDF with Mandy' to convert it.");
    println!("\nConfiguration file location: {}", config_dest.display());
    pause();
}

fn uninstall() {
    println!("\n=== Uninstalling Mandy ===\n");

    let install_path = Path::new(INSTALL_DIR);

    if !install_path.exists() {
        println!("Mandy is not installed.");
        pause();
        return;
    }

    if let Err(e) = remove_context_menu() {
        eprintln!("Warning: Error removing context menu: {}", e);
    } else {
        println!("✓ Removed context menu");
    }

    if let Err(e) = fs::remove_dir_all(install_path) {
        eprintln!("Error removing installation directory: {}", e);
        pause();
        std::process::exit(1);
    }
    println!("✓ Removed installation files");

    println!("\n✓ Uninstallation complete!");
    pause();
}

fn add_context_menu(converter_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);

    let shell_key = hkcr.create_subkey(r"SystemFileAssociations\.md\shell\Mandy")?;
    shell_key.0.set_value("", &"Convert to PDF with Mandy")?;
    shell_key
        .0
        .set_value("Icon", &converter_path.to_string_lossy().to_string())?;

    let command_key = shell_key.0.create_subkey("command")?;
    let command = format!("\"{}\" \"%1\"", converter_path.display());
    command_key.0.set_value("", &command)?;

    Ok(())
}

fn remove_context_menu() -> Result<(), Box<dyn std::error::Error>> {
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);

    let shell_key = hkcr.open_subkey_with_flags(r"SystemFileAssociations\.md\shell", KEY_WRITE)?;
    shell_key.delete_subkey_all("Mandy")?;

    Ok(())
}

fn is_admin() -> bool {
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::Security::{
        GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY,
    };
    use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    unsafe {
        let mut token = windows::Win32::Foundation::HANDLE::default();

        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_err() {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut return_length = 0u32;

        let result = GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut return_length,
        );

        let _ = CloseHandle(token);

        result.is_ok() && elevation.TokenIsElevated != 0
    }
}

fn pause() {
    println!("\nPress Enter to continue...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
