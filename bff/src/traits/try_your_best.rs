pub trait TryYourBest<T> {
    type Report: Default;
    fn update_report(
        from: T,
        platform: crate::bigfile::platforms::Platform,
        report: &mut Self::Report,
    );
    fn report(from: T, platform: crate::bigfile::platforms::Platform) -> Self::Report {
        let mut report = <Self::Report as Default>::default();
        Self::update_report(from, platform, &mut report);
        report
    }
}
