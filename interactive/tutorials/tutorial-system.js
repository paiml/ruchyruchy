// RuchyRuchy Interactive Tutorial System
// Guided learning with progress tracking and achievements

class TutorialSystem {
    constructor() {
        this.currentTutorial = null;
        this.currentStep = 0;
        this.hintsUsed = 0;
        this.totalPoints = 0;
        this.badgesEarned = [];
        this.completedTutorials = [];
        
        this.tutorials = this.loadTutorials();
        this.initializeEditor();
        this.bindEvents();
        this.loadTutorial('hello_world');
        this.loadProgress();
    }

    loadTutorials() {
        return {
            hello_world: {
                id: 'hello_world',
                title: 'Hello, RuchyRuchy!',
                difficulty: 'beginner',
                estimatedTime: 10,
                steps: [
                    {
                        title: 'Your First Ruchy Program',
                        instruction: 'Write a simple program that prints "Hello, World!" to the console.',
                        initialCode: 'fn main() {\n    // Write your code here\n    \n}',
                        expectedOutput: 'Hello, World!',
                        hints: [
                            'Use the println function to print text',
                            'The syntax is: println("your text");',
                            'Complete solution: println("Hello, World!");'
                        ],
                        validation: {
                            mustContain: ['println'],
                            outputMatch: 'Hello, World!'
                        }
                    }
                ],
                badge: {
                    id: 'first_program',
                    name: 'First Steps',
                    icon: '🎯',
                    points: 10
                }
            },
            variables: {
                id: 'variables',
                title: 'Variables and Types',
                difficulty: 'beginner',
                estimatedTime: 15,
                steps: [
                    {
                        title: 'Declaring Variables',
                        instruction: 'Create a variable named "age" with value 25 and print it.',
                        initialCode: 'fn main() {\n    // Declare your variable here\n    \n    // Print the variable\n}',
                        expectedOutput: 'Age: 25',
                        hints: [
                            'Use "let" to declare variables',
                            'Syntax: let age = 25;',
                            'Use {} in println for formatting: println("Age: {}", age);'
                        ],
                        validation: {
                            mustContain: ['let age', 'println'],
                            outputMatch: 'Age: 25'
                        }
                    },
                    {
                        title: 'Different Types',
                        instruction: 'Create variables of different types: string, number, and boolean.',
                        initialCode: 'fn main() {\n    // Create a string variable\n    \n    // Create a number variable\n    \n    // Create a boolean variable\n    \n    println("Variables created!");\n}',
                        expectedOutput: 'Variables created!',
                        hints: [
                            'String: let name = "Alice";',
                            'Number: let count = 42;',
                            'Boolean: let is_ready = true;'
                        ],
                        validation: {
                            mustContain: ['let'],
                            outputMatch: 'Variables created!'
                        }
                    }
                ],
                badge: {
                    id: 'type_master',
                    name: 'Type Explorer',
                    icon: '📊',
                    points: 15
                }
            },
            functions: {
                id: 'functions',
                title: 'Functions in Ruchy',
                difficulty: 'intermediate',
                estimatedTime: 20,
                steps: [
                    {
                        title: 'Creating Functions',
                        instruction: 'Create a function that adds two numbers and returns the result.',
                        initialCode: '// Define your add function here\n\nfn main() {\n    let result = add(5, 3);\n    println("5 + 3 = {}", result);\n}',
                        expectedOutput: '5 + 3 = 8',
                        hints: [
                            'Function syntax: fn name(param: type) -> return_type { }',
                            'Parameters need types: (x: u32, y: u32)',
                            'Return the sum: fn add(x: u32, y: u32) -> u32 { x + y }'
                        ],
                        validation: {
                            mustContain: ['fn add'],
                            outputMatch: '5 + 3 = 8'
                        }
                    }
                ],
                badge: {
                    id: 'function_builder',
                    name: 'Function Builder',
                    icon: '🔧',
                    points: 20
                }
            },
            structs: {
                id: 'structs',
                title: 'Working with Structs',
                difficulty: 'intermediate',
                estimatedTime: 25,
                steps: [
                    {
                        title: 'Defining Structs',
                        instruction: 'Create a Person struct with name and age fields, then create an instance.',
                        initialCode: '// Define your Person struct here\n\nfn main() {\n    // Create a Person instance\n    \n    // Print the person\'s details\n    println("Person created!");\n}',
                        expectedOutput: 'Person created!',
                        hints: [
                            'Struct syntax: struct Person { name: str, age: u32 }',
                            'Create instance: let person = Person { name: "Alice", age: 30 };',
                            'Access fields: person.name and person.age'
                        ],
                        validation: {
                            mustContain: ['struct Person'],
                            outputMatch: 'Person created!'
                        }
                    }
                ],
                badge: {
                    id: 'data_architect',
                    name: 'Data Architect',
                    icon: '🏗️',
                    points: 25
                }
            },
            compiler: {
                id: 'compiler',
                title: 'Compiler Basics',
                difficulty: 'advanced',
                estimatedTime: 30,
                steps: [
                    {
                        title: 'Understanding Tokenization',
                        instruction: 'Create a function that checks if a word is a Ruchy keyword.',
                        initialCode: 'fn is_keyword(word: str) -> bool {\n    // Check if word is a keyword (fn, let, if, while)\n    false\n}\n\nfn main() {\n    let word = "fn";\n    if is_keyword(word) {\n        println("{} is a keyword", word);\n    } else {\n        println("{} is not a keyword", word);\n    }\n}',
                        expectedOutput: 'fn is a keyword',
                        hints: [
                            'Check against known keywords: fn, let, if, while',
                            'Use || (or) operator: word == "fn" || word == "let"',
                            'Return true for keywords, false otherwise'
                        ],
                        validation: {
                            mustContain: ['is_keyword'],
                            outputMatch: 'fn is a keyword'
                        }
                    }
                ],
                badge: {
                    id: 'compiler_explorer',
                    name: 'Compiler Explorer',
                    icon: '🔬',
                    points: 30
                }
            }
        };
    }

