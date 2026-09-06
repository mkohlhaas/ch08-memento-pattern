// ================================================== //
// 1. The Memento: A simple, immutable state wrapper. //
// ================================================== //

#[derive(Clone, Debug)]
pub struct TextEditorMemento {
    content: String,
}

// =========================================================================== //
// 2. The Originator: The active object that generates and consumes snapshots. //
// =========================================================================== //

pub struct TextEditor {
    content: String,
}

impl Default for TextEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl TextEditor {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn type_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn print_content(&self) {
        println!("\"{}\"", self.content);
    }

    // Creates the Memento snapshot by cloning internal state data
    pub fn save(&self) -> TextEditorMemento {
        TextEditorMemento {
            content: self.content.clone(),
        }
    }

    // Restores internal state from a Memento
    pub fn restore(&mut self, memento: TextEditorMemento) {
        self.content = memento.content;
    }
}

// ============================================================================ //
// 3. The Caretaker: Manages the history array without touching the inner data. //
// ============================================================================ //

pub struct HistoryCaretaker {
    history: Vec<TextEditorMemento>,
}

impl Default for HistoryCaretaker {
    fn default() -> Self {
        Self::new()
    }
}

impl HistoryCaretaker {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    pub fn save_state(&mut self, memento: TextEditorMemento) {
        self.history.push(memento);
    }

    pub fn undo(&mut self) -> Option<TextEditorMemento> {
        self.history.pop()
    }
}

// ===== //
// Usage //
// ===== //

fn main() {
    let mut editor = TextEditor::new();
    let mut caretaker = HistoryCaretaker::new();

    editor.type_text("Hello, ");
    caretaker.save_state(editor.save());

    editor.type_text("World!");
    caretaker.save_state(editor.save());

    editor.type_text(" This will be undone.");
    editor.print_content(); // "Hello, World! This will be undone."

    if let Some(memento) = caretaker.undo() {
        editor.restore(memento);
    }
    editor.print_content(); // "Hello, World!"

    if let Some(memento) = caretaker.undo() {
        editor.restore(memento);
    }
    editor.print_content(); // "Hello, "
}
