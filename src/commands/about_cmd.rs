pub fn execute() {
    let text = "The 'hotspot' CLI contains various tools for inspecting source code to try find those areas that warrant a closer look.
It uses code metrics and combines that with source control history to highlight issues not just due to metrics but those metrics in light of how often and how frequent code changes.
It also considers contributor diversity and warns on bus factor for complex and/or high change code.
Supports: C++, C#, Go, Java, JavaScript, Python, Rust, TypeScript";

    println!("{}", text);
}
