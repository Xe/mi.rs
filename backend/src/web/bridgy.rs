use super::Result;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct Info {
    pub target: String,
    pub author: String,
}

pub fn parse(body: &str) -> Result<Option<Info>> {
    let doc = Html::parse_document(body);
    let target = doc
        .select(&Selector::parse(r#"meta[http-equiv="refresh"]"#).unwrap())
        .next()
        .unwrap()
        .value()
        .attr("content")
        .map(|val| {
            let mut val = val.clone().to_string();
            val.drain(6..val.len()).collect()
        });
    let author = doc
        .select(&Selector::parse(r#"span[class="p-nickname"]"#).unwrap())
        .next()
        .unwrap()
        .inner_html();

    Ok(if target.is_some() {
        Some(Info {
            target: target.unwrap(),
            author: author,
        })
    } else {
        None
    })
}
