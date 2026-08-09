use axum::response::Html;
use template::{IndexTemplate, Template};

pub async fn index_handler() -> Html<String> {
    let html = IndexTemplate { title: "Home" }
        .render()
        .expect("index template should render");
    Html(html)
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[tokio::test]
    async fn index_handler_renders_index_template() {
        let Html(html) = index_handler().await;
        expect!["<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"utf-8\">\n    <title>Home</title>\n</head>\n<body>\n    \n</body>\n</html>"]
            .assert_eq(&html);
    }
}
