pub trait LangConfig<'a> {
    fn format(&self) -> &'a str;
    fn version_format(&self) -> &'a str;
    fn style(&self) -> &'a str;
    fn symbol(&self) -> &'a str;
    fn disabled(&self) -> bool;
    fn detect_extensions(&self) -> &Vec<&'a str>;
    fn detect_files(&self) -> &Vec<&'a str>;
    fn detect_folders(&self) -> &Vec<&'a str>;
    fn commands(&self) -> &Vec<Vec<&'a str>>;
}

#[macro_export]
macro_rules! impl_lang_config {
    ($struct_name:ident) => {
        impl<'a> LangConfig<'a> for $struct_name<'a> {
            fn format(&self) -> &'a str {
                self.format
            }
            fn version_format(&self) -> &'a str {
                self.version_format
            }
            fn style(&self) -> &'a str {
                self.style
            }
            fn symbol(&self) -> &'a str {
                self.symbol
            }
            fn disabled(&self) -> bool {
                self.disabled
            }
            fn detect_extensions(&self) -> &Vec<&'a str> {
                &self.detect_extensions
            }
            fn detect_files(&self) -> &Vec<&'a str> {
                &self.detect_files
            }
            fn detect_folders(&self) -> &Vec<&'a str> {
                &self.detect_folders
            }
            fn commands(&self) -> &Vec<Vec<&'a str>> {
                &self.commands
            }
        }
    };
}
