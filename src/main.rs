fn main() {
    let org_arr: Vec<u32> = vec![];
    let sub_arr: Vec<u32> = vec![];

    match check_sub_arr(org_arr, sub_arr) {
        true => println!("This is a sub-array"),
        false => println!("This is NOT a sub-array"),
    }
}

fn check_sub_arr(org_arr: Vec<u32>, sub_arr: Vec<u32>) -> bool {
    if org_arr.len() >= sub_arr.len() {
        match sub_arr.len() {
            0 => {
                return true;
            }
            _ => {
                let diff_len = org_arr.len() - sub_arr.len() + 1;
                let mut is_sub = false;
                for i in 0..diff_len {
                    match sub_arr.get(0) {
                        Some(first_element_sub_arr) => {
                            if first_element_sub_arr == &org_arr[i] {
                                is_sub = true;
                                for j in 0..sub_arr.len() {
                                    if org_arr[i + j] != sub_arr[j] {
                                        is_sub = false;
                                        break;
                                    }
                                }
                            }
                        }
                        None => continue,
                    };
                }
                return is_sub;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::check_sub_arr;

    #[test]
    fn two_empty_arrs() {
        let org_arr: Vec<u32> = vec![];
        let sub_arr: Vec<u32> = vec![];
        assert_eq!(check_sub_arr(org_arr, sub_arr), true);
    }

    #[test]
    fn empty_sub_arr() {
        let org_arr: Vec<u32> = vec![1];
        let sub_arr: Vec<u32> = vec![];
        assert_eq!(check_sub_arr(org_arr, sub_arr), true);
    }

    #[test]
    fn empty_org_arr() {
        let org_arr: Vec<u32> = vec![];
        let sub_arr: Vec<u32> = vec![1];
        assert_eq!(check_sub_arr(org_arr, sub_arr), false);
    }

    #[test]
    fn is_sub_arr() {
        let org_arr: Vec<u32> = vec![1, 2, 3, 3, 4, 5, 8, 9];
        let sub_arr: Vec<u32> = vec![4, 5, 8];
        assert_eq!(check_sub_arr(org_arr, sub_arr), true);
    }

    #[test]
    fn is_not_sub_arr() {
        let org_arr: Vec<u32> = vec![3, 2, 3, 1, 9, 3, 1];
        let sub_arr: Vec<u32> = vec![2, 3, 9];
        assert_eq!(check_sub_arr(org_arr, sub_arr), false);
    }
}
