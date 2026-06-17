fn parse_inline_elements(line: &str) -> String {
    let mut process_line = String::with_capacity(line.len());
    for (index, segment) in line.split("**").enumerate() {
        if index % 2 == 1 { //chia dong theo **, neu index la so le thi no nam ngoai **, con neu so chan thi no nam trong **
            if !segment.is_empty() {
                process_line.push_str("<strong>");
                process_line.push_str(segment);
                process_line.push_str("</strong>");
            }
        } else {
            process_line.push_str(segment);
        }
    }

    let mut italic_process = String::with_capacity(process_line.len());
    for (index, segment) in process_line.split("*").enumerate() {
        if index % 2 == 1 {
            if !segment.is_empty() {
                italic_process.push_str("<em>");
                italic_process.push_str(segment);
                italic_process.push_str("</em>");
            }
        } else {
            italic_process.push_str(segment);
        }
    }
    italic_process
}
// xin chao **viet nam**, toi *yeu* viet nam


pub fn parse(markdown_input: &str) -> String {

    let mut html_output = String::new();

    let lines = markdown_input.lines();

    for line in lines {
        if line.starts_with("## "){
            let raw_content = line.strip_prefix("## ").unwrap();
            let processed_content = parse_inline_elements(raw_content);
            let h2_tag = format!("<h2>{}</h2>\n", processed_content);

            html_output.push_str(&h2_tag);
        }
        else if line.starts_with("# "){
            let raw_content = line.strip_prefix("# ").unwrap();
            let processed_content = parse_inline_elements(raw_content);
            let h1_tag = format!("<h1>{}</h1>\n", processed_content);

            html_output.push_str(&h1_tag);
        }
        else {
            if !line.is_empty() {
                let processed_content = parse_inline_elements(line);
                let p_tag = format!("<p>{}</p>\n", processed_content);
                html_output.push_str(&p_tag);
            }
        }
    }

    html_output
}
