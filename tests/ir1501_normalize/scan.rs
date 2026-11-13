// SPDX-License-Identifier: AGPL-3.0-or-later
//! The allow-list scan of FR-401-AC-4 over the normalization module's text,
//! with comments, strings and attributes removed.

const CALLEES: [&str; 14] = [
    "Some",
    "Ok",
    "Err",
    "usize::from",
    "u32::from",
    "u8::try_from",
    "checked_add",
    "checked_sub",
    "wrapping_add",
    "wrapping_sub",
    "min",
    "max",
    // Macros, named with their bang.
    "trip!",
    "entry!",
];
const BANNED_WORDS: [&str; 10] = [
    "String", "Vec", "Box", "alloc", "dyn", "Fn", "FnMut", "FnOnce", "while", "loop",
];
const KEYWORDS: [&str; 9] = [
    "if", "match", "for", "in", "return", "let", "else", "as", "while",
];

pub fn module_source() -> String {
    std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/native_text/normalize.rs"
    ))
    .unwrap()
}

fn site_list() -> Vec<(String, String)> {
    let trips = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/native_text/normalize/trips.rs"
    ))
    .unwrap();
    let start = trips.find("SITE_LIST").unwrap();
    let block = &trips[start..];
    let block = &block[..block.find("];").unwrap()];
    let mut sites = Vec::new();
    for line in block.lines() {
        let quoted: Vec<&str> = line.split('"').collect();
        if quoted.len() >= 4 {
            sites.push((quoted[1].to_owned(), quoted[3].to_owned()));
        }
    }
    sites
}

fn strip(source: &str) -> String {
    let chars: Vec<char> = source.chars().collect();
    let mut out = String::new();
    let mut index = 0;
    let mut in_string = false;
    while index < chars.len() {
        let c = chars[index];
        if in_string {
            if c == '"' {
                in_string = false;
                out.push('"');
            }
            index += 1;
        } else if c == '"' {
            in_string = true;
            out.push('"');
            index += 1;
        } else if c == '/' && chars.get(index + 1) == Some(&'/') {
            while index < chars.len() && chars[index] != '\n' {
                index += 1;
            }
        } else if c == '#' && chars.get(index + 1) == Some(&'[') {
            let mut depth = 0;
            while index < chars.len() {
                match chars[index] {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    _ => {}
                }
                index += 1;
                if depth == 0 {
                    break;
                }
            }
        } else {
            out.push(c);
            index += 1;
        }
    }
    out
}

/// The text between the brace that opens at `open` and its match.
fn block_at(code: &str, open: usize) -> &str {
    let mut depth = 0;
    for (offset, c) in code[open..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return &code[open + 1..open + offset];
                }
            }
            _ => {}
        }
    }
    &code[open + 1..]
}

/// Comments and strings removed but attributes kept.
fn without_comments(source: &str) -> String {
    let mut out = String::new();
    for line in source.lines() {
        let code = line.split("//").next().unwrap_or("");
        out.push_str(code);
        out.push('\n');
    }
    out
}

/// Findings on the macros and test-only modules of the source: each
/// `macro_rules!` arm is exactly one statement that begins with
/// `#[cfg(test)]`, and `mod trips;` and `mod trip_tests;` are `cfg(test)`.
fn macro_findings(source: &str) -> Vec<String> {
    let raw = without_comments(source);
    let mut findings = Vec::new();
    let mut rest = raw.as_str();
    while let Some(at) = rest.find("macro_rules!") {
        let open = at + rest[at..].find('{').unwrap();
        let body = block_at(rest, open);
        let mut arms = body;
        while let Some(arrow) = arms.find("=> {") {
            let inner = block_at(arms, arrow + 3);
            if !inner.trim_start().starts_with("#[cfg(test)]") || inner.matches(';').count() != 1 {
                findings.push("macro not test-only".to_owned());
            }
            arms = &arms[arrow + 4 + inner.len()..];
        }
        rest = &rest[open + body.len() + 2..];
    }
    for module in ["mod trips;", "mod trip_tests;"] {
        for (at, _) in raw.match_indices(module) {
            if !raw[..at].trim_end().ends_with("#[cfg(test)]") {
                findings.push(format!("{module} not test-only"));
            }
        }
    }
    findings
}

