use crate::{ file::{ FileError, open }, parser::{ Value, parse }, tokenizer::tokenize };

pub fn get(path: &str, search: &str) -> Result<Option<Value>, FileError> {
    let contents = open(path)?;
    let tokens = tokenize(&contents);
    let list = parse(&contents, &tokens);
    let keys: Vec<&str> = search.split('.').collect();

    if keys.len() == 1 {
        return Ok(list.get(keys[0]).cloned());
    } else if keys.len() > 1 {
        let mut pos = 0;
        let mut cur = &list;
        while pos < keys.len() {
            if pos + 1 == keys.len() {
                return Ok(cur.get(keys[pos]).cloned());
            } else if let Some(Value::Struct(nlist)) = cur.get(keys[pos]) {
                cur = nlist;
                pos += 1;
            } else {
                break;
            }
        }

        return Ok(None);
    }

    Ok(None)
}
