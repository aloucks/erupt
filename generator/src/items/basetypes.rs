use crate::{
    comment_gen::DocCommentGen,
    declaration::{Declaration, Type},
    header::DeclarationInfo,
    name::TypeName,
    origin::Origin,
    source::{NotApplicable, Source},
};
use lang_c::ast::{Declaration as CDeclaration, DeclarationSpecifier, TypeSpecifier};
use proc_macro2::TokenStream;
use quote::quote;
use std::convert::{TryFrom, TryInto};
use treexml::Element;

#[derive(Debug)]
pub struct Basetype {
    pub origin: Option<Origin>,
    pub name: TypeName,
    pub ty: Type,
}

impl Basetype {
    pub fn tokens(&self, comment_gen: &DocCommentGen, source: &Source) -> TokenStream {
        let name = self.name.ident();
        let doc_alias = &self.name.original;

        let ty = self.ty.rust_type(source);
        let doc = comment_gen.def(Some(&self.name.original), "Basetype", None);

        quote! {
            #[doc = #doc]
            #[doc(alias = #doc_alias)]
            pub type #name = #ty;
        }
    }
}

impl TryFrom<&CDeclaration> for Basetype {
    type Error = NotApplicable;

    fn try_from(declaration: &CDeclaration) -> Result<Self, Self::Error> {
        match declaration.declarators.as_slice() {
            [init_declarator] => {
                let declaration = Declaration::from(DeclarationInfo {
                    type_info: declaration.specifiers.as_slice().try_into()?,
                    declarator: Some(&init_declarator.node.declarator.node),
                });

                let ty = declaration.ty;
                declaration.name.ok_or(NotApplicable).map(|name| Basetype {
                    origin: Default::default(),
                    name: TypeName::new(&name),
                    ty,
                })
            }
            [] => match declaration.specifiers.as_slice() {
                [specifier] => match &specifier.node {
                    DeclarationSpecifier::TypeSpecifier(ty) => match &ty.node {
                        TypeSpecifier::Struct(struct_type) => match &struct_type.node.identifier {
                            Some(identifier) => Ok(Basetype {
                                origin: Default::default(),
                                name: TypeName::new(&identifier.node.name),
                                ty: Type::Void,
                            }),
                            _ => Err(NotApplicable),
                        },
                        _ => Err(NotApplicable),
                    },
                    _ => Err(NotApplicable),
                },
                _ => Err(NotApplicable),
            },
            _ => Err(NotApplicable),
        }
    }
}

impl Source {
    pub fn collect_basetype(&mut self, element: &Element) {
        if let Ok(Some(name)) = element.find_value::<String>("name") {
            if let Some(basetype) = self.header.take_basetype(&name) {
                self.basetypes.push(basetype);
            }
        }
    }
}
