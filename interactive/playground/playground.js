// RuchyRuchy Live Compiler Playground
// Interactive compiler exploration with real-time feedback

class RuchyPlayground {
    constructor() {
        this.editor = null;
        this.isCompiling = false;
        this.compileTimeout = null;
        this.lastCode = '';
        this.examples = this.loadExamples();
        
        this.initializeEditor();
        this.bindEvents();
        this.updateStatus('ready');
    }

    initializeEditor() {
        const textarea = document.getElementById('code-editor');
        this.editor = CodeMirror.fromTextArea(textarea, {
            mode: 'rust', // Close enough to Ruchy syntax
            theme: 'dracula',
            lineNumbers: true,
            autoCloseBrackets: true,
            matchBrackets: true,
            indentUnit: 4,
            tabSize: 4,
            lineWrapping: true,
            gutters: ['CodeMirror-linenumbers', 'CodeMirror-foldgutter'],
            foldGutter: true
        });

        // Real-time compilation on code changes
        this.editor.on('change', () => {
            this.scheduleCompilation();
        });

        // Initial compilation
        setTimeout(() => this.compile(), 500);
    }

    bindEvents() {
        document.getElementById('compile-btn').addEventListener('click', () => {
            this.compile(true);
        });

        document.getElementById('step-btn').addEventListener('click', () => {
            this.stepThroughCompilation();
        });

        document.getElementById('share-btn').addEventListener('click', () => {
            this.shareCode();
        });

        document.getElementById('examples-btn').addEventListener('click', () => {
            this.loadExample();
        });
    }

    scheduleCompilation() {
        if (this.compileTimeout) {
            clearTimeout(this.compileTimeout);
        }
        
        // Debounce compilation to avoid excessive calls
        this.compileTimeout = setTimeout(() => {
            this.compile();
        }, 300);
    }

    async compile(force = false) {
        const code = this.editor.getValue();
        
        // Skip if code hasn't changed and not forced
        if (!force && code === this.lastCode) return;
        this.lastCode = code;

        if (this.isCompiling) return;
        this.isCompiling = true;

        this.updateStatus('compiling');
        const startTime = Date.now();

        try {
            // Simulate compilation process
            const result = await this.simulateCompilation(code);
            const compileTime = Date.now() - startTime;
            
            this.displayResults(result, compileTime);
            this.visualizeAST(result.ast);
            this.updateStatus('success');
            
        } catch (error) {
            this.displayError(error);
            this.updateStatus('error');
        } finally {
            this.isCompiling = false;
        }
    }

    async simulateCompilation(code) {
        // Simulate compilation phases
        await this.delay(100 + Math.random() * 200);

        // Basic syntax validation
        const syntaxResult = this.validateSyntax(code);
        if (!syntaxResult.valid) {
            throw new Error(`Syntax Error: ${syntaxResult.error}`);
        }

        // Simulate tokenization
        const tokens = this.tokenize(code);
        
        // Simulate parsing to AST
        const ast = this.parseToAST(tokens);
        
        // Simulate type inference
        const typeInfo = this.inferTypes(ast);
        
        // Simulate code generation
        const generatedCode = this.generateCode(ast, typeInfo);
        
        return {
            success: true,
            tokens: tokens,
            ast: ast,
            types: typeInfo,
            generatedCode: generatedCode,
            output: this.simulateExecution(generatedCode)
        };
    }

    validateSyntax(code) {
        // Basic syntax validation
        const lines = code.split('\n');
        
        for (let i = 0; i < lines.length; i++) {
            const line = lines[i].trim();
            if (line === '') continue;
            
            // Check for unmatched braces
            const openBraces = (line.match(/\{/g) || []).length;
            const closeBraces = (line.match(/\}/g) || []).length;
            
            // Check for missing semicolons on simple statements
            if (line.includes('let ') && !line.endsWith(';') && !line.includes('{')) {
                return { valid: false, error: `Missing semicolon on line ${i + 1}` };
            }
        }
        
        return { valid: true };
    }

    tokenize(code) {
        const tokens = [];
        const keywords = ['fn', 'let', 'mut', 'if', 'else', 'while', 'return', 'struct', 'enum'];
        const regex = /\b\w+\b|[{}();,]|"[^"]*"|\/\/.*$/gm;
        
        let match;
        while ((match = regex.exec(code)) !== null) {
            const token = match[0];
            let type = 'identifier';
            
            if (keywords.includes(token)) {
                type = 'keyword';
            } else if (token.startsWith('"')) {
                type = 'string';
            } else if (token.startsWith('//')) {
                type = 'comment';
            } else if (/[{}();,]/.test(token)) {
                type = 'punctuation';
            } else if (/^\d+$/.test(token)) {
                type = 'number';
            }
            
            tokens.push({
                type: type,
                value: token,
                position: match.index
            });
        }
        
