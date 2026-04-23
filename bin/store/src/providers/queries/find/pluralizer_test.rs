#[cfg(test)]
mod tests {
    use crate::utils::helpers::pluralize_wrapper;

    #[test]
    fn test_pluralize_classroom() {
        let result = pluralize_wrapper("classroom", 2, Some(false));
        assert_eq!(result, "classrooms");
    }

    #[test]
    fn test_pluralize_activities() {
        let result = pluralize_wrapper("activities", 2, Some(false));
        assert_eq!(result, "activities");
    }

    #[test]
    fn test_pluralize_school_preserves_case() {
        let result = pluralize_wrapper("School", 2, Some(false));
        assert_eq!(result, "Schools");
    }

    #[test]
    fn test_pluralize_stuff() {
        let result = pluralize_wrapper("Stuff", 2, Some(false));
        assert_eq!(result, "Stuffs");
    }

    #[test]
    fn test_pluralize_part() {
        let result = pluralize_wrapper("Part", 2, Some(false));
        assert_eq!(result, "Parts");
    }

    #[test]
    fn test_pluralize_student_stats_last_activity() {
        let result = pluralize_wrapper("student_stats_last_activity", 2, Some(false));
        assert_eq!(result, "student_stats_last_activities");
    }
}
