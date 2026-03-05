#[cfg(test)]
mod tests {
    use pluralizer::pluralize;

    #[test]
    fn test_pluralize_classroom() {
        let result = pluralize("classroom", 2, false);
        assert_eq!(result, "classrooms");
    }

    #[test]
    fn test_pluralize_activities() {
        let result = pluralize("activities", 2, false);
        assert_eq!(result, "activities");
    }

    #[test]
    fn test_pluralize_school_preserves_case() {
        let result = pluralize("School", 2, false);
        assert_eq!(result, "Schools");
    }

    #[test]
    fn test_pluralize_stuff() {
        let result = pluralize("Stuff", 2, false);
        assert_eq!(result, "Stuffs");
    }

    #[test]
    fn test_pluralize_part() {
        let result = pluralize("Part", 2, false);
        assert_eq!(result, "Parts");
    }

    #[test]
    fn test_pluralize_student_stats_last_activity() {
        let result = pluralize("student_stats_last_activity", 2, false);
        assert_eq!(result, "student_stats_last_activities");
    }
}
