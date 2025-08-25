# Interactive Tokenization Tutorial

## Understanding Lexical Analysis in Ruchy

**Learning Objective**: Understand how source code text is converted into tokens for compilation

### What is Tokenization?

Tokenization (lexical analysis) is the first stage of compilation where raw source code text is broken down into meaningful units called **tokens**.

```ruchy
// Source code (string)
let x = 42

// Becomes tokens (structured data)
KEYWORD_LET
IDENTIFIER("x")
EQUAL
NUMBER(42)
EOF
```

## Step 1: Character Recognition

The lexer processes source code character by character:

### Interactive Example: Character Classification

```ruchy
// Character types in Ruchy
fn is_letter(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' => true,
        _ => false
    }
}

fn is_digit(c: char) -> bool {
    match c {
        '0'..='9' => true,
        _ => false
    }
}

fn is_whitespace(c: char) -> bool {
    match c {
        ' ' | '\t' | '\n' | '\r' => true,
        _ => false
    }
}
```

### Try It: Character Recognition
**Input**: `"let x = 42"`

**Character Analysis**:
- `l`, `e`, `t` → Letters (start of identifier/keyword)
- ` ` → Whitespace (separator)
- `x` → Letter (identifier)
- ` ` → Whitespace (separator)
- `=` → Operator symbol
- ` ` → Whitespace (separator)
- `4`, `2` → Digits (number literal)

## Step 2: Token Classification

Once characters are recognized, they're grouped into tokens:

### Token Types in Ruchy

```ruchy
enum TokenType {
    // Keywords
    KeywordLet,
    KeywordFun,
    KeywordIf,
    KeywordElse,
    
    // Literals
    Identifier(String),
    Number(f64),
    String(String),
    
    // Operators
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    
    // Special
    EOF,
    Error(String)
}
```

### Interactive Token Generation

**Source**: `let add = fun(x, y) x + y`

**Step-by-step tokenization**:

1. **`let`** → Check keyword table → `KEYWORD_LET`
2. **` `** → Whitespace → Skip
3. **`add`** → Letters → `IDENTIFIER("add")`
4. **` `** → Whitespace → Skip
5. **`=`** → Operator → `EQUAL`
6. **` `** → Whitespace → Skip
7. **`fun`** → Check keyword table → `KEYWORD_FUN`
8. **`(`** → Delimiter → `LEFT_PAREN`
9. **`x`** → Letter → `IDENTIFIER("x")`
10. **`,`** → Delimiter → `COMMA`
11. **` `** → Whitespace → Skip
12. **`y`** → Letter → `IDENTIFIER("y")`
13. **`)`** → Delimiter → `RIGHT_PAREN`
14. **` `** → Whitespace → Skip
15. **`x`** → Letter → `IDENTIFIER("x")`
16. **` `** → Whitespace → Skip
17. **`+`** → Operator → `PLUS`
18. **` `** → Whitespace → Skip
19. **`y`** → Letter → `IDENTIFIER("y")`
20. **End of input** → `EOF`

## Step 3: Keyword vs Identifier Distinction

Critical decision: Is a letter sequence a keyword or identifier?

```ruchy
fn classify_word(word: String) -> TokenType {
    match word.as_str() {
        "let" => KeywordLet,
        "fun" => KeywordFun,
        "if" => KeywordIf,
        "else" => KeywordElse,
        _ => Identifier(word)
    }
}
```

### Try It: Classification Challenge

**Input words**:
- `"let"` → `KEYWORD_LET` ✅
- `"letter"` → `IDENTIFIER("letter")` ✅ (starts with "let" but not exact match)
- `"x"` → `IDENTIFIER("x")` ✅
- `"if"` → `KEYWORD_IF` ✅

## Step 4: Number Parsing

Numbers require special handling for different formats:

