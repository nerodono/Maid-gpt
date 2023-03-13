// use std::fmt::Display;

// #[derive(Clone, Copy)]
// pub enum Mutability {
//     Mutable,
//     Immutable,
// }

// pub struct Ref(pub Mutability);
// pub struct MaybeRef(pub Option<Ref>);
// pub struct FunctionArguments(pub Vec<FunctionArgument>);
// pub struct Meta(pub String);

// pub struct Struct {
//     pub ident: String,
//     pub meta: Meta,

//     pub body: String,
// }

// pub enum FunctionArgument {
//     SelfA {
//         ref_: MaybeRef,
//         mutability: Mutability,
//     },

//     Plain {
//         ref_: MaybeRef,
//         mutability: Mutability,
//         pat: String,
//         ty: String,
//     },
// }

// pub struct Function {
//     pub vis: String,
//     pub ident: String,
//     pub ret_ty: String,
//     pub args: FunctionArguments,
//     pub body: String,
// }

// impl Display for FunctionArgument {
//     fn fmt(
//         &self,
//         f: &mut std::fmt::Formatter<'_>,
//     ) -> std::fmt::Result {
//         match self {
//             Self::SelfA { ref_, mutability } => {
//                 f.write_fmt(format_args!("{mutability}
// {ref_} self"))             }

//             Self::Plain {
//                 ref_,
//                 mutability,
//                 ty,
//                 pat,
//             } => f.write_fmt(format_args!(
//                 "{mutability} {pat}: {ref_} {ty}"
//             )),
//         }
//     }
// }

// impl Display for Function {
//     fn fmt(
//         &self,
//         f: &mut std::fmt::Formatter<'_>,
//     ) -> std::fmt::Result {
//         f.write_fmt(format_args!(
//             "{vis} fn {name}({args}) {{\n {body} \n}}",
//             vis = self.vis,
//             name = self.ident,
//             args = self.args,
//             body = self.body,
//         ))
//     }
// }

// impl Display for FunctionArguments {
//     fn fmt(
//         &self,
//         f: &mut std::fmt::Formatter<'_>,
//     ) -> std::fmt::Result {
//         for argument in &self.0 {
//             f.write_fmt(format_args!("{argument}, "))?;
//         }
//         Ok(())
//     }
// }

// impl Display for MaybeRef {
//     fn fmt(
//         &self,
//         f: &mut std::fmt::Formatter<'_>,
//     ) -> std::fmt::Result {
//         match &self.0 {
//             Some(r) => f.write_fmt(format_args!("{r}")),
//             None => f.write_str(""),
//         }
//     }
// }

// impl Display for Ref {
//     fn fmt(
//         &self,
//         f: &mut std::fmt::Formatter<'_>,
//     ) -> std::fmt::Result {
//         f.write_fmt(format_args!("&{}", self.0))
//     }
// }

// impl Display for Meta {
//     fn fmt(
//         &self,
//         f: &mut std::fmt::Formatter<'_>,
//     ) -> std::fmt::Result {
//         f.write_fmt(format_args!("#[{}]", self.0))
//     }
// }

// impl Display for Mutability {
//     fn fmt(
//         &self,
//         f: &mut std::fmt::Formatter<'_>,
//     ) -> std::fmt::Result {
//         f.write_str(match self {
//             Self::Mutable => "mut",
//             Self::Immutable => "",
//         })
//     }
// }
