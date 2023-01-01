use rand::Rng;

fn main() {
    // 作成するパスワードの長さ
    const PW_LENGTH:usize = 32;

    // 文字種の定義
    let alphabets_u = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let apphabets_l = "abcdefghijklmnopqrstuvwxyz";
    let numbers = "0123456789";
    let symbols = "$=-_";
    let char_grp_list = [alphabets_u, apphabets_l, numbers, symbols];

    let mut rng = rand::thread_rng();

    loop {
        let pw = mkpw(&char_grp_list, PW_LENGTH, &mut rng);

        // 使われている文字種が、定義されている文字種の数と同じであれば、すべて文字種が使われていると判断出来る
        if check_pw(&pw, &char_grp_list) == char_grp_list.len() {
            println!("{}", pw);
            break;
        }
    }
}

fn mkpw(char_grp_list: &[&str], length: usize, rng: &mut rand::rngs::ThreadRng) -> String {
    // 使える文字のリストを作成する
    let char_list: Vec<char> = char_grp_list.iter().fold(String::from(""), |n, i| n + i).chars().collect();
    let char_list_size = char_list.len();

    let mut pw = String::from("");

    for _ in 0..length {
        pw.push(char_list[rng.gen_range(0..char_list_size)]);
    }

    return pw;
}

fn check_pw(s: &String, char_grp_list: &[&str]) -> usize {
    // sに含まれる文字種の数を返す
    return char_grp_list.iter()
        .map(|char_grp| s.chars().any(|i| char_grp.find(i).is_some()))
        .filter(|i| *i).count();
}

#[test]
fn test_check_pw() {
    let char_grp_list = ["abc", "123"];
    let s1 = String::from("a1");
    assert_eq!(check_pw(&s1, &char_grp_list), 2);

    let s2 = String::from("ax");
    assert_eq!(check_pw(&s2, &char_grp_list), 1);

    let s3 = String::from("12");
    assert_eq!(check_pw(&s3, &char_grp_list), 1);

    let s4 = String::from("xy");
    assert_eq!(check_pw(&s4, &char_grp_list), 0);
}
