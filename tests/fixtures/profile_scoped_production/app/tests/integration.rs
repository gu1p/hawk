#[test]
fn exercises_non_production_surface() {
    internal::profile_union_api();
    internal::debug_api();
    internal::test_only_api();
}
