# Mandy - Markdown to PDF Converter

A Windows application that adds a right-click context menu option to convert Markdown files to PDF.

## Features

- Right-click context menu integration for .md files
- Customizable PDF styling via config.json
- Syntax highlighting for code blocks
- Image support with captions
- Headless browser conversion using Edge or Chrome

## Building

### Prerequisites

- Rust (latest stable version)
- Windows 10/11
- Microsoft Edge or Google Chrome installed

### Build Instructions

1. Clone or download this repository

2. Build both executables:

```bash
cargo build --release
```

This will create two executables in `target/release/`:

- `mandy-installer.exe` - The installer
- `mandy-converter.exe` - The actual converter

3. Copy both executables to the same folder for distribution

## Installation

1. **Run as Administrator**: Right-click `mandy-installer.exe` and select "Run as administrator"

2. Choose option **1** to install

3. The installer will:
   - Copy the converter to `C:\Program Files\Mandy\`
   - Create a default config.json file
   - Add a right-click context menu option for .md files

## Usage

After installation:

1. Right-click any `.md` file in File Explorer
2. Select **"Convert to PDF with Mandy"**
3. The PDF will be created in the same directory as the markdown file

## Configuration

Edit the config file at `C:\Program Files\Mandy\config.json` to customize:

- Page margins
- Font families and sizes
- Heading styles
- Code block appearance
- Syntax highlighting theme
- Image caption styling
- And more...

## Uninstallation

1. Run `mandy-installer.exe` as Administrator
2. Choose option **2** to uninstall

This will remove:

- All installed files from `C:\Program Files\Mandy\`
- The context menu entry

## Troubleshooting

**"This installer requires administrator privileges"**

- Right-click the installer and select "Run as administrator"

**"Could not find Edge or Chrome"**

- Install Microsoft Edge or Google Chrome
- The converter requires one of these browsers for PDF generation

**Context menu doesn't appear**

- Make sure you ran the installer as Administrator
- Try logging out and back in to Windows
- Check that the file extension is `.md`

## Technical Details

- Installation directory: `C:\Program Files\Mandy\`
- Registry key: `HKEY_CLASSES_ROOT\SystemFileAssociations\.md\shell\Mandy`
- Config location: `C:\Program Files\Mandy\config.json`
