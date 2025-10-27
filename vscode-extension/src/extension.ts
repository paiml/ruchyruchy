// VS Code Extension for Ruchy Language Support
// IDE-002: VS Code extension with LSP client integration

import * as path from 'path';
import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind,
} from 'vscode-languageclient/node';

let client: LanguageClient | undefined;

/**
 * Activate the Ruchy extension
 */
export function activate(context: vscode.ExtensionContext): void {
    console.log('Ruchy extension is now active!');

    // Get configuration
    const config = vscode.workspace.getConfiguration('ruchy');
    const lspPath = config.get<string>('lsp.path', 'ruchylsp');

    // Check if LSP server is available
    // For now, we'll start with a simple implementation
    // In the future, this will launch the actual Rust LSP server

    // Server options: Use stdio to communicate with the LSP server
    const serverOptions: ServerOptions = {
        command: lspPath,
        args: [],
        transport: TransportKind.stdio,
    };

    // Client options: Configure the language client
    const clientOptions: LanguageClientOptions = {
        // Register the server for Ruchy documents
        documentSelector: [
            {
                scheme: 'file',
                language: 'ruchy',
            },
        ],
        synchronize: {
            // Notify the server about file configuration changes
            fileEvents: vscode.workspace.createFileSystemWatcher('**/.ruchyrc'),
        },
    };

    // Create the language client
    client = new LanguageClient(
        'ruchyLanguageServer',
        'Ruchy Language Server',
        serverOptions,
        clientOptions
    );

    // Start the client (this will also launch the server)
    // For now, we'll handle the case where the server isn't available yet
    client
        .start()
        .then(() => {
            console.log('Ruchy LSP client started successfully');
        })
        .catch((error) => {
            console.error('Failed to start Ruchy LSP client:', error);
            vscode.window.showWarningMessage(
                'Ruchy Language Server not found. Install ruchylsp or configure ruchy.lsp.path in settings.'
            );
        });

    // Register commands
    const helloCommand = vscode.commands.registerCommand(
        'ruchy.helloWorld',
        () => {
            vscode.window.showInformationMessage('Hello from Ruchy!');
        }
    );

    const checkSyntaxCommand = vscode.commands.registerCommand(
        'ruchy.checkSyntax',
        async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) {
                vscode.window.showErrorMessage('No active editor');
                return;
            }

            if (editor.document.languageId !== 'ruchy') {
                vscode.window.showErrorMessage('Not a Ruchy file');
                return;
            }

            // Save the document first
            await editor.document.save();

            // Run ruchy check via terminal
            const terminal = vscode.window.createTerminal('Ruchy Check');
            terminal.show();
            terminal.sendText(`ruchy check "${editor.document.fileName}"`);
        }
    );

    const formatCommand = vscode.commands.registerCommand(
        'ruchy.format',
        async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) {
                vscode.window.showErrorMessage('No active editor');
                return;
            }

            if (editor.document.languageId !== 'ruchy') {
                vscode.window.showErrorMessage('Not a Ruchy file');
                return;
            }

            // Save the document first
            await editor.document.save();

            // Run ruchy fmt
            const terminal = vscode.window.createTerminal('Ruchy Format');
            terminal.show();
            terminal.sendText(`ruchy fmt "${editor.document.fileName}"`);
        }
    );

    context.subscriptions.push(
        helloCommand,
        checkSyntaxCommand,
        formatCommand
    );
}

/**
 * Deactivate the extension
 */
export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
