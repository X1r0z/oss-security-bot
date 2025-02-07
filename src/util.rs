pub fn capture_subject(body: &str) -> Option<String> {
    let re = regex::Regex::new(r#"(?m)^Subject: (.*?)$"#).unwrap();
    let captures = re.captures(body)?;
    let subject = captures.get(1)?;

    Some(subject.as_str().to_string())
}

pub fn capture_content(body: &str) -> Option<String> {
    let re = regex::Regex::new(r#"(?s)<pre style="white-space: pre-wrap">(.*?)</pre>"#).unwrap();
    let captures = re.captures(body)?;
    let content = captures.get(1)?;

    Some(content.as_str().to_string())
}
