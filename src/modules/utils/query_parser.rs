use serde_json::Value;

pub fn parse_query(qstr: &str) -> (&str, Option<Value>) {
    qstr.find('?')
        .and_then(|index| {
            let (module_name, query_with_qmark) = qstr.split_at(index);
            let query = queryst::parse(query_with_qmark.get(1..).unwrap()).ok();
            Some((module_name, query))
        })
        .unwrap_or((qstr, None))
}
