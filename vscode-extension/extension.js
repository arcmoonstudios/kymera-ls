const vscode = require('vscode');
const { LanguageClient } = require('vscode-languageclient/node');

let client;

function activate(context) {
    const serverOptions = {
        command: vscode.workspace.getConfiguration('kymeraLanguageServer').get('serverPath'),
        transport: vscode.TransportKind.stdio
    };

    const clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'kymera' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.ky')
        }
    };

    client = new LanguageClient(
        'kymeraLanguageServer',
        'Kymera Language Server',
        serverOptions,
        clientOptions
    );

    client.start();
}

function deactivate() {
    if (client) {
        return client.stop();
    }
    return undefined;
}

module.exports = { activate, deactivate }; 