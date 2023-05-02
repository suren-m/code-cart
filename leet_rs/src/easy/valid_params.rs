// https://leetcode.com/problems/valid-parentheses/

// An input string is valid if:
// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']',
// determine if the input string is valid.
// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.

/// The stack data structure can come in handy here in representing this
/// recursive structure of the problem. We can't really process this
/// from the inside out because we don't have an idea about the overall structure.
/// But, the stack can help us process this recursively i.e. from outside to inwards.

// if string len is odd, it's very likely to be invalid
pub fn is_valid_params(s: String) -> bool {
    if s.len() % 2 == 1 {
        return false;
    }
    let mut stack: Vec<char> = Vec::new();
    for ch in s.chars() {
        if ch == '(' || ch == '[' || ch == '{' {
            stack.push(ch);
            continue;
        } else if stack.is_empty() {
            // invalid because starting wtih closing params not allowed
            return false;
        }

        let last_item = *stack.last().unwrap();
        match (last_item, ch) {
            ('(', ')') | ('[', ']') | ('{', '}') => {
                stack.pop();
            }
            _ => {
                stack.push(ch);
            }
        }
    }
    stack.is_empty()
}

// clever approach by pushing matching param
pub fn is_valid_params_v2(s: String) -> bool {
    if s.len() % 2 == 1 {
        return false;
    }
    let mut stack: Vec<char> = Vec::new();
    for ch in s.chars() {
        if ch == '(' {
            stack.push(')');
        } else if ch == '[' {
            stack.push(']')
        } else if ch == '{' {
            stack.push('}')
        }
        // has to start with opening params
        // if most recent item in stack doesn't match closing brace in seq, then also invalid
        else if stack.is_empty() || stack.pop().unwrap() != ch {
            return false;
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    // let mut param_pairs: HashMap<char, char> = HashMap::new();
    //     param_pairs.insert('{', '}');
    //     param_pairs.insert('(', ')');
    //     param_pairs.insert('[', ']');

    use super::*;

    #[test]
    fn is_valid_params_test() {
        //write test here

        assert_eq!(true, is_valid_params("()".to_string()));
        assert_eq!(true, is_valid_params("()[]{}".to_string()));
        assert_eq!(false, is_valid_params("(]".to_string()));
        assert_eq!(true, is_valid_params("(((((())))))".to_string()));
        assert_eq!(false, is_valid_params("(((((((()".to_string()));
        assert_eq!(true, is_valid_params("((()(())))".to_string()));

        assert_eq!(true, is_valid_params_v2("()".to_string()));
        assert_eq!(true, is_valid_params_v2("()[]{}".to_string()));
        assert_eq!(false, is_valid_params_v2("(]".to_string()));
        assert_eq!(true, is_valid_params_v2("(((((())))))".to_string()));
        assert_eq!(false, is_valid_params_v2("(((((((()".to_string()));
        assert_eq!(true, is_valid_params_v2("((()(())))".to_string()));
    }
}
