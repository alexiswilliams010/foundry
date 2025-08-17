use super::PascalCase;
use crate::{
    linter::{EarlyLintPass, LintContext},
    sol::{Severity, SolLint},
};
use solar_ast::{ContractKind, ItemContract, ItemEnum, ItemEvent, ItemStruct};

declare_forge_lint!(
    PASCAL_CASE_STRUCT,
    Severity::Info,
    "pascal-case-struct",
    "structs should use PascalCase"
);

declare_forge_lint!(
    PASCAL_CASE_ENUM,
    Severity::Info,
    "pascal-case-enum",
    "enums should use PascalCase"
);

declare_forge_lint!(
    PASCAL_CASE_EVENT,
    Severity::Info,
    "pascal-case-event",
    "events should use PascalCase"
);

declare_forge_lint!(
    PASCAL_CASE_CONTRACT,
    Severity::Info,
    "pascal-case-contract",
    "contracts should use PascalCase"
);

declare_forge_lint!(
    PASCAL_CASE_LIBRARY,
    Severity::Info,
    "pascal-case-library",
    "libraries should use PascalCase"
);

impl<'ast> EarlyLintPass<'ast> for PascalCase {
    fn check_item_struct(&mut self, ctx: &LintContext<'_>, strukt: &'ast ItemStruct<'ast>) {
        let name = strukt.name.as_str();
        if name.len() > 1 && !is_pascal_case(name) {
            ctx.emit(&PASCAL_CASE_STRUCT, strukt.name.span);
        }
    }

    fn check_item_contract(&mut self, ctx: &LintContext<'_>, contract: &'ast ItemContract<'ast>) {
        let name = contract.name.as_str();
        let invalid_case = name.len() > 1 && !is_pascal_case(name);

        match contract.kind {
            // Interfaces are not required to be PascalCase
            ContractKind::Interface => {}
            ContractKind::AbstractContract | ContractKind::Contract => {
                if invalid_case {
                    ctx.emit(&PASCAL_CASE_CONTRACT, contract.name.span);
                }
            }
            ContractKind::Library => {
                if invalid_case {
                    ctx.emit(&PASCAL_CASE_LIBRARY, contract.name.span);
                }
            }
        }
    }

    fn check_item_enum(&mut self, ctx: &LintContext<'_>, enum_: &'ast ItemEnum<'ast>) {
        let name = enum_.name.as_str();
        if name.len() > 1 && !is_pascal_case(name) {
            ctx.emit(&PASCAL_CASE_ENUM, enum_.name.span);
        }
    }

    fn check_item_event(&mut self, ctx: &LintContext<'_>, event: &'ast ItemEvent<'ast>) {
        let name = event.name.as_str();
        if name.len() > 1 && !is_pascal_case(name) {
            ctx.emit(&PASCAL_CASE_EVENT, event.name.span);
        }
    }
}

/// Check if a string is PascalCase
pub fn is_pascal_case(s: &str) -> bool {
    if s.len() <= 1 {
        return true;
    }

    s == format!("{}", heck::AsPascalCase(s)).as_str()
}
