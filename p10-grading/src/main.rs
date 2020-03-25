use std::io;

fn main() {
    println!("Input score (0-100):");
    let mut score = String::new();
    io::stdin()
        .read_line(&mut score)
        .expect("failed to read input");

    let score: i32 = score.trim().parse().expect("score must be a number");

    let grade = grading(score);
    match grade {
        Ok(grade) => println!("grade: {}", grade.to_string()),
        Err(e) => println!("{}", e),
    }
}

#[derive(Debug, PartialEq)]
enum Grade {
    A,
    B,
    C,
    D,
    F,
}

impl ToString for Grade {
    fn to_string(&self) -> String {
        match self {
            Grade::A => "A",
            Grade::B => "B",
            Grade::C => "C",
            Grade::D => "D",
            Grade::F => "F",
        }
            .into()
    }
}

fn grading(score: i32) -> Result<Grade, &'static str> {
    match score {
        0..=60 => Ok(Grade::F),
        61..=70 => Ok(Grade::D),
        71..=80 => Ok(Grade::C),
        81..=90 => Ok(Grade::B),
        91..=100 => Ok(Grade::A),
        _ => Err("invalid score"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_grading() {
        let cases = vec![
            (20, Grade::F),
            (60, Grade::F),
            (65, Grade::D),
            (81, Grade::B),
            (91, Grade::A),
            (100, Grade::A),
        ];

        for (score, grade) in cases.iter() {
            assert_eq!(grading(*score).ok().unwrap(), *grade);
        }
    }

    #[test]
    fn test_grading_invalid() {
        let cases = vec![-10, 101];

        for &score in cases.iter() {
            assert!(grading(score).is_err());
        }
    }
}