    initializeEditor() {
        const textarea = document.getElementById('code-editor');
        this.editor = CodeMirror.fromTextArea(textarea, {
            mode: 'rust',
            theme: 'dracula',
            lineNumbers: true,
            autoCloseBrackets: true,
            matchBrackets: true,
            indentUnit: 4,
            tabSize: 4,
            lineWrapping: true
        });
    }

    bindEvents() {
        // Tutorial selection
        document.querySelectorAll('.tutorial-item').forEach(item => {
            item.addEventListener('click', (e) => {
                const tutorialId = item.dataset.tutorial;
                this.loadTutorial(tutorialId);
                
                // Update active state
                document.querySelectorAll('.tutorial-item').forEach(i => i.classList.remove('active'));
                item.classList.add('active');
            });
        });

        // Action buttons
        document.getElementById('run-btn').addEventListener('click', () => this.runCode());
        document.getElementById('check-btn').addEventListener('click', () => this.checkAnswer());
        document.getElementById('hint-btn').addEventListener('click', () => this.showHint());
        document.getElementById('reset-btn').addEventListener('click', () => this.resetCode());
    }

    loadTutorial(tutorialId) {
        const tutorial = this.tutorials[tutorialId];
        if (!tutorial) return;

        this.currentTutorial = tutorial;
        this.currentStep = 0;
        this.hintsUsed = 0;

        // Update UI
        document.getElementById('tutorial-title').textContent = tutorial.title;
        this.loadStep(0);
        this.updateProgress();
    }

