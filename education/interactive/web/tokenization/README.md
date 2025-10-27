# Interactive Tokenization Tutorial

## Overview

This interactive web-based tutorial teaches the fundamentals of lexical analysis (tokenization) using the Ruchy programming language. Students can type Ruchy code and see it tokenized in real-time, learning how the lexer breaks source code into meaningful tokens.

## Features

- **Real-time Tokenization**: Type code and instantly see the token stream
- **Syntax Highlighting**: Tokens are color-coded by type (keywords, identifiers, literals, operators, delimiters)
- **Position Tracking**: Each token shows its line and column position
- **Statistics Dashboard**: View counts of different token types
- **Example Programs**: Pre-loaded examples demonstrating various language features
- **Educational Content**: Learn key concepts like maximal munch, lookahead, and error recovery
- **Responsive Design**: Works on desktop, tablet, and mobile devices

## Files

- `index.html` - Main HTML structure
- `styles.css` - Responsive CSS styling (~400 lines)
- `tokenizer.js` - JavaScript lexer implementation (~450 lines)
- `README.md` - This documentation file

## Token Types

The tutorial implements the following token types matching Ruchy's lexer:

### Keywords (18)
`fun`, `let`, `if`, `else`, `match`, `loop`, `while`, `for`, `return`, `break`, `continue`, `in`, `struct`, `enum`, `trait`, `impl`, `type`, `true`, `false`

### Literals
- **Identifiers**: Variable and function names
- **Numbers**: Integer and floating-point literals
- **Strings**: String literals with escape sequences

### Operators
`+`, `-`, `*`, `/`, `%`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `&&`, `||`, `!`, `=`

### Delimiters
`(`, `)`, `{`, `}`, `[`, `]`, `,`, `;`, `:`, `->`, `.`

## Usage

### Local Development

1. Open `index.html` in a web browser:
   ```bash
   open education/interactive/web/tokenization/index.html
   ```

2. Or use a local HTTP server:
   ```bash
   cd education/interactive/web/tokenization
   python3 -m http.server 8000
   ```
   Then navigate to `http://localhost:8000`

### Integration with Ruchy Compiler

The JavaScript lexer in `tokenizer.js` is designed to match the behavior of the actual Ruchy lexer. For production use, you can integrate this with the Ruchy compiler via:

1. **WebAssembly**: Compile Ruchy's lexer to WASM and call it from JavaScript
2. **HTTP API**: Call a Ruchy compiler endpoint for tokenization
3. **Educational Mode**: Use the JS implementation for teaching purposes

## Example Programs

The tutorial includes 4 pre-loaded examples:

1. **Hello World**: Basic function and string printing
2. **Variables**: Variable declarations and literals
3. **Functions**: Function definitions and calls
4. **Loops**: For loops and while loops

## Educational Concepts

The tutorial teaches:

- **Maximal Munch**: Longest match principle (e.g., `==` vs `=`)
- **Lookahead**: Peeking ahead to determine token boundaries
- **Keywords vs Identifiers**: How keywords are distinguished
- **Error Recovery**: How lexers handle invalid characters

## Customization

### Adding New Examples

Edit `tokenizer.js` and add to the `examples` object:

```javascript
const examples = {
    myExample: `fun my_func() {
        // Your code here
    }`
};
```

### Styling

Modify `styles.css` to change colors, layout, or responsive breakpoints. CSS custom properties (variables) are defined at the top for easy theming:

```css
:root {
    --primary-color: #2563eb;
    --secondary-color: #7c3aed;
    /* ... */
}
```

### Token Types

Add new token types in `tokenizer.js`:

```javascript
const TokenType = {
    MY_NEW_TOKEN: 'my-category',
    // ...
};
```

Then add corresponding CSS styling in `styles.css`:

```css
.my-category-token {
    background: #color;
    color: #text-color;
}
```

## Browser Compatibility

- **Chrome/Edge**: ✅ Fully supported
- **Firefox**: ✅ Fully supported
- **Safari**: ✅ Fully supported
- **Mobile browsers**: ✅ Responsive design

## Accessibility

- Semantic HTML5 elements
- ARIA labels where appropriate
- Keyboard navigation support
- High contrast color scheme
- Readable font sizes

## Future Enhancements

- [ ] Character-by-character step-through animation
- [ ] Lexer state machine visualization
- [ ] Performance metrics (tokens/second)
- [ ] Error highlighting with suggestions
- [ ] Export tokens as JSON
- [ ] Integration with AST Explorer tutorial
- [ ] Comparison with other languages' lexers
- [ ] Quiz system with scoring

## Related Tutorials

Part of the RuchyRuchy Educational Platform:

- [AST Explorer](../ast-explorer/) - Parse tree visualization
- [Type Inference Playground](../type-inference/) - Algorithm W step-through
- [Code Generation Visualizer](../code-generation/) - Multi-target code gen

## Contributing

This tutorial is part of the RuchyRuchy project. To contribute:

1. Follow the Extreme TDD workflow (RED-GREEN-REFACTOR)
2. Ensure all changes pass quality gates
3. Update this README with new features
4. Add validation tests in `validation/education/`

## License

Part of the RuchyRuchy Bootstrap Compiler project.

## Credits

- Lexer algorithm based on the Ruchy compiler's Stage 0 lexer
- UI design inspired by modern educational platforms
- Built with vanilla HTML, CSS, and JavaScript (no framework dependencies)
