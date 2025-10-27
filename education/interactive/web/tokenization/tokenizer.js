// Interactive Tokenization Tutorial
// JavaScript implementation of Ruchy lexer for educational purposes

// Token types matching Ruchy's lexer
const TokenType = {
    // Keywords
    FUN: 'keyword',
    LET: 'keyword',
    IF: 'keyword',
    ELSE: 'keyword',
    MATCH: 'keyword',
    LOOP: 'keyword',
    WHILE: 'keyword',
    FOR: 'keyword',
    RETURN: 'keyword',
    BREAK: 'keyword',
    CONTINUE: 'keyword',
    IN: 'keyword',
    STRUCT: 'keyword',
    ENUM: 'keyword',
    TRAIT: 'keyword',
    IMPL: 'keyword',
    TYPE: 'keyword',
    TRUE: 'keyword',
    FALSE: 'keyword',

    // Literals
    IDENTIFIER: 'identifier',
    NUMBER: 'number',
    STRING: 'string',

    // Operators
    PLUS: 'operator',
    MINUS: 'operator',
    STAR: 'operator',
    SLASH: 'operator',
    PERCENT: 'operator',
    EQUAL_EQUAL: 'operator',
    NOT_EQUAL: 'operator',
    LESS: 'operator',
    LESS_EQUAL: 'operator',
    GREATER: 'operator',
    GREATER_EQUAL: 'operator',
    AND_AND: 'operator',
    OR_OR: 'operator',
    NOT: 'operator',
    EQUAL: 'operator',

    // Delimiters
    LEFT_PAREN: 'delimiter',
    RIGHT_PAREN: 'delimiter',
    LEFT_BRACE: 'delimiter',
    RIGHT_BRACE: 'delimiter',
    LEFT_BRACKET: 'delimiter',
    RIGHT_BRACKET: 'delimiter',
    COMMA: 'delimiter',
    SEMICOLON: 'delimiter',
    COLON: 'delimiter',
    ARROW: 'delimiter',
    DOT: 'delimiter',

    // Special
    NEWLINE: 'whitespace',
    EOF: 'eof',
    ERROR: 'error'
};

// Keywords map
const keywords = {
    'fun': 'FUN',
    'let': 'LET',
    'if': 'IF',
    'else': 'ELSE',
    'match': 'MATCH',
    'loop': 'LOOP',
    'while': 'WHILE',
    'for': 'FOR',
    'return': 'RETURN',
    'break': 'BREAK',
    'continue': 'CONTINUE',
    'in': 'IN',
    'struct': 'STRUCT',
    'enum': 'ENUM',
    'trait': 'TRAIT',
    'impl': 'IMPL',
    'type': 'TYPE',
    'true': 'TRUE',
    'false': 'FALSE'
};

class Token {
    constructor(type, value, line, column) {
        this.type = type;
        this.value = value;
        this.line = line;
        this.column = column;
    }
}

class Lexer {
    constructor(source) {
        this.source = source;
        this.position = 0;
        this.line = 1;
        this.column = 1;
        this.tokens = [];
    }

    isAtEnd() {
        return this.position >= this.source.length;
    }

    peek() {
        if (this.isAtEnd()) return '\0';
        return this.source[this.position];
    }

    peekNext() {
        if (this.position + 1 >= this.source.length) return '\0';
        return this.source[this.position + 1];
    }

    advance() {
        const ch = this.source[this.position];
        this.position++;
        if (ch === '\n') {
            this.line++;
            this.column = 1;
        } else {
            this.column++;
        }
        return ch;
    }

    skipWhitespace() {
        while (!this.isAtEnd()) {
            const ch = this.peek();
            if (ch === ' ' || ch === '\t' || ch === '\r' || ch === '\n') {
                this.advance();
            } else if (ch === '/' && this.peekNext() === '/') {
                // Skip line comment
                while (!this.isAtEnd() && this.peek() !== '\n') {
                    this.advance();
                }
            } else {
                break;
            }
        }
    }

    scanString() {
        const startLine = this.line;
        const startColumn = this.column;
        const quote = this.advance(); // consume opening quote
        let value = '';

        while (!this.isAtEnd() && this.peek() !== quote) {
            if (this.peek() === '\\') {
                this.advance(); // consume backslash
                const escaped = this.advance();
                switch (escaped) {
                    case 'n': value += '\n'; break;
                    case 't': value += '\t'; break;
                    case 'r': value += '\r'; break;
                    case '\\': value += '\\'; break;
                    case '"': value += '"'; break;
                    case "'": value += "'"; break;
                    default: value += escaped;
                }
            } else {
                value += this.advance();
            }
        }

        if (this.isAtEnd()) {
            return new Token(TokenType.ERROR, 'Unterminated string', startLine, startColumn);
        }

        this.advance(); // consume closing quote
        return new Token(TokenType.STRING, quote + value + quote, startLine, startColumn);
    }

