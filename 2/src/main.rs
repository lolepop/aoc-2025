use fancy_regex::Regex;

const INPUT: &str = "5529687-5587329,50-82,374-560,83-113,226375-287485,293169-368713,2034-2634,9945560-9993116,4872472-4904227,3218-5121,1074-1357,15451-26093,483468003-483498602,51513-85385,1466-1992,7600-13034,710570-789399,407363-480868,3996614725-3996662113,3-17,5414907798-5414992881,86274-120443,828669-909588,607353-700604,4242340614-4242556443,28750-44009,935177-1004747,20-41,74678832-74818251,8484825082-8484860878,2784096938-2784156610,5477-7589,621-952,2424167145-2424278200,147085-217900,93043740-93241586";
// const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";


#[inline]
fn dup_num(n: u64) -> u64 {
    let digits = n.ilog10() + 1;
    n * 10u64.pow(digits) + n
}

fn p1(start: u64, end: u64) -> u64 {
    println!("{start}-{end}");

    let start_digits = start.ilog10() + 1;
    let end_digits = end.ilog10() + 1;

    let mut invalid_count = 0;
    for digits in start_digits..=end_digits {
        if digits % 2 != 0 {
            continue;
        }

        let a = 10u64.pow(digits / 2 - 1);
        let b = 10u64.pow(digits / 2) - 1;

        for half in a..=b {
            let invalid_num = dup_num(half);
            if invalid_num < start {
                continue;
            } else if invalid_num > end {
                break;
            }
            // println!("{invalid_num}");
            invalid_count += invalid_num;
        }
    }

    invalid_count
}

fn p2(start: u64, end: u64) -> u64 {
    println!("{start}-{end}");
    let pat = Regex::new(r"^(\d+?)\1+$").unwrap();
    (start..=end).into_iter()
        .filter(|invalid_num| pat.is_match(invalid_num.to_string().as_str()).unwrap())
        .sum()
}

fn exec(f: impl Fn(u64, u64) -> u64) -> u64 {
    INPUT.split(",")
        .map(|pair| {
            let Some((left, right)) = pair.split_once("-") else { panic!("no") };
            let (s, e) = (left.parse().unwrap(), right.parse().unwrap());
            f(s, e)
        })
        .sum()
}

fn main() {
    let part1 = exec(p1);
    let part2 = exec(p2);
    println!("part 1: {part1}");
    println!("part 2: {part2}");
}
