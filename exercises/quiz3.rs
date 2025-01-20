// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

// 首先定义一个特征，用于格式化成绩
pub trait PrintGrade {
    fn format_grade(&self) -> String;
}

// 为 f32 实现 PrintGrade 特征
impl PrintGrade for f32 {
    fn format_grade(&self) -> String {
        self.to_string()
    }
}

// 为 String 实现 PrintGrade 特征
impl PrintGrade for String {
    fn format_grade(&self) -> String {
        self.to_string()
    }
}

// 为 &str 实现 PrintGrade 特征
impl PrintGrade for &str {
    fn format_grade(&self) -> String {
        self.to_string()
    }
}

pub struct ReportCard<T: PrintGrade> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: PrintGrade> ReportCard<T> {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name,
            &self.student_age,
            &self.grade.format_grade()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
