use std::sync::Arc;

use gpui::*;

const TEXT: &str = "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy \
    dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The \
    quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog.";

fn new_text_system() -> WindowTextSystem {
    let platform = gpui_platform::current_platform(true);
    WindowTextSystem::new(Arc::new(TextSystem::new(platform.text_system())))
}

fn line_count(text_system: &WindowTextSystem, line_clamp: Option<usize>) -> usize {
    let runs = [TextStyle::default().to_run(TEXT.len())];
    text_system
        .shape_text(TEXT.into(), px(16.), &runs, Some(px(300.)), line_clamp)
        .unwrap()
        .iter()
        .map(|line| line.wrap_boundaries().len() + 1)
        .sum()
}

fn main() {
    let expected = line_count(&new_text_system(), None);
    println!("fresh unclamped: {expected}");

    let text_system = new_text_system();
    let a = line_count(&text_system, None);
    let b = line_count(&text_system, Some(2));
    println!("unclamped then clamp(2): {a}, {b}");

    let text_system = new_text_system();
    let c = line_count(&text_system, Some(2));
    let d = line_count(&text_system, None);
    println!("clamp(2) then unclamped: {c}, {d}");

    let ok = expected > 2 && a == expected && b == 2 && c == 2 && d == expected;
    println!("RESULT: {}", if ok { "PASS" } else { "FAIL" });
    if !ok {
        std::process::exit(1);
    }
}
