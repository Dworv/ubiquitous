use bitflags::Flags;
use crate::tools::Tools;

#[test]
fn test_tool_default() {
    let tools = Tools::default();
    assert_eq!(tools.contains(Tools::PEEK_SKILL_REQ), false);
}

#[test]
fn test_tool_edit() {
    let mut tools = Tools::default();
    tools = tools | Tools::PEEK_SKILL_REQ;
    assert_eq!(tools.contains(Tools::PEEK_SKILL_REQ), true);
}