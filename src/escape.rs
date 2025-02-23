use std::borrow;
/// [shell_escape::escape]
pub fn escape(s: borrow::Cow<'_, str>) -> borrow::Cow<'_, str> {
    if !s.is_empty() && !s.contains(|ch|!matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '=' | '/' | ',' | '.' | '+')) {
        return s;
    }
    let mut es = String::with_capacity(s.len() + 2);
    es.push('\'');
    for ch in s.chars() {
        match ch {
            '\'' | '!' => {
                es.push_str("'\\");
                es.push(ch);
                es.push('\'');
            }
            _ => es.push(ch),
        }
    }
    es.push('\'');
    es.into()
}
