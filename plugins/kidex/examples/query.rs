// Headless harness that mirrors the plugin's matching logic, so the ranking can
// be checked without opening the launcher: cargo run --example query -- plan pdf
use fuzzy_matcher::FuzzyMatcher;

fn main() {
    let input = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    let matcher = fuzzy_matcher::skim::SkimMatcherV2::default().smart_case();
    let tokens = input.split_whitespace().collect::<Vec<_>>();
    if tokens.is_empty() {
        return;
    }

    let mut index = kidex_common::util::get_index(None)
        .unwrap()
        .into_iter()
        .filter_map(|entry| {
            let path = entry.path.as_os_str().to_string_lossy().to_string();
            let mut score = 0;
            for token in &tokens {
                score += matcher.fuzzy_match(&path, token)?;
            }
            Some((path, score))
        })
        .collect::<Vec<_>>();

    index.sort_by(|a, b| b.1.cmp(&a.1));
    for (path, score) in index.iter().take(15) {
        println!("{score:>5}  {path}");
    }
}