    scanNumber() {
        const startLine = this.line;
        const startColumn = this.column;
        let value = '';

        while (!this.isAtEnd() && /[0-9]/.test(this.peek())) {
            value += this.advance();
        }

        if (this.peek() === '.' && /[0-9]/.test(this.peekNext())) {
            value += this.advance(); // consume '.'
            while (!this.isAtEnd() && /[0-9]/.test(this.peek())) {
                value += this.advance();
            }
        }

        return new Token(TokenType.NUMBER, value, startLine, startColumn);
    }

    scanIdentifier() {
        const startLine = this.line;
        const startColumn = this.column;
        let value = '';

        while (!this.isAtEnd() && /[a-zA-Z0-9_]/.test(this.peek())) {
            value += this.advance();
        }

        const tokenName = keywords[value] || 'IDENTIFIER';
        const tokenType = TokenType[tokenName];
        return new Token(tokenType, value, startLine, startColumn);
    }

    scanToken() {
        this.skipWhitespace();

        if (this.isAtEnd()) {
            return new Token(TokenType.EOF, '', this.line, this.column);
        }

        const startLine = this.line;
        const startColumn = this.column;
        const ch = this.peek();

        // Strings
        if (ch === '"' || ch === "'") {
            return this.scanString();
        }

        // Numbers
        if (/[0-9]/.test(ch)) {
            return this.scanNumber();
        }

        // Identifiers and keywords
        if (/[a-zA-Z_]/.test(ch)) {
            return this.scanIdentifier();
        }

        // Operators and delimiters
        this.advance();

        switch (ch) {
            case '(': return new Token(TokenType.LEFT_PAREN, '(', startLine, startColumn);
            case ')': return new Token(TokenType.RIGHT_PAREN, ')', startLine, startColumn);
            case '{': return new Token(TokenType.LEFT_BRACE, '{', startLine, startColumn);
            case '}': return new Token(TokenType.RIGHT_BRACE, '}', startLine, startColumn);
            case '[': return new Token(TokenType.LEFT_BRACKET, '[', startLine, startColumn);
            case ']': return new Token(TokenType.RIGHT_BRACKET, ']', startLine, startColumn);
            case ',': return new Token(TokenType.COMMA, ',', startLine, startColumn);
            case ';': return new Token(TokenType.SEMICOLON, ';', startLine, startColumn);
            case ':': return new Token(TokenType.COLON, ':', startLine, startColumn);
            case '.': return new Token(TokenType.DOT, '.', startLine, startColumn);
            case '+': return new Token(TokenType.PLUS, '+', startLine, startColumn);
            case '-':
                if (this.peek() === '>') {
                    this.advance();
                    return new Token(TokenType.ARROW, '->', startLine, startColumn);
                }
                return new Token(TokenType.MINUS, '-', startLine, startColumn);
            case '*': return new Token(TokenType.STAR, '*', startLine, startColumn);
            case '/': return new Token(TokenType.SLASH, '/', startLine, startColumn);
            case '%': return new Token(TokenType.PERCENT, '%', startLine, startColumn);
            case '=':
                if (this.peek() === '=') {
                    this.advance();
                    return new Token(TokenType.EQUAL_EQUAL, '==', startLine, startColumn);
                }
                return new Token(TokenType.EQUAL, '=', startLine, startColumn);
            case '!':
                if (this.peek() === '=') {
                    this.advance();
                    return new Token(TokenType.NOT_EQUAL, '!=', startLine, startColumn);
                }
                return new Token(TokenType.NOT, '!', startLine, startColumn);
            case '<':
                if (this.peek() === '=') {
                    this.advance();
                    return new Token(TokenType.LESS_EQUAL, '<=', startLine, startColumn);
                }
                return new Token(TokenType.LESS, '<', startLine, startColumn);
            case '>':
                if (this.peek() === '=') {
                    this.advance();
                    return new Token(TokenType.GREATER_EQUAL, '>=', startLine, startColumn);
                }
                return new Token(TokenType.GREATER, '>', startLine, startColumn);
            case '&':
                if (this.peek() === '&') {
                    this.advance();
                    return new Token(TokenType.AND_AND, '&&', startLine, startColumn);
                }
                return new Token(TokenType.ERROR, '&', startLine, startColumn);
            case '|':
                if (this.peek() === '|') {
                    this.advance();
                    return new Token(TokenType.OR_OR, '||', startLine, startColumn);
                }
                return new Token(TokenType.ERROR, '|', startLine, startColumn);
            default:
                return new Token(TokenType.ERROR, ch, startLine, startColumn);
        }
    }

