//! Askama templates for rendering the application's HTML.
//!
//! [`Base`] provides the shared HTML document shell and accepts the page title
//! used in the document's `<title>` element. [`IndexTemplate`] is the home page
//! template; it extends [`Base`] and fills the content block.

pub use askama::Template;

/// The shared HTML document template.
#[derive(Template)]
#[template(path = "base.html")]
pub struct Base<'a> {
    /// The page title rendered in the document head.
    pub title: &'a str,
}

/// The home page template.
///
/// Extends [`Base`] via `templates/index.html` and overrides the content block.
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    /// The page title rendered in the document head.
    pub title: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn renders_title() {
        let html = Base { title: "Home" }
            .render()
            .expect("base template should render");

        expect!["<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"utf-8\">\n    <title>Home</title>\n</head>\n<body>\n    \n</body>\n</html>"]
            .assert_eq(&html);
    }

    #[test]
    fn escapes_title() {
        let html = Base {
            title: "News < Updates & More",
        }
        .render()
        .expect("base template should render");

        expect!["<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"utf-8\">\n    <title>News &#60; Updates &#38; More</title>\n</head>\n<body>\n    \n</body>\n</html>"]
            .assert_eq(&html);
    }

    #[test]
    fn index_template_renders_base_shell() {
        let html = IndexTemplate { title: "Home" }
            .render()
            .expect("index template should render");

        expect!["<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"utf-8\">\n    <title>Home</title>\n</head>\n<body>\n    \n</body>\n</html>"]
            .assert_eq(&html);
    }
}
