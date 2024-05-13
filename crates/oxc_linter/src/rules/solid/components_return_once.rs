use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{context::LintContext, rule::Rule, AstNode};

#[derive(Debug, Default, Clone)]
pub struct ComponentsReturnOnce;

declare_oxc_lint!(
    /// ### What it does
    ///
    ///
    /// ### Why is this bad?
    ///
    ///
    /// ### Example
    /// ```javascript
    /// ```
    ComponentsReturnOnce,
    nursery, // TODO: change category to `correctness`, `suspicious`, `pedantic`, `perf`, `restriction`, or `style`
             // See <https://oxc-project.github.io/docs/contribute/linter.html#rule-category> for details
);

impl Rule for ComponentsReturnOnce {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {}
}

#[test]
fn test() {
    use crate::tester::Tester;
    use std::path::PathBuf;

    let pass = vec![
        "function Component() {
			      return <div />;
			    }",
        "function someFunc() {
			      if (condition) {
			        return 5;
			      }
			      return 10;
			    }",
        "function notAComponent() {
			      if (condition) {
			        return <div />;
			      }
			      return <div />;
			    }",
        "callback(() => {
			      if (condition) {
			        return <div />;
			      }
			      return <div />;
			    });",
        "function Component() {
			      const renderContent = () => {
			        if (false) return <></>;
			        return <></>;
			      }
			      return <>{renderContent()}</>;
			    }",
        "function Component() {
			      function renderContent() {
			        if (false) return <></>;
			        return <></>;
			      }
			      return <>{renderContent()}</>;
			    }",
        "function Component() {
			      const renderContent = () => {
			        const renderContentInner = () => {
			          // ifs in render functions are fine no matter what nesting level this is
			          if (false) return;
			          return <></>;
			        };
			        return <>{renderContentInner()}</>;
			      };
			      return <></>;
			    }",
    ];

    let fail = vec![
        "function Component() {
			        if (condition) {
			          return <div />;
			        };
			        return <span />;
			      }",
        "const Component = () => {
			        if (condition) {
			          return <div />;
			        }
			        return <span />;
			      }",
        "function Component() {
			  return Math.random() > 0.5 ? <div>Big!</div> : <div>Small!</div>;
			}",
        r#"function Component() {
			  return Math.random() > 0.5 ? <div>Big!</div> : "Small!";
			}"#,
        "function Component() {
			  return Math.random() > 0.5 ? (
			    <div>
			      Big!
			      No, really big!
			    </div>
			  ) : <div>Small!</div>;
			}",
        "function Component(props) {
			  return props.cond1 ? (
			    <div>Condition 1</div>
			  ) : Boolean(props.cond2) ? (
			    <div>Not condition 1, but condition 2</div>
			  ) : (
			    <div>Neither condition 1 or 2</div>
			  );
			}",
        "function Component(props) {
			  return !!props.cond && <div>Conditional</div>;
			}",
        "function Component(props) {
			  return props.primary || <div>{props.secondaryText}</div>;
			}",
        "HOC(() => {
			        if (condition) {
			          return <div />;
			        }
			        return <div />;
			      });",
    ];

    Tester::new(ComponentsReturnOnce::NAME, pass, fail).test_and_snapshot();
}
