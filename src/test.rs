use crate::{consistent_sort, cut_pipes, PipeBlank};

#[test]
fn test1() {
    let blanks = vec![10, 10];
    let requests = vec![5, 5, 8];
    let expected = [vec![10, 5, 5], vec![10, 8]];

    let mut blanks = cut_pipes(&blanks, requests.to_vec());
    consistent_sort(&mut blanks);

    assert_eq!(blanks, parse_expected(&expected))
}

#[test]
fn test2() {
    let blanks = vec![20, 20, 20];
    let requests = vec![4, 4, 4, 6, 10];
    let expected = [vec![20, 10, 6, 4], vec![20, 4, 4], vec![20]];

    let mut blanks = cut_pipes(&blanks, requests.to_vec());
    consistent_sort(&mut blanks);

    assert_eq!(blanks, parse_expected(&expected))
}

#[test]
fn test3() {
    let blanks = vec![12, 12, 12];
    let requests = vec![6, 6, 6, 6];
    let expected = [vec![12, 6, 6], vec![12, 6, 6], vec![12]];

    let mut blanks = cut_pipes(&blanks, requests.to_vec());
    consistent_sort(&mut blanks);

    assert_eq!(blanks, parse_expected(&expected))
}

#[test]
fn test4() {
    let blanks = vec![15, 15];
    let requests = vec![12, 5, 5];
    let expected = [vec![15, 12], vec![15, 5, 5]];

    let mut blanks = cut_pipes(&blanks, requests.to_vec());
    consistent_sort(&mut blanks);

    assert_eq!(blanks, parse_expected(&expected))
}

#[test]
fn test5() {
    let blanks = vec![50, 50];
    let requests = vec![10, 20, 30, 40];
    let expected = [vec![50, 40, 10], vec![50, 30, 20]];

    let mut blanks = cut_pipes(&blanks, requests.to_vec());
    consistent_sort(&mut blanks);

    assert_eq!(blanks, parse_expected(&expected))
}

#[test]
fn test6() {
    let blanks = vec![6, 6];
    let requests = vec![7, 5];
    let expected = [vec![6, 5], vec![6]];

    let mut blanks = cut_pipes(&blanks, requests.to_vec());
    consistent_sort(&mut blanks);

    assert_eq!(blanks, parse_expected(&expected))
}

#[test]
fn test7() {
    let blanks = vec![15, 6, 20];
    let requests = vec![9, 9, 6];

    let mut blanks = cut_pipes(&blanks, requests.to_vec());
    consistent_sort(&mut blanks);

    let expected = [vec![6, 6], vec![15, 9], vec![20, 9]];
    assert_ne!(blanks, parse_expected(&expected));
    
    let expected = [vec![6], vec![15, 9, 6], vec![20, 9]];
    assert_eq!(blanks, parse_expected(&expected));
}

#[test]
fn test8() {
    let blanks = vec![8, 8];
    let requests = vec![10, 6];
    let expected = [vec![8, 6], vec![8]];

    let mut blanks = cut_pipes(&blanks, requests.to_vec());
    consistent_sort(&mut blanks);

    assert_eq!(blanks, parse_expected(&expected));
}

#[test]
fn test9() {
    let blanks = vec![11, 10];
    let requests = vec![6, 5, 5];

    let mut blanks = cut_pipes(&blanks, requests.to_vec());
    consistent_sort(&mut blanks);

    let expected = [vec![11, 6, 5], vec![10, 5]];
    assert_ne!(blanks, parse_expected(&expected));

    let expected = [vec![10, 6], vec![11, 5, 5]];
    assert_eq!(blanks, parse_expected(&expected));
}

#[test]
fn test10() {
    let blanks = vec![10, 10, 10];
    let requests = vec![3, 3, 3];
    let expected = [vec![10, 3, 3, 3], vec![10], vec![10]];

    let mut blanks = cut_pipes(&blanks, requests.to_vec());
    consistent_sort(&mut blanks);

    assert_eq!(blanks, parse_expected(&expected));
}

#[test]
fn test11() {
    let blanks = vec![15, 15, 15];
    let requests = vec![5, 10, 15];
    let expected = [vec![15, 15], vec![15], vec![15, 10, 5]];

    let mut blanks = cut_pipes(&blanks, requests.to_vec());
    consistent_sort(&mut blanks);

    assert_eq!(blanks, parse_expected(&expected));
}

#[test]
fn test12() {
    let blanks = vec![20, 20];
    let requests = vec![10, 10, 10, 10];
    let expected = [vec![20, 10, 10], vec![20, 10, 10]];

    let mut blanks = cut_pipes(&blanks, requests.to_vec());
    consistent_sort(&mut blanks);

    assert_eq!(blanks, parse_expected(&expected));
}

#[test]
fn test13() {
    let blanks = vec![10, 15, 20];
    let requests = vec![9, 6, 6, 5];
    let expected = [vec![10, 9], vec![15, 6, 6], vec![20, 5]];

    let mut blanks = cut_pipes(&blanks, requests.to_vec());
    consistent_sort(&mut blanks);

    assert_eq!(blanks, parse_expected(&expected));
}

#[test]
fn test14() {
    let blanks = vec![3, 10];
    let requests = vec![7, 2];
    let expected = [vec![3], vec![10, 7, 2]];

    let mut blanks = cut_pipes(&blanks, requests.to_vec());
    consistent_sort(&mut blanks);

    assert_eq!(blanks, parse_expected(&expected));
}

#[test]
fn test15() {
    let blanks = vec![8, 8];
    let requests = vec![9];
    let expected = [vec![8], vec![8]];

    let mut blanks = cut_pipes(&blanks, requests.to_vec());
    consistent_sort(&mut blanks);

    assert_eq!(blanks, parse_expected(&expected));
}

/// Parse `PipeBlank` from array: `[ ORIGINAL_SIZE, CUTS... ]`
fn parse_expected(exp: &[Vec<u32>]) -> Vec<PipeBlank> {
    let mut ret = exp.iter()
        .map(|val| {
            let mut val = val.iter();

            // use first as original size
            let original_size = val.next().cloned().unwrap();

            // use remaining as cuts
            let mut cuts = val.cloned().collect::<Vec<_>>();
            cuts.sort();
            cuts.reverse();

            // calculate remaining size
            let total_cut = cuts.iter().fold(0, |acc, x| acc + x);
            let current_size = original_size - total_cut;

            PipeBlank { original_size, current_size, cuts }
        })
        .collect::<Vec<_>>();

    consistent_sort(&mut ret); // make sure all the tests run with the same sorting
    ret
}
