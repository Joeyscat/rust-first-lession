mod tests {

    #[test]
    fn test_trait1() {
        use std::ops::Add;

        #[derive(Debug)]
        struct Complex {
            real: f64,
            imagine: f64,
        }

        impl Complex {
            fn new(real: f64, imagine: f64) -> Self {
                Self { real, imagine }
            }
        }

        impl Add for Complex {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self::new(self.real + rhs.real, self.imagine + rhs.imagine)
            }
        }

        impl Add for &Complex {
            type Output = Complex;

            fn add(self, rhs: Self) -> Self::Output {
                Complex::new(self.real + rhs.real, self.imagine + rhs.imagine)
            }
        }

        impl Add<f64> for &Complex {
            type Output = Complex;

            fn add(self, rhs: f64) -> Self::Output {
                Complex::new(self.real + rhs, self.imagine)
            }
        }

        let c1 = Complex::new(1.0, 1f64);
        let c2 = Complex::new(2 as f64, 3.0);
        println!("{:?}", &c1 + &c2);
        println!("{:?}", &c1 + 5.0);
        println!("{:?}", c1 + c2);
    }

    #[test]
    fn test_formatter() {
        pub trait Formatter {
            fn format(&self, input: &mut String) -> bool;
        }

        struct MarkdownFormatter;
        impl Formatter for MarkdownFormatter {
            fn format(&self, input: &mut String) -> bool {
                input.push_str("\nformatted with Markdown formatter");
                true
            }
        }
        struct RustFormatter;
        impl Formatter for RustFormatter {
            fn format(&self, input: &mut String) -> bool {
                input.push_str("\nformatted with Rust formatter");
                true
            }
        }
        struct HtmlFormatter;
        impl Formatter for HtmlFormatter {
            fn format(&self, input: &mut String) -> bool {
                input.push_str("\nformatted with HTML formatter");
                true
            }
        }

        pub fn format(input: &mut String, formatters: Vec<&dyn Formatter>) {
            for f in formatters {
                f.format(input);
            }
        }

        let mut text = "Hello World!".to_string();
        let html: &dyn Formatter = &HtmlFormatter;
        let rust: &dyn Formatter = &RustFormatter;
        let formatters = vec![html, rust];
        format(&mut text, formatters);

        println!("text: {}", text);
    }
}
