use sdkwork_skills_contract::SkillInvocationKind;

#[test]
fn skill_invocation_kind_roundtrip() {
    for value in [
        "local-workflow",
        "process-adapter",
        "mcp-tool",
        "kernel-provider",
    ] {
        let parsed = SkillInvocationKind::parse(value).expect("known kind");
        assert_eq!(parsed.as_str(), value);
    }
}
