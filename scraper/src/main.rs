use scraper::{Html, Selector};


fn main() {

    let html = r#"
        <ul>
            <li>Foo</li>
            <li>Bar</li>
            <li>Baz</li>
        </ul>
    "#;

    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse("li").unwrap();

    for element in fragment.select(&selector) {
        assert_eq!("ul", element.value().name());
    }
}