    tokenize() {
        this.tokens = [];
        while (!this.isAtEnd()) {
            const token = this.scanToken();
            if (token.type !== TokenType.EOF) {
                this.tokens.push(token);
            }
        }
        return this.tokens;
    }
}

// Example programs
const examples = {
    hello: `fun main() {
    println("Hello, World!")
}`,
    variables: `fun main() {
    let x = 42
    let y = 3.14
    let name = "Ruchy"
    println(name)
}`,
    functions: `fun add(a: i32, b: i32) -> i32 {
    return a + b
}

fun main() {
    let result = add(10, 20)
    println(result)
}`,
    loops: `fun main() {
    for i in 0..10 {
        println(i)
    }

    let mut count = 0
    while count < 5 {
        count = count + 1
    }
}`
};

// UI functions
function displayTokens(tokens) {
    const output = document.getElementById('tokens-output');
    output.innerHTML = '';

    if (tokens.length === 0) {
        output.innerHTML = '<p style="color: #6b7280; padding: 1rem;">No tokens to display. Enter some code and click "Tokenize".</p>';
        return;
    }

    tokens.forEach(token => {
        const item = document.createElement('div');
        item.className = 'token-item';

        const badge = document.createElement('span');
        badge.className = `token-badge ${token.type}-token`;
        badge.textContent = token.type.toUpperCase();

        const value = document.createElement('span');
        value.className = 'token-value';
        value.textContent = token.value;

        const position = document.createElement('span');
        position.className = 'token-position';
        position.textContent = `Line ${token.line}, Col ${token.column}`;

        item.appendChild(badge);
        item.appendChild(value);
        item.appendChild(position);
        output.appendChild(item);
    });
}

function displayStats(tokens) {
    const stats = document.getElementById('stats');

    // Count token types
    const typeCounts = {};
    tokens.forEach(token => {
        typeCounts[token.type] = (typeCounts[token.type] || 0) + 1;
    });

    const statsHTML = `
        <div class="stat-card">
            <div class="stat-value">${tokens.length}</div>
            <div class="stat-label">Total Tokens</div>
        </div>
        <div class="stat-card">
            <div class="stat-value">${typeCounts.keyword || 0}</div>
            <div class="stat-label">Keywords</div>
        </div>
        <div class="stat-card">
            <div class="stat-value">${typeCounts.identifier || 0}</div>
            <div class="stat-label">Identifiers</div>
        </div>
        <div class="stat-card">
            <div class="stat-value">${typeCounts.number || 0}</div>
            <div class="stat-label">Numbers</div>
        </div>
        <div class="stat-card">
            <div class="stat-value">${typeCounts.string || 0}</div>
            <div class="stat-label">Strings</div>
        </div>
        <div class="stat-card">
            <div class="stat-value">${typeCounts.operator || 0}</div>
            <div class="stat-label">Operators</div>
        </div>
    `;
    stats.innerHTML = statsHTML;
}

function tokenizeCode() {
    const sourceCode = document.getElementById('source-code').value;
    const lexer = new Lexer(sourceCode);
    const tokens = lexer.tokenize();

    displayTokens(tokens);
    displayStats(tokens);
}

function clearCode() {
    document.getElementById('source-code').value = '';
    document.getElementById('tokens-output').innerHTML = '<p style="color: #6b7280; padding: 1rem;">No tokens to display. Enter some code and click "Tokenize".</p>';
    document.getElementById('stats').innerHTML = '';
}

function loadExample(exampleName) {
    document.getElementById('source-code').value = examples[exampleName];
    tokenizeCode();
}

// Event listeners
document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('tokenize-btn').addEventListener('click', tokenizeCode);
    document.getElementById('clear-btn').addEventListener('click', clearCode);

    document.querySelectorAll('.example-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            const example = btn.getAttribute('data-example');
            loadExample(example);
        });
    });

    // Tokenize initial code on load
    tokenizeCode();
});
