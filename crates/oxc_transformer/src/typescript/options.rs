use std::borrow::Cow;

use serde::Deserialize;

use crate::context::Ctx;

fn default_for_jsx_pragma() -> Cow<'static, str> {
    Cow::Borrowed("React.createElement")
}

fn default_for_jsx_pragma_frag() -> Cow<'static, str> {
    Cow::Borrowed("React.Fragment")
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct TypeScriptOptions {
    /// Replace the function used when compiling JSX expressions.
    /// This is so that we know that the import is not a type import, and should not be removed.
    /// defaults to React
    #[serde(default = "default_for_jsx_pragma")]
    pub jsx_pragma: Cow<'static, str>,

    /// Replace the function used when compiling JSX fragment expressions.
    /// This is so that we know that the import is not a type import, and should not be removed.
    /// defaults to React.Fragment
    #[serde(default = "default_for_jsx_pragma_frag")]
    pub jsx_pragma_frag: Cow<'static, str>,
    /// When set to true, the transform will only remove type-only imports (introduced in TypeScript 3.8).
    /// This should only be used if you are using TypeScript >= 3.8.
    pub only_remove_type_imports: bool,
}

impl TypeScriptOptions {
    /// Scan through all comments and find the following pragmas
    ///
    /// * @jsx React.createElement
    /// * @jsxFrag React.Fragment
    ///
    /// The comment does not need to be a jsdoc,
    /// otherwise `JSDoc` could be used instead.
    ///
    /// This behavior is aligned with babel.
    pub(crate) fn update_with_comments(mut self, ctx: &Ctx) -> Self {
        for (_, span) in ctx.trivias.comments() {
            let mut comment = span.source_text(ctx.source_text).trim_start();
            // strip leading jsdoc comment `*` and then whitespaces
            while let Some(cur_comment) = comment.strip_prefix('*') {
                comment = cur_comment.trim_start();
            }
            // strip leading `@`
            let Some(comment) = comment.strip_prefix('@') else { continue };

            // read jsxFrag
            if let Some(pragma_frag) = comment.strip_prefix("jsxFrag").map(str::trim) {
                self.jsx_pragma_frag = Cow::from(pragma_frag.to_string());
                continue;
            }

            // Put this condition at the end to avoid breaking @jsxXX
            // read jsx
            if let Some(pragma) = comment.strip_prefix("jsx").map(str::trim) {
                self.jsx_pragma = Cow::from(pragma.to_string());
            }
        }

        self
    }
}

impl Default for TypeScriptOptions {
    fn default() -> Self {
        Self {
            jsx_pragma: default_for_jsx_pragma(),
            jsx_pragma_frag: default_for_jsx_pragma_frag(),
            only_remove_type_imports: false,
        }
    }
}
