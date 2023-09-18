use chrono::{Locale, NaiveDate};
use gray_matter::{ParsedEntity, Pod};

pub struct Article {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub date: String,
}

impl From<ParsedEntity> for Article {
    fn from(parsed_entity: ParsedEntity) -> Self {
        let data = parsed_entity
            .data
            .clone()
            .unwrap()
            .as_hashmap()
            .unwrap_or_default();
        let title = data
            .get("title")
            .unwrap_or(&Pod::String("".to_string()))
            .as_string()
            .unwrap_or_default();
        let slug = data
            .get("slug")
            .unwrap_or(&Pod::String("".to_string()))
            .as_string()
            .unwrap_or_default();
        let content = parsed_entity.content;

        let date = data
            .get("date")
            .unwrap_or(&Pod::String("".to_string()))
            .as_string()
            .unwrap_or_default();

        let mut splitted = date.split('-');

        let year = splitted
            .next()
            .unwrap_or_default()
            .parse::<i32>()
            .unwrap_or_default();
        let month = splitted
            .next()
            .unwrap_or_default()
            .parse::<u32>()
            .unwrap_or_default();
        let day = splitted
            .next()
            .unwrap_or_default()
            .parse::<u32>()
            .unwrap_or_default();

        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        let date = date.format_localized("%e de %B del %Y", Locale::es_ES);

        Self {
            title,
            slug,
            content,
            date: date.to_string(),
        }
    }
}