    loadStep(stepIndex) {
        if (!this.currentTutorial || stepIndex >= this.currentTutorial.steps.length) return;

        const step = this.currentTutorial.steps[stepIndex];
        this.currentStep = stepIndex;

        // Update step content
        document.querySelector('.step-header h3').innerHTML = `
            <span class="step-number">${stepIndex + 1}</span>
            ${step.title}
        `;

        document.querySelector('.instruction-box').innerHTML = `
            <strong>📝 Instructions:</strong><br>
            ${step.instruction}
        `;

        // Set initial code
        this.editor.setValue(step.initialCode);

        // Reset hints
        this.hintsUsed = 0;
        document.getElementById('hint-btn').textContent = `💡 Get Hint (0/${step.hints.length})`;
        document.getElementById('hints-panel').classList.remove('show');
        document.getElementById('hints-content').innerHTML = '';

        // Clear output
        document.getElementById('output-content').textContent = 'Run your code to see the output...';
        document.getElementById('feedback-panel').innerHTML = '';

        // Update meta info
        document.querySelector('.tutorial-meta').innerHTML = `
            <span>📊 Step ${stepIndex + 1} of ${this.currentTutorial.steps.length}</span>
            <span>⏱️ Estimated: ${this.currentTutorial.estimatedTime} minutes</span>
            <span>🎯 Difficulty: ${this.currentTutorial.difficulty}</span>
        `;
    }

    runCode() {
        const code = this.editor.getValue();
        
        // Simulate code execution
        const output = this.simulateExecution(code);
        
        document.getElementById('output-content').textContent = output;
        
        // Check if output matches expected
        const step = this.currentTutorial.steps[this.currentStep];
        if (output === step.expectedOutput) {
            this.showSuccess();
        }
    }

    simulateExecution(code) {
        // Basic simulation of code execution
        if (code.includes('println("Hello, World!")')) {
            return 'Hello, World!';
        } else if (code.includes('println("Age: {}", age)') && code.includes('let age = 25')) {
            return 'Age: 25';
        } else if (code.includes('println("5 + 3 = {}", result)') && code.includes('fn add')) {
            return '5 + 3 = 8';
        } else if (code.includes('println("Variables created!")')) {
            return 'Variables created!';
        } else if (code.includes('println("Person created!")') && code.includes('struct Person')) {
            return 'Person created!';
        } else if (code.includes('is_keyword') && code.includes('word == "fn"')) {
            return 'fn is a keyword';
        } else if (code.includes('println')) {
            return '[Program output]';
        } else {
            return 'No output';
        }
    }

    checkAnswer() {
        const code = this.editor.getValue();
        const step = this.currentTutorial.steps[this.currentStep];
        const validation = step.validation;
        
        let isValid = true;
        let errors = [];

        // Check for required content
        if (validation.mustContain) {
            for (const required of validation.mustContain) {
                if (!code.includes(required)) {
                    isValid = false;
                    errors.push(`Your code should include: ${required}`);
                }
            }
        }

        // Check output
        const output = this.simulateExecution(code);
        if (validation.outputMatch && output !== validation.outputMatch) {
            isValid = false;
            errors.push(`Expected output: "${validation.outputMatch}", but got: "${output}"`);
        }

        if (isValid) {
            this.showSuccess();
        } else {
            this.showErrors(errors);
        }
    }

    showSuccess() {
        const feedbackPanel = document.getElementById('feedback-panel');
        feedbackPanel.innerHTML = `
            <div class="success-message">
                ✅ Excellent! Your solution is correct!
                ${this.hintsUsed === 0 ? '<br>🌟 Perfect! No hints used!' : ''}
            </div>
        `;

        // Mark step as complete
        document.getElementById('step-status').textContent = '✅ Complete';

        // Award points
        const points = this.calculatePoints();
        this.totalPoints += points;
        this.updateProgress();

        // Check if tutorial is complete
        if (this.currentStep === this.currentTutorial.steps.length - 1) {
            this.completeTutorial();
        } else {
            // Show next step button
            feedbackPanel.innerHTML += `
                <button class="btn btn-primary" onclick="tutorialSystem.nextStep()" style="margin-top: 1rem;">
                    Next Step ➡️
                </button>
            `;
        }
    }

    showErrors(errors) {
        const feedbackPanel = document.getElementById('feedback-panel');
        feedbackPanel.innerHTML = `
            <div class="error-message">
                ❌ Not quite right. Here's what to fix:<br>
                ${errors.map(e => `• ${e}`).join('<br>')}
            </div>
        `;
    }