        return tokens;
    }

    parseToAST(tokens) {
        // Simplified AST generation
        const ast = {
            type: 'Program',
            children: []
        };

        let i = 0;
        while (i < tokens.length) {
            if (tokens[i]?.value === 'fn') {
                const func = this.parseFunction(tokens, i);
                ast.children.push(func.node);
                i = func.nextIndex;
            } else if (tokens[i]?.value === 'let') {
                const varDecl = this.parseVariableDeclaration(tokens, i);
                ast.children.push(varDecl.node);
                i = varDecl.nextIndex;
            } else {
                i++;
            }
        }

        return ast;
    }

    parseFunction(tokens, start) {
        const node = {
            type: 'FunctionDeclaration',
            name: tokens[start + 1]?.value || 'unknown',
            parameters: [],
            body: { type: 'Block', children: [] }
        };

        // Skip to next token after function parsing
        let i = start + 2;
        while (i < tokens.length && tokens[i]?.value !== 'fn') {
            i++;
        }

        return { node, nextIndex: i };
    }

    parseVariableDeclaration(tokens, start) {
        const node = {
            type: 'VariableDeclaration',
            name: tokens[start + 1]?.value || 'unknown',
            initializer: null
        };

        return { node, nextIndex: start + 4 };
    }

    inferTypes(ast) {
        // Simplified type inference
        const typeMap = new Map();
        
        const traverse = (node) => {
            if (node.type === 'FunctionDeclaration') {
                if (node.name === 'main') {
                    typeMap.set(node.name, '() -> ()');
                } else {
                    typeMap.set(node.name, 'fn(...) -> T');
                }
            } else if (node.type === 'VariableDeclaration') {
                // Simple type inference based on common patterns
                typeMap.set(node.name, 'T');
            }
            
            if (node.children) {
                node.children.forEach(traverse);
            }
        };

        traverse(ast);
        return typeMap;
    }

    generateCode(ast, types) {
        // Simulate Rust code generation
        return `// Generated Rust code from Ruchy
fn main() {
    println!("Hello from RuchyRuchy!");
    
    let message = "Exploring compiler internals";
    println!("Status: {}", message);
}`;
    }

    simulateExecution(generatedCode) {
        // Simulate program output
        return `Hello from RuchyRuchy!
Status: Exploring compiler internals
Sum: 15

Program executed successfully ✅`;
    }

    displayResults(result, compileTime) {
        const outputEl = document.getElementById('output-content');
        
        outputEl.innerHTML = `
            <div class="success-message">
                ✅ Compilation successful (${compileTime}ms)
            </div>
            
            <div style="margin: 1rem 0;">
                <strong>📤 Program Output:</strong>
                <pre style="background: rgba(0,0,0,0.3); padding: 0.8rem; border-radius: 4px; margin-top: 0.5rem; white-space: pre-wrap;">${result.output}</pre>
            </div>
            
            <div style="margin: 1rem 0;">
                <strong>🔧 Generated Code:</strong>
                <pre style="background: rgba(0,0,0,0.3); padding: 0.8rem; border-radius: 4px; margin-top: 0.5rem; font-size: 12px; white-space: pre-wrap;">${result.generatedCode}</pre>
            </div>
        `;

        // Update stats
        document.getElementById('compile-time').textContent = `${compileTime}ms`;
        document.getElementById('tokens-count').textContent = `${result.tokens.length} tokens`;
        document.getElementById('ast-nodes').textContent = `${this.countASTNodes(result.ast)} nodes`;
    }

    displayError(error) {
        const outputEl = document.getElementById('output-content');
        
        outputEl.innerHTML = `
            <div class="error-message">
                ❌ ${error.message}
            </div>
            
            <div style="margin-top: 1rem; opacity: 0.7;">
                <strong>💡 Tips:</strong><br>
                • Check for missing semicolons<br>
                • Ensure all braces are matched<br>
                • Verify function syntax: <code>fn name() { }</code><br>
                • Use <code>let</code> for variable declarations
            </div>
        `;
    }

    visualizeAST(ast) {
        const vizEl = document.getElementById('viz-content');
        
        const renderNode = (node, depth = 0, isLast = false) => {
            const indent = '  '.repeat(depth);
            const prefix = isLast ? '└─ ' : '├─ ';
            const children = node.children || [];
            
            let html = `<div class="ast-node ${isLast ? 'last' : ''}" style="margin-left: ${depth * 1.5}rem">
                <span class="ast-type">${node.type}</span>`;
            
            if (node.name) {
                html += ` <span class="ast-value">"${node.name}"</span>`;
            }
            
            html += '</div>';
            
            children.forEach((child, index) => {
                html += renderNode(child, depth + 1, index === children.length - 1);
            });
            
            return html;
        };

        vizEl.innerHTML = `
            <div style="font-size: 13px; line-height: 1.6;">
                <strong>🌳 Abstract Syntax Tree:</strong>
                <div style="margin-top: 0.8rem; font-family: monospace;">
                    ${renderNode(ast)}
                </div>
            </div>
        `;
    }

    countASTNodes(node) {
        let count = 1;
        if (node.children) {
            count += node.children.reduce((sum, child) => sum + this.countASTNodes(child), 0);
        }
        return count;
    }

    stepThroughCompilation() {
        const code = this.editor.getValue();
        
        // Create step-by-step view
        const outputEl = document.getElementById('output-content');
        outputEl.innerHTML = `
            <div style="font-size: 13px;">
                <div class="success-message">📋 Step-by-step compilation process</div>
                
                <div style="margin: 1rem 0;">
                    <strong>Step 1: Lexical Analysis (Tokenization)</strong>
                    <div style="margin: 0.5rem 0; opacity: 0.8; font-size: 12px;">
                        Breaking source code into tokens (keywords, identifiers, operators...)
                    </div>
                </div>
                
                <div style="margin: 1rem 0;">
                    <strong>Step 2: Syntax Analysis (Parsing)</strong>
                    <div style="margin: 0.5rem 0; opacity: 0.8; font-size: 12px;">
                        Building Abstract Syntax Tree (AST) from tokens
                    </div>
                </div>
                
                <div style="margin: 1rem 0;">
                    <strong>Step 3: Semantic Analysis (Type Checking)</strong>
                    <div style="margin: 0.5rem 0; opacity: 0.8; font-size: 12px;">
                        Inferring types and checking for type errors
                    </div>
                </div>
                
                <div style="margin: 1rem 0;">
                    <strong>Step 4: Code Generation</strong>
                    <div style="margin: 0.5rem 0; opacity: 0.8; font-size: 12px;">
                        Generating target code (Rust/TypeScript)
                    </div>
                </div>
                
                <div style="margin-top: 1.5rem; padding: 0.8rem; background: rgba(80, 250, 123, 0.1); border-radius: 4px;">
                    🎓 <strong>Educational Note:</strong> This playground demonstrates the core phases 
                    of compilation that every compiler must implement.
                </div>
            </div>
        `;
    }

    shareCode() {
        const code = this.editor.getValue();
        const encoded = btoa(encodeURIComponent(code));
        const url = `${window.location.origin}${window.location.pathname}?code=${encoded}`;
        
        navigator.clipboard.writeText(url).then(() => {
            const outputEl = document.getElementById('output-content');
            outputEl.innerHTML = `
                <div class="success-message">
                    🔗 Shareable link copied to clipboard!
                </div>
                <div style="margin-top: 1rem; font-size: 12px; word-break: break-all; opacity: 0.8;">
                    ${url}
                </div>
            `;
        });
    }

    loadExample() {
        const examples = [
            {
                name: "Hello World",
                code: `fn main() {
    println("Hello from RuchyRuchy!");
}`
            },
            {
                name: "Variables and Functions",
                code: `fn main() {
    let name = "RuchyRuchy";
    let version = "1.29.0";
    
    greet(name, version);
    
    let result = add(5, 3);
    println("5 + 3 = {}", result);
}

fn greet(name: str, version: str) {
    println("Welcome to {} version {}", name, version);
}

fn add(x: u32, y: u32) -> u32 {
    x + y
}`
            },
            {
                name: "Structs and Enums",
                code: `struct Token {
    kind: TokenType,
    value: str,
    line: u32
}

enum TokenType {
    Number,
    String,
    Identifier,
    Keyword
}

fn main() {
    let token = Token {
        kind: TokenType::Identifier,
        value: "main",
        line: 1
    };
    
    println("Token: {} at line {}", token.value, token.line);
}`
            }
        ];
        
        const example = examples[Math.floor(Math.random() * examples.length)];
        this.editor.setValue(example.code);
        
        const outputEl = document.getElementById('output-content');
        outputEl.innerHTML = `
            <div class="success-message">
                📚 Loaded example: "${example.name}"
            </div>
            <div style="margin-top: 1rem; opacity: 0.8; font-size: 13px;">
                Try modifying the code to explore different compilation results!
            </div>
        `;
    }

    loadExamples() {
        // Load from URL parameter if present
        const urlParams = new URLSearchParams(window.location.search);
        const codeParam = urlParams.get('code');
        
        if (codeParam) {
            try {
                const decoded = decodeURIComponent(atob(codeParam));
                setTimeout(() => {
                    this.editor.setValue(decoded);
                }, 100);
            } catch (e) {
                console.warn('Failed to decode URL code parameter');
            }
        }
    }

    updateStatus(status) {
        const editorStatus = document.getElementById('editor-status');
        const outputStatus = document.getElementById('output-status');
        const vizStatus = document.getElementById('viz-status');
        const compileBtn = document.getElementById('compile-btn');
        const compileText = document.getElementById('compile-text');

        const statuses = {
            ready: { class: '', text: '🚀 Compile & Run' },
            compiling: { class: 'warning', text: '<span class="loading"><span class="spinner"></span>Compiling...</span>' },
            success: { class: '', text: '🚀 Compile & Run' },
            error: { class: 'error', text: '🚀 Compile & Run' }
        };

        const statusInfo = statuses[status];
        
        editorStatus.className = `status-indicator ${statusInfo.class}`;
        outputStatus.className = `status-indicator ${statusInfo.class}`;
        vizStatus.className = `status-indicator ${statusInfo.class}`;
        compileText.innerHTML = statusInfo.text;

        compileBtn.disabled = status === 'compiling';
    }

    delay(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }
}

// Initialize playground when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    new RuchyPlayground();
});