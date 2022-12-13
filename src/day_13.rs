pub fn part_1() -> u32 {
    //
    0
}

#[derive(Clone)]
enum Value {
    Number(usize),
    List(Vec<Box<Value>>),
}

#[derive(PartialEq)]
enum Comparison {
    Ok,
    NotOk,
    Next,
}

fn compare(left: &Value, right: &Value) -> Comparison {
    use Comparison::*;
    use Value::*;

    match (left, right) {
        (Number(left), Number(right)) => {
            if left < right {
                Ok
            } else if left > right {
                NotOk
            } else {
                Next
            }
        }
        (List(left), List(right)) => {
            for (left, right) in left.iter().zip(right.iter()) {
                let result = compare(left, right);

                if result == Ok || result == NotOk {
                    return result;
                }
            }

            compare(&Number(left.len()), &Number(right.len()))
        }
        (List(_), Number(_)) => compare(left, &List(vec![Box::new(right.clone())])),
        (Number(_), List(_)) => compare(&List(vec![Box::new(left.clone())]), right),
    }
}
