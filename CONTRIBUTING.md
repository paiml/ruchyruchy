# Contributing to RuchyRuchy

Welcome to the RuchyRuchy project! We're excited that you're interested in contributing to our educational compiler construction resources. This guide will help you understand how to contribute effectively.

## 🎯 Project Mission

RuchyRuchy provides **educational resources and development tools** for understanding bootstrap compilers, supporting the main [Ruchy programming language](https://github.com/paiml/ruchy) ecosystem.

## 📝 Types of Contributions We Welcome

### 1. Educational Content
- **Interactive tutorials** demonstrating compiler concepts
- **Visual learning tools** for AST, type inference, code generation
- **Example programs** showcasing language features
- **Documentation improvements** for clarity and accuracy

### 2. Infrastructure Tools
- **Code generation enhancements** for the Ruchy→Rust pipeline
- **Performance optimizations** maintaining our 24M+ LOC/s benchmark
- **Testing utilities** for validation and benchmarking
- **Development tools** supporting the Ruchy ecosystem

### 3. Community Resources
- **Workshop materials** for teaching compiler construction
- **Video tutorials** and screencasts
- **Blog posts** explaining compiler concepts
- **Translation** of educational materials

## 🚀 Getting Started

### Prerequisites
1. Install the Ruchy compiler: `cargo install ruchy`
2. Clone the repository: `git clone https://github.com/paiml/ruchyruchy.git`
3. Read our [README.md](./README.md) and [ROADMAP.md](./ROADMAP.md)

### Development Setup
```bash
# Clone and enter the repository
git clone https://github.com/paiml/ruchyruchy.git
cd ruchyruchy

# Run quality checks
make quality-gate

# Test the educational examples
make stage0-demo
make performance-demo
```

## 📋 Contribution Process

### Step 1: Choose a Ticket
1. Check our [ROADMAP.md](./ROADMAP.md) for open tickets
2. Look for tickets marked as **"Open"** in the current quarter
3. Comment on the ticket to claim it
4. Wait for maintainer confirmation before starting

### Step 2: Create Your Branch
```bash
# Create a feature branch from main
git checkout -b TICKET-ID-description

# Examples:
git checkout -b EDUCATION-002-documentation-hub
git checkout -b INFRA-002-performance-tools
```

### Step 3: Implement Your Contribution

#### For Educational Content:
1. Follow our tutorial template (see below)
2. Include interactive examples where possible
3. Test all code samples
4. Add visual aids and diagrams
5. Ensure accessibility (proper HTML structure, alt text)

#### For Code Contributions:
1. Follow existing code patterns and style
2. Maintain performance benchmarks
3. Add comprehensive tests
4. Update relevant documentation
5. Ensure all quality gates pass

### Step 4: Quality Standards

All contributions must meet our **Toyota Way** quality standards:

#### Mandatory Quality Gates (BLOCKING):
```bash
# Run all quality checks
make quality-gate

# Individual checks:
make lint              # Zero-warning linting
make test              # All tests must pass
make complexity        # Functions <20 cyclomatic complexity
make performance       # Maintain throughput benchmarks
```

#### Educational Value Criteria:
- ✅ Clear learning objectives stated
- ✅ Progressive difficulty levels
- ✅ Working, tested code examples
- ✅ Visual aids where appropriate
- ✅ Links to related concepts

### Step 5: Submit Pull Request

#### PR Title Format:
```
TICKET-ID: Brief description

Examples:
EDUCATION-002: Add comprehensive documentation hub
INFRA-003: Implement performance profiling tools
```

#### PR Description Template:
```markdown
## Summary
Brief description of what this PR accomplishes

## Ticket Reference
Closes #TICKET-ID

## Changes Made
- List of specific changes
- Implementation approach
- Any design decisions

## Testing
- How to test the changes
- Test results/screenshots
- Performance impact (if applicable)

## Educational Value (if applicable)
- Learning objectives addressed
- Target audience
- Prerequisites needed

## Checklist
- [ ] All quality gates pass (`make quality-gate`)
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] Performance benchmarks maintained
- [ ] Educational value validated
```

## 📚 Tutorial Template System

When creating new educational content, use these templates:

### Interactive Tutorial Template
```html
<!-- Save as: docs/tutorials/YOUR_TUTORIAL.html -->
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Tutorial Title - RuchyRuchy</title>
    <link rel="stylesheet" href="../../assets/tutorial.css">
</head>
<body>
    <h1>🎯 Tutorial Title</h1>
    
    <section class="learning-objectives">
        <h2>Learning Objectives</h2>
        <ul>
            <li>Objective 1</li>
            <li>Objective 2</li>
        </ul>
    </section>
    
    <section class="prerequisites">
        <h2>Prerequisites</h2>
        <ul>
            <li>Required knowledge</li>
            <li>Related tutorials</li>
        </ul>
    </section>
    
    <section class="content">
        <!-- Tutorial content here -->
    </section>
    
    <section class="exercises">
        <h2>Exercises</h2>
        <!-- Interactive exercises -->
    </section>
    
    <section class="next-steps">
        <h2>Next Steps</h2>
        <!-- Links to related content -->
    </section>
</body>
</html>
```

### Markdown Tutorial Template
```markdown
# Tutorial Title

## 🎯 Learning Objectives
- What students will learn
- Specific skills gained
- Concepts understood

## 📋 Prerequisites
- Required background knowledge
- Previous tutorials to complete
- Tools needed

## 📖 Introduction
Brief overview of the topic and why it matters

## 🔧 Main Content

### Section 1: Topic
Explanation with examples

```ruchy
// Code example
fn example() {
    // Implementation
}
```

### Section 2: Another Topic
More detailed explanation

## 🧪 Exercises
1. Exercise description
2. Another exercise

## 📊 Summary
Key takeaways from this tutorial

## 🔗 Additional Resources
- Links to documentation
- Related tutorials
- External resources
```

## 🔍 Code Review Process

### Review Criteria
1. **Technical Correctness**: Code works as intended
2. **Educational Value**: Clear learning outcomes
3. **Code Quality**: Follows project standards
4. **Performance**: Maintains benchmarks
5. **Documentation**: Clear and comprehensive
6. **Testing**: Adequate test coverage

### Review Workflow
1. Automated checks run on PR submission
2. Peer review by community members
3. Maintainer review for final approval
4. Merge when all checks pass

## 💬 Feedback Collection System

We value feedback on our educational materials:

### How to Provide Feedback
1. **GitHub Issues**: For bugs or feature requests
2. **Discussion Forum**: For questions and suggestions
3. **Tutorial Feedback**: Use the feedback widget in tutorials
4. **Survey Forms**: Periodic community surveys

### Feedback Categories
- **Content Quality**: Accuracy, clarity, completeness
- **Learning Experience**: Difficulty, pacing, engagement
- **Technical Issues**: Bugs, performance, compatibility
- **Feature Requests**: New tutorials, tools, improvements

## 🤝 Community Guidelines

### Code of Conduct
- Be respectful and inclusive
- Welcome newcomers and help them learn
- Provide constructive feedback
- Focus on education and learning
- Credit others' contributions

### Communication Channels
- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and ideas
- **Pull Requests**: Code contributions and reviews

## 📈 Recognition

We recognize contributors in several ways:
- **Contributors list** in README.md
- **Author attribution** in tutorials
- **Monthly highlights** for significant contributions
- **Contributor badges** for regular contributors

## ❓ Getting Help

If you need help:
1. Check the [documentation](./docs/)
2. Search existing [issues](https://github.com/paiml/ruchyruchy/issues)
3. Ask in [discussions](https://github.com/paiml/ruchyruchy/discussions)
4. Contact maintainers via issues

## 📜 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to RuchyRuchy! Your efforts help others learn compiler construction and support the Ruchy ecosystem. 🎉