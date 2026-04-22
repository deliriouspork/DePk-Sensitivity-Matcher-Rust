fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_style("fluent-dark".into()); // i dont give a shit if you dont want dark mode
    slint_build::compile_with_config("ui/mainwindow.slint", config).unwrap();
}
