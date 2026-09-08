// ================================================== //
// 1. The Memento: A simple, immutable state wrapper. //
// ================================================== //

#[derive(Clone, Debug, Default)]
pub struct TextEditorMemento {
    content: String,
}

// =========================================================================== //
// 2. The Originator: The active object that generates and consumes snapshots. //
// =========================================================================== //

#[derive(Default)]
pub struct TextEditor {
    content: String,
}

impl TextEditor {
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

#[derive(Default)]
pub struct HistoryCaretaker {
    history: Vec<TextEditorMemento>,
}

impl HistoryCaretaker {
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
    let mut editor = TextEditor::default();
    let mut caretaker = HistoryCaretaker::default();

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

// ===== //
// Tests //
// ===== //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editor_defaults_to_empty() {
        let editor = TextEditor::default();
        assert_eq!(editor.content, "");
    }

    #[test]
    fn type_text_accumulates() {
        let mut editor = TextEditor::default();
        editor.type_text("Hello");
        editor.type_text(" World");
        assert_eq!(editor.content, "Hello World");
    }

    #[test]
    fn save_creates_snapshot_of_current_state() {
        let mut editor = TextEditor::default();
        editor.type_text("Hello, World!");
        let memento = editor.save();
        editor.type_text(" This will be undone.");
        editor.restore(memento);
        assert_eq!(editor.content, "Hello, World!");
    }

    #[test]
    fn restore_is_isolated_from_further_edits() {
        let mut editor = TextEditor::default();
        editor.type_text("Original");
        let memento = editor.save();
        editor.type_text("More text");
        editor.restore(memento);
        assert_eq!(editor.content, "Original");
    }

    #[test]
    fn caretaker_defaults_to_empty() {
        let mut caretaker = HistoryCaretaker::default();
        assert_eq!(caretaker.history.len(), 0);
        assert!(caretaker.undo().is_none());
    }

    #[test]
    fn save_state_and_undo_restores_last_snapshot() {
        let mut editor = TextEditor::default();
        let mut caretaker = HistoryCaretaker::default();

        editor.type_text("Hello, ");
        caretaker.save_state(editor.save());

        editor.type_text("World!");
        caretaker.save_state(editor.save());

        // Undo the last change.
        if let Some(memento) = caretaker.undo() {
            editor.restore(memento);
        }
        assert_eq!(editor.content, "Hello, World!");

        // Undo the previous change.
        if let Some(memento) = caretaker.undo() {
            editor.restore(memento);
        }
        assert_eq!(editor.content, "Hello, ");
    }

    #[test]
    fn undo_returns_none_when_history_is_exhausted() {
        let mut caretaker = HistoryCaretaker::default();
        assert!(caretaker.undo().is_none());

        let mut editor = TextEditor::default();
        editor.type_text("One");
        caretaker.save_state(editor.save());
        assert!(caretaker.undo().is_some());
        assert!(caretaker.undo().is_none());
    }
}
