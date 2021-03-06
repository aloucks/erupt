use crate::{
    comment_gen::DocCommentGen,
    items::functions::{ExtensionType, Function, Requirement},
    name::{Name, TypeName},
    origin::Origin,
    source::Source,
};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Alias {
    pub origin: Option<Origin>,
    pub extension_type: Option<ExtensionType>,
    pub requirements: Vec<Requirement>,
    pub name: Name,
    pub alias: Name,
}

impl Alias {
    pub fn new(name: Name, alias: Name) -> Alias {
        Alias {
            origin: None,
            extension_type: None,
            requirements: Vec::new(),
            name,
            alias,
        }
    }

    /// Resolves this alias to the target it is pointing to
    pub fn resolve<'a>(&'a self, source: &'a Source) -> &'a Name {
        let mut name = &self.alias;
        while let Some(alias) = source.find_alias(name) {
            name = &alias.alias;
        }

        name
    }

    pub fn resolve_to_type_name<'a>(&'a self, source: &'a Source) -> &TypeName {
        match self.resolve(source) {
            Name::Type(name) => name,
            _ => panic!("Alias is not pointing to a type name"),
        }
    }

    pub fn emulate_function(&self, source: &Source) -> Option<Function> {
        if let Alias {
            origin,
            extension_type,
            requirements,
            name: Name::Function(name),
            ..
        } = &self
        {
            match source.find_function(match self.resolve(source) {
                Name::Function(name) => name,
                _ => unreachable!(),
            }) {
                Some(function) => {
                    let mut function = function.clone();
                    function.origin = origin.clone();
                    function.extension_type = extension_type.clone();
                    function.requirements = requirements.clone();
                    function.name = name.clone();
                    Some(function)
                }
                None => panic!("Invalid function alias: {:?}", self),
            }
        } else {
            None
        }
    }

    pub fn tokens(
        &self,
        comment_gen: &DocCommentGen,
        source: &Source,
    ) -> HashMap<Origin, TokenStream> {
        let origin = self.origin.as_ref().expect("Alias missing origin");

        let mut is_builder_alias = false;
        let mut builder_tokens = None;
        let mut doc_alias_code = None;
        if let Alias {
            name: Name::Type(name),
            alias: Name::Type(alias),
            ..
        } = &self
        {
            // only generate the attribute for type aliases
            let doc_alias = self.name.original();
            doc_alias_code = Some(quote! { #[doc(alias = #doc_alias)] });

            is_builder_alias = name.builder || alias.builder;
            if !is_builder_alias {
                if let Some(structure) = source.find_structure(self.resolve_to_type_name(source)) {
                    if structure.qualifies_as_builder() {
                        let mut builder_alias = self.clone();
                        builder_alias.name = Name::Type(name.clone().set_builder(true));
                        builder_alias.alias = Name::Type(alias.clone().set_builder(true));
                        builder_tokens = builder_alias.tokens(comment_gen, source).remove(origin);
                    }
                }
            }
        }

        let ident = self.name.ident();
        let alias = self.alias.path(source);
        let doc = comment_gen.def(Some(self.name.original()), "Alias", self.origin.as_ref());
        let lifetime = is_builder_alias.then(|| quote! { <'a> });

        let mut tokens = HashMap::new();
        tokens.insert(
            origin.clone(),
            quote! {
                #[doc = #doc]
                #doc_alias_code
                #[allow(non_camel_case_types)]
                pub type #ident #lifetime = #alias #lifetime;

                #builder_tokens
            },
        );

        tokens
    }
}
