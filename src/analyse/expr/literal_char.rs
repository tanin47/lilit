use parse::tree::{NewInstance, Expr, NativeChar, Char};
use analyse::scope::Scope;
use std::cell::Cell;
use tokenize::span::CharAt;

pub fn apply<'def>(
    char: &mut Char<'def>,
    scope: &mut Scope<'def>,
) {
    char.instance = Some(Box::new(NewInstance {
        name_opt: None,
        args: vec![
            Expr::NewInstance(Box::new(NewInstance {
                name_opt: None,
                args: vec![
                    Expr::NativeChar(Box::new(NativeChar { value: char.span.fragment.char_at(1) }))
                ],
                class_def: Some(scope.find_class("Native__Char").unwrap().parse)
            })),
        ],
        class_def: Some(scope.find_class("Char").unwrap().parse)
    }));
}

#[cfg(test)]
mod tests {
    use std::ops::{Deref, DerefMut};

    use index::build;
    use parse;
    use test_common::span2;
    use analyse::apply;
    use std::cell::{Cell, RefCell};
    use parse::tree::{Char, Expr, NewInstance, NativeChar};

    #[test]
    fn test_simple() {
        let content = r#"
class Native__Char
end

class Char(underlying: Native__Char)
end

def main: Char
  'a'
end
        "#;
        let mut file = unwrap!(Ok, parse::apply(content.trim(), ""));
        let root = build(&[file.deref()]);

        apply(&mut [file.deref_mut()], &root);

        assert_eq!(
            root.find_method("main").exprs.get(0).unwrap(),
            &Expr::Char(Box::new(Char {
                span: span2(8, 3, "'a'", file.deref()),
                instance: Some(Box::new(
                    NewInstance {
                        name_opt: None,
                        args: vec![
                            Expr::NewInstance(Box::new(NewInstance {
                                name_opt: None,
                                args: vec![
                                    Expr::NativeChar(Box::new(NativeChar { value: 'a' }))
                                ],
                                class_def: Some(root.find_class("Native__Char"))
                            })),
                        ],
                        class_def: Some(root.find_class("Char"))
                    }
                ))
            }))
        )
    }
}