/// The code without `macro_rules!` definitions and `::<..>` turbofish.
fn prepare(source: &str) -> String {
    let mut code = strip(source);
    while let Some(at) = code.find("macro_rules!") {
        let open = at + code[at..].find('{').unwrap();
        let body = block_at(&code, open).to_owned();
        let end = open + body.len() + 2;
        code.replace_range(at..end, "");
    }
    while let Some(at) = code.find("::<") {
        let mut depth = 0;
        let mut end = at;
        for (offset, c) in code[at + 2..].char_indices() {
            match c {
                '<' => depth += 1,
                '>' => {
                    depth -= 1;
                    if depth == 0 {
                        end = at + 2 + offset + 1;
                        break;
                    }
                }
                _ => {}
            }
        }
        code.replace_range(at..end, "");
    }
    code
}

fn is_ident(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn words(code: &str) -> Vec<&str> {
    code.split(|c: char| !is_ident(c))
        .filter(|w| !w.is_empty())
        .collect()
}

struct Function {
    name: String,
    body: String,
}

fn functions(code: &str) -> Vec<Function> {
    let mut found = Vec::new();
    let mut search = 0;
    while let Some(at) = code[search..].find("fn ") {
        let start = search + at;
        search = start + 3;
        if code[..start].chars().last().is_some_and(is_ident) {
            continue;
        }
        let name: String = code[start + 3..]
            .chars()
            .take_while(|c| is_ident(*c))
            .collect();
        let Some(open) = code[start..].find('{') else {
            continue;
        };
        found.push(Function {
            name,
            body: block_at(code, start + open).to_owned(),
        });
    }
    found
}

/// Every call in `body` as a path, a method name, or a macro name with bang.
fn calls_in(body: &str) -> Vec<String> {
    let chars: Vec<char> = body.chars().collect();
    let mut calls = Vec::new();
    for (index, c) in chars.iter().enumerate() {
        if *c != '(' {
            continue;
        }
        let mut end = index;
        let bang = end > 0 && chars[end - 1] == '!';
        if bang {
            end -= 1;
        }
        let mut start = end;
        while start > 0 && (is_ident(chars[start - 1]) || chars[start - 1] == ':') {
            start -= 1;
        }
        if start == end || chars[start].is_numeric() {
            continue;
        }
        let mut name: String = chars[start..end].iter().collect();
        if bang {
            name.push('!');
        }
        if !KEYWORDS.contains(&name.as_str()) {
            calls.push(name);
        }
    }
    calls
}

fn array_names(code: &str) -> (Vec<String>, Vec<String>) {
    let chars: Vec<char> = code.chars().collect();
    let mut arrays = Vec::new();
    let mut tables = Vec::new();
    for at in 0..chars.len().saturating_sub(2) {
        if chars[at] != ':' || chars[at + 1] != ' ' || (at > 0 && chars[at - 1] == ':') {
            continue;
        }
        let mut end = at;
        while end > 0 && is_ident(chars[end - 1]) {
            end -= 1;
        }
        if end == at {
            continue;
        }
        let name: String = chars[end..at].iter().collect();
        let ty: String = chars[at + 2..].iter().take(14).collect();
        let ty = ty.trim_start_matches("&mut ").trim_start_matches('&');
        if ty.starts_with("[[") {
            tables.push(name.clone());
        }
        if ty.starts_with('[') && !ty.starts_with("[u8]") || ty.starts_with("[u8]") {
            arrays.push(name);
        }
    }
    (arrays, tables)
}

fn operand_is_array(token: &str, next: Option<char>, arrays: &[String], tables: &[String]) -> bool {
    let token = token.trim_start_matches(['&', '*']);
    let token = token.trim_start_matches("mut ");
    let last = token.rsplit(['.', ':']).next().unwrap_or("");
    if last.is_empty() {
        return false;
    }
    match next {
        Some('[') => tables.iter().any(|t| t == last),
        Some('(') | Some('.') => false,
        _ => arrays.iter().any(|a| a == last) || tables.iter().any(|t| t == last),
    }
}

fn comparisons(code: &str, arrays: &[String], tables: &[String], out: &mut Vec<String>) {
    let chars: Vec<char> = code.chars().collect();
    for at in 0..chars.len() {
        let width = match (chars[at], chars.get(at + 1)) {
            ('=' | '!' | '<' | '>', Some('=')) => 2,
            ('<' | '>', _) => 1,
            _ => continue,
        };
        let before = at.checked_sub(1).map(|i| chars[i]);
        let after = chars.get(at + width).copied();
        if before != Some(' ') || after != Some(' ') {
            continue;
        }
        let mut left_end = at - 1;
        while left_end > 0 && chars[left_end - 1] == ' ' {
            left_end -= 1;
        }
        let mut left_start = left_end;
        while left_start > 0
            && (is_ident(chars[left_start - 1]) || ".:&*".contains(chars[left_start - 1]))
        {
            left_start -= 1;
        }
        let left: String = chars[left_start..left_end].iter().collect();
        let left_next = chars.get(left_end).copied();
        let mut right_start = at + width + 1;
        while chars.get(right_start) == Some(&' ') {
            right_start += 1;
        }
        let mut right_end = right_start;
        while chars
            .get(right_end)
            .is_some_and(|c| is_ident(*c) || ".:&*".contains(*c))
        {
            right_end += 1;
        }
        let right: String = chars[right_start..right_end].iter().collect();
        let right_next = chars.get(right_end).copied();
        // An operand that is an element of a table whose elements are arrays.
        let left_is_index = left_end > 0
            && chars[left_end - 1] == ']'
            && (0..left_end)
                .rev()
                .find(|i| chars[*i] == '[')
                .is_some_and(|open| {
                    let mut s = open;
                    while s > 0 && is_ident(chars[s - 1]) {
                        s -= 1;
                    }
                    let name: String = chars[s..open].iter().collect();
                    tables.contains(&name)
                });
        if operand_is_array(&left, left_next, arrays, tables)
            || operand_is_array(&right, right_next, arrays, tables)
            || left_is_index
        {
            out.push("array comparison".into());
        }
    }
}

fn min_max_on_arrays(code: &str, arrays: &[String], out: &mut Vec<String>) {
    for method in [".min(", ".max("] {
        let mut rest = code;
        while let Some(at) = rest.find(method) {
            let before: String = rest[..at]
                .chars()
                .rev()
                .take_while(|c| is_ident(*c))
                .collect::<String>()
                .chars()
                .rev()
                .collect();
            let after: String = rest[at + method.len()..]
                .trim_start_matches(['&', '*'])
                .chars()
                .take_while(|c| is_ident(*c))
                .collect();
            if arrays.contains(&before) || arrays.contains(&after) {
                out.push("min/max on arrays".into());
            }
            rest = &rest[at + method.len()..];
        }
    }
}

fn range_in_index(code: &str, out: &mut Vec<String>) {
    let chars: Vec<char> = code.chars().collect();
    for (at, c) in chars.iter().enumerate() {
        if *c == '[' && at > 0 && (is_ident(chars[at - 1]) || chars[at - 1] == ']') {
            let mut depth = 0;
            let mut text = String::new();
            for next in &chars[at..] {
                match next {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    _ => {}
                }
                if depth == 0 {
                    break;
                }
                text.push(*next);
            }
            if text.contains("..") {
                out.push("range in index".into());
            }
        }
    }
}

fn closures(code: &str, out: &mut Vec<String>) {
    let chars: Vec<char> = code.chars().collect();
    for (at, c) in chars.iter().enumerate() {
        if *c != '|' || chars.get(at + 1) == Some(&'|') || (at > 0 && chars[at - 1] == '|') {
            continue;
        }
        let mut back = at;
        while back > 0 && chars[back - 1] == ' ' {
            back -= 1;
        }
        let previous = back.checked_sub(1).map(|i| chars[i]);
        let moved = code[..back].ends_with("move");
        if matches!(previous, Some('(' | ',' | '=')) || moved {
            out.push("closure".into());
        }
    }
}

fn unannotated_table_lets(code: &str, tables: &[String], out: &mut Vec<String>) {
    for table in tables {
        let needle = format!("= {table}[");
        let mut rest = code;
        while let Some(at) = rest.find("let ") {
            rest = &rest[at + 4..];
            let statement = rest.split(';').next().unwrap_or("");
            if let Some(equals) = statement.find(&needle) {
                if !statement[..equals].contains(':') {
                    out.push("unannotated table row".into());
                }
            }
        }
    }
}

fn loops(code: &str, out: &mut Vec<String>) {
    let consts: Vec<&str> = words(code)
        .windows(2)
        .filter(|pair| pair[0] == "const")
        .map(|pair| pair[1])
        .chain(["N", "D", "R"])
        .collect();
    let sites = site_list();
    let mut search = 0;
    while let Some(at) = code[search..].find("for ") {
        let start = search + at;
        search = start + 4;
        if code[..start].chars().last().is_some_and(is_ident) {
            continue;
        }
        let Some(open) = code[start..].find('{') else {
            continue;
        };
        let header = &code[start..start + open];
        let parts: Vec<&str> = header.split_whitespace().collect();
        let bound = (parts.len() == 4 && parts[2] == "in")
            .then(|| parts[3].strip_prefix("0.."))
            .flatten()
            .filter(|name| consts.contains(name));
        let Some(bound) = bound else {
            out.push(format!("loop header {}", header.trim()));
            continue;
        };
        let body = block_at(code, start + open);
        let tracked = sites
            .iter()
            .any(|(site, declared)| declared == bound && body.contains(&format!("trip!({site})")));
        if !tracked {
            out.push(format!("untracked loop {}", header.trim()));
        }
    }
}

fn cycles(functions: &[Function], out: &mut Vec<String>) {
    let names: Vec<&str> = functions.iter().map(|f| f.name.as_str()).collect();
    let edges: Vec<(&str, Vec<String>)> = functions
        .iter()
        .map(|f| {
            let calls = calls_in(&f.body)
                .into_iter()
                .filter(|c| names.contains(&c.as_str()))
                .collect();
            (f.name.as_str(), calls)
        })
        .collect();
    for (start, first) in &edges {
        let mut stack = first.clone();
        let mut seen: Vec<String> = Vec::new();
        while let Some(next) = stack.pop() {
            if next == *start {
                out.push(format!("cycle through {start}"));
                break;
            }
            if seen.contains(&next) {
                continue;
            }
            seen.push(next.clone());
            for (name, calls) in &edges {
                if *name == next {
                    stack.extend(calls.clone());
                }
            }
        }
    }
}

pub fn scan(source: &str) -> Vec<String> {
    let code = prepare(source);
    let mut out = macro_findings(source);
    let word_list = words(&code);
    for banned in BANNED_WORDS {
        if word_list.contains(&banned) {
            out.push(format!("banned word {banned}"));
        }
    }
    if code.contains("..=") {
        out.push("inclusive range".into());
    }
    let functions = functions(&code);
    let own: Vec<&str> = functions.iter().map(|f| f.name.as_str()).collect();
    for function in &functions {
        for call in calls_in(&function.body) {
            let known = own.contains(&call.as_str()) || CALLEES.contains(&call.as_str());
            if !known {
                out.push(format!("callee {call}"));
            }
        }
    }
    let (arrays, tables) = array_names(&code);
    comparisons(&code, &arrays, &tables, &mut out);
    min_max_on_arrays(&code, &arrays, &mut out);
    range_in_index(&code, &mut out);
    closures(&code, &mut out);
    unannotated_table_lets(&code, &tables, &mut out);
    loops(&code, &mut out);
    cycles(&functions, &mut out);
    out.sort();
    out.dedup();
    out
}

/// One seeded violating addition per banned construct, with a word that the
/// scan's finding must contain.
const SEEDS: &[(&str, &str)] = &[
    ("fn seeded(a: &[u32; 4]) { let _ = a.binary_search(&1); }", "callee binary_search"),
    ("fn seeded(a: &[u32; 4]) { let _ = a.contains(&1); }", "callee contains"),
    ("fn seeded(a: &mut [u32; 4], b: &[u32; 4]) { a.copy_from_slice(b); }", "callee copy_from_slice"),
    ("fn seeded(a: &mut [u32; 4]) { a.fill(0); }", "callee fill"),
    ("fn seeded(a: &mut [u32; 4]) { let _ = a.iter_mut(); }", "callee iter_mut"),
    ("fn seeded(a: [u32; 4]) { let _ = a.into_iter(); }", "callee into_iter"),
    ("fn seeded(a: &[u32; 4]) { let _ = a.iter().zip(a.iter()); }", "callee zip"),
    ("fn seeded(a: &[u32; 4]) { let _ = a.windows(2); }", "callee windows"),
    ("fn seeded(s: &str) { let _ = s.chars(); }", "callee chars"),
    ("fn seeded(a: &[u32; 4]) { let _ = a.iter(); }", "callee iter"),
    ("fn seeded(a: &[u32; 4]) { let _ = a.iter().map(|x| x); }", "callee map"),
    ("fn seeded(a: &[u32; 4]) { let _ = a.iter().collect(); }", "callee collect"),
    ("fn seeded(payload: &[u8; 4]) -> bool { payload == OTHER }\nconst OTHER: [u8; 4] = [0; 4];", "array comparison"),
    ("fn seeded(a: &[u8; 4], b: &[u8; 4]) -> bool { &a == &b }", "array comparison"),
    ("fn seeded(a: &[u8; 4], b: &[u8; 4]) -> bool { *a == *b }", "array comparison"),
    ("struct Buf { buf: [u8; 4] }\nfn seeded(x: &Buf, y: &Buf) -> bool { x.buf == y.buf }", "array comparison"),
    ("fn seeded(a: [u8; 4], b: [u8; 4]) -> bool { a < b }", "array comparison"),
    ("fn seeded(a: [u8; 4], b: [u8; 4]) { let _ = a.min(b); }", "min/max on arrays"),
    ("const DECOMP: [[u32; 4]; 8] = [[0; 4]; 8];\nfn seeded(i: usize, j: usize) -> bool { DECOMP[i] == DECOMP[j] }", "array comparison"),
    ("const DECOMP: [[u32; 4]; 8] = [[0; 4]; 8];\nfn seeded(i: usize, other: [u32; 4]) -> bool { let row = DECOMP[i]; row == other }", "unannotated table row"),
    ("fn seeded(a: &[u32; 8], low: usize, high: usize) { let _ = &a[low..high]; }", "range in index"),
    ("fn seeded(n: usize) -> usize { if n == 0 { 0 } else { seeded(n - 1) } }", "cycle through seeded"),
    ("fn seeded_one(n: usize) -> usize { seeded_two(n) }\nfn seeded_two(n: usize) -> usize { seeded_one(n) }", "cycle through seeded_one"),
    ("fn seeded(slice: &[u32; 4]) { for x in slice { trip!(DECODE); } }", "loop header"),
    ("fn seeded(len: usize) { for i in 0..len { trip!(DECODE); } }", "loop header"),
    ("fn seeded() { for i in 0..CANON_STEPS { } }", "untracked loop"),
    ("fn seeded() { for i in 0..CANON_STEPS { trip!(COPY); } }", "untracked loop"),
    ("fn seeded() { for i in 0..=N { trip!(DECODE); } }", "inclusive range"),
    ("fn seeded() { let _ = String::new(); }", "banned word String"),
    ("fn seeded() { let _ = Vec::<u8>::new(); }", "banned word Vec"),
    ("fn seeded() { let _ = Box::new(1); }", "banned word Box"),
    ("fn seeded() { while true { } }", "banned word while"),
    ("fn seeded() { loop { } }", "banned word loop"),
    ("fn seeded(a: u32) -> u32 { let f = |x| x; f(a) }", "closure"),
    ("fn seeded(a: u32) -> u32 { let f = move |x| x; f(a) }", "closure"),
    ("fn seeded(f: &dyn Fn(u32) -> u32) -> u32 { f(1) }", "banned word dyn"),
    ("fn seeded(a: &[u32; 4]) -> usize { a.len() }", "callee len"),
    ("fn seeded() { let _ = alloc::vec::Vec::new(); }", "banned word alloc"),
    ("fn seeded(f: impl Fn(u32)) { }", "banned word Fn"),
    ("fn seeded(f: impl FnMut(u32)) { }", "banned word FnMut"),
    ("fn seeded(f: impl FnOnce(u32)) { }", "banned word FnOnce"),
    ("fn seeded() { let _ = seeded_two(|x| x); }\nfn seeded_two(a: u32) {}", "closure"),
    ("fn seeded() { let _ = seeded_two(1, |x| x); }\nfn seeded_two(a: u32) {}", "closure"),
    ("macro_rules! evil { ($s:ident) => { while true { } trips::hit(trips::$s); }; }", "macro not test-only"),
    ("macro_rules! evil { ($s:ident) => { #[cfg(test)] trips::hit(trips::$s); let _ = String::new(); }; }", "macro not test-only"),
    ("mod trips;", "mod trips; not test-only"),
];

pub fn each_seed_is_rejected() {
    let base = module_source();
    assert_eq!(
        scan(&base),
        Vec::<String>::new(),
        "the base module is clean"
    );
    for (seed, expected) in SEEDS.iter().copied() {
        let found = scan(&format!("{base}\n{seed}\n"));
        assert!(
            found.iter().any(|finding| finding.contains(expected)),
            "seed {seed:?} not rejected as {expected:?}: {found:?}"
        );
    }
}