    showHint() {
        const step = this.currentTutorial.steps[this.currentStep];
        if (this.hintsUsed >= step.hints.length) return;

        const hintsPanel = document.getElementById('hints-panel');
        const hintsContent = document.getElementById('hints-content');
        
        hintsPanel.classList.add('show');
        
        const hint = step.hints[this.hintsUsed];
        hintsContent.innerHTML += `
            <div class="hint-item">
                ${hint}
            </div>
        `;

        this.hintsUsed++;
        document.getElementById('hint-btn').textContent = `💡 Get Hint (${this.hintsUsed}/${step.hints.length})`;
        
        if (this.hintsUsed >= step.hints.length) {
            document.getElementById('hint-btn').disabled = true;
        }
    }

    resetCode() {
        const step = this.currentTutorial.steps[this.currentStep];
        this.editor.setValue(step.initialCode);
        document.getElementById('output-content').textContent = 'Run your code to see the output...';
        document.getElementById('feedback-panel').innerHTML = '';
    }

    nextStep() {
        if (this.currentStep < this.currentTutorial.steps.length - 1) {
            this.loadStep(this.currentStep + 1);
        }
    }

    completeTutorial() {
        // Mark tutorial as complete
        this.completedTutorials.push(this.currentTutorial.id);
        
        // Award badge
        const badge = this.currentTutorial.badge;
        this.badgesEarned.push(badge);
        this.totalPoints += badge.points;
        
        // Show achievement
        this.showAchievement(badge);
        
        // Update UI
        this.updateProgress();
        this.saveProgress();
        
        // Mark tutorial item as complete
        const tutorialItem = document.querySelector(`[data-tutorial="${this.currentTutorial.id}"]`);
        if (tutorialItem) {
            tutorialItem.classList.add('completed');
        }
    }

    showAchievement(badge) {
        const popup = document.getElementById('achievement-popup');
        document.getElementById('achievement-text').textContent = `You earned the "${badge.name}" badge!`;
        
        popup.style.display = 'block';
        setTimeout(() => {
            popup.style.animation = 'popIn 0.5s ease forwards';
        }, 10);
        
        setTimeout(() => {
            popup.style.display = 'none';
        }, 3000);
    }

    calculatePoints() {
        let points = 10; // Base points
        
        // Bonus for no hints
        if (this.hintsUsed === 0) {
            points += 5;
        }
        
        // Penalty for hints
        points -= this.hintsUsed * 2;
        
        return Math.max(points, 5); // Minimum 5 points
    }

    updateProgress() {
        // Update progress bar
        const progress = ((this.currentStep + 1) / this.currentTutorial.steps.length) * 100;
        document.getElementById('progress-fill').style.width = `${progress}%`;
        
        // Update header stats
        document.getElementById('total-points').textContent = `${this.totalPoints} points`;
        document.getElementById('badges-count').textContent = `${this.badgesEarned.length} badges`;
        document.getElementById('tutorials-completed').textContent = 
            `${this.completedTutorials.length}/5 completed`;
    }

    saveProgress() {
        const progress = {
            totalPoints: this.totalPoints,
            badgesEarned: this.badgesEarned,
            completedTutorials: this.completedTutorials
        };
        localStorage.setItem('ruchy-tutorial-progress', JSON.stringify(progress));
    }

    loadProgress() {
        const saved = localStorage.getItem('ruchy-tutorial-progress');
        if (saved) {
            const progress = JSON.parse(saved);
            this.totalPoints = progress.totalPoints || 0;
            this.badgesEarned = progress.badgesEarned || [];
            this.completedTutorials = progress.completedTutorials || [];
            
            // Mark completed tutorials
            this.completedTutorials.forEach(id => {
                const item = document.querySelector(`[data-tutorial="${id}"]`);
                if (item) item.classList.add('completed');
            });
            
            this.updateProgress();
        }
    }
}

// Initialize the tutorial system
let tutorialSystem;
document.addEventListener('DOMContentLoaded', () => {
    tutorialSystem = new TutorialSystem();
});