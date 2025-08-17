use crate::{
    linter::{EarlyLintPass, LintContext},
    sol::{Severity, SolLint},
};

use solar_ast::{self as ast};

use super::Errors;

declare_forge_lint!(
    USE_ERROR_IN_REQUIRE,
    Severity::Info,
    "error-in-require",
    "use if...revert pattern or custom errors with require statements instead of strings"
);

impl<'ast> EarlyLintPass<'ast> for Errors {
    fn check_expr(&mut self, ctx: &LintContext<'_>, expr: &'ast ast::Expr<'ast>) {
        if let ast::ExprKind::Call(call_expr, args) = &expr.kind {
            if let ast::ExprKind::Ident(id) = &call_expr.kind {
                if id.name.as_str() == "require" && args.len() > 1 {
                    // Check if the second argument is an error string
                    if let Some(require_arg) = args.exprs().nth(1) {
                        if let ast::ExprKind::Lit(err_str, _) = &require_arg.kind {
                            if let ast::LitKind::Str(_, _, _) = &err_str.kind {
                                ctx.emit(&USE_ERROR_IN_REQUIRE, err_str.span);
                            }
                        }
                    }
                }
            }
        }
    }
}
