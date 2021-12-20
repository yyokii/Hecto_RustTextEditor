
mod editor;
mod terminal;
use editor::Editor;
use terminal::Terminal;
use editor::Position;

fn main() {
    Editor::default().run();
}
