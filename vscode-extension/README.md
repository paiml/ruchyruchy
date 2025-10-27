# Ruchy Language Support for VS Code

Official Visual Studio Code extension for the Ruchy programming language.

## Features

- **Syntax Highlighting**: Full syntax highlighting for `.ruchy` files
- **Language Server Protocol**: Real-time error checking and diagnostics
- **Code Formatting**: Integration with `ruchy fmt`
- **Syntax Checking**: Integration with `ruchy check`
- **Auto-closing Pairs**: Smart bracket and quote completion
- **Code Folding**: Region-based code folding support

## Requirements

- Visual Studio Code 1.80.0 or higher
- Ruchy compiler installed and available in PATH
- (Optional) Ruchy LSP server for advanced features

## Installation

### From VSIX

1. Download the latest `.vsix` file from releases
2. In VS Code, run: Extensions: Install from VSIX
3. Select the downloaded `.vsix` file

### From Source

```bash
cd vscode-extension
npm install
npm run compile
npm run package
code --install-extension ruchy-*.vsix
```

## Configuration

Configure the extension via VS Code settings:

```json
{
  "ruchy.lsp.path": "ruchylsp",
  "ruchy.trace.server": "off"
}
```

### Settings

- `ruchy.lsp.path`: Path to the Ruchy LSP server binary (default: `ruchylsp`)
- `ruchy.trace.server`: Trace LSP communication (`off`, `messages`, `verbose`)

## Commands

- `Ruchy: Check Syntax` - Run `ruchy check` on current file
- `Ruchy: Format` - Run `ruchy fmt` on current file
- `Ruchy: Hello World` - Test command

## Syntax Highlighting

The extension provides comprehensive syntax highlighting for:

- Keywords: `fun`, `let`, `if`, `else`, `match`, `loop`, `type`, etc.
- Types: `i32`, `u64`, `String`, `bool`, custom types
- Functions: Function definitions and calls
- Strings: Double and single quoted with escape sequences
- Numbers: Decimal, hex, binary, octal
- Comments: Line (`//`) and block (`/* */`)
- Operators: Arithmetic, comparison, logical, assignment

## Language Features

### Auto-closing Pairs

Automatic closing for:
- `{` → `}`
- `[` → `]`
- `(` → `)`
- `"` → `"`
- `'` → `'`

### Code Folding

Use `// #region` and `// #endregion` to define foldable regions.

## Development

### Building

```bash
npm install
npm run compile
```

### Testing

```bash
npm test
```

### Packaging

```bash
npm run package
```

## License

MIT License - See LICENSE file in the root repository.

## Contributing

Contributions welcome! Please see the main repository:
https://github.com/paiml/ruchyruchy

## Support

- GitHub Issues: https://github.com/paiml/ruchyruchy/issues
- Documentation: https://github.com/paiml/ruchyruchy

## Changelog

See CHANGELOG.md in the root repository for version history.
