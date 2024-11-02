To support syntax auto-completion (like inserting a comma after the last value in JSON) using the Language Server Protocol (LSP), you’d use the *TextDocumentOnTypeFormatting* feature. This feature allows a language server to react to specific keystrokes and make modifications to the document, such as adding syntax elements. Here’s how you can approach implementing this feature for your JSON auto-completion.

### 1. Configure `TextDocumentOnTypeFormatting` in LSP

The `onTypeFormatting` request allows you to trigger a server action whenever a specific character is typed. This request expects the server to return a list of *text edits* that should be applied in response.

### 2. Implementing `onTypeFormatting` for JSON

For JSON, we want to watch for the newline character (`\n`) while within an object. The LSP request `textDocument/onTypeFormatting` is used as follows:

- **Trigger characters**: Set `\n` as a trigger character, so every time you press enter within a JSON object, the server will evaluate if a comma is needed.
- **Position and Context Analysis**: When the server receives the `onTypeFormatting` request, it will analyze the document’s context to see if a comma should be inserted. Specifically, it should look at:
    - The current line and previous characters to detect if it’s the end of a key-value pair.
    - Check if a comma is already present after the last key-value pair.

### 3. Example Setup in a Language Server

Here’s an example implementation for handling this in Rust with `tower_lsp`, a common LSP implementation.

```rust
use tower_lsp::lsp_types::*;
use tower_lsp::jsonrpc::Result;
use tower_lsp::LanguageServer;

#[derive(Debug)]
struct Server;

#[tower_lsp::async_trait]
impl LanguageServer for Server {
    async fn on_type_formatting(
        &self,
        params: DocumentOnTypeFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri;
        let position = params.position;
        
        // Get document contents (simplified; real code would fetch actual content)
        let document_text = get_document_text(&uri).await;
        
        // Check if the position is before a closing brace and not followed by a comma
        if should_insert_comma(&document_text, position) {
            let edit = TextEdit {
                range: Range {
                    start: position,
                    end: position,
                },
                new_text: ",".to_string(),
            };
            Ok(Some(vec![edit]))
        } else {
            Ok(None)
        }
    }
}

fn should_insert_comma(document_text: &str, position: Position) -> bool {
    // Logic to determine if a comma should be inserted
    // 1. Find if the cursor is at the end of a key-value pair
    // 2. Confirm that no comma follows the current key-value
    // 3. Only return true if conditions are met
    true // Placeholder; needs actual implementation
}

async fn get_document_text(uri: &Url) -> String {
    // Retrieve the document's content; in real usage, it fetches from the editor's cache
    "{}".to_string()
}
```

### Explanation

- **`on_type_formatting`**: This handler is invoked when `\n` is pressed.
- **`TextEdit`**: If a comma needs to be inserted, a `TextEdit` object defines the position and character to insert (in this case, a comma).
- **`should_insert_comma`**: This helper function checks the context within the document to determine if a comma should be added. It would analyze the character before the `\n` and confirm the next character isn’t already a comma or bracket.

### Integrating into Your Editor

Once implemented, you’ll need to make sure your editor is configured to call `onTypeFormatting` with `\n` as a trigger character in JSON files. Many editors support this configuration in `.lsp` or settings files.

Using this approach with `onTypeFormatting` will allow your language server to proactively suggest syntax adjustments and improve the overall editing experience for JSON files.