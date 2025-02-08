use regex::Regex;

pub fn capture_mails(body: &str) -> Option<Vec<String>> {
    let re = Regex::new("(?m)<a href=\"(\\d{4}/\\d{2}/\\d{2}/\\d+)\">").unwrap();
    let captures = re.captures_iter(body);

    Some(captures.map(|cap| cap[1].to_string()).collect())
}

pub fn capture_subject(body: &str) -> Option<String> {
    let re = Regex::new(r#"(?m)^Subject: (.*?)$"#).unwrap();
    let captures = re.captures(body)?;
    let subject = captures.get(1)?;

    Some(subject.as_str().to_string())
}

pub fn capture_text(body: &str) -> Option<String> {
    let re = Regex::new(r#"(?s)<pre style="white-space: pre-wrap">(.*?)</pre>"#).unwrap();
    let captures = re.captures(body)?;
    let text = captures.get(1)?;

    Some(text.as_str().to_string())
}
