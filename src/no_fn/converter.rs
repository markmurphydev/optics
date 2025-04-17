// use std::marker::PhantomData;
// use crate::no_fn::profunctor::{Profunctor, ProfunctorFamily};
// 
// /// HKT for `impl Converter`
// pub trait ConverterFamily {
//     type Converter<A, B>: Converter<A, B>;
// }
// 
// impl <ConvFam> ProfunctorFamily for ConvFam where ConvFam: ConverterFamily {
//     type Pro<X, Y> = ConvFam::Converter<X, Y>;
// }
// 
// // /// Profunctor-family for any `impl Converter<A, B>`
// // struct ConverterProfunctorFamily<ConvFamily>(PhantomData<ConvFamily>);
// // impl <ConvFamily> ProfunctorFamily for ConverterProfunctorFamily<ConvFamily>
// // where ConvFamily: ConverterFamily {
// //     type Pro<X, Y> = ConvFamily::Converter<X, Y>;
// // }
// 
// pub trait Converter<A, B> {
//     type ConverterFamily: ConverterFamily;
// 
//     fn convert(&self, t: A) -> B;
//     fn with<BToC, C>(self, b_to_c: BToC) -> With<Self, BToC, B> where Self: Sized, BToC: Converter<B, C> {
//         With {
//             a_to_b: self,
//             b_to_c,
//             _t: Default::default(),
//         }
//     }
// }
// 
// impl <Conv, X, Y> Profunctor<X, Y> for Conv where Conv: Converter<X, Y> {
//     type Family = Conv::ConverterFamily;
// 
//     fn dimap<Pre, Post, W, Z>(self, pre: Pre, post: Post) -> <Self::Family as ProfunctorFamily>::Pro<W, Z>
//     where
//         Pre: Converter<W, X>,
//         Post: Converter<Y, Z>
//     {
//         pre.with(self).with(post)
//     }
// }
// 
// pub struct With<AToB, BToC, B> {
//     a_to_b: AToB,
//     b_to_c: BToC,
//     _t: PhantomData<B>
// }
// 
// pub struct WithFamily;
// impl ConverterFamily for WithFamily {
//     type Converter<A, B> = With<A, B>;
// }
// 
// impl <AToB, BToC, A, B, C> Converter<A, C> for With<AToB, BToC, B>
// where AToB: Converter<A, B>, BToC: Converter<B, C> {
//     type ConverterFamily = ();
// 
//     fn convert(&self, a: A) -> C {
//         self.b_to_c.convert(self.a_to_b.convert(a))
//     }
// }
// 
// #[cfg(test)]
// mod test {
//     use super::*;
//     
//     #[test]
//     pub fn compose_converters() {
//         #[derive(Debug, PartialEq)]
//         struct A;
//         #[derive(Debug, PartialEq)]
//         struct B;
//         #[derive(Debug, PartialEq)]
//         struct C;
//         
//         struct AToB;
//         impl Converter<A, B> for AToB {
//             fn convert(&self, _a: A) -> B {
//                 B
//             }
//         }
//         
//         struct BToC;
//         impl Converter<B, C> for BToC {
//             fn convert(&self, _a: B) -> C {
//                 C
//             }
//         }
// 
//         let with = AToB.with(BToC);
//         assert_eq!(with.convert(A), C);
//     }
// }