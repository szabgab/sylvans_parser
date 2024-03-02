#[derive(PartialEq, Debug)]
pub enum ParsingError {
    NoSuchFlag(String)
}

pub fn parse(args: &Vec<String>, flags: Vec<String>) -> Vec<Result<String, ParsingError>> {
    let mut params: Vec<Result<String, ParsingError>> = Vec::new();

    // Yes, this is an O(n^2) time algorithm. Yes, it can be way faster. That's okay for now.

    for flag in flags {
        // iterate through args to find this flag!
        let mut found = false;
        for i in 0..(args.len() - 1) {
            if args[i] == flag {
                params.push(Ok(args[i + 1].clone()));
                found = true;
                break;
            }
        }

        if found {
            continue;
        }

        // if we get through the for loop, we couldn't find the argument!
        params.push(Err(ParsingError::NoSuchFlag(flag)));
    }

    params
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {

        let args = vec!["this".to_string(), "is".to_string(), "-a".to_string(), "test".to_string(), "and".to_string(), "thats".to_string(), "-i".to_string(), "the_path".to_string()];
        let flags = vec!["-i".to_string(), "-a".to_string(), "-q".to_string()];
        let expected = vec![Ok("the_path".to_string()), Ok("test".to_string()), Err(ParsingError::NoSuchFlag("-q".to_string()))];

        assert_eq!(parse(&args, flags), expected);
    }
}
