use log::error;
use rustpython_parser::ast::{Expr, ExprKind, Keyword};

use ruff_diagnostics::{AlwaysAutofixableViolation, Diagnostic};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::types::Range;

use crate::checkers::ast::Checker;
use crate::registry::AsRule;
use crate::rules::flake8_comprehensions::fixes;

use super::helpers;

#[violation]
pub struct UnnecessaryListComprehensionSet;

impl AlwaysAutofixableViolation for UnnecessaryListComprehensionSet {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Unnecessary `list` comprehension (rewrite as a `set` comprehension)")
    }

    fn autofix_title(&self) -> String {
        "Rewrite as a `set` comprehension".to_string()
    }
}

/// C403 (`set([...])`)
pub fn unnecessary_list_comprehension_set(
    checker: &mut Checker,
    expr: &Expr,
    func: &Expr,
    args: &[Expr],
    keywords: &[Keyword],
) {
    let Some(argument) = helpers::exactly_one_argument_with_matching_function("set", func, args, keywords) else {
        return;
    };
    if !checker.ctx.is_builtin("set") {
        return;
    }
    if let ExprKind::ListComp { .. } = &argument {
        let mut diagnostic = Diagnostic::new(UnnecessaryListComprehensionSet, Range::from(expr));
        if checker.patch(diagnostic.kind.rule()) {
            match fixes::fix_unnecessary_list_comprehension_set(
                checker.locator,
                checker.stylist,
                expr,
            ) {
                Ok(fix) => {
                    diagnostic.amend(fix);
                }
                Err(e) => error!("Failed to generate fix: {e}"),
            }
        }
        checker.diagnostics.push(diagnostic);
    }
}
