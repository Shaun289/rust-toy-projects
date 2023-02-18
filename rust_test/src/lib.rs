fn match_some_1(option: Option<i32>) -> bool {
    match option {
        Some(val) => {
            if val == 1 {
                return true;
            } else {
                return false;
            }
        }
        None => return false,
    };
}

fn unwrap_some_1(option: Option<i32>) -> bool {
    let val = option.unwrap();
    if val == 1 {
        return true;
    } else {
        return false;
    }
}

fn if_let_some_1(option: Option<i32>) -> bool {
    if let Some(1) = option {
        return true;
    } else {
        return false;
    }
}

fn match_result_1(result: Result<i32, bool>) -> bool {
    match result {
        Ok(val) => {
            if val == 1 {
                return true;
            } else {
                return false;
            }
        }
        Err(b) => return false,
    };
}

fn unwrap_result_1(result: Result<i32, bool>) -> bool {
    let val = result.unwrap();
    if val == 1 {
        return true;
    } else {
        return false;
    }
}

fn if_let_result_1(result: Result<i32, bool>) -> bool {
    if let Ok(1) = result {
        return true;
    } else {
        return false;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_match_some_1() {
        assert!(match_some_1(Some(1)));
        assert!(!match_some_1(Some(2)));
        assert!(!match_some_1(None));
    }

    #[test]
    fn test_unwrap_some_1() {
        assert!(unwrap_some_1(Some(1)));
        assert!(!unwrap_some_1(Some(2)));
        assert!(!unwrap_some_1(None));
    }

    #[test]
    fn test_if_let_some_1() {
        assert!(if_let_some_1(Some(1)));
        assert!(!if_let_some_1(Some(2)));
        assert!(!if_let_some_1(None));
    }

    #[test]
    fn test_match_result_1() {
        assert!(match_result_1(Ok(1)));
        assert!(!match_result_1(Ok(2)));
        assert!(!match_result_1(Err(true)));
        assert!(!match_result_1(Err(false)));
    }
    #[test]
    fn test_unwrap_result_1() {
        assert!(unwrap_result_1(Ok(1)));
        assert!(!unwrap_result_1(Ok(2)));
        assert!(!unwrap_result_1(Err(true)));
        assert!(!unwrap_result_1(Err(false)));
    }
    #[test]
    fn test_if_let_result_1() {
        assert!(if_let_result_1(Ok(1)));
        assert!(!if_let_result_1(Ok(2)));
        assert!(!if_let_result_1(Err(true)));
        assert!(!if_let_result_1(Err(false)));
    }
}
