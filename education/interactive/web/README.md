# RuchyRuchy Interactive Learning Modules

This directory contains the web-based interactive learning modules for RuchyRuchy, implementing the EDUCATION-001 ticket from Phase 4 of the roadmap.

## Module Structure

Each interactive learning module follows a consistent structure:

- `index.html` - Main entry point for the module
- `module.js` - JavaScript logic specific to the module
- `styles.css` - Module-specific styling
- `assets/` - Images, diagrams, and other static assets
- `examples/` - Example code snippets for the module
- `exercises/` - Interactive exercises for practice

## Modules

1. **Tokenization Tutorial**
   - Visual step-by-step guide to lexical analysis
   - Interactive token highlighting
   - Custom tokenizer playground

2. **AST Explorer**
   - Interactive tree visualization
   - Parser step-by-step visualization
   - Source-to-AST mapping

3. **Type Inference Playground**
   - Algorithm W visualization
   - Type constraint graph visualization
   - Type error explanation system

4. **Code Generation Visualizer**
   - Multi-target code generation comparison
   - Optimization visualization
   - Performance impact estimation

## Shared Components

Common components used across all modules are in the `shared/` directory:

- `shared/components/` - Reusable UI components
- `shared/styles/` - Common styling and themes
- `shared/utils/` - Utility functions and helpers
- `shared/compiler/` - Integration with Ruchy compiler

## Development

To run the modules locally:

1. Navigate to the project root
2. Run `npm run dev:interactive`
3. Open `http://localhost:8080/education/interactive/web/`

## Building

To build the modules for production:

1. Run `npm run build:interactive`
2. The built files will be in `dist/education/interactive/web/`

## Integration

These modules integrate with the main RuchyRuchy website and documentation. They can be embedded in:

- Documentation pages
- Tutorial sections
- Online courses
- Workshops and training materials

## Contributing

When adding new features or modules:

1. Follow the established pattern and directory structure
2. Maintain consistent styling and interaction patterns
3. Ensure all modules are responsive and accessible
4. Add appropriate tests for interactive elements
5. Update the module documentation

## License

This educational content is licensed under the terms specified in the repository's LICENSE file.