```ruchy
fn parse_number(input: String, start: usize) -> (Token, usize) {
    let mut end = start;
    let mut has_dot = false;
    
    while end < input.len() {
        match input.chars().nth(end) {
            Some('0'..='9') => end += 1,
            Some('.') if !has_dot => {
                has_dot = true;
                end += 1;
            },
            _ => break
        }
    }
    
    let number_str = &input[start..end];
    let value = number_str.parse::<f64>().unwrap();
    (Token::Number(value), end)
}
```

### Number Parsing Examples

- `"42"` → `NUMBER(42.0)`
- `"3.14"` → `NUMBER(3.14)`
- `"123.456"` → `NUMBER(123.456)`

## Step 5: Complete Tokenizer Implementation

Here's a simplified tokenizer that demonstrates the concepts:

```ruchy
struct Tokenizer {
    input: String,
    position: usize,
    current_char: Option<char>
}

impl Tokenizer {
    fn new(input: String) -> Self {
        let mut tokenizer = Tokenizer {
            input,
            position: 0,
            current_char: None
        };
        tokenizer.advance();
        tokenizer
    }
    
    fn advance(&mut self) {
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = self.input.chars().nth(self.position);
            self.position += 1;
        }
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn read_identifier(&mut self) -> String {
        let start = self.position - 1;
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.position-1].to_string()
    }
    
    fn get_next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        match self.current_char {
            None => Token::EOF,
            Some('=') => {
                self.advance();
                Token::Equal
            },
            Some('+') => {
                self.advance();
                Token::Plus
            },
            Some(c) if c.is_alphabetic() => {
                let word = self.read_identifier();
                match word.as_str() {
                    "let" => Token::KeywordLet,
                    "fun" => Token::KeywordFun,
                    _ => Token::Identifier(word)
                }
            },
            Some(c) if c.is_digit(10) => {
                // Number parsing logic here
                let value = 42.0; // Simplified
                self.advance();
                Token::Number(value)
            },
            Some(c) => {
                self.advance();
                Token::Error(format!("Unexpected character: {}", c))
            }
        }
    }
}
```

## Interactive Exercise: Build Your Own Tokenizer

**Challenge**: Tokenize this Ruchy code:

```ruchy
fun factorial(n) {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}
```

**Expected tokens**:
1. `KEYWORD_FUN`
2. `IDENTIFIER("factorial")`
3. `LEFT_PAREN`
4. `IDENTIFIER("n")`
5. `RIGHT_PAREN`
6. `LEFT_BRACE`
7. `KEYWORD_IF`
8. `IDENTIFIER("n")`
9. `LESS_EQUAL`
10. `NUMBER(1.0)`
11. `LEFT_BRACE`
12. `NUMBER(1.0)`
13. `RIGHT_BRACE`
14. `KEYWORD_ELSE`
15. `LEFT_BRACE`
16. `IDENTIFIER("n")`
17. `STAR`
18. `IDENTIFIER("factorial")`
19. `LEFT_PAREN`
20. `IDENTIFIER("n")`
21. `MINUS`
22. `NUMBER(1.0)`
23. `RIGHT_PAREN`
24. `RIGHT_BRACE`
25. `RIGHT_BRACE`
26. `EOF`

## Key Learning Points

1. **Character Recognition**: Foundation of tokenization
2. **State Management**: Tracking position and current character
3. **Keyword vs Identifier**: Critical for language syntax
4. **Number Parsing**: Handling different numeric formats
5. **Error Handling**: Managing unexpected characters
6. **Token Stream**: Structured output for the parser

## Next Steps

Once you understand tokenization, you're ready for:
- **Parsing**: Converting tokens into an Abstract Syntax Tree (AST)
- **Type Checking**: Ensuring program correctness
- **Code Generation**: Producing executable output

## Working Example

Try running this tokenization example from the RuchyRuchy project:

```bash
# Build the educational tokenizer
make stage0-demo

# Test with sample code
echo 'let x = 42' | ./build/stage0/tokenizer
```

This tutorial demonstrates the first stage of compilation - transforming text into structured tokens that can be processed by later compiler stages.