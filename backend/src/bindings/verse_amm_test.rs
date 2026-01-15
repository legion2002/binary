///Module containing a contract's types and functions.
/**

```solidity
library StdInvariant {
    struct FuzzArtifactSelector { string artifact; bytes4[] selectors; }
    struct FuzzInterface { address addr; string[] artifacts; }
    struct FuzzSelector { address addr; bytes4[] selectors; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod StdInvariant {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct FuzzArtifactSelector { string artifact; bytes4[] selectors; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzArtifactSelector {
        #[allow(missing_docs)]
        pub artifact: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub selectors: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<4>,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<4>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::String,
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<4>>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<FuzzArtifactSelector> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzArtifactSelector) -> Self {
                (value.artifact, value.selectors)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzArtifactSelector {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    artifact: tuple.0,
                    selectors: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzArtifactSelector {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzArtifactSelector {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.artifact,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::tokenize(&self.selectors),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for FuzzArtifactSelector {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for FuzzArtifactSelector {
            const NAME: &'static str = "FuzzArtifactSelector";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzArtifactSelector(string artifact,bytes4[] selectors)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.artifact,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.selectors)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FuzzArtifactSelector {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.artifact,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.selectors,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.artifact,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<4>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.selectors,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct FuzzInterface { address addr; string[] artifacts; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzInterface {
        #[allow(missing_docs)]
        pub addr: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub artifacts: alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<FuzzInterface> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzInterface) -> Self {
                (value.addr, value.artifacts)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzInterface {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    addr: tuple.0,
                    artifacts: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzInterface {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzInterface {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.addr,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self.artifacts),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for FuzzInterface {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for FuzzInterface {
            const NAME: &'static str = "FuzzInterface";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzInterface(address addr,string[] artifacts)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.addr,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.artifacts)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FuzzInterface {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.addr,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.artifacts,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.addr,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::String,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.artifacts,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct FuzzSelector { address addr; bytes4[] selectors; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzSelector {
        #[allow(missing_docs)]
        pub addr: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub selectors: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<4>,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<4>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<4>>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<FuzzSelector> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzSelector) -> Self {
                (value.addr, value.selectors)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzSelector {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    addr: tuple.0,
                    selectors: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzSelector {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzSelector {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.addr,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::tokenize(&self.selectors),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for FuzzSelector {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for FuzzSelector {
            const NAME: &'static str = "FuzzSelector";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzSelector(address addr,bytes4[] selectors)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.addr,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.selectors)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FuzzSelector {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.addr,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.selectors,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.addr,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<4>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.selectors,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`StdInvariant`](self) contract instance.

See the [wrapper's documentation](`StdInvariantInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> StdInvariantInstance<P, N> {
        StdInvariantInstance::<P, N>::new(address, __provider)
    }
    /**A [`StdInvariant`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`StdInvariant`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct StdInvariantInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for StdInvariantInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("StdInvariantInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StdInvariantInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`StdInvariant`](self) contract instance.

See the [wrapper's documentation](`StdInvariantInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> StdInvariantInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> StdInvariantInstance<P, N> {
            StdInvariantInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StdInvariantInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > StdInvariantInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library StdInvariant {
    struct FuzzArtifactSelector {
        string artifact;
        bytes4[] selectors;
    }
    struct FuzzInterface {
        address addr;
        string[] artifacts;
    }
    struct FuzzSelector {
        address addr;
        bytes4[] selectors;
    }
}

interface VerseAMMTest {
    event log(string);
    event log_address(address);
    event log_array(uint256[] val);
    event log_array(int256[] val);
    event log_array(address[] val);
    event log_bytes(bytes);
    event log_bytes32(bytes32);
    event log_int(int256);
    event log_named_address(string key, address val);
    event log_named_array(string key, uint256[] val);
    event log_named_array(string key, int256[] val);
    event log_named_array(string key, address[] val);
    event log_named_bytes(string key, bytes val);
    event log_named_bytes32(string key, bytes32 val);
    event log_named_decimal_int(string key, int256 val, uint256 decimals);
    event log_named_decimal_uint(string key, uint256 val, uint256 decimals);
    event log_named_int(string key, int256 val);
    event log_named_string(string key, string val);
    event log_named_uint(string key, uint256 val);
    event log_string(string);
    event log_uint(uint256);
    event logs(bytes);

    function IS_TEST() external view returns (bool);
    function QUESTION_HASH() external view returns (bytes32);
    function RESOLUTION_DEADLINE() external view returns (uint32);
    function UNISWAP_V2_FACTORY() external view returns (address);
    function alice() external view returns (address);
    function bob() external view returns (address);
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external view returns (bool);
    function lpProvider() external view returns (address);
    function multiVerse() external view returns (address);
    function oracle() external view returns (address);
    function setUp() external;
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function testCrossAssetVerseTradingOnUniswapV2() external;
    function testMultipleVersePairsOnUniswap() external;
    function testPriceDivergenceBetweenVerses() external;
    function usdc() external view returns (address);
    function weth() external view returns (address);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "IS_TEST",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "QUESTION_HASH",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "RESOLUTION_DEADLINE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "UNISWAP_V2_FACTORY",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IUniswapV2Factory"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "alice",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "bob",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeArtifacts",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedArtifacts_",
        "type": "string[]",
        "internalType": "string[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeContracts",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedContracts_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzSelector[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "selectors",
            "type": "bytes4[]",
            "internalType": "bytes4[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeSenders",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedSenders_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "failed",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "lpProvider",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "multiVerse",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract MultiVerse"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "oracle",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract TrustedOracle"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "setUp",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "targetArtifactSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedArtifactSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzArtifactSelector[]",
        "components": [
          {
            "name": "artifact",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "selectors",
            "type": "bytes4[]",
            "internalType": "bytes4[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetArtifacts",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedArtifacts_",
        "type": "string[]",
        "internalType": "string[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetContracts",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedContracts_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetInterfaces",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedInterfaces_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzInterface[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "artifacts",
            "type": "string[]",
            "internalType": "string[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzSelector[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "selectors",
            "type": "bytes4[]",
            "internalType": "bytes4[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetSenders",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedSenders_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "testCrossAssetVerseTradingOnUniswapV2",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testMultipleVersePairsOnUniswap",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testPriceDivergenceBetweenVerses",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "usdc",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract MockERC20"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "weth",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract MockERC20"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "log",
    "inputs": [
      {
        "name": "",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_address",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "int256[]",
        "indexed": false,
        "internalType": "int256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_bytes",
    "inputs": [
      {
        "name": "",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_bytes32",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_int",
    "inputs": [
      {
        "name": "",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_address",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256[]",
        "indexed": false,
        "internalType": "int256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_bytes",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_bytes32",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_decimal_int",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      },
      {
        "name": "decimals",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_decimal_uint",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "decimals",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_int",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_string",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_uint",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_string",
    "inputs": [
      {
        "name": "",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_uint",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "logs",
    "inputs": [
      {
        "name": "",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod VerseAMMTest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x600c8054600160ff199182168117909255601f8054909116909117905560c06040526005608090815264616c69636560d81b60a05261003d906100fb565b602380546001600160a01b0319166001600160a01b03929092169190911790556040805180820190915260038152623137b160e91b6020820152610080906100fb565b602480546001600160a01b0319166001600160a01b039290921691909117905560408051808201909152600a8152693638283937bb34b232b960b11b60208201526100ca906100fb565b602580546001600160a01b0319166001600160a01b03929092169190911790553480156100f5575f5ffd5b506102a1565b5f6101058261010c565b5092915050565b5f5f8260405160200161011f919061021a565b60408051808303601f190181529082905280516020909101206001625e79b760e01b03198252600482018190529150737109709ecfa91a80626ff3989d68f67f5b1dd12d9063ffa1864990602401602060405180830381865afa158015610188573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101ac9190610230565b6040516318caf8e360e31b8152909250737109709ecfa91a80626ff3989d68f67f5b1dd12d9063c657c718906101e8908590879060040161025d565b5f604051808303815f87803b1580156101ff575f5ffd5b505af1158015610211573d5f5f3e3d5ffd5b50505050915091565b5f82518060208501845e5f920191825250919050565b5f60208284031215610240575f5ffd5b81516001600160a01b0381168114610256575f5ffd5b9392505050565b60018060a01b0383168152604060208201525f82518060408401528060208501606085015e5f606082850101526060601f19601f8301168401019150509392505050565b617789806102ae5f395ff3fe608060405234801561000f575f5ffd5b5060043610610187575f3560e01c80637dc0d1d0116100d9578063b0464fdc11610093578063c09cec771161006e578063c09cec771461032c578063e20c9f711461033f578063fa7626d414610347578063fb47e3a214610354575f5ffd5b8063b0464fdc14610304578063b5508aa91461030c578063ba414fa614610314575f5ffd5b80637dc0d1d01461027457806385226c81146102875780638c809fbf1461029c578063916a17c6146102b457806399d8fae3146102c9578063a4234b4e146102e4575f5ffd5b80633e413bee116101445780633fc8cef31161011f5780633fc8cef3146102225780635af7071a1461023557806366d9a9a0146102575780637140dc731461026c575f5ffd5b80633e413bee146101ff5780633e5e3c23146102125780633f7286f41461021a575f5ffd5b80630a9254e41461018b5780630d23581714610195578063137da30c146101c55780631d828835146101cd5780631ed7831c146101d55780632ade3880146101ea575b5f5ffd5b610193610367565b005b6025546101a8906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b610193610ddc565b61019361104e565b6101dd611492565b6040516101bc91906137a1565b6101f26114f2565b6040516101bc919061381a565b6022546101a8906001600160a01b031681565b6101dd61162e565b6101dd61168c565b6021546101a8906001600160a01b031681565b6102495f5160206177345f395f51905f5281565b6040519081526020016101bc565b61025f6116ea565b6040516101bc919061391b565b61019361184e565b6020546101a8906001600160a01b031681565b61028f61226c565b6040516101bc9190613999565b601f546101a89061010090046001600160a01b031681565b6102bc612337565b6040516101bc91906139f0565b6101a8735c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f81565b6102ef636b36ec8081565b60405163ffffffff90911681526020016101bc565b6102bc612418565b61028f6124f9565b61031c6125c4565b60405190151581526020016101bc565b6024546101a8906001600160a01b031681565b6101dd61267e565b601f5461031c9060ff1681565b6023546101a8906001600160a01b031681565b60405163f877cb1960e01b815260206004820152600b60248201526a11551217d49410d7d5549360aa1b60448201525f5160206176f15f395f51905f52906371ee464d90829063f877cb19906064015f60405180830381865afa1580156103d0573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526103f79190810190613a7b565b630112a8806040518363ffffffff1660e01b8152600401610419929190613b2e565b6020604051808303815f875af1158015610435573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104599190613b4f565b506040516104669061377a565b604051809103905ff08015801561047f573d5f5f3e3d5ffd5b50601f60016101000a8154816001600160a01b0302191690836001600160a01b031602179055506040516104b290613787565b604051809103905ff0801580156104cb573d5f5f3e3d5ffd5b50602080546001600160a01b0319166001600160a01b03929092169190911790556040516012906104fb90613794565b6060808252600d908201526c2bb930b83832b21022ba3432b960991b608082015260a060208201819052600490820152630ae8aa8960e31b60c082015260ff909116604082015260e001604051809103905ff08015801561055e573d5f5f3e3d5ffd5b50602180546001600160a01b0319166001600160a01b039290921691909117905560405160069061058e90613794565b6060808252600890820152672aa9a21021b7b4b760c11b608082015260a060208201819052600490820152635553444360e01b60c082015260ff909116604082015260e001604051809103905ff0801580156105ec573d5f5f3e3d5ffd5b50602280546001600160a01b0319166001600160a01b039283161790556021546023546040516340c10f1960e01b8152918316926340c10f1992610641929091169068056bc75e2d6310000090600401613b66565b5f604051808303815f87803b158015610658575f5ffd5b505af115801561066a573d5f5f3e3d5ffd5b50506021546024546040516340c10f1960e01b81526001600160a01b0392831694506340c10f1993506106ae929091169068056bc75e2d6310000090600401613b66565b5f604051808303815f87803b1580156106c5575f5ffd5b505af11580156106d7573d5f5f3e3d5ffd5b50506021546025546040516340c10f1960e01b81526001600160a01b0392831694506340c10f19935061071b9290911690683635c9adc5dea0000090600401613b66565b5f604051808303815f87803b158015610732575f5ffd5b505af1158015610744573d5f5f3e3d5ffd5b50506022546023546040516340c10f1960e01b81526001600160a01b0392831694506340c10f199350610784929091169064174876e80090600401613b66565b5f604051808303815f87803b15801561079b575f5ffd5b505af11580156107ad573d5f5f3e3d5ffd5b50506022546024546040516340c10f1960e01b81526001600160a01b0392831694506340c10f1993506107ed929091169064174876e80090600401613b66565b5f604051808303815f87803b158015610804575f5ffd5b505af1158015610816573d5f5f3e3d5ffd5b50506022546025546040516340c10f1960e01b81526001600160a01b0392831694506340c10f199350610857929091169065048c2739500090600401613b66565b5f604051808303815f87803b15801561086e575f5ffd5b505af1158015610880573d5f5f3e3d5ffd5b505060235460405163ca669fa760e01b81526001600160a01b0390911660048201525f5160206176f15f395f51905f52925063ca669fa791506024015f604051808303815f87803b1580156108d3575f5ffd5b505af11580156108e5573d5f5f3e3d5ffd5b5050602154601f5460405163095ea7b360e01b81526001600160a01b03928316945063095ea7b3935061092792610100909204909116905f1990600401613b66565b6020604051808303815f875af1158015610943573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109679190613b7f565b5060235460405163ca669fa760e01b81526001600160a01b0390911660048201525f5160206176f15f395f51905f529063ca669fa7906024015f604051808303815f87803b1580156109b7575f5ffd5b505af11580156109c9573d5f5f3e3d5ffd5b5050602254601f5460405163095ea7b360e01b81526001600160a01b03928316945063095ea7b39350610a0b92610100909204909116905f1990600401613b66565b6020604051808303815f875af1158015610a27573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a4b9190613b7f565b506024805460405163ca669fa760e01b81526001600160a01b0390911660048201525f5160206176f15f395f51905f529163ca669fa791015f604051808303815f87803b158015610a9a575f5ffd5b505af1158015610aac573d5f5f3e3d5ffd5b5050602154601f5460405163095ea7b360e01b81526001600160a01b03928316945063095ea7b39350610aee92610100909204909116905f1990600401613b66565b6020604051808303815f875af1158015610b0a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b2e9190613b7f565b506024805460405163ca669fa760e01b81526001600160a01b0390911660048201525f5160206176f15f395f51905f529163ca669fa791015f604051808303815f87803b158015610b7d575f5ffd5b505af1158015610b8f573d5f5f3e3d5ffd5b5050602254601f5460405163095ea7b360e01b81526001600160a01b03928316945063095ea7b39350610bd192610100909204909116905f1990600401613b66565b6020604051808303815f875af1158015610bed573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c119190613b7f565b5060255460405163ca669fa760e01b81526001600160a01b0390911660048201525f5160206176f15f395f51905f529063ca669fa7906024015f604051808303815f87803b158015610c61575f5ffd5b505af1158015610c73573d5f5f3e3d5ffd5b5050602154601f5460405163095ea7b360e01b81526001600160a01b03928316945063095ea7b39350610cb592610100909204909116905f1990600401613b66565b6020604051808303815f875af1158015610cd1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cf59190613b7f565b5060255460405163ca669fa760e01b81526001600160a01b0390911660048201525f5160206176f15f395f51905f529063ca669fa7906024015f604051808303815f87803b158015610d45575f5ffd5b505af1158015610d57573d5f5f3e3d5ffd5b5050602254601f5460405163095ea7b360e01b81526001600160a01b03928316945063095ea7b39350610d9992610100909204909116905f1990600401613b66565b6020604051808303815f875af1158015610db5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610dd99190613b7f565b50565b60408051610100810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e0810191909152601f5460205460405163c2a33c1d60e01b81526001600160a01b0361010090930483169263c2a33c1d92610e69925f5160206177345f395f51905f5292636b36ec80921690600401613ba5565b5f604051808303815f87803b158015610e80575f5ffd5b505af1158015610e92573d5f5f3e3d5ffd5b505060208054604051610ec894505f5160206177345f395f51905f529350636b36ec80926001600160a01b039092169101613ba5565b60408051601f198184030181529181528151602090920191909120808352601f54602154925163a3def92360e01b81526001600160a01b0361010090920482169363a3def92393610f20939190911691600401613b66565b60408051808303815f875af1158015610f3b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f5f9190613be5565b6001600160a01b039081166040808501919091529181166020840152601f546022548451935163a3def92360e01b815261010090920483169363a3def92393610fad93921691600401613b66565b60408051808303815f875af1158015610fc8573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610fec9190613be5565b6001600160a01b039081166080840152166060820181905281516020830151611014926126dc565b61102681602001518260600151612885565b6001600160a01b031660a082015261103d81612b84565b610dd9815f01518260600151612e74565b601f5460205460405163c2a33c1d60e01b81526001600160a01b0361010090930483169263c2a33c1d9261109b925f5160206177345f395f51905f5292636b36ec80921690600401613ba5565b5f604051808303815f87803b1580156110b2575f5ffd5b505af11580156110c4573d5f5f3e3d5ffd5b5050602080546040515f94506110fa93505f5160206177345f395f51905f5292636b36ec80926001600160a01b03169101613ba5565b60408051601f19818403018152908290528051602090910120601f5460215463a3def92360e01b84529193505f9283926001600160a01b0361010090930483169263a3def9239261115392909116908790600401613b66565b60408051808303815f875af115801561116e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111929190613be5565b601f5460225460405163a3def92360e01b81529395509193505f9283926001600160a01b0361010090930483169263a3def923926111d892909116908990600401613b66565b60408051808303815f875af11580156111f3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112179190613be5565b6040516364e329cb60e11b81526001600160a01b0380881660048301528616602482015291935091505f90735c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f9063c9c65396906044016020604051808303815f875af115801561127d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112a19190613c16565b6040516364e329cb60e11b81526001600160a01b038086166004830152841660248201529091505f90735c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f9063c9c65396906044016020604051808303815f875af1158015611305573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113299190613c16565b6040516364e329cb60e11b81526001600160a01b038089166004830152861660248201529091505f90735c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f9063c9c65396906044016020604051808303815f875af115801561138d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113b19190613c16565b90506114075f6001600160a01b0316846001600160a01b031614156040518060400160405280601d81526020017f5945532f4e4f205745544820706169722073686f756c64206578697374000000815250613246565b60408051808201909152601d81527f5945532f4e4f205553444320706169722073686f756c642065786973740000006020820152611451906001600160a01b038416151590613246565b6114885f6001600160a01b0316826001600160a01b0316141560405180606001604052806023815260200161771160239139613246565b5050505050505050565b606060168054806020026020016040519081016040528092919081815260200182805480156114e857602002820191905f5260205f20905b81546001600160a01b031681526001909101906020018083116114ca575b5050505050905090565b6060601e805480602002602001604051908101604052809291908181526020015f905b82821015611625575f84815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b8282101561160e578382905f5260205f2001805461158390613c2f565b80601f01602080910402602001604051908101604052809291908181526020018280546115af90613c2f565b80156115fa5780601f106115d1576101008083540402835291602001916115fa565b820191905f5260205f20905b8154815290600101906020018083116115dd57829003601f168201915b505050505081526020019060010190611566565b505050508152505081526020019060010190611515565b50505050905090565b606060188054806020026020016040519081016040528092919081815260200182805480156114e857602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116114ca575050505050905090565b606060178054806020026020016040519081016040528092919081815260200182805480156114e857602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116114ca575050505050905090565b6060601b805480602002602001604051908101604052809291908181526020015f905b82821015611625578382905f5260205f2090600202016040518060400160405290815f8201805461173d90613c2f565b80601f016020809104026020016040519081016040528092919081815260200182805461176990613c2f565b80156117b45780601f1061178b576101008083540402835291602001916117b4565b820191905f5260205f20905b81548152906001019060200180831161179757829003601f168201915b505050505081526020016001820180548060200260200160405190810160405280929190818152602001828054801561183657602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116117f85790505b5050505050815250508152602001906001019061170d565b601f5460205460405163c2a33c1d60e01b81526001600160a01b0361010090930483169263c2a33c1d9261189b925f5160206177345f395f51905f5292636b36ec80921690600401613ba5565b5f604051808303815f87803b1580156118b2575f5ffd5b505af11580156118c4573d5f5f3e3d5ffd5b5050602080546040515f94506118fa93505f5160206177345f395f51905f5292636b36ec80926001600160a01b03169101613ba5565b60408051601f19818403018152908290528051602090910120601f5460215463a3def92360e01b84529193505f9283926001600160a01b0361010090930483169263a3def9239261195392909116908790600401613b66565b60408051808303815f875af115801561196e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119929190613be5565b601f5460225460405163a3def92360e01b81529395509193505f9283926001600160a01b0361010090930483169263a3def923926119d892909116908990600401613b66565b60408051808303815f875af11580156119f3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a179190613be5565b6025546040516303223eab60e11b81526001600160a01b03909116600482015291935091505f5160206176f15f395f51905f52906306447d56906024015f604051808303815f87803b158015611a6b575f5ffd5b505af1158015611a7d573d5f5f3e3d5ffd5b5050601f54602154604051639ba730a960e01b81526101009092046001600160a01b039081169450639ba730a99350611ac892911690680ad78ebc5ac6200000908a90600401613c67565b5f604051808303815f87803b158015611adf575f5ffd5b505af1158015611af1573d5f5f3e3d5ffd5b5050601f54602254604051639ba730a960e01b81526101009092046001600160a01b039081169450639ba730a99350611b389291169064e8d4a51000908a90600401613c67565b5f604051808303815f87803b158015611b4f575f5ffd5b505af1158015611b61573d5f5f3e3d5ffd5b505050505f5160206176f15f395f51905f526001600160a01b03166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015611baa575f5ffd5b505af1158015611bbc573d5f5f3e3d5ffd5b50506040516364e329cb60e11b81526001600160a01b038088166004830152851660248201525f9250735c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f915063c9c65396906044016020604051808303815f875af1158015611c21573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c459190613c16565b6025546040516303223eab60e11b81526001600160a01b0390911660048201529091505f5160206176f15f395f51905f52906306447d56906024015f604051808303815f87803b158015611c97575f5ffd5b505af1158015611ca9573d5f5f3e3d5ffd5b505060405163a9059cbb60e01b81526001600160a01b038816925063a9059cbb9150611ce490849068056bc75e2d6310000090600401613b66565b6020604051808303815f875af1158015611d00573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d249190613b7f565b5060405163a9059cbb60e01b81526001600160a01b0384169063a9059cbb90611d5890849064ba43b7400090600401613b66565b6020604051808303815f875af1158015611d74573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d989190613b7f565b506025546040516335313c2160e11b81526001600160a01b03918216600482015290821690636a627842906024016020604051808303815f875af1158015611de2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e069190613b4f565b505f5160206176f15f395f51905f526001600160a01b03166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015611e4c575f5ffd5b505af1158015611e5e573d5f5f3e3d5ffd5b50506040516364e329cb60e11b81526001600160a01b038088166004830152851660248201525f9250735c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f915063c9c65396906044016020604051808303815f875af1158015611ec3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ee79190613c16565b6025546040516303223eab60e11b81526001600160a01b0390911660048201529091505f5160206176f15f395f51905f52906306447d56906024015f604051808303815f87803b158015611f39575f5ffd5b505af1158015611f4b573d5f5f3e3d5ffd5b505060405163a9059cbb60e01b81526001600160a01b038816925063a9059cbb9150611f8690849068056bc75e2d6310000090600401613b66565b6020604051808303815f875af1158015611fa2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611fc69190613b7f565b5060405163a9059cbb60e01b81526001600160a01b0384169063a9059cbb90611ffa9084906445d964b80090600401613b66565b6020604051808303815f875af1158015612016573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061203a9190613b7f565b506025546040516335313c2160e11b81526001600160a01b03918216600482015290821690636a627842906024016020604051808303815f875af1158015612084573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120a89190613b4f565b505f5160206176f15f395f51905f526001600160a01b03166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b1580156120ee575f5ffd5b505af1158015612100573d5f5f3e3d5ffd5b505050505f5f836001600160a01b0316630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa158015612142573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121669190613c9e565b50915091505f5f846001600160a01b0316630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa1580156121a9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121cd9190613c9e565b509150915061221e5f6001600160a01b0316876001600160a01b0316141560405180604001604052806015815260200174165154c81c185a5c881cda1bdd5b1908195e1a5cdd605a1b815250613246565b6040805180820190915260148152731393c81c185a5c881cda1bdd5b1908195e1a5cdd60621b602082015261225f906001600160a01b038716151590613246565b5050505050505050505050565b6060601a805480602002602001604051908101604052809291908181526020015f905b82821015611625578382905f5260205f200180546122ac90613c2f565b80601f01602080910402602001604051908101604052809291908181526020018280546122d890613c2f565b80156123235780601f106122fa57610100808354040283529160200191612323565b820191905f5260205f20905b81548152906001019060200180831161230657829003601f168201915b50505050508152602001906001019061228f565b6060601d805480602002602001604051908101604052809291908181526020015f905b82821015611625575f8481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561240057602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116123c25790505b5050505050815250508152602001906001019061235a565b6060601c805480602002602001604051908101604052809291908181526020015f905b82821015611625575f8481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156124e157602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116124a35790505b5050505050815250508152602001906001019061243b565b60606019805480602002602001604051908101604052809291908181526020015f905b82821015611625578382905f5260205f2001805461253990613c2f565b80601f016020809104026020016040519081016040528092919081815260200182805461256590613c2f565b80156125b05780601f10612587576101008083540402835291602001916125b0565b820191905f5260205f20905b81548152906001019060200180831161259357829003601f168201915b50505050508152602001906001019061251c565b6008545f9060ff16156125db575060085460ff1690565b604051630667f9d760e41b81525f905f5160206176f15f395f51905f529063667f9d7090612638907f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d906519985a5b195960d21b90600401613b66565b602060405180830381865afa158015612653573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126779190613b4f565b1415905090565b606060158054806020026020016040519081016040528092919081815260200182805480156114e857602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116114ca575050505050905090565b6025546040516303223eab60e11b81526001600160a01b0390911660048201525f5160206176f15f395f51905f52906306447d56906024015f604051808303815f87803b15801561272b575f5ffd5b505af115801561273d573d5f5f3e3d5ffd5b5050601f54602154604051639ba730a960e01b81526101009092046001600160a01b039081169450639ba730a9935061278892911690680ad78ebc5ac6200000908890600401613c67565b5f604051808303815f87803b15801561279f575f5ffd5b505af11580156127b1573d5f5f3e3d5ffd5b5050601f54602254604051639ba730a960e01b81526101009092046001600160a01b039081169450639ba730a993506127f89291169064e8d4a51000908890600401613c67565b5f604051808303815f87803b15801561280f575f5ffd5b505af1158015612821573d5f5f3e3d5ffd5b505050505f5160206176f15f395f51905f526001600160a01b03166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b15801561286a575f5ffd5b505af115801561287c573d5f5f3e3d5ffd5b50505050505050565b6040516364e329cb60e11b81526001600160a01b038084166004830152821660248201525f908190735c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f9063c9c65396906044016020604051808303815f875af11580156128e8573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061290c9190613c16565b90506001600160a01b0381166129605760405162461bcd60e51b815260206004820152601460248201527314185a5c8818dc99585d1a5bdb8819985a5b195960621b60448201526064015b60405180910390fd5b6025546040516303223eab60e11b81526001600160a01b0390911660048201525f5160206176f15f395f51905f52906306447d56906024015f604051808303815f87803b1580156129af575f5ffd5b505af11580156129c1573d5f5f3e3d5ffd5b505060405163a9059cbb60e01b81526001600160a01b038716925063a9059cbb91506129fc90849068056bc75e2d6310000090600401613b66565b6020604051808303815f875af1158015612a18573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612a3c9190613b7f565b5060405163a9059cbb60e01b81526001600160a01b0384169063a9059cbb90612a7090849064ba43b7400090600401613b66565b6020604051808303815f875af1158015612a8c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ab09190613b7f565b506025546040516335313c2160e11b81526001600160a01b03918216600482015290821690636a627842906024016020604051808303815f875af1158015612afa573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b1e9190613b4f565b505f5160206176f15f395f51905f526001600160a01b03166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015612b64575f5ffd5b505af1158015612b76573d5f5f3e3d5ffd5b509293505050505b92915050565b60235460405163ca669fa760e01b81526001600160a01b0390911660048201525f5160206176f15f395f51905f529063ca669fa7906024015f604051808303815f87803b158015612bd3575f5ffd5b505af1158015612be5573d5f5f3e3d5ffd5b5050601f546021548451604051639ba730a960e01b81526101009093046001600160a01b039081169550639ba730a99450612c3093921691678ac7230489e800009190600401613c67565b5f604051808303815f87803b158015612c47575f5ffd5b505af1158015612c59573d5f5f3e3d5ffd5b5050505060208101516023546040516370a0823160e01b81526001600160a01b0391821660048201529116906370a0823190602401602060405180830381865afa158015612ca9573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ccd9190613b4f565b60c082015260608101516023546040516370a0823160e01b81526001600160a01b0391821660048201529116906370a0823190602401602060405180830381865afa158015612d1e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d429190613b4f565b60e082015260a081015160208201516060830151612d619291906132a8565b60208101516023546040516370a0823160e01b81526001600160a01b039182166004820152612df49291909116906370a0823190602401602060405180830381865afa158015612db3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612dd79190613b4f565b670de0b6b3a76400008360c00151612def9190613cfe565b61361c565b60608101516023546040516370a0823160e01b81526001600160a01b039182166004820152610dd99291909116906370a0823190602401602060405180830381865afa158015612e46573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612e6a9190613b4f565b8260e00151613654565b6020546040516317ee366f60e11b815260048101849052600160248201526001600160a01b0390911690632fdc6cde906044015f604051808303815f87803b158015612ebe575f5ffd5b505af1158015612ed0573d5f5f3e3d5ffd5b5050601f54604051635c23bdf560e01b8152600481018690526101009091046001600160a01b03169250635c23bdf591506024015f604051808303815f87803b158015612f1b575f5ffd5b505af1158015612f2d573d5f5f3e3d5ffd5b50506022546023546040516370a0823160e01b81526001600160a01b0391821660048201525f9450911691506370a0823190602401602060405180830381865afa158015612f7d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612fa19190613b4f565b6023546040516370a0823160e01b81526001600160a01b0391821660048201529192505f91908416906370a0823190602401602060405180830381865afa158015612fee573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130129190613b4f565b6023546040516303223eab60e11b81526001600160a01b0390911660048201529091505f5160206176f15f395f51905f52906306447d56906024015f604051808303815f87803b158015613064575f5ffd5b505af1158015613076573d5f5f3e3d5ffd5b5050601f5460405163095ea7b360e01b81526001600160a01b03808816945063095ea7b393506130b192610100900416908590600401613b66565b6020604051808303815f875af11580156130cd573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130f19190613b7f565b50601f546040516301e9a69560e41b81526101009091046001600160a01b031690631e9a6950906131289086908590600401613b66565b6020604051808303815f875af1158015613144573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906131689190613b4f565b505f5160206176f15f395f51905f526001600160a01b03166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b1580156131ae575f5ffd5b505af11580156131c0573d5f5f3e3d5ffd5b50506022546023546040516370a0823160e01b81526001600160a01b0391821660048201526132409450911691506370a0823190602401602060405180830381865afa158015613212573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906132369190613b4f565b612def8385613d11565b50505050565b60405163a34edc0360e01b81525f5160206176f15f395f51905f529063a34edc03906132789085908590600401613d24565b5f6040518083038186803b15801561328e575f5ffd5b505afa1580156132a0573d5f5f3e3d5ffd5b505050505050565b5f5f846001600160a01b0316630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa1580156132e6573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061330a9190613c9e565b50915091505f856001600160a01b0316630dfe16816040518163ffffffff1660e01b8152600401602060405180830381865afa15801561334c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133709190613c16565b90505f5f866001600160a01b0316836001600160a01b0316146133a657836001600160701b0316856001600160701b03166133bb565b846001600160701b0316846001600160701b03165b9092509050670de0b6b3a76400005f6133d582858561368c565b6023546040516303223eab60e11b81526001600160a01b0390911660048201529091505f5160206176f15f395f51905f52906306447d56906024015f604051808303815f87803b158015613427575f5ffd5b505af1158015613439573d5f5f3e3d5ffd5b505060405163a9059cbb60e01b81526001600160a01b038c16925063a9059cbb915061346b908d908690600401613b66565b6020604051808303815f875af1158015613487573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906134ab9190613b7f565b50886001600160a01b0316856001600160a01b03160361353f5760235460405163022c0d9f60e01b81525f60048201819052602482018490526001600160a01b039283166044830152608060648301526084820152908b169063022c0d9f9060a4015f604051808303815f87803b158015613524575f5ffd5b505af1158015613536573d5f5f3e3d5ffd5b505050506135b5565b60235460405163022c0d9f60e01b8152600481018390525f602482018190526001600160a01b039283166044830152608060648301526084820152908b169063022c0d9f9060a4015f604051808303815f87803b15801561359e575f5ffd5b505af11580156135b0573d5f5f3e3d5ffd5b505050505b5f5160206176f15f395f51905f526001600160a01b03166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b1580156135fa575f5ffd5b505af115801561360c573d5f5f3e3d5ffd5b5050505050505050505050505050565b60405163260a5b1560e21b815260048101839052602481018290525f5160206176f15f395f51905f52906398296c5490604401613278565b604051636d83fe6960e11b815260048101839052602481018290525f5160206176f15f395f51905f529063db07fcd290604401613278565b5f5f84116136dc5760405162461bcd60e51b815260206004820152601960248201527f494e53554646494349454e545f494e5055545f414d4f554e54000000000000006044820152606401612957565b5f831180156136ea57505f82115b61372f5760405162461bcd60e51b8152602060048201526016602482015275494e53554646494349454e545f4c495155494449545960501b6044820152606401612957565b5f61373c856103e5613d46565b90505f6137498483613d46565b90505f82613759876103e8613d46565b6137639190613d11565b905061376f8183613d5d565b979650505050505050565b61259680613d7d83390190565b61050a8061631383390190565b610ed48061681d83390190565b602080825282518282018190525f918401906040840190835b818110156137e15783516001600160a01b03168352602093840193909201916001016137ba565b509095945050505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015612b7657603f19878603018452815180516001600160a01b03168652602090810151604082880181905281519088018190529101906060600582901b8801810191908801905f5b818110156138bd57605f198a85030183526138a78486516137ec565b602095860195909450929092019160010161388b565b509197505050602094850194929092019150600101613840565b5f8151808452602084019350602083015f5b828110156139115781516001600160e01b0319168652602095860195909101906001016138e9565b5093949350505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015612b7657603f19878603018452815180516040875261396760408801826137ec565b905060208201519150868103602088015261398281836138d7565b965050506020938401939190910190600101613941565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015612b7657603f198786030184526139db8583516137ec565b945060209384019391909101906001016139bf565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015612b7657868503603f19018452815180516001600160a01b03168652602090810151604091870182905290613a51908701826138d7565b9550506020938401939190910190600101613a16565b634e487b7160e01b5f52604160045260245ffd5b5f60208284031215613a8b575f5ffd5b815167ffffffffffffffff811115613aa1575f5ffd5b8201601f81018413613ab1575f5ffd5b805167ffffffffffffffff811115613acb57613acb613a67565b604051601f8201601f19908116603f0116810167ffffffffffffffff81118282101715613afa57613afa613a67565b604052818152828201602001861015613b11575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b604081525f613b4060408301856137ec565b90508260208301529392505050565b5f60208284031215613b5f575f5ffd5b5051919050565b6001600160a01b03929092168252602082015260400190565b5f60208284031215613b8f575f5ffd5b81518015158114613b9e575f5ffd5b9392505050565b92835263ffffffff9190911660208301526001600160a01b0316604082015260600190565b80516001600160a01b0381168114613be0575f5ffd5b919050565b5f5f60408385031215613bf6575f5ffd5b613bff83613bca565b9150613c0d60208401613bca565b90509250929050565b5f60208284031215613c26575f5ffd5b613b9e82613bca565b600181811c90821680613c4357607f821691505b602082108103613c6157634e487b7160e01b5f52602260045260245ffd5b50919050565b6001600160a01b039390931683526020830191909152604082015260600190565b80516001600160701b0381168114613be0575f5ffd5b5f5f5f60608486031215613cb0575f5ffd5b613cb984613c88565b9250613cc760208501613c88565b9150604084015163ffffffff81168114613cdf575f5ffd5b809150509250925092565b634e487b7160e01b5f52601160045260245ffd5b81810381811115612b7e57612b7e613cea565b80820180821115612b7e57612b7e613cea565b8215158152604060208201525f613d3e60408301846137ec565b949350505050565b8082028115828204841417612b7e57612b7e613cea565b5f82613d7757634e487b7160e01b5f52601260045260245ffd5b50049056fe6080604052348015600e575f5ffd5b5061257a8061001c5f395ff3fe608060405234801561000f575f5ffd5b506004361061009b575f3560e01c80639ba730a9116100635780639ba730a91461017a578063a3def9231461018d578063c2a33c1d146101c0578063d4b06fb8146101d3578063e973955014610202575f5ffd5b80631e9a69501461009f57806341055a76146100c55780635c23bdf5146100da5780637183d24a146100ed5780637564912b14610110575b5f5ffd5b6100b26100ad3660046111df565b610215565b6040519081526020015b60405180910390f35b6100d86100d3366004611209565b61062a565b005b6100d86100e836600461123b565b6108e1565b6101006100fb366004611252565b610a46565b60405190151581526020016100bc565b61015061011e36600461123b565b5f602081905290815260409020805460019091015463ffffffff82169164010000000090046001600160a01b03169083565b6040805163ffffffff90941684526001600160a01b039092166020840152908201526060016100bc565b6100d8610188366004611209565b610bf1565b6101a061019b3660046111df565b610dcd565b604080516001600160a01b039384168152929091166020830152016100bc565b6100d86101ce366004611274565b610f0f565b6101f56101e136600461123b565b60016020525f908152604090205460ff1681565b6040516100bc91906112cf565b6101a06102103660046111df565b61104c565b5f5f604051602001610230906259455360e81b815260030190565b60405160208183030381529060405280519060200120846001600160a01b03166395d89b416040518163ffffffff1660e01b81526004015f60405180830381865afa158015610281573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526102a89190810190611309565b6040516020016102b891906113d3565b604051602081830303815290604052805190602001201490505f846001600160a01b031663c0474d0b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561030e573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061033291906113de565b90505f856001600160a01b0316634800d97f6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610371573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061039591906113f5565b90506103a2838284611070565b6001600160a01b0316866001600160a01b0316146103d3576040516340fe50fd60e01b815260040160405180910390fd5b60025f8381526001602052604090205460ff1660048111156103f7576103f76112bb565b1480156104015750825b1561040e5784935061049e565b60035f8381526001602052604090205460ff166004811115610432576104326112bb565b14801561043d575082155b1561044a5784935061049e565b60045f8381526001602052604090205460ff16600481111561046e5761046e6112bb565b036104855761047e600286611410565b935061049e565b604051637256f64560e11b815260040160405180910390fd5b6040516323b872dd60e01b81526001600160a01b038716906323b872dd906104ce90339030908a9060040161142f565b6020604051808303815f875af11580156104ea573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061050e9190611453565b50604051632770a7eb60e21b8152306004820152602481018690526001600160a01b03871690639dc29fac906044015f604051808303815f87803b158015610554575f5ffd5b505af1158015610566573d5f5f3e3d5ffd5b505060405163a9059cbb60e01b8152336004820152602481018790526001600160a01b038416925063a9059cbb91506044016020604051808303815f875af11580156105b4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105d89190611453565b50604080518681526020810186905283916001600160a01b0389169133917fae1d804a86b6ba2a7725d9824d4ccb4e7f9a55e7a1bf4379c26692ce5ad00665910160405180910390a450505092915050565b60015f8281526001602052604090205460ff16600481111561064e5761064e6112bb565b1461066c5760405163239dd4ad60e11b815260040160405180910390fd5b5f5f610678858461104c565b6040516323b872dd60e01b815291935091506001600160a01b038316906323b872dd906106ad9033903090899060040161142f565b6020604051808303815f875af11580156106c9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106ed9190611453565b506040516323b872dd60e01b81526001600160a01b038216906323b872dd9061071e9033903090899060040161142f565b6020604051808303815f875af115801561073a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061075e9190611453565b50604051632770a7eb60e21b8152306004820152602481018590526001600160a01b03831690639dc29fac906044015f604051808303815f87803b1580156107a4575f5ffd5b505af11580156107b6573d5f5f3e3d5ffd5b5050604051632770a7eb60e21b8152306004820152602481018790526001600160a01b0384169250639dc29fac91506044015f604051808303815f87803b1580156107ff575f5ffd5b505af1158015610811573d5f5f3e3d5ffd5b505060405163a9059cbb60e01b8152336004820152602481018790526001600160a01b038816925063a9059cbb91506044016020604051808303815f875af115801561085f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108839190611453565b50604080518581526001600160a01b03848116602083015283811692820192909252849187169033907f9a853bfbdd6034e0e7553bb8ec7789b2603dcccb5759cf390a7a601c0b6da003906060015b60405180910390a45050505050565b60015f8281526001602052604090205460ff166004811115610905576109056112bb565b146109235760405163239dd4ad60e11b815260040160405180910390fd5b5f8181526020819052604081205463ffffffff164210610945575060046109d8565b5f828152602081905260408082205490516301f46b2960e11b8152600481018590526401000000009091046001600160a01b0316906303e8d652906024016020604051808303815f875af115801561099f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109c39190611453565b9050806109d15760036109d4565b60025b9150505b5f8281526001602081905260409091208054839260ff1990911690836004811115610a0557610a056112bb565b0217905550817ff34984473148051bc1bdf1be6ecc462d7b228d591058a8a27977b84770b738b982604051610a3a91906112cf565b60405180910390a25050565b5f5f826001600160a01b031663c0474d0b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610a84573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610aa891906113de565b90505f836001600160a01b0316634800d97f6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610ae7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b0b91906113f5565b90505f604051602001610b27906259455360e81b815260030190565b60405160208183030381529060405280519060200120856001600160a01b03166395d89b416040518163ffffffff1660e01b81526004015f60405180830381865afa158015610b78573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610b9f9190810190611309565b604051602001610baf91906113d3565b60405160208183030381529060405280519060200120149050610bd3818385611070565b6001600160a01b0316856001600160a01b0316149350505050919050565b60015f8281526001602052604090205460ff166004811115610c1557610c156112bb565b14610c335760405163239dd4ad60e11b815260040160405180910390fd5b6040516323b872dd60e01b81526001600160a01b038416906323b872dd90610c639033903090879060040161142f565b6020604051808303815f875af1158015610c7f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ca39190611453565b505f5f610cb0858461104c565b6040516340c10f1960e01b81523360048201526024810187905291935091506001600160a01b038316906340c10f19906044015f604051808303815f87803b158015610cfa575f5ffd5b505af1158015610d0c573d5f5f3e3d5ffd5b50506040516340c10f1960e01b8152336004820152602481018790526001600160a01b03841692506340c10f1991506044015f604051808303815f87803b158015610d55575f5ffd5b505af1158015610d67573d5f5f3e3d5ffd5b5050505082856001600160a01b0316336001600160a01b03167f8fb8a329bc68853568e0dd27c6aee2e361a1f07892cf65ab49cc2f30cd21c3c58786866040516108d2939291909283526001600160a01b03918216602084015216604082015260600190565b5f5f828484604051610dde906111bb565b608080825260039082018190526259657360e81b60a083015260c0602083018190528201526259455360e81b60e08201526001600160a01b0390921660408301526060820152610100018190604051809103905ff5905080158015610e45573d5f5f3e3d5ffd5b509150828484604051610e57906111bb565b60808082526002908201819052614e6f60f01b60a083015260c060208301819052820152614e4f60f01b60e08201526001600160a01b0390921660408301526060820152610100018190604051809103905ff5905080158015610ebc573d5f5f3e3d5ffd5b50604080516001600160a01b0385811682528381166020830152825193945087169286927f9e431caef5753e3e939653fc056c910725980029b3653db5a36bebca71d6bc2c928290030190a39250929050565b604080516020810185905263ffffffff8416918101919091526001600160a01b03821660608201525f9060800160408051601f19818403018152919052805160209091012090505f5f8281526001602052604090205460ff166004811115610f7957610f796112bb565b14610f97576040516337a5bd9b60e11b815260040160405180910390fd5b5f818152600160208181526040808420805460ff191684179055805160608101825263ffffffff8881168083526001600160a01b038981168487018181528587018e81528b8b528a89529987902095518654915195166001600160c01b0319909116176401000000009490921693909302178355955191909401558051938452519192879285927f094eba69d7ca9dfafa34c896067fc8e19395a610e47deb8b25b21b87bec32a34928290030190a450505050565b5f5f61105a60018585611070565b91506110675f8585611070565b90509250929050565b5f5f60405180602001611082906111bb565b601f1982820381018352601f90910116604052856110ba57604051806040016040528060028152602001614e6f60f01b8152506110d7565b6040518060400160405280600381526020016259657360e81b8152505b866110fc57604051806040016040528060028152602001614e4f60f01b815250611119565b6040518060400160405280600381526020016259455360e81b8152505b868660405160200161112e94939291906114a0565b60408051601f198184030181529082905261114c92916020016114e1565b60408051808303601f1901815282825280516020918201206001600160f81b0319828501523060601b6bffffffffffffffffffffffff19166021850152603584019690965260558084019690965281518084039096018652607590920190528351930192909220949350505050565b611047806114fe83390190565b6001600160a01b03811681146111dc575f5ffd5b50565b5f5f604083850312156111f0575f5ffd5b82356111fb816111c8565b946020939093013593505050565b5f5f5f6060848603121561121b575f5ffd5b8335611226816111c8565b95602085013595506040909401359392505050565b5f6020828403121561124b575f5ffd5b5035919050565b5f60208284031215611262575f5ffd5b813561126d816111c8565b9392505050565b5f5f5f60608486031215611286575f5ffd5b83359250602084013563ffffffff811681146112a0575f5ffd5b915060408401356112b0816111c8565b809150509250925092565b634e487b7160e01b5f52602160045260245ffd5b60208101600583106112ef57634e487b7160e01b5f52602160045260245ffd5b91905290565b634e487b7160e01b5f52604160045260245ffd5b5f60208284031215611319575f5ffd5b815167ffffffffffffffff81111561132f575f5ffd5b8201601f8101841361133f575f5ffd5b805167ffffffffffffffff811115611359576113596112f5565b604051601f8201601f19908116603f0116810167ffffffffffffffff81118282101715611388576113886112f5565b60405281815282820160200186101561139f575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b5f81518060208401855e5f93019283525090919050565b5f61126d82846113bc565b5f602082840312156113ee575f5ffd5b5051919050565b5f60208284031215611405575f5ffd5b815161126d816111c8565b5f8261142a57634e487b7160e01b5f52601260045260245ffd5b500490565b6001600160a01b039384168152919092166020820152604081019190915260600190565b5f60208284031215611463575f5ffd5b8151801515811461126d575f5ffd5b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b608081525f6114b26080830187611472565b82810360208401526114c48187611472565b6001600160a01b0395909516604084015250506060015292915050565b5f6114f56114ef83866113bc565b846113bc565b94935050505056fe60c060405234801561000f575f5ffd5b5060405161104738038061104783398101604081905261002e91610143565b5f6100398582610252565b5060016100468482610252565b506001600160a01b03821660805260a08190526100623361006b565b5050505061030c565b6001600160a01b0316638b78c6d819819055805f7f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08180a350565b634e487b7160e01b5f52604160045260245ffd5b5f82601f8301126100c9575f5ffd5b81516001600160401b038111156100e2576100e26100a6565b604051601f8201601f19908116603f011681016001600160401b0381118282101715610110576101106100a6565b604052818152838201602001851015610127575f5ffd5b8160208501602083015e5f918101602001919091529392505050565b5f5f5f5f60808587031215610156575f5ffd5b84516001600160401b0381111561016b575f5ffd5b610177878288016100ba565b602087015190955090506001600160401b03811115610194575f5ffd5b6101a0878288016100ba565b604087015190945090506001600160a01b03811681146101be575f5ffd5b6060959095015193969295505050565b600181811c908216806101e257607f821691505b60208210810361020057634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561024d57805f5260205f20601f840160051c8101602085101561022b5750805b601f840160051c820191505b8181101561024a575f8155600101610237565b50505b505050565b81516001600160401b0381111561026b5761026b6100a6565b61027f8161027984546101ce565b84610206565b6020601f8211600181146102b1575f831561029a5750848201515b5f19600385901b1c1916600184901b17845561024a565b5f84815260208120601f198516915b828110156102e057878501518255602094850194600190920191016102c0565b50848210156102fd57868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b60805160a051610d1a61032d5f395f61036e01525f6102470152610d1a5ff3fe60806040526004361061013c575f3560e01c8063715018a6116100b3578063c0474d0b1161006d578063c0474d0b1461035d578063d505accf14610390578063dd62ed3e146103af578063f04e283e146103ce578063f2fde38b146103e1578063fee81cf4146103f4575f5ffd5b8063715018a6146102ba5780637ecebe00146102c25780638da5cb5b146102f357806395d89b411461030b5780639dc29fac1461031f578063a9059cbb1461033e575f5ffd5b8063313ce56711610104578063313ce567146101e85780633644e5151461020357806340c10f19146102175780634800d97f1461023657806354d1f13d1461028157806370a0823114610289575f5ffd5b806306fdde0314610140578063095ea7b31461016a57806318160ddd1461019957806323b872dd146101bf57806325692962146101de575b5f5ffd5b34801561014b575f5ffd5b50610154610425565b6040516101619190610b1c565b60405180910390f35b348015610175575f5ffd5b50610189610184366004610b6c565b6104b4565b6040519015158152602001610161565b3480156101a4575f5ffd5b506805345cdf77eb68f44c545b604051908152602001610161565b3480156101ca575f5ffd5b506101896101d9366004610b94565b610534565b6101e66105f0565b005b3480156101f3575f5ffd5b5060405160128152602001610161565b34801561020e575f5ffd5b506101b161063d565b348015610222575f5ffd5b506101e6610231366004610b6c565b6106b9565b348015610241575f5ffd5b506102697f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610161565b6101e66106cf565b348015610294575f5ffd5b506101b16102a3366004610bce565b6387a211a2600c9081525f91909152602090205490565b6101e6610708565b3480156102cd575f5ffd5b506101b16102dc366004610bce565b6338377508600c9081525f91909152602090205490565b3480156102fe575f5ffd5b50638b78c6d81954610269565b348015610316575f5ffd5b5061015461071b565b34801561032a575f5ffd5b506101e6610339366004610b6c565b61072a565b348015610349575f5ffd5b50610189610358366004610b6c565b61073c565b348015610368575f5ffd5b506101b17f000000000000000000000000000000000000000000000000000000000000000081565b34801561039b575f5ffd5b506101e66103aa366004610bee565b6107a0565b3480156103ba575f5ffd5b506101b16103c9366004610c5b565b610954565b6101e66103dc366004610bce565b610998565b6101e66103ef366004610bce565b6109d5565b3480156103ff575f5ffd5b506101b161040e366004610bce565b63389a75e1600c9081525f91909152602090205490565b60605f805461043390610c8c565b80601f016020809104026020016040519081016040528092919081815260200182805461045f90610c8c565b80156104aa5780601f10610481576101008083540402835291602001916104aa565b820191905f5260205f20905b81548152906001019060200180831161048d57829003601f168201915b5050505050905090565b5f6001600160a01b0383166e22d473030f116ddee9f6b43ac78ba318821915176104e557633f68539a5f526004601cfd5b82602052637f5e9f20600c52335f52816034600c2055815f52602c5160601c337f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560205fa35060015b92915050565b5f8360601b6e22d473030f116ddee9f6b43ac78ba333146105895733602052637f5e9f208117600c526034600c2080548019156105865780851115610580576313be252b5f526004601cfd5b84810382555b50505b6387a211a28117600c526020600c208054808511156105af5763f4d678b85f526004601cfd5b84810382555050835f526020600c208381540181555082602052600c5160601c8160601c5f516020610cc55f395f51905f52602080a3505060019392505050565b5f6202a30067ffffffffffffffff164201905063389a75e1600c52335f52806020600c2055337fdbf36a107da19e49527a7176a1babf963b4b0ff8cde35ee35d6cd8f1f9ac7e1d5f5fa250565b5f80610647610425565b805190602001209050604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f815260208101929092527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc69082015246606082015230608082015260a09020919050565b6106c16109fb565b6106cb8282610a15565b5050565b63389a75e1600c52335f525f6020600c2055337ffa7b8eab7da67f412cc9575ed43464468f9bfbae89d1675917346ca6d8fe3c925f5fa2565b6107106109fb565b6107195f610a7e565b565b60606001805461043390610c8c565b6107326109fb565b6106cb8282610abb565b5f6387a211a2600c52335f526020600c208054808411156107645763f4d678b85f526004601cfd5b83810382555050825f526020600c208281540181555081602052600c5160601c335f516020610cc55f395f51905f52602080a350600192915050565b6001600160a01b0386166e22d473030f116ddee9f6b43ac78ba318851915176107d057633f68539a5f526004601cfd5b5f6107d9610425565b8051906020012090507fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc64286101561081857631a15a3cc5f526004601cfd5b6040518960601b60601c99508860601b60601c985065383775081901600e52895f526020600c2080547f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f835284602084015283604084015246606084015230608084015260a08320602e527f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c983528b60208401528a60408401528960608401528060808401528860a084015260c08320604e526042602c205f528760ff16602052866040528560605260208060805f60015afa8c3d51146109005763ddafbaef5f526004601cfd5b0190556303faf4f960a51b89176040526034602c20889055888a7f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925602060608501a360405250505f60605250505050505050565b5f6e22d473030f116ddee9f6b43ac78ba2196001600160a01b0383160161097d57505f1961052e565b50602052637f5e9f20600c9081525f91909152603490205490565b6109a06109fb565b63389a75e1600c52805f526020600c2080544211156109c657636f5e88185f526004601cfd5b5f90556109d281610a7e565b50565b6109dd6109fb565b8060601b6109f257637448fbae5f526004601cfd5b6109d281610a7e565b638b78c6d819543314610719576382b429005f526004601cfd5b6805345cdf77eb68f44c5481810181811015610a385763e5cfe9575f526004601cfd5b806805345cdf77eb68f44c5550506387a211a2600c52815f526020600c208181540181555080602052600c5160601c5f5f516020610cc55f395f51905f52602080a35050565b638b78c6d81980546001600160a01b039092169182907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a355565b6387a211a2600c52815f526020600c20805480831115610ae25763f4d678b85f526004601cfd5b82900390556805345cdf77eb68f44c805482900390555f8181526001600160a01b0383165f516020610cc55f395f51905f52602083a35050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b80356001600160a01b0381168114610b67575f5ffd5b919050565b5f5f60408385031215610b7d575f5ffd5b610b8683610b51565b946020939093013593505050565b5f5f5f60608486031215610ba6575f5ffd5b610baf84610b51565b9250610bbd60208501610b51565b929592945050506040919091013590565b5f60208284031215610bde575f5ffd5b610be782610b51565b9392505050565b5f5f5f5f5f5f5f60e0888a031215610c04575f5ffd5b610c0d88610b51565b9650610c1b60208901610b51565b95506040880135945060608801359350608088013560ff81168114610c3e575f5ffd5b9699959850939692959460a0840135945060c09093013592915050565b5f5f60408385031215610c6c575f5ffd5b610c7583610b51565b9150610c8360208401610b51565b90509250929050565b600181811c90821680610ca057607f821691505b602082108103610cbe57634e487b7160e01b5f52602260045260245ffd5b5091905056feddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa2646970667358221220f2558ad4460ca00ebebab8d02372dd6b027f19de7bd9d732ad74c5cf24635f4564736f6c634300081e0033a2646970667358221220919de7b4590df56bc11aeb304fa29d7b44145f3508341dc0a20c84eeab358d1564736f6c634300081e00336080604052348015600e575f5ffd5b50601633601a565b6055565b6001600160a01b0316638b78c6d819819055805f7f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08180a350565b6104a8806100625f395ff3fe60806040526004361061009a575f3560e01c80638da5cb5b116100625780638da5cb5b1461010b578063d4b06fb814610136578063f04e283e14610164578063f2fde38b14610177578063f636e0f61461018a578063fee81cf4146101b8575f5ffd5b806303e8d6521461009e57806325692962146100d25780632fdc6cde146100dc57806354d1f13d146100fb578063715018a614610103575b5f5ffd5b3480156100a9575f5ffd5b506100bd6100b83660046103fc565b6101f7565b60405190151581526020015b60405180910390f35b6100da61023a565b005b3480156100e7575f5ffd5b506100da6100f6366004610413565b610287565b6100da6102f6565b6100da61032f565b348015610116575f5ffd5b50638b78c6d819546040516001600160a01b0390911681526020016100c9565b348015610141575f5ffd5b506100bd6101503660046103fc565b5f6020819052908152604090205460ff1681565b6100da610172366004610445565b610342565b6100da610185366004610445565b61037f565b348015610195575f5ffd5b506100bd6101a43660046103fc565b60016020525f908152604090205460ff1681565b3480156101c3575f5ffd5b506101e96101d2366004610445565b63389a75e1600c9081525f91909152602090205490565b6040519081526020016100c9565b5f8181526001602052604081205460ff1661022557604051634aec5ac760e01b815260040160405180910390fd5b505f9081526020819052604090205460ff1690565b5f6202a30067ffffffffffffffff164201905063389a75e1600c52335f52806020600c2055337fdbf36a107da19e49527a7176a1babf963b4b0ff8cde35ee35d6cd8f1f9ac7e1d5f5fa250565b61028f6103a5565b5f82815260208181526040808320805485151560ff19918216811790925560018085529483902080549091169094179093555191825283917f57c39a2480accb4077522f2253a3915afdc67e1ca5883a57957ed61f2f04d023910160405180910390a25050565b63389a75e1600c52335f525f6020600c2055337ffa7b8eab7da67f412cc9575ed43464468f9bfbae89d1675917346ca6d8fe3c925f5fa2565b6103376103a5565b6103405f6103bf565b565b61034a6103a5565b63389a75e1600c52805f526020600c20805442111561037057636f5e88185f526004601cfd5b5f905561037c816103bf565b50565b6103876103a5565b8060601b61039c57637448fbae5f526004601cfd5b61037c816103bf565b638b78c6d819543314610340576382b429005f526004601cfd5b638b78c6d81980546001600160a01b039092169182907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a355565b5f6020828403121561040c575f5ffd5b5035919050565b5f5f60408385031215610424575f5ffd5b823591506020830135801515811461043a575f5ffd5b809150509250929050565b5f60208284031215610455575f5ffd5b81356001600160a01b038116811461046b575f5ffd5b939250505056fea2646970667358221220758a44ba0589aa1e9c5c5158ff502f3cbb50162cde6c15d8d597e83a6706cb0264736f6c634300081e003360a060405234801561000f575f5ffd5b50604051610ed4380380610ed483398101604081905261002e91610109565b5f610039848261020a565b506001610046838261020a565b506002805460ff191660ff929092169190911790555080516020909101206080526102c4565b634e487b7160e01b5f52604160045260245ffd5b5f82601f83011261008f575f5ffd5b81516001600160401b038111156100a8576100a861006c565b604051601f8201601f19908116603f011681016001600160401b03811182821017156100d6576100d661006c565b6040528181528382016020018510156100ed575f5ffd5b8160208501602083015e5f918101602001919091529392505050565b5f5f5f6060848603121561011b575f5ffd5b83516001600160401b03811115610130575f5ffd5b61013c86828701610080565b602086015190945090506001600160401b03811115610159575f5ffd5b61016586828701610080565b925050604084015160ff8116811461017b575f5ffd5b809150509250925092565b600181811c9082168061019a57607f821691505b6020821081036101b857634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561020557805f5260205f20601f840160051c810160208510156101e35750805b601f840160051c820191505b81811015610202575f81556001016101ef565b50505b505050565b81516001600160401b038111156102235761022361006c565b610237816102318454610186565b846101be565b6020601f821160018114610269575f83156102525750848201515b5f19600385901b1c1916600184901b178455610202565b5f84815260208120601f198516915b828110156102985787850151825560209485019460019092019101610278565b50848210156102b557868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b608051610bf16102e35f395f818161039901526104db0152610bf15ff3fe608060405234801561000f575f5ffd5b50600436106100fb575f3560e01c80637ecebe0011610093578063d30ed3b311610063578063d30ed3b314610217578063d505accf1461022a578063dd62ed3e1461023d578063f83d179114610250575f5ffd5b80637ecebe00146101c457806395d89b41146101e95780639dc29fac146101f1578063a9059cbb14610204575f5ffd5b8063313ce567116100ce578063313ce5671461016d5780633644e5151461018257806340c10f191461018a57806370a082311461019f575f5ffd5b806306fdde03146100ff578063095ea7b31461011d57806318160ddd1461014057806323b872dd1461015a575b5f5ffd5b610107610263565b60405161011491906109ff565b60405180910390f35b61013061012b366004610a4a565b6102f2565b6040519015158152602001610114565b6805345cdf77eb68f44c545b604051908152602001610114565b610130610168366004610a72565b610372565b60025460405160ff9091168152602001610114565b61014c610396565b61019d610198366004610a4a565b610438565b005b61014c6101ad366004610aac565b6387a211a2600c9081525f91909152602090205490565b61014c6101d2366004610aac565b6338377508600c9081525f91909152602090205490565b61010761044e565b61019d6101ff366004610a4a565b61045d565b610130610212366004610a4a565b61046f565b61019d610225366004610a72565b610489565b61019d610238366004610ac5565b6104a9565b61014c61024b366004610b32565b610683565b61019d61025e366004610a72565b6106c7565b60605f805461027190610b63565b80601f016020809104026020016040519081016040528092919081815260200182805461029d90610b63565b80156102e85780601f106102bf576101008083540402835291602001916102e8565b820191905f5260205f20905b8154815290600101906020018083116102cb57829003601f168201915b5050505050905090565b5f6001600160a01b0383166e22d473030f116ddee9f6b43ac78ba3188219151761032357633f68539a5f526004601cfd5b82602052637f5e9f20600c52335f52816034600c2055815f52602c5160601c337f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560205fa35060015b92915050565b5f61038e61037f856106e2565b610388856106e2565b84610703565b949350505050565b5f7f0000000000000000000000000000000000000000000000000000000000000000806103cf576103c5610263565b8051906020012090505b604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f815260208101929092527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc69082015246606082015230608082015260a09020919050565b61044a610444836106e2565b826107bf565b5050565b60606001805461027190610b63565b61044a610469836106e2565b82610828565b5f61048261047c846106e2565b83610889565b9392505050565b6104a4610495846106e2565b61049e846106e2565b836108ed565b505050565b6001600160a01b0386166e22d473030f116ddee9f6b43ac78ba318851915176104d957633f68539a5f526004601cfd5b7f00000000000000000000000000000000000000000000000000000000000000008061051157610507610263565b8051906020012090505b7fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc64286101561054757631a15a3cc5f526004601cfd5b6040518960601b60601c99508860601b60601c985065383775081901600e52895f526020600c2080547f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f835284602084015283604084015246606084015230608084015260a08320602e527f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c983528b60208401528a60408401528960608401528060808401528860a084015260c08320604e526042602c205f528760ff16602052866040528560605260208060805f60015afa8c3d511461062f5763ddafbaef5f526004601cfd5b0190556303faf4f960a51b89176040526034602c20889055888a7f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925602060608501a360405250505f60605250505050505050565b5f6e22d473030f116ddee9f6b43ac78ba2196001600160a01b038316016106ac57505f1961036c565b50602052637f5e9f20600c9081525f91909152603490205490565b6104a46106d3846106e2565b6106dc846106e2565b83610951565b5f6001600160a01b0382168060a06106f9826109b7565b901b189392505050565b5f8360601b6e22d473030f116ddee9f6b43ac78ba333146107585733602052637f5e9f208117600c526034600c208054801915610755578085111561074f576313be252b5f526004601cfd5b84810382555b50505b6387a211a28117600c526020600c2080548085111561077e5763f4d678b85f526004601cfd5b84810382555050835f526020600c208381540181555082602052600c5160601c8160601c5f516020610b9c5f395f51905f52602080a3505060019392505050565b6805345cdf77eb68f44c54818101818110156107e25763e5cfe9575f526004601cfd5b806805345cdf77eb68f44c5550506387a211a2600c52815f526020600c208181540181555080602052600c5160601c5f5f516020610b9c5f395f51905f52602080a35050565b6387a211a2600c52815f526020600c2080548083111561084f5763f4d678b85f526004601cfd5b82900390556805345cdf77eb68f44c805482900390555f8181526001600160a01b0383165f516020610b9c5f395f51905f52602083a35050565b5f6387a211a2600c52335f526020600c208054808411156108b15763f4d678b85f526004601cfd5b83810382555050825f526020600c208281540181555081602052600c5160601c335f516020610b9c5f395f51905f52602080a350600192915050565b6e22d473030f116ddee9f6b43ac78ba2196001600160a01b0383160161091257505050565b81602052637f5e9f20600c52825f526034600c20805480191561094a5780831115610944576313be252b5f526004601cfd5b82810382555b5050505050565b8260601b6387a211a28117600c526020600c2080548084111561097b5763f4d678b85f526004601cfd5b83810382555050825f526020600c208281540181555081602052600c5160601c8160601c5f516020610b9c5f395f51905f52602080a350505050565b604051365f8237368120602052601051821860105260885f2090508060105260bc19700100000000000000000000000000000051820960801c6007166109fa57505f5b919050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b80356001600160a01b03811681146109fa575f5ffd5b5f5f60408385031215610a5b575f5ffd5b610a6483610a34565b946020939093013593505050565b5f5f5f60608486031215610a84575f5ffd5b610a8d84610a34565b9250610a9b60208501610a34565b929592945050506040919091013590565b5f60208284031215610abc575f5ffd5b61048282610a34565b5f5f5f5f5f5f5f60e0888a031215610adb575f5ffd5b610ae488610a34565b9650610af260208901610a34565b95506040880135945060608801359350608088013560ff81168114610b15575f5ffd5b9699959850939692959460a0840135945060c09093013592915050565b5f5f60408385031215610b43575f5ffd5b610b4c83610a34565b9150610b5a60208401610a34565b90509250929050565b600181811c90821680610b7757607f821691505b602082108103610b9557634e487b7160e01b5f52602260045260245ffd5b5091905056feddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa26469706673582212200c45acc3c6a9d15890ebba8720cb2063e27c6daab33079386bb3dcaecfa64cfb64736f6c634300081e00330000000000000000000000007109709ecfa91a80626ff3989d68f67f5b1dd12d5945535f574554482f5945535f5553444320706169722073686f756c64206578697374229a5127dba549edf6c45917cdacdeebdcb0f57ad33d2d0aa6e8d12a21692902a264697066735822122040aff2e2915a35197d1fa209b4934271c2c256f46215fb8e2d03e283fa76e88d64736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x0C\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x1F\x80T\x90\x91\x16\x90\x91\x17\x90U`\xC0`@R`\x05`\x80\x90\x81Rdalice`\xD8\x1B`\xA0Ra\0=\x90a\0\xFBV[`#\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81Rb17\xB1`\xE9\x1B` \x82\x01Ra\0\x80\x90a\0\xFBV[`$\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80Q\x80\x82\x01\x90\x91R`\n\x81Ri68(97\xBB4\xB22\xB9`\xB1\x1B` \x82\x01Ra\0\xCA\x90a\0\xFBV[`%\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U4\x80\x15a\0\xF5W__\xFD[Pa\x02\xA1V[_a\x01\x05\x82a\x01\x0CV[P\x92\x91PPV[__\x82`@Q` \x01a\x01\x1F\x91\x90a\x02\x1AV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01b^y\xB7`\xE0\x1B\x03\x19\x82R`\x04\x82\x01\x81\x90R\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xFF\xA1\x86I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x88W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xAC\x91\x90a\x020V[`@Qc\x18\xCA\xF8\xE3`\xE3\x1B\x81R\x90\x92Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xC6W\xC7\x18\x90a\x01\xE8\x90\x85\x90\x87\x90`\x04\x01a\x02]V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x01\xFFW__\xFD[PZ\xF1\x15\x80\x15a\x02\x11W=__>=_\xFD[PPPP\x91P\x91V[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15a\x02@W__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02VW__\xFD[\x93\x92PPPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R_\x82Q\x80`@\x84\x01R\x80` \x85\x01``\x85\x01^_``\x82\x85\x01\x01R```\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x93\x92PPPV[aw\x89\x80a\x02\xAE_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x87W_5`\xE0\x1C\x80c}\xC0\xD1\xD0\x11a\0\xD9W\x80c\xB0FO\xDC\x11a\0\x93W\x80c\xC0\x9C\xECw\x11a\0nW\x80c\xC0\x9C\xECw\x14a\x03,W\x80c\xE2\x0C\x9Fq\x14a\x03?W\x80c\xFAv&\xD4\x14a\x03GW\x80c\xFBG\xE3\xA2\x14a\x03TW__\xFD[\x80c\xB0FO\xDC\x14a\x03\x04W\x80c\xB5P\x8A\xA9\x14a\x03\x0CW\x80c\xBAAO\xA6\x14a\x03\x14W__\xFD[\x80c}\xC0\xD1\xD0\x14a\x02tW\x80c\x85\"l\x81\x14a\x02\x87W\x80c\x8C\x80\x9F\xBF\x14a\x02\x9CW\x80c\x91j\x17\xC6\x14a\x02\xB4W\x80c\x99\xD8\xFA\xE3\x14a\x02\xC9W\x80c\xA4#KN\x14a\x02\xE4W__\xFD[\x80c>A;\xEE\x11a\x01DW\x80c?\xC8\xCE\xF3\x11a\x01\x1FW\x80c?\xC8\xCE\xF3\x14a\x02\"W\x80cZ\xF7\x07\x1A\x14a\x025W\x80cf\xD9\xA9\xA0\x14a\x02WW\x80cq@\xDCs\x14a\x02lW__\xFD[\x80c>A;\xEE\x14a\x01\xFFW\x80c>^<#\x14a\x02\x12W\x80c?r\x86\xF4\x14a\x02\x1AW__\xFD[\x80c\n\x92T\xE4\x14a\x01\x8BW\x80c\r#X\x17\x14a\x01\x95W\x80c\x13}\xA3\x0C\x14a\x01\xC5W\x80c\x1D\x82\x885\x14a\x01\xCDW\x80c\x1E\xD7\x83\x1C\x14a\x01\xD5W\x80c*\xDE8\x80\x14a\x01\xEAW[__\xFD[a\x01\x93a\x03gV[\0[`%Ta\x01\xA8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x93a\r\xDCV[a\x01\x93a\x10NV[a\x01\xDDa\x14\x92V[`@Qa\x01\xBC\x91\x90a7\xA1V[a\x01\xF2a\x14\xF2V[`@Qa\x01\xBC\x91\x90a8\x1AV[`\"Ta\x01\xA8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xDDa\x16.V[a\x01\xDDa\x16\x8CV[`!Ta\x01\xA8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02I_Q` aw4_9_Q\x90_R\x81V[`@Q\x90\x81R` \x01a\x01\xBCV[a\x02_a\x16\xEAV[`@Qa\x01\xBC\x91\x90a9\x1BV[a\x01\x93a\x18NV[` Ta\x01\xA8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\x8Fa\"lV[`@Qa\x01\xBC\x91\x90a9\x99V[`\x1FTa\x01\xA8\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xBCa#7V[`@Qa\x01\xBC\x91\x90a9\xF0V[a\x01\xA8s\\i\xBE\xE7\x01\xEF\x81J+j>\xDDK\x16R\xCB\x9C\xC5\xAAo\x81V[a\x02\xEFck6\xEC\x80\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xBCV[a\x02\xBCa$\x18V[a\x02\x8Fa$\xF9V[a\x03\x1Ca%\xC4V[`@Q\x90\x15\x15\x81R` \x01a\x01\xBCV[`$Ta\x01\xA8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xDDa&~V[`\x1FTa\x03\x1C\x90`\xFF\x16\x81V[`#Ta\x01\xA8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Qc\xF8w\xCB\x19`\xE0\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj\x11U\x12\x17\xD4\x94\x10\xD7\xD5T\x93`\xAA\x1B`D\x82\x01R_Q` av\xF1_9_Q\x90_R\x90cq\xEEFM\x90\x82\x90c\xF8w\xCB\x19\x90`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xD0W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\xF7\x91\x90\x81\x01\x90a:{V[c\x01\x12\xA8\x80`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x19\x92\x91\x90a;.V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x045W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04Y\x91\x90a;OV[P`@Qa\x04f\x90a7zV[`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x04\x7FW=__>=_\xFD[P`\x1F`\x01a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`@Qa\x04\xB2\x90a7\x87V[`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x04\xCBW=__>=_\xFD[P` \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@Q`\x12\x90a\x04\xFB\x90a7\x94V[``\x80\x82R`\r\x90\x82\x01Rl+\xB90\xB882\xB2\x10\"\xBA42\xB9`\x99\x1B`\x80\x82\x01R`\xA0` \x82\x01\x81\x90R`\x04\x90\x82\x01Rc\n\xE8\xAA\x89`\xE3\x1B`\xC0\x82\x01R`\xFF\x90\x91\x16`@\x82\x01R`\xE0\x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x05^W=__>=_\xFD[P`!\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@Q`\x06\x90a\x05\x8E\x90a7\x94V[``\x80\x82R`\x08\x90\x82\x01Rg*\xA9\xA2\x10!\xB7\xB4\xB7`\xC1\x1B`\x80\x82\x01R`\xA0` \x82\x01\x81\x90R`\x04\x90\x82\x01RcUSDC`\xE0\x1B`\xC0\x82\x01R`\xFF\x90\x91\x16`@\x82\x01R`\xE0\x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x05\xECW=__>=_\xFD[P`\"\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`!T`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R\x91\x83\x16\x92c@\xC1\x0F\x19\x92a\x06A\x92\x90\x91\x16\x90h\x05k\xC7^-c\x10\0\0\x90`\x04\x01a;fV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06XW__\xFD[PZ\xF1\x15\x80\x15a\x06jW=__>=_\xFD[PP`!T`$T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc@\xC1\x0F\x19\x93Pa\x06\xAE\x92\x90\x91\x16\x90h\x05k\xC7^-c\x10\0\0\x90`\x04\x01a;fV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06\xC5W__\xFD[PZ\xF1\x15\x80\x15a\x06\xD7W=__>=_\xFD[PP`!T`%T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc@\xC1\x0F\x19\x93Pa\x07\x1B\x92\x90\x91\x16\x90h65\xC9\xAD\xC5\xDE\xA0\0\0\x90`\x04\x01a;fV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x072W__\xFD[PZ\xF1\x15\x80\x15a\x07DW=__>=_\xFD[PP`\"T`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc@\xC1\x0F\x19\x93Pa\x07\x84\x92\x90\x91\x16\x90d\x17Hv\xE8\0\x90`\x04\x01a;fV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07\x9BW__\xFD[PZ\xF1\x15\x80\x15a\x07\xADW=__>=_\xFD[PP`\"T`$T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc@\xC1\x0F\x19\x93Pa\x07\xED\x92\x90\x91\x16\x90d\x17Hv\xE8\0\x90`\x04\x01a;fV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\x04W__\xFD[PZ\xF1\x15\x80\x15a\x08\x16W=__>=_\xFD[PP`\"T`%T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc@\xC1\x0F\x19\x93Pa\x08W\x92\x90\x91\x16\x90e\x04\x8C'9P\0\x90`\x04\x01a;fV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08nW__\xFD[PZ\xF1\x15\x80\x15a\x08\x80W=__>=_\xFD[PP`#T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` av\xF1_9_Q\x90_R\x92Pc\xCAf\x9F\xA7\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\xD3W__\xFD[PZ\xF1\x15\x80\x15a\x08\xE5W=__>=_\xFD[PP`!T`\x1FT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\t^\xA7\xB3\x93Pa\t'\x92a\x01\0\x90\x92\x04\x90\x91\x16\x90_\x19\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\tCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tg\x91\x90a;\x7FV[P`#T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` av\xF1_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\t\xB7W__\xFD[PZ\xF1\x15\x80\x15a\t\xC9W=__>=_\xFD[PP`\"T`\x1FT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\t^\xA7\xB3\x93Pa\n\x0B\x92a\x01\0\x90\x92\x04\x90\x91\x16\x90_\x19\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\n'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nK\x91\x90a;\x7FV[P`$\x80T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` av\xF1_9_Q\x90_R\x91c\xCAf\x9F\xA7\x91\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n\x9AW__\xFD[PZ\xF1\x15\x80\x15a\n\xACW=__>=_\xFD[PP`!T`\x1FT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\t^\xA7\xB3\x93Pa\n\xEE\x92a\x01\0\x90\x92\x04\x90\x91\x16\x90_\x19\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0B\nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B.\x91\x90a;\x7FV[P`$\x80T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` av\xF1_9_Q\x90_R\x91c\xCAf\x9F\xA7\x91\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0B}W__\xFD[PZ\xF1\x15\x80\x15a\x0B\x8FW=__>=_\xFD[PP`\"T`\x1FT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\t^\xA7\xB3\x93Pa\x0B\xD1\x92a\x01\0\x90\x92\x04\x90\x91\x16\x90_\x19\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0B\xEDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x11\x91\x90a;\x7FV[P`%T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` av\xF1_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0CaW__\xFD[PZ\xF1\x15\x80\x15a\x0CsW=__>=_\xFD[PP`!T`\x1FT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\t^\xA7\xB3\x93Pa\x0C\xB5\x92a\x01\0\x90\x92\x04\x90\x91\x16\x90_\x19\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0C\xD1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xF5\x91\x90a;\x7FV[P`%T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` av\xF1_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\rEW__\xFD[PZ\xF1\x15\x80\x15a\rWW=__>=_\xFD[PP`\"T`\x1FT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\t^\xA7\xB3\x93Pa\r\x99\x92a\x01\0\x90\x92\x04\x90\x91\x16\x90_\x19\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\r\xB5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xD9\x91\x90a;\x7FV[PV[`@\x80Qa\x01\0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R`\x1FT` T`@Qc\xC2\xA3<\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x93\x04\x83\x16\x92c\xC2\xA3<\x1D\x92a\x0Ei\x92_Q` aw4_9_Q\x90_R\x92ck6\xEC\x80\x92\x16\x90`\x04\x01a;\xA5V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0E\x80W__\xFD[PZ\xF1\x15\x80\x15a\x0E\x92W=__>=_\xFD[PP` \x80T`@Qa\x0E\xC8\x94P_Q` aw4_9_Q\x90_R\x93Pck6\xEC\x80\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x01a;\xA5V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x90\x92\x01\x91\x90\x91 \x80\x83R`\x1FT`!T\x92Qc\xA3\xDE\xF9#`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x92\x04\x82\x16\x93c\xA3\xDE\xF9#\x93a\x0F \x93\x91\x90\x91\x16\x91`\x04\x01a;fV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0F;W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F_\x91\x90a;\xE5V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x80\x85\x01\x91\x90\x91R\x91\x81\x16` \x84\x01R`\x1FT`\"T\x84Q\x93Qc\xA3\xDE\xF9#`\xE0\x1B\x81Ra\x01\0\x90\x92\x04\x83\x16\x93c\xA3\xDE\xF9#\x93a\x0F\xAD\x93\x92\x16\x91`\x04\x01a;fV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0F\xC8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xEC\x91\x90a;\xE5V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x84\x01R\x16``\x82\x01\x81\x90R\x81Q` \x83\x01Qa\x10\x14\x92a&\xDCV[a\x10&\x81` \x01Q\x82``\x01Qa(\x85V[`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x82\x01Ra\x10=\x81a+\x84V[a\r\xD9\x81_\x01Q\x82``\x01Qa.tV[`\x1FT` T`@Qc\xC2\xA3<\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x93\x04\x83\x16\x92c\xC2\xA3<\x1D\x92a\x10\x9B\x92_Q` aw4_9_Q\x90_R\x92ck6\xEC\x80\x92\x16\x90`\x04\x01a;\xA5V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10\xB2W__\xFD[PZ\xF1\x15\x80\x15a\x10\xC4W=__>=_\xFD[PP` \x80T`@Q_\x94Pa\x10\xFA\x93P_Q` aw4_9_Q\x90_R\x92ck6\xEC\x80\x92`\x01`\x01`\xA0\x1B\x03\x16\x91\x01a;\xA5V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x1FT`!Tc\xA3\xDE\xF9#`\xE0\x1B\x84R\x91\x93P_\x92\x83\x92`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x93\x04\x83\x16\x92c\xA3\xDE\xF9#\x92a\x11S\x92\x90\x91\x16\x90\x87\x90`\x04\x01a;fV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x11nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x92\x91\x90a;\xE5V[`\x1FT`\"T`@Qc\xA3\xDE\xF9#`\xE0\x1B\x81R\x93\x95P\x91\x93P_\x92\x83\x92`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x93\x04\x83\x16\x92c\xA3\xDE\xF9#\x92a\x11\xD8\x92\x90\x91\x16\x90\x89\x90`\x04\x01a;fV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x11\xF3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x17\x91\x90a;\xE5V[`@Qcd\xE3)\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x04\x83\x01R\x86\x16`$\x82\x01R\x91\x93P\x91P_\x90s\\i\xBE\xE7\x01\xEF\x81J+j>\xDDK\x16R\xCB\x9C\xC5\xAAo\x90c\xC9\xC6S\x96\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x12}W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xA1\x91\x90a<\x16V[`@Qcd\xE3)\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x04\x83\x01R\x84\x16`$\x82\x01R\x90\x91P_\x90s\\i\xBE\xE7\x01\xEF\x81J+j>\xDDK\x16R\xCB\x9C\xC5\xAAo\x90c\xC9\xC6S\x96\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13\x05W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13)\x91\x90a<\x16V[`@Qcd\xE3)\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x89\x16`\x04\x83\x01R\x86\x16`$\x82\x01R\x90\x91P_\x90s\\i\xBE\xE7\x01\xEF\x81J+j>\xDDK\x16R\xCB\x9C\xC5\xAAo\x90c\xC9\xC6S\x96\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13\x8DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xB1\x91\x90a<\x16V[\x90Pa\x14\x07_`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15`@Q\x80`@\x01`@R\x80`\x1D\x81R` \x01\x7FYES/NO WETH pair should exist\0\0\0\x81RPa2FV[`@\x80Q\x80\x82\x01\x90\x91R`\x1D\x81R\x7FYES/NO USDC pair should exist\0\0\0` \x82\x01Ra\x14Q\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x15\x15\x90a2FV[a\x14\x88_`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14\x15`@Q\x80``\x01`@R\x80`#\x81R` \x01aw\x11`#\x919a2FV[PPPPPPPPV[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x14\xE8W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x14\xCAW[PPPPP\x90P\x90V[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x16%W_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x16\x0EW\x83\x82\x90_R` _ \x01\x80Ta\x15\x83\x90a</V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\xAF\x90a</V[\x80\x15a\x15\xFAW\x80`\x1F\x10a\x15\xD1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\xFAV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\xDDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x15fV[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x15\x15V[PPPP\x90P\x90V[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x14\xE8W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x14\xCAWPPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x14\xE8W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x14\xCAWPPPPP\x90P\x90V[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x16%W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x17=\x90a</V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x17i\x90a</V[\x80\x15a\x17\xB4W\x80`\x1F\x10a\x17\x8BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x17\xB4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x17\x97W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x186W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x17\xF8W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x17\rV[`\x1FT` T`@Qc\xC2\xA3<\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x93\x04\x83\x16\x92c\xC2\xA3<\x1D\x92a\x18\x9B\x92_Q` aw4_9_Q\x90_R\x92ck6\xEC\x80\x92\x16\x90`\x04\x01a;\xA5V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x18\xB2W__\xFD[PZ\xF1\x15\x80\x15a\x18\xC4W=__>=_\xFD[PP` \x80T`@Q_\x94Pa\x18\xFA\x93P_Q` aw4_9_Q\x90_R\x92ck6\xEC\x80\x92`\x01`\x01`\xA0\x1B\x03\x16\x91\x01a;\xA5V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x1FT`!Tc\xA3\xDE\xF9#`\xE0\x1B\x84R\x91\x93P_\x92\x83\x92`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x93\x04\x83\x16\x92c\xA3\xDE\xF9#\x92a\x19S\x92\x90\x91\x16\x90\x87\x90`\x04\x01a;fV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x92\x91\x90a;\xE5V[`\x1FT`\"T`@Qc\xA3\xDE\xF9#`\xE0\x1B\x81R\x93\x95P\x91\x93P_\x92\x83\x92`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x93\x04\x83\x16\x92c\xA3\xDE\xF9#\x92a\x19\xD8\x92\x90\x91\x16\x90\x89\x90`\x04\x01a;fV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19\xF3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x17\x91\x90a;\xE5V[`%T`@Qc\x03\">\xAB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x91\x93P\x91P_Q` av\xF1_9_Q\x90_R\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1AkW__\xFD[PZ\xF1\x15\x80\x15a\x1A}W=__>=_\xFD[PP`\x1FT`!T`@Qc\x9B\xA70\xA9`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc\x9B\xA70\xA9\x93Pa\x1A\xC8\x92\x91\x16\x90h\n\xD7\x8E\xBCZ\xC6 \0\0\x90\x8A\x90`\x04\x01a<gV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1A\xDFW__\xFD[PZ\xF1\x15\x80\x15a\x1A\xF1W=__>=_\xFD[PP`\x1FT`\"T`@Qc\x9B\xA70\xA9`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc\x9B\xA70\xA9\x93Pa\x1B8\x92\x91\x16\x90d\xE8\xD4\xA5\x10\0\x90\x8A\x90`\x04\x01a<gV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1BOW__\xFD[PZ\xF1\x15\x80\x15a\x1BaW=__>=_\xFD[PPPP_Q` av\xF1_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1B\xAAW__\xFD[PZ\xF1\x15\x80\x15a\x1B\xBCW=__>=_\xFD[PP`@Qcd\xE3)\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x04\x83\x01R\x85\x16`$\x82\x01R_\x92Ps\\i\xBE\xE7\x01\xEF\x81J+j>\xDDK\x16R\xCB\x9C\xC5\xAAo\x91Pc\xC9\xC6S\x96\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1C!W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CE\x91\x90a<\x16V[`%T`@Qc\x03\">\xAB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x91P_Q` av\xF1_9_Q\x90_R\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1C\x97W__\xFD[PZ\xF1\x15\x80\x15a\x1C\xA9W=__>=_\xFD[PP`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x16\x92Pc\xA9\x05\x9C\xBB\x91Pa\x1C\xE4\x90\x84\x90h\x05k\xC7^-c\x10\0\0\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1D\0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D$\x91\x90a;\x7FV[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xA9\x05\x9C\xBB\x90a\x1DX\x90\x84\x90d\xBAC\xB7@\0\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1DtW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x98\x91\x90a;\x7FV[P`%T`@Qc51<!`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x82\x16\x90cjbxB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1D\xE2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x06\x91\x90a;OV[P_Q` av\xF1_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1ELW__\xFD[PZ\xF1\x15\x80\x15a\x1E^W=__>=_\xFD[PP`@Qcd\xE3)\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x04\x83\x01R\x85\x16`$\x82\x01R_\x92Ps\\i\xBE\xE7\x01\xEF\x81J+j>\xDDK\x16R\xCB\x9C\xC5\xAAo\x91Pc\xC9\xC6S\x96\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1E\xC3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xE7\x91\x90a<\x16V[`%T`@Qc\x03\">\xAB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x91P_Q` av\xF1_9_Q\x90_R\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1F9W__\xFD[PZ\xF1\x15\x80\x15a\x1FKW=__>=_\xFD[PP`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x16\x92Pc\xA9\x05\x9C\xBB\x91Pa\x1F\x86\x90\x84\x90h\x05k\xC7^-c\x10\0\0\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1F\xA2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xC6\x91\x90a;\x7FV[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xA9\x05\x9C\xBB\x90a\x1F\xFA\x90\x84\x90dE\xD9d\xB8\0\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a \x16W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a :\x91\x90a;\x7FV[P`%T`@Qc51<!`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x82\x16\x90cjbxB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a \x84W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xA8\x91\x90a;OV[P_Q` av\xF1_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a \xEEW__\xFD[PZ\xF1\x15\x80\x15a!\0W=__>=_\xFD[PPPP__\x83`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!f\x91\x90a<\x9EV[P\x91P\x91P__\x84`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\xA9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xCD\x91\x90a<\x9EV[P\x91P\x91Pa\"\x1E_`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14\x15`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x16QT\xC8\x1C\x18Z\\\x88\x1C\xDA\x1B\xDD[\x19\x08\x19^\x1A\\\xDD`Z\x1B\x81RPa2FV[`@\x80Q\x80\x82\x01\x90\x91R`\x14\x81Rs\x13\x93\xC8\x1C\x18Z\\\x88\x1C\xDA\x1B\xDD[\x19\x08\x19^\x1A\\\xDD`b\x1B` \x82\x01Ra\"_\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x15\x15\x90a2FV[PPPPPPPPPPPV[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x16%W\x83\x82\x90_R` _ \x01\x80Ta\"\xAC\x90a</V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\"\xD8\x90a</V[\x80\x15a##W\x80`\x1F\x10a\"\xFAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a##V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a#\x06W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\"\x8FV[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x16%W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a$\0W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a#\xC2W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a#ZV[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x16%W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a$\xE1W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a$\xA3W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a$;V[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x16%W\x83\x82\x90_R` _ \x01\x80Ta%9\x90a</V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta%e\x90a</V[\x80\x15a%\xB0W\x80`\x1F\x10a%\x87Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a%\xB0V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a%\x93W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a%\x1CV[`\x08T_\x90`\xFF\x16\x15a%\xDBWP`\x08T`\xFF\x16\x90V[`@Qc\x06g\xF9\xD7`\xE4\x1B\x81R_\x90_Q` av\xF1_9_Q\x90_R\x90cf\x7F\x9Dp\x90a&8\x90\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90e\x19\x98Z[\x19Y`\xD2\x1B\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&SW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&w\x91\x90a;OV[\x14\x15\x90P\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x14\xE8W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x14\xCAWPPPPP\x90P\x90V[`%T`@Qc\x03\">\xAB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` av\xF1_9_Q\x90_R\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a'+W__\xFD[PZ\xF1\x15\x80\x15a'=W=__>=_\xFD[PP`\x1FT`!T`@Qc\x9B\xA70\xA9`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc\x9B\xA70\xA9\x93Pa'\x88\x92\x91\x16\x90h\n\xD7\x8E\xBCZ\xC6 \0\0\x90\x88\x90`\x04\x01a<gV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a'\x9FW__\xFD[PZ\xF1\x15\x80\x15a'\xB1W=__>=_\xFD[PP`\x1FT`\"T`@Qc\x9B\xA70\xA9`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc\x9B\xA70\xA9\x93Pa'\xF8\x92\x91\x16\x90d\xE8\xD4\xA5\x10\0\x90\x88\x90`\x04\x01a<gV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a(\x0FW__\xFD[PZ\xF1\x15\x80\x15a(!W=__>=_\xFD[PPPP_Q` av\xF1_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a(jW__\xFD[PZ\xF1\x15\x80\x15a(|W=__>=_\xFD[PPPPPPPV[`@Qcd\xE3)\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R\x82\x16`$\x82\x01R_\x90\x81\x90s\\i\xBE\xE7\x01\xEF\x81J+j>\xDDK\x16R\xCB\x9C\xC5\xAAo\x90c\xC9\xC6S\x96\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a(\xE8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\x0C\x91\x90a<\x16V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a)`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x18Z\\\x88\x18\xDC\x99X]\x1A[\xDB\x88\x19\x98Z[\x19Y`b\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`%T`@Qc\x03\">\xAB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` av\xF1_9_Q\x90_R\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a)\xAFW__\xFD[PZ\xF1\x15\x80\x15a)\xC1W=__>=_\xFD[PP`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16\x92Pc\xA9\x05\x9C\xBB\x91Pa)\xFC\x90\x84\x90h\x05k\xC7^-c\x10\0\0\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a*\x18W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*<\x91\x90a;\x7FV[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xA9\x05\x9C\xBB\x90a*p\x90\x84\x90d\xBAC\xB7@\0\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a*\x8CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xB0\x91\x90a;\x7FV[P`%T`@Qc51<!`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x82\x16\x90cjbxB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a*\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\x1E\x91\x90a;OV[P_Q` av\xF1_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a+dW__\xFD[PZ\xF1\x15\x80\x15a+vW=__>=_\xFD[P\x92\x93PPPP[\x92\x91PPV[`#T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` av\xF1_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a+\xD3W__\xFD[PZ\xF1\x15\x80\x15a+\xE5W=__>=_\xFD[PP`\x1FT`!T\x84Q`@Qc\x9B\xA70\xA9`\xE0\x1B\x81Ra\x01\0\x90\x93\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x95Pc\x9B\xA70\xA9\x94Pa,0\x93\x92\x16\x91g\x8A\xC7#\x04\x89\xE8\0\0\x91\x90`\x04\x01a<gV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a,GW__\xFD[PZ\xF1\x15\x80\x15a,YW=__>=_\xFD[PPPP` \x81\x01Q`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xA9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xCD\x91\x90a;OV[`\xC0\x82\x01R``\x81\x01Q`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\x1EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-B\x91\x90a;OV[`\xE0\x82\x01R`\xA0\x81\x01Q` \x82\x01Q``\x83\x01Qa-a\x92\x91\x90a2\xA8V[` \x81\x01Q`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra-\xF4\x92\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\xB3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\xD7\x91\x90a;OV[g\r\xE0\xB6\xB3\xA7d\0\0\x83`\xC0\x01Qa-\xEF\x91\x90a<\xFEV[a6\x1CV[``\x81\x01Q`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\r\xD9\x92\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.j\x91\x90a;OV[\x82`\xE0\x01Qa6TV[` T`@Qc\x17\xEE6o`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c/\xDCl\xDE\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a.\xBEW__\xFD[PZ\xF1\x15\x80\x15a.\xD0W=__>=_\xFD[PP`\x1FT`@Qc\\#\xBD\xF5`\xE0\x1B\x81R`\x04\x81\x01\x86\x90Ra\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x92Pc\\#\xBD\xF5\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/\x1BW__\xFD[PZ\xF1\x15\x80\x15a/-W=__>=_\xFD[PP`\"T`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R_\x94P\x91\x16\x91Pcp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/}W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\xA1\x91\x90a;OV[`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x92P_\x91\x90\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\xEEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\x12\x91\x90a;OV[`#T`@Qc\x03\">\xAB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x91P_Q` av\xF1_9_Q\x90_R\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a0dW__\xFD[PZ\xF1\x15\x80\x15a0vW=__>=_\xFD[PP`\x1FT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16\x94Pc\t^\xA7\xB3\x93Pa0\xB1\x92a\x01\0\x90\x04\x16\x90\x85\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a0\xCDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\xF1\x91\x90a;\x7FV[P`\x1FT`@Qc\x01\xE9\xA6\x95`\xE4\x1B\x81Ra\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90c\x1E\x9AiP\x90a1(\x90\x86\x90\x85\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1h\x91\x90a;OV[P_Q` av\xF1_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a1\xAEW__\xFD[PZ\xF1\x15\x80\x15a1\xC0W=__>=_\xFD[PP`\"T`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra2@\x94P\x91\x16\x91Pcp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\x12W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a26\x91\x90a;OV[a-\xEF\x83\x85a=\x11V[PPPPV[`@Qc\xA3N\xDC\x03`\xE0\x1B\x81R_Q` av\xF1_9_Q\x90_R\x90c\xA3N\xDC\x03\x90a2x\x90\x85\x90\x85\x90`\x04\x01a=$V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a2\x8EW__\xFD[PZ\xFA\x15\x80\x15a2\xA0W=__>=_\xFD[PPPPPPV[__\x84`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\xE6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\n\x91\x90a<\x9EV[P\x91P\x91P_\x85`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3LW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3p\x91\x90a<\x16V[\x90P__\x86`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14a3\xA6W\x83`\x01`\x01`p\x1B\x03\x16\x85`\x01`\x01`p\x1B\x03\x16a3\xBBV[\x84`\x01`\x01`p\x1B\x03\x16\x84`\x01`\x01`p\x1B\x03\x16[\x90\x92P\x90Pg\r\xE0\xB6\xB3\xA7d\0\0_a3\xD5\x82\x85\x85a6\x8CV[`#T`@Qc\x03\">\xAB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x91P_Q` av\xF1_9_Q\x90_R\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a4'W__\xFD[PZ\xF1\x15\x80\x15a49W=__>=_\xFD[PP`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8C\x16\x92Pc\xA9\x05\x9C\xBB\x91Pa4k\x90\x8D\x90\x86\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a4\x87W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\xAB\x91\x90a;\x7FV[P\x88`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x03a5?W`#T`@Qc\x02,\r\x9F`\xE0\x1B\x81R_`\x04\x82\x01\x81\x90R`$\x82\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`D\x83\x01R`\x80`d\x83\x01R`\x84\x82\x01R\x90\x8B\x16\x90c\x02,\r\x9F\x90`\xA4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a5$W__\xFD[PZ\xF1\x15\x80\x15a56W=__>=_\xFD[PPPPa5\xB5V[`#T`@Qc\x02,\r\x9F`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R_`$\x82\x01\x81\x90R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`D\x83\x01R`\x80`d\x83\x01R`\x84\x82\x01R\x90\x8B\x16\x90c\x02,\r\x9F\x90`\xA4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a5\x9EW__\xFD[PZ\xF1\x15\x80\x15a5\xB0W=__>=_\xFD[PPPP[_Q` av\xF1_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a5\xFAW__\xFD[PZ\xF1\x15\x80\x15a6\x0CW=__>=_\xFD[PPPPPPPPPPPPPPV[`@Qc&\n[\x15`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R_Q` av\xF1_9_Q\x90_R\x90c\x98)lT\x90`D\x01a2xV[`@Qcm\x83\xFEi`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R_Q` av\xF1_9_Q\x90_R\x90c\xDB\x07\xFC\xD2\x90`D\x01a2xV[__\x84\x11a6\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FINSUFFICIENT_INPUT_AMOUNT\0\0\0\0\0\0\0`D\x82\x01R`d\x01a)WV[_\x83\x11\x80\x15a6\xEAWP_\x82\x11[a7/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuINSUFFICIENT_LIQUIDITY`P\x1B`D\x82\x01R`d\x01a)WV[_a7<\x85a\x03\xE5a=FV[\x90P_a7I\x84\x83a=FV[\x90P_\x82a7Y\x87a\x03\xE8a=FV[a7c\x91\x90a=\x11V[\x90Pa7o\x81\x83a=]V[\x97\x96PPPPPPPV[a%\x96\x80a=}\x839\x01\x90V[a\x05\n\x80ac\x13\x839\x01\x90V[a\x0E\xD4\x80ah\x1D\x839\x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a7\xE1W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a7\xBAV[P\x90\x95\x94PPPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a+vW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90```\x05\x82\x90\x1B\x88\x01\x81\x01\x91\x90\x88\x01\x90_[\x81\x81\x10\x15a8\xBDW`_\x19\x8A\x85\x03\x01\x83Ra8\xA7\x84\x86Qa7\xECV[` \x95\x86\x01\x95\x90\x94P\x92\x90\x92\x01\x91`\x01\x01a8\x8BV[P\x91\x97PPP` \x94\x85\x01\x94\x92\x90\x92\x01\x91P`\x01\x01a8@V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a9\x11W\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a8\xE9V[P\x93\x94\x93PPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a+vW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`@\x87Ra9g`@\x88\x01\x82a7\xECV[\x90P` \x82\x01Q\x91P\x86\x81\x03` \x88\x01Ra9\x82\x81\x83a8\xD7V[\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a9AV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a+vW`?\x19\x87\x86\x03\x01\x84Ra9\xDB\x85\x83Qa7\xECV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a9\xBFV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a+vW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90a:Q\x90\x87\x01\x82a8\xD7V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a:\x16V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a:\x8BW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\xA1W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a:\xB1W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\xCBWa:\xCBa:gV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a:\xFAWa:\xFAa:gV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a;\x11W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`@\x81R_a;@`@\x83\x01\x85a7\xECV[\x90P\x82` \x83\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a;_W__\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[_` \x82\x84\x03\x12\x15a;\x8FW__\xFD[\x81Q\x80\x15\x15\x81\x14a;\x9EW__\xFD[\x93\x92PPPV[\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x90\x91\x16` \x83\x01R`\x01`\x01`\xA0\x1B\x03\x16`@\x82\x01R``\x01\x90V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a;\xE0W__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a;\xF6W__\xFD[a;\xFF\x83a;\xCAV[\x91Pa<\r` \x84\x01a;\xCAV[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a<&W__\xFD[a;\x9E\x82a;\xCAV[`\x01\x81\x81\x1C\x90\x82\x16\x80a<CW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a<aWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R`@\x82\x01R``\x01\x90V[\x80Q`\x01`\x01`p\x1B\x03\x81\x16\x81\x14a;\xE0W__\xFD[___``\x84\x86\x03\x12\x15a<\xB0W__\xFD[a<\xB9\x84a<\x88V[\x92Pa<\xC7` \x85\x01a<\x88V[\x91P`@\x84\x01Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a<\xDFW__\xFD[\x80\x91PP\x92P\x92P\x92V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a+~Wa+~a<\xEAV[\x80\x82\x01\x80\x82\x11\x15a+~Wa+~a<\xEAV[\x82\x15\x15\x81R`@` \x82\x01R_a=>`@\x83\x01\x84a7\xECV[\x94\x93PPPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a+~Wa+~a<\xEAV[_\x82a=wWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE`\x80`@R4\x80\x15`\x0EW__\xFD[Pa%z\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\x9BW_5`\xE0\x1C\x80c\x9B\xA70\xA9\x11a\0cW\x80c\x9B\xA70\xA9\x14a\x01zW\x80c\xA3\xDE\xF9#\x14a\x01\x8DW\x80c\xC2\xA3<\x1D\x14a\x01\xC0W\x80c\xD4\xB0o\xB8\x14a\x01\xD3W\x80c\xE9s\x95P\x14a\x02\x02W__\xFD[\x80c\x1E\x9AiP\x14a\0\x9FW\x80cA\x05Zv\x14a\0\xC5W\x80c\\#\xBD\xF5\x14a\0\xDAW\x80cq\x83\xD2J\x14a\0\xEDW\x80cud\x91+\x14a\x01\x10W[__\xFD[a\0\xB2a\0\xAD6`\x04a\x11\xDFV[a\x02\x15V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xD8a\0\xD36`\x04a\x12\tV[a\x06*V[\0[a\0\xD8a\0\xE86`\x04a\x12;V[a\x08\xE1V[a\x01\0a\0\xFB6`\x04a\x12RV[a\nFV[`@Q\x90\x15\x15\x81R` \x01a\0\xBCV[a\x01Pa\x01\x1E6`\x04a\x12;V[_` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x82\x16\x91d\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90\x83V[`@\x80Qc\xFF\xFF\xFF\xFF\x90\x94\x16\x84R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x84\x01R\x90\x82\x01R``\x01a\0\xBCV[a\0\xD8a\x01\x886`\x04a\x12\tV[a\x0B\xF1V[a\x01\xA0a\x01\x9B6`\x04a\x11\xDFV[a\r\xCDV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\0\xBCV[a\0\xD8a\x01\xCE6`\x04a\x12tV[a\x0F\x0FV[a\x01\xF5a\x01\xE16`\x04a\x12;V[`\x01` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Qa\0\xBC\x91\x90a\x12\xCFV[a\x01\xA0a\x02\x106`\x04a\x11\xDFV[a\x10LV[__`@Q` \x01a\x020\x90bYES`\xE8\x1B\x81R`\x03\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x81W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\xA8\x91\x90\x81\x01\x90a\x13\tV[`@Q` \x01a\x02\xB8\x91\x90a\x13\xD3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14\x90P_\x84`\x01`\x01`\xA0\x1B\x03\x16c\xC0GM\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x0EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x032\x91\x90a\x13\xDEV[\x90P_\x85`\x01`\x01`\xA0\x1B\x03\x16cH\0\xD9\x7F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03qW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x95\x91\x90a\x13\xF5V[\x90Pa\x03\xA2\x83\x82\x84a\x10pV[`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x14a\x03\xD3W`@Qc@\xFEP\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_\x83\x81R`\x01` R`@\x90 T`\xFF\x16`\x04\x81\x11\x15a\x03\xF7Wa\x03\xF7a\x12\xBBV[\x14\x80\x15a\x04\x01WP\x82[\x15a\x04\x0EW\x84\x93Pa\x04\x9EV[`\x03_\x83\x81R`\x01` R`@\x90 T`\xFF\x16`\x04\x81\x11\x15a\x042Wa\x042a\x12\xBBV[\x14\x80\x15a\x04=WP\x82\x15[\x15a\x04JW\x84\x93Pa\x04\x9EV[`\x04_\x83\x81R`\x01` R`@\x90 T`\xFF\x16`\x04\x81\x11\x15a\x04nWa\x04na\x12\xBBV[\x03a\x04\x85Wa\x04~`\x02\x86a\x14\x10V[\x93Pa\x04\x9EV[`@QcrV\xF6E`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c#\xB8r\xDD\x90a\x04\xCE\x903\x900\x90\x8A\x90`\x04\x01a\x14/V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x04\xEAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x0E\x91\x90a\x14SV[P`@Qc'p\xA7\xEB`\xE2\x1B\x81R0`\x04\x82\x01R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c\x9D\xC2\x9F\xAC\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05TW__\xFD[PZ\xF1\x15\x80\x15a\x05fW=__>=_\xFD[PP`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x92Pc\xA9\x05\x9C\xBB\x91P`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x05\xB4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD8\x91\x90a\x14SV[P`@\x80Q\x86\x81R` \x81\x01\x86\x90R\x83\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x913\x91\x7F\xAE\x1D\x80J\x86\xB6\xBA*w%\xD9\x82ML\xCBN\x7F\x9AU\xE7\xA1\xBFCy\xC2f\x92\xCEZ\xD0\x06e\x91\x01`@Q\x80\x91\x03\x90\xA4PPP\x92\x91PPV[`\x01_\x82\x81R`\x01` R`@\x90 T`\xFF\x16`\x04\x81\x11\x15a\x06NWa\x06Na\x12\xBBV[\x14a\x06lW`@Qc#\x9D\xD4\xAD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a\x06x\x85\x84a\x10LV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c#\xB8r\xDD\x90a\x06\xAD\x903\x900\x90\x89\x90`\x04\x01a\x14/V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06\xC9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xED\x91\x90a\x14SV[P`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c#\xB8r\xDD\x90a\x07\x1E\x903\x900\x90\x89\x90`\x04\x01a\x14/V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x07:W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07^\x91\x90a\x14SV[P`@Qc'p\xA7\xEB`\xE2\x1B\x81R0`\x04\x82\x01R`$\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\x9D\xC2\x9F\xAC\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07\xA4W__\xFD[PZ\xF1\x15\x80\x15a\x07\xB6W=__>=_\xFD[PP`@Qc'p\xA7\xEB`\xE2\x1B\x81R0`\x04\x82\x01R`$\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x92Pc\x9D\xC2\x9F\xAC\x91P`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07\xFFW__\xFD[PZ\xF1\x15\x80\x15a\x08\x11W=__>=_\xFD[PP`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16\x92Pc\xA9\x05\x9C\xBB\x91P`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08_W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x83\x91\x90a\x14SV[P`@\x80Q\x85\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16` \x83\x01R\x83\x81\x16\x92\x82\x01\x92\x90\x92R\x84\x91\x87\x16\x903\x90\x7F\x9A\x85;\xFB\xDD`4\xE0\xE7U;\xB8\xECw\x89\xB2`=\xCC\xCBWY\xCF9\nz`\x1C\x0Bm\xA0\x03\x90``\x01[`@Q\x80\x91\x03\x90\xA4PPPPPV[`\x01_\x82\x81R`\x01` R`@\x90 T`\xFF\x16`\x04\x81\x11\x15a\t\x05Wa\t\x05a\x12\xBBV[\x14a\t#W`@Qc#\x9D\xD4\xAD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R` \x81\x90R`@\x81 Tc\xFF\xFF\xFF\xFF\x16B\x10a\tEWP`\x04a\t\xD8V[_\x82\x81R` \x81\x90R`@\x80\x82 T\x90Qc\x01\xF4k)`\xE1\x1B\x81R`\x04\x81\x01\x85\x90Rd\x01\0\0\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90c\x03\xE8\xD6R\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\t\x9FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xC3\x91\x90a\x14SV[\x90P\x80a\t\xD1W`\x03a\t\xD4V[`\x02[\x91PP[_\x82\x81R`\x01` \x81\x90R`@\x90\x91 \x80T\x83\x92`\xFF\x19\x90\x91\x16\x90\x83`\x04\x81\x11\x15a\n\x05Wa\n\x05a\x12\xBBV[\x02\x17\x90UP\x81\x7F\xF3I\x84G1H\x05\x1B\xC1\xBD\xF1\xBEn\xCCF-{\"\x8DY\x10X\xA8\xA2yw\xB8Gp\xB78\xB9\x82`@Qa\n:\x91\x90a\x12\xCFV[`@Q\x80\x91\x03\x90\xA2PPV[__\x82`\x01`\x01`\xA0\x1B\x03\x16c\xC0GM\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x84W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xA8\x91\x90a\x13\xDEV[\x90P_\x83`\x01`\x01`\xA0\x1B\x03\x16cH\0\xD9\x7F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xE7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x0B\x91\x90a\x13\xF5V[\x90P_`@Q` \x01a\x0B'\x90bYES`\xE8\x1B\x81R`\x03\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BxW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0B\x9F\x91\x90\x81\x01\x90a\x13\tV[`@Q` \x01a\x0B\xAF\x91\x90a\x13\xD3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14\x90Pa\x0B\xD3\x81\x83\x85a\x10pV[`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x14\x93PPPP\x91\x90PV[`\x01_\x82\x81R`\x01` R`@\x90 T`\xFF\x16`\x04\x81\x11\x15a\x0C\x15Wa\x0C\x15a\x12\xBBV[\x14a\x0C3W`@Qc#\x9D\xD4\xAD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c#\xB8r\xDD\x90a\x0Cc\x903\x900\x90\x87\x90`\x04\x01a\x14/V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0C\x7FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xA3\x91\x90a\x14SV[P__a\x0C\xB0\x85\x84a\x10LV[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x87\x90R\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c@\xC1\x0F\x19\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\xFAW__\xFD[PZ\xF1\x15\x80\x15a\r\x0CW=__>=_\xFD[PP`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x92Pc@\xC1\x0F\x19\x91P`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\rUW__\xFD[PZ\xF1\x15\x80\x15a\rgW=__>=_\xFD[PPPP\x82\x85`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8F\xB8\xA3)\xBCh\x855h\xE0\xDD'\xC6\xAE\xE2\xE3a\xA1\xF0x\x92\xCFe\xABI\xCC/0\xCD!\xC3\xC5\x87\x86\x86`@Qa\x08\xD2\x93\x92\x91\x90\x92\x83R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[__\x82\x84\x84`@Qa\r\xDE\x90a\x11\xBBV[`\x80\x80\x82R`\x03\x90\x82\x01\x81\x90RbYes`\xE8\x1B`\xA0\x83\x01R`\xC0` \x83\x01\x81\x90R\x82\x01RbYES`\xE8\x1B`\xE0\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`@\x83\x01R``\x82\x01Ra\x01\0\x01\x81\x90`@Q\x80\x91\x03\x90_\xF5\x90P\x80\x15\x80\x15a\x0EEW=__>=_\xFD[P\x91P\x82\x84\x84`@Qa\x0EW\x90a\x11\xBBV[`\x80\x80\x82R`\x02\x90\x82\x01\x81\x90RaNo`\xF0\x1B`\xA0\x83\x01R`\xC0` \x83\x01\x81\x90R\x82\x01RaNO`\xF0\x1B`\xE0\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`@\x83\x01R``\x82\x01Ra\x01\0\x01\x81\x90`@Q\x80\x91\x03\x90_\xF5\x90P\x80\x15\x80\x15a\x0E\xBCW=__>=_\xFD[P`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x83\x81\x16` \x83\x01R\x82Q\x93\x94P\x87\x16\x92\x86\x92\x7F\x9EC\x1C\xAE\xF5u>>\x93\x96S\xFC\x05l\x91\x07%\x98\0)\xB3e=\xB5\xA3k\xEB\xCAq\xD6\xBC,\x92\x82\x90\x03\x01\x90\xA3\x92P\x92\x90PV[`@\x80Q` \x81\x01\x85\x90Rc\xFF\xFF\xFF\xFF\x84\x16\x91\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x82\x16``\x82\x01R_\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x90P__\x82\x81R`\x01` R`@\x90 T`\xFF\x16`\x04\x81\x11\x15a\x0FyWa\x0Fya\x12\xBBV[\x14a\x0F\x97W`@Qc7\xA5\xBD\x9B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x01` \x81\x81R`@\x80\x84 \x80T`\xFF\x19\x16\x84\x17\x90U\x80Q``\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x88\x81\x16\x80\x83R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x84\x87\x01\x81\x81R\x85\x87\x01\x8E\x81R\x8B\x8BR\x8A\x89R\x99\x87\x90 \x95Q\x86T\x91Q\x95\x16`\x01`\x01`\xC0\x1B\x03\x19\x90\x91\x16\x17d\x01\0\0\0\0\x94\x90\x92\x16\x93\x90\x93\x02\x17\x83U\x95Q\x91\x90\x94\x01U\x80Q\x93\x84RQ\x91\x92\x87\x92\x85\x92\x7F\tN\xBAi\xD7\xCA\x9D\xFA\xFA4\xC8\x96\x06\x7F\xC8\xE1\x93\x95\xA6\x10\xE4}\xEB\x8B%\xB2\x1B\x87\xBE\xC3*4\x92\x82\x90\x03\x01\x90\xA4PPPPV[__a\x10Z`\x01\x85\x85a\x10pV[\x91Pa\x10g_\x85\x85a\x10pV[\x90P\x92P\x92\x90PV[__`@Q\x80` \x01a\x10\x82\x90a\x11\xBBV[`\x1F\x19\x82\x82\x03\x81\x01\x83R`\x1F\x90\x91\x01\x16`@R\x85a\x10\xBAW`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aNo`\xF0\x1B\x81RPa\x10\xD7V[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bYes`\xE8\x1B\x81RP[\x86a\x10\xFCW`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aNO`\xF0\x1B\x81RPa\x11\x19V[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bYES`\xE8\x1B\x81RP[\x86\x86`@Q` \x01a\x11.\x94\x93\x92\x91\x90a\x14\xA0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x11L\x92\x91` \x01a\x14\xE1V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\x01`\x01`\xF8\x1B\x03\x19\x82\x85\x01R0``\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`!\x85\x01R`5\x84\x01\x96\x90\x96R`U\x80\x84\x01\x96\x90\x96R\x81Q\x80\x84\x03\x90\x96\x01\x86R`u\x90\x92\x01\x90R\x83Q\x93\x01\x92\x90\x92 \x94\x93PPPPV[a\x10G\x80a\x14\xFE\x839\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x11\xDCW__\xFD[PV[__`@\x83\x85\x03\x12\x15a\x11\xF0W__\xFD[\x825a\x11\xFB\x81a\x11\xC8V[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a\x12\x1BW__\xFD[\x835a\x12&\x81a\x11\xC8V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x12KW__\xFD[P5\x91\x90PV[_` \x82\x84\x03\x12\x15a\x12bW__\xFD[\x815a\x12m\x81a\x11\xC8V[\x93\x92PPPV[___``\x84\x86\x03\x12\x15a\x12\x86W__\xFD[\x835\x92P` \x84\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x12\xA0W__\xFD[\x91P`@\x84\x015a\x12\xB0\x81a\x11\xC8V[\x80\x91PP\x92P\x92P\x92V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[` \x81\x01`\x05\x83\x10a\x12\xEFWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x91\x90R\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x13\x19W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13/W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x13?W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13YWa\x13Ya\x12\xF5V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x13\x88Wa\x13\x88a\x12\xF5V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x13\x9FW__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\x12m\x82\x84a\x13\xBCV[_` \x82\x84\x03\x12\x15a\x13\xEEW__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15a\x14\x05W__\xFD[\x81Qa\x12m\x81a\x11\xC8V[_\x82a\x14*WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[_` \x82\x84\x03\x12\x15a\x14cW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x12mW__\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x80\x81R_a\x14\xB2`\x80\x83\x01\x87a\x14rV[\x82\x81\x03` \x84\x01Ra\x14\xC4\x81\x87a\x14rV[`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16`@\x84\x01RPP``\x01R\x92\x91PPV[_a\x14\xF5a\x14\xEF\x83\x86a\x13\xBCV[\x84a\x13\xBCV[\x94\x93PPPPV\xFE`\xC0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x10G8\x03\x80a\x10G\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01CV[_a\09\x85\x82a\x02RV[P`\x01a\0F\x84\x82a\x02RV[P`\x01`\x01`\xA0\x1B\x03\x82\x16`\x80R`\xA0\x81\x90Ra\0b3a\0kV[PPPPa\x03\x0CV[`\x01`\x01`\xA0\x1B\x03\x16c\x8Bx\xC6\xD8\x19\x81\x90U\x80_\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x81\x80\xA3PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a\0\xC9W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\0\xE2Wa\0\xE2a\0\xA6V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x01\x10Wa\x01\x10a\0\xA6V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\x01'W__\xFD[\x81` \x85\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[____`\x80\x85\x87\x03\x12\x15a\x01VW__\xFD[\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01kW__\xFD[a\x01w\x87\x82\x88\x01a\0\xBAV[` \x87\x01Q\x90\x95P\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\x94W__\xFD[a\x01\xA0\x87\x82\x88\x01a\0\xBAV[`@\x87\x01Q\x90\x94P\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xBEW__\xFD[``\x95\x90\x95\x01Q\x93\x96\x92\x95PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01\xE2W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x02\0WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x02MW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x02+WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x02JW_\x81U`\x01\x01a\x027V[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02kWa\x02ka\0\xA6V[a\x02\x7F\x81a\x02y\x84Ta\x01\xCEV[\x84a\x02\x06V[` `\x1F\x82\x11`\x01\x81\x14a\x02\xB1W_\x83\x15a\x02\x9AWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x02JV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x02\xE0W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x02\xC0V[P\x84\x82\x10\x15a\x02\xFDW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80Q`\xA0Qa\r\x1Aa\x03-_9_a\x03n\x01R_a\x02G\x01Ra\r\x1A_\xF3\xFE`\x80`@R`\x046\x10a\x01<W_5`\xE0\x1C\x80cqP\x18\xA6\x11a\0\xB3W\x80c\xC0GM\x0B\x11a\0mW\x80c\xC0GM\x0B\x14a\x03]W\x80c\xD5\x05\xAC\xCF\x14a\x03\x90W\x80c\xDDb\xED>\x14a\x03\xAFW\x80c\xF0N(>\x14a\x03\xCEW\x80c\xF2\xFD\xE3\x8B\x14a\x03\xE1W\x80c\xFE\xE8\x1C\xF4\x14a\x03\xF4W__\xFD[\x80cqP\x18\xA6\x14a\x02\xBAW\x80c~\xCE\xBE\0\x14a\x02\xC2W\x80c\x8D\xA5\xCB[\x14a\x02\xF3W\x80c\x95\xD8\x9BA\x14a\x03\x0BW\x80c\x9D\xC2\x9F\xAC\x14a\x03\x1FW\x80c\xA9\x05\x9C\xBB\x14a\x03>W__\xFD[\x80c1<\xE5g\x11a\x01\x04W\x80c1<\xE5g\x14a\x01\xE8W\x80c6D\xE5\x15\x14a\x02\x03W\x80c@\xC1\x0F\x19\x14a\x02\x17W\x80cH\0\xD9\x7F\x14a\x026W\x80cT\xD1\xF1=\x14a\x02\x81W\x80cp\xA0\x821\x14a\x02\x89W__\xFD[\x80c\x06\xFD\xDE\x03\x14a\x01@W\x80c\t^\xA7\xB3\x14a\x01jW\x80c\x18\x16\r\xDD\x14a\x01\x99W\x80c#\xB8r\xDD\x14a\x01\xBFW\x80c%i)b\x14a\x01\xDEW[__\xFD[4\x80\x15a\x01KW__\xFD[Pa\x01Ta\x04%V[`@Qa\x01a\x91\x90a\x0B\x1CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01uW__\xFD[Pa\x01\x89a\x01\x846`\x04a\x0BlV[a\x04\xB4V[`@Q\x90\x15\x15\x81R` \x01a\x01aV[4\x80\x15a\x01\xA4W__\xFD[Ph\x054\\\xDFw\xEBh\xF4LT[`@Q\x90\x81R` \x01a\x01aV[4\x80\x15a\x01\xCAW__\xFD[Pa\x01\x89a\x01\xD96`\x04a\x0B\x94V[a\x054V[a\x01\xE6a\x05\xF0V[\0[4\x80\x15a\x01\xF3W__\xFD[P`@Q`\x12\x81R` \x01a\x01aV[4\x80\x15a\x02\x0EW__\xFD[Pa\x01\xB1a\x06=V[4\x80\x15a\x02\"W__\xFD[Pa\x01\xE6a\x0216`\x04a\x0BlV[a\x06\xB9V[4\x80\x15a\x02AW__\xFD[Pa\x02i\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01aV[a\x01\xE6a\x06\xCFV[4\x80\x15a\x02\x94W__\xFD[Pa\x01\xB1a\x02\xA36`\x04a\x0B\xCEV[c\x87\xA2\x11\xA2`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[a\x01\xE6a\x07\x08V[4\x80\x15a\x02\xCDW__\xFD[Pa\x01\xB1a\x02\xDC6`\x04a\x0B\xCEV[c87u\x08`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[4\x80\x15a\x02\xFEW__\xFD[Pc\x8Bx\xC6\xD8\x19Ta\x02iV[4\x80\x15a\x03\x16W__\xFD[Pa\x01Ta\x07\x1BV[4\x80\x15a\x03*W__\xFD[Pa\x01\xE6a\x0396`\x04a\x0BlV[a\x07*V[4\x80\x15a\x03IW__\xFD[Pa\x01\x89a\x03X6`\x04a\x0BlV[a\x07<V[4\x80\x15a\x03hW__\xFD[Pa\x01\xB1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\x9BW__\xFD[Pa\x01\xE6a\x03\xAA6`\x04a\x0B\xEEV[a\x07\xA0V[4\x80\x15a\x03\xBAW__\xFD[Pa\x01\xB1a\x03\xC96`\x04a\x0C[V[a\tTV[a\x01\xE6a\x03\xDC6`\x04a\x0B\xCEV[a\t\x98V[a\x01\xE6a\x03\xEF6`\x04a\x0B\xCEV[a\t\xD5V[4\x80\x15a\x03\xFFW__\xFD[Pa\x01\xB1a\x04\x0E6`\x04a\x0B\xCEV[c8\x9Au\xE1`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[``_\x80Ta\x043\x90a\x0C\x8CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04_\x90a\x0C\x8CV[\x80\x15a\x04\xAAW\x80`\x1F\x10a\x04\x81Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xAAV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x8DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[_`\x01`\x01`\xA0\x1B\x03\x83\x16n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x18\x82\x19\x15\x17a\x04\xE5Wc?hS\x9A_R`\x04`\x1C\xFD[\x82` Rc\x7F^\x9F `\x0CR3_R\x81`4`\x0C U\x81_R`,Q``\x1C3\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` _\xA3P`\x01[\x92\x91PPV[_\x83``\x1Bn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA33\x14a\x05\x89W3` Rc\x7F^\x9F \x81\x17`\x0CR`4`\x0C \x80T\x80\x19\x15a\x05\x86W\x80\x85\x11\x15a\x05\x80Wc\x13\xBE%+_R`\x04`\x1C\xFD[\x84\x81\x03\x82U[PP[c\x87\xA2\x11\xA2\x81\x17`\x0CR` `\x0C \x80T\x80\x85\x11\x15a\x05\xAFWc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x84\x81\x03\x82UPP\x83_R` `\x0C \x83\x81T\x01\x81UP\x82` R`\x0CQ``\x1C\x81``\x1C_Q` a\x0C\xC5_9_Q\x90_R` \x80\xA3PP`\x01\x93\x92PPPV[_b\x02\xA3\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x01\x90Pc8\x9Au\xE1`\x0CR3_R\x80` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D__\xA2PV[_\x80a\x06Ga\x04%V[\x80Q\x90` \x01 \x90P`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R` \x81\x01\x92\x90\x92R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6\x90\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 \x91\x90PV[a\x06\xC1a\t\xFBV[a\x06\xCB\x82\x82a\n\x15V[PPV[c8\x9Au\xE1`\x0CR3_R_` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92__\xA2V[a\x07\x10a\t\xFBV[a\x07\x19_a\n~V[V[```\x01\x80Ta\x043\x90a\x0C\x8CV[a\x072a\t\xFBV[a\x06\xCB\x82\x82a\n\xBBV[_c\x87\xA2\x11\xA2`\x0CR3_R` `\x0C \x80T\x80\x84\x11\x15a\x07dWc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x83\x81\x03\x82UPP\x82_R` `\x0C \x82\x81T\x01\x81UP\x81` R`\x0CQ``\x1C3_Q` a\x0C\xC5_9_Q\x90_R` \x80\xA3P`\x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x86\x16n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x18\x85\x19\x15\x17a\x07\xD0Wc?hS\x9A_R`\x04`\x1C\xFD[_a\x07\xD9a\x04%V[\x80Q\x90` \x01 \x90P\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6B\x86\x10\x15a\x08\x18Wc\x1A\x15\xA3\xCC_R`\x04`\x1C\xFD[`@Q\x89``\x1B``\x1C\x99P\x88``\x1B``\x1C\x98Pe87u\x08\x19\x01`\x0ER\x89_R` `\x0C \x80T\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x83R\x84` \x84\x01R\x83`@\x84\x01RF``\x84\x01R0`\x80\x84\x01R`\xA0\x83 `.R\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x83R\x8B` \x84\x01R\x8A`@\x84\x01R\x89``\x84\x01R\x80`\x80\x84\x01R\x88`\xA0\x84\x01R`\xC0\x83 `NR`B`, _R\x87`\xFF\x16` R\x86`@R\x85``R` \x80`\x80_`\x01Z\xFA\x8C=Q\x14a\t\0Wc\xDD\xAF\xBA\xEF_R`\x04`\x1C\xFD[\x01\x90Uc\x03\xFA\xF4\xF9`\xA5\x1B\x89\x17`@R`4`, \x88\x90U\x88\x8A\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` ``\x85\x01\xA3`@RPP_``RPPPPPPPV[_n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA2\x19`\x01`\x01`\xA0\x1B\x03\x83\x16\x01a\t}WP_\x19a\x05.V[P` Rc\x7F^\x9F `\x0C\x90\x81R_\x91\x90\x91R`4\x90 T\x90V[a\t\xA0a\t\xFBV[c8\x9Au\xE1`\x0CR\x80_R` `\x0C \x80TB\x11\x15a\t\xC6Wco^\x88\x18_R`\x04`\x1C\xFD[_\x90Ua\t\xD2\x81a\n~V[PV[a\t\xDDa\t\xFBV[\x80``\x1Ba\t\xF2WctH\xFB\xAE_R`\x04`\x1C\xFD[a\t\xD2\x81a\n~V[c\x8Bx\xC6\xD8\x19T3\x14a\x07\x19Wc\x82\xB4)\0_R`\x04`\x1C\xFD[h\x054\\\xDFw\xEBh\xF4LT\x81\x81\x01\x81\x81\x10\x15a\n8Wc\xE5\xCF\xE9W_R`\x04`\x1C\xFD[\x80h\x054\\\xDFw\xEBh\xF4LUPPc\x87\xA2\x11\xA2`\x0CR\x81_R` `\x0C \x81\x81T\x01\x81UP\x80` R`\x0CQ``\x1C__Q` a\x0C\xC5_9_Q\x90_R` \x80\xA3PPV[c\x8Bx\xC6\xD8\x19\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3UV[c\x87\xA2\x11\xA2`\x0CR\x81_R` `\x0C \x80T\x80\x83\x11\x15a\n\xE2Wc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x82\x90\x03\x90Uh\x054\\\xDFw\xEBh\xF4L\x80T\x82\x90\x03\x90U_\x81\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16_Q` a\x0C\xC5_9_Q\x90_R` \x83\xA3PPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0BgW__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x0B}W__\xFD[a\x0B\x86\x83a\x0BQV[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a\x0B\xA6W__\xFD[a\x0B\xAF\x84a\x0BQV[\x92Pa\x0B\xBD` \x85\x01a\x0BQV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_` \x82\x84\x03\x12\x15a\x0B\xDEW__\xFD[a\x0B\xE7\x82a\x0BQV[\x93\x92PPPV[_______`\xE0\x88\x8A\x03\x12\x15a\x0C\x04W__\xFD[a\x0C\r\x88a\x0BQV[\x96Pa\x0C\x1B` \x89\x01a\x0BQV[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015`\xFF\x81\x16\x81\x14a\x0C>W__\xFD[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x0ClW__\xFD[a\x0Cu\x83a\x0BQV[\x91Pa\x0C\x83` \x84\x01a\x0BQV[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0C\xA0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0C\xBEWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \xF2U\x8A\xD4F\x0C\xA0\x0E\xBE\xBA\xB8\xD0#r\xDDk\x02\x7F\x19\xDE{\xD9\xD72\xADt\xC5\xCF$c_EdsolcC\0\x08\x1E\x003\xA2dipfsX\"\x12 \x91\x9D\xE7\xB4Y\r\xF5k\xC1\x1A\xEB0O\xA2\x9D{D\x14_5\x084\x1D\xC0\xA2\x0C\x84\xEE\xAB5\x8D\x15dsolcC\0\x08\x1E\x003`\x80`@R4\x80\x15`\x0EW__\xFD[P`\x163`\x1AV[`UV[`\x01`\x01`\xA0\x1B\x03\x16c\x8Bx\xC6\xD8\x19\x81\x90U\x80_\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x81\x80\xA3PV[a\x04\xA8\x80a\0b_9_\xF3\xFE`\x80`@R`\x046\x10a\0\x9AW_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0bW\x80c\x8D\xA5\xCB[\x14a\x01\x0BW\x80c\xD4\xB0o\xB8\x14a\x016W\x80c\xF0N(>\x14a\x01dW\x80c\xF2\xFD\xE3\x8B\x14a\x01wW\x80c\xF66\xE0\xF6\x14a\x01\x8AW\x80c\xFE\xE8\x1C\xF4\x14a\x01\xB8W__\xFD[\x80c\x03\xE8\xD6R\x14a\0\x9EW\x80c%i)b\x14a\0\xD2W\x80c/\xDCl\xDE\x14a\0\xDCW\x80cT\xD1\xF1=\x14a\0\xFBW\x80cqP\x18\xA6\x14a\x01\x03W[__\xFD[4\x80\x15a\0\xA9W__\xFD[Pa\0\xBDa\0\xB86`\x04a\x03\xFCV[a\x01\xF7V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xDAa\x02:V[\0[4\x80\x15a\0\xE7W__\xFD[Pa\0\xDAa\0\xF66`\x04a\x04\x13V[a\x02\x87V[a\0\xDAa\x02\xF6V[a\0\xDAa\x03/V[4\x80\x15a\x01\x16W__\xFD[Pc\x8Bx\xC6\xD8\x19T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xC9V[4\x80\x15a\x01AW__\xFD[Pa\0\xBDa\x01P6`\x04a\x03\xFCV[_` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[a\0\xDAa\x01r6`\x04a\x04EV[a\x03BV[a\0\xDAa\x01\x856`\x04a\x04EV[a\x03\x7FV[4\x80\x15a\x01\x95W__\xFD[Pa\0\xBDa\x01\xA46`\x04a\x03\xFCV[`\x01` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x01\xC3W__\xFD[Pa\x01\xE9a\x01\xD26`\x04a\x04EV[c8\x9Au\xE1`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[`@Q\x90\x81R` \x01a\0\xC9V[_\x81\x81R`\x01` R`@\x81 T`\xFF\x16a\x02%W`@QcJ\xECZ\xC7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x90\x81R` \x81\x90R`@\x90 T`\xFF\x16\x90V[_b\x02\xA3\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x01\x90Pc8\x9Au\xE1`\x0CR3_R\x80` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D__\xA2PV[a\x02\x8Fa\x03\xA5V[_\x82\x81R` \x81\x81R`@\x80\x83 \x80T\x85\x15\x15`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x01\x80\x85R\x94\x83\x90 \x80T\x90\x91\x16\x90\x94\x17\x90\x93UQ\x91\x82R\x83\x91\x7FW\xC3\x9A$\x80\xAC\xCB@wR/\"S\xA3\x91Z\xFD\xC6~\x1C\xA5\x88:W\x95~\xD6\x1F/\x04\xD0#\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[c8\x9Au\xE1`\x0CR3_R_` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92__\xA2V[a\x037a\x03\xA5V[a\x03@_a\x03\xBFV[V[a\x03Ja\x03\xA5V[c8\x9Au\xE1`\x0CR\x80_R` `\x0C \x80TB\x11\x15a\x03pWco^\x88\x18_R`\x04`\x1C\xFD[_\x90Ua\x03|\x81a\x03\xBFV[PV[a\x03\x87a\x03\xA5V[\x80``\x1Ba\x03\x9CWctH\xFB\xAE_R`\x04`\x1C\xFD[a\x03|\x81a\x03\xBFV[c\x8Bx\xC6\xD8\x19T3\x14a\x03@Wc\x82\xB4)\0_R`\x04`\x1C\xFD[c\x8Bx\xC6\xD8\x19\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3UV[_` \x82\x84\x03\x12\x15a\x04\x0CW__\xFD[P5\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x04$W__\xFD[\x825\x91P` \x83\x015\x80\x15\x15\x81\x14a\x04:W__\xFD[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\x04UW__\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04kW__\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 u\x8AD\xBA\x05\x89\xAA\x1E\x9C\\QX\xFFP/<\xBBP\x16,\xDEl\x15\xD8\xD5\x97\xE8:g\x06\xCB\x02dsolcC\0\x08\x1E\x003`\xA0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x0E\xD48\x03\x80a\x0E\xD4\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01\tV[_a\09\x84\x82a\x02\nV[P`\x01a\0F\x83\x82a\x02\nV[P`\x02\x80T`\xFF\x19\x16`\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UP\x80Q` \x90\x91\x01 `\x80Ra\x02\xC4V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a\0\x8FW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\0\xA8Wa\0\xA8a\0lV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\0\xD6Wa\0\xD6a\0lV[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\0\xEDW__\xFD[\x81` \x85\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[___``\x84\x86\x03\x12\x15a\x01\x1BW__\xFD[\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x010W__\xFD[a\x01<\x86\x82\x87\x01a\0\x80V[` \x86\x01Q\x90\x94P\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01YW__\xFD[a\x01e\x86\x82\x87\x01a\0\x80V[\x92PP`@\x84\x01Q`\xFF\x81\x16\x81\x14a\x01{W__\xFD[\x80\x91PP\x92P\x92P\x92V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01\x9AW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x01\xB8WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x02\x05W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x01\xE3WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x02\x02W_\x81U`\x01\x01a\x01\xEFV[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02#Wa\x02#a\0lV[a\x027\x81a\x021\x84Ta\x01\x86V[\x84a\x01\xBEV[` `\x1F\x82\x11`\x01\x81\x14a\x02iW_\x83\x15a\x02RWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x02\x02V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x02\x98W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x02xV[P\x84\x82\x10\x15a\x02\xB5W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80Qa\x0B\xF1a\x02\xE3_9_\x81\x81a\x03\x99\x01Ra\x04\xDB\x01Ra\x0B\xF1_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xFBW_5`\xE0\x1C\x80c~\xCE\xBE\0\x11a\0\x93W\x80c\xD3\x0E\xD3\xB3\x11a\0cW\x80c\xD3\x0E\xD3\xB3\x14a\x02\x17W\x80c\xD5\x05\xAC\xCF\x14a\x02*W\x80c\xDDb\xED>\x14a\x02=W\x80c\xF8=\x17\x91\x14a\x02PW__\xFD[\x80c~\xCE\xBE\0\x14a\x01\xC4W\x80c\x95\xD8\x9BA\x14a\x01\xE9W\x80c\x9D\xC2\x9F\xAC\x14a\x01\xF1W\x80c\xA9\x05\x9C\xBB\x14a\x02\x04W__\xFD[\x80c1<\xE5g\x11a\0\xCEW\x80c1<\xE5g\x14a\x01mW\x80c6D\xE5\x15\x14a\x01\x82W\x80c@\xC1\x0F\x19\x14a\x01\x8AW\x80cp\xA0\x821\x14a\x01\x9FW__\xFD[\x80c\x06\xFD\xDE\x03\x14a\0\xFFW\x80c\t^\xA7\xB3\x14a\x01\x1DW\x80c\x18\x16\r\xDD\x14a\x01@W\x80c#\xB8r\xDD\x14a\x01ZW[__\xFD[a\x01\x07a\x02cV[`@Qa\x01\x14\x91\x90a\t\xFFV[`@Q\x80\x91\x03\x90\xF3[a\x010a\x01+6`\x04a\nJV[a\x02\xF2V[`@Q\x90\x15\x15\x81R` \x01a\x01\x14V[h\x054\\\xDFw\xEBh\xF4LT[`@Q\x90\x81R` \x01a\x01\x14V[a\x010a\x01h6`\x04a\nrV[a\x03rV[`\x02T`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\x14V[a\x01La\x03\x96V[a\x01\x9Da\x01\x986`\x04a\nJV[a\x048V[\0[a\x01La\x01\xAD6`\x04a\n\xACV[c\x87\xA2\x11\xA2`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[a\x01La\x01\xD26`\x04a\n\xACV[c87u\x08`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[a\x01\x07a\x04NV[a\x01\x9Da\x01\xFF6`\x04a\nJV[a\x04]V[a\x010a\x02\x126`\x04a\nJV[a\x04oV[a\x01\x9Da\x02%6`\x04a\nrV[a\x04\x89V[a\x01\x9Da\x0286`\x04a\n\xC5V[a\x04\xA9V[a\x01La\x02K6`\x04a\x0B2V[a\x06\x83V[a\x01\x9Da\x02^6`\x04a\nrV[a\x06\xC7V[``_\x80Ta\x02q\x90a\x0BcV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\x9D\x90a\x0BcV[\x80\x15a\x02\xE8W\x80`\x1F\x10a\x02\xBFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xE8V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xCBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[_`\x01`\x01`\xA0\x1B\x03\x83\x16n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x18\x82\x19\x15\x17a\x03#Wc?hS\x9A_R`\x04`\x1C\xFD[\x82` Rc\x7F^\x9F `\x0CR3_R\x81`4`\x0C U\x81_R`,Q``\x1C3\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` _\xA3P`\x01[\x92\x91PPV[_a\x03\x8Ea\x03\x7F\x85a\x06\xE2V[a\x03\x88\x85a\x06\xE2V[\x84a\x07\x03V[\x94\x93PPPPV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80a\x03\xCFWa\x03\xC5a\x02cV[\x80Q\x90` \x01 \x90P[`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R` \x81\x01\x92\x90\x92R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6\x90\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 \x91\x90PV[a\x04Ja\x04D\x83a\x06\xE2V[\x82a\x07\xBFV[PPV[```\x01\x80Ta\x02q\x90a\x0BcV[a\x04Ja\x04i\x83a\x06\xE2V[\x82a\x08(V[_a\x04\x82a\x04|\x84a\x06\xE2V[\x83a\x08\x89V[\x93\x92PPPV[a\x04\xA4a\x04\x95\x84a\x06\xE2V[a\x04\x9E\x84a\x06\xE2V[\x83a\x08\xEDV[PPPV[`\x01`\x01`\xA0\x1B\x03\x86\x16n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x18\x85\x19\x15\x17a\x04\xD9Wc?hS\x9A_R`\x04`\x1C\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80a\x05\x11Wa\x05\x07a\x02cV[\x80Q\x90` \x01 \x90P[\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6B\x86\x10\x15a\x05GWc\x1A\x15\xA3\xCC_R`\x04`\x1C\xFD[`@Q\x89``\x1B``\x1C\x99P\x88``\x1B``\x1C\x98Pe87u\x08\x19\x01`\x0ER\x89_R` `\x0C \x80T\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x83R\x84` \x84\x01R\x83`@\x84\x01RF``\x84\x01R0`\x80\x84\x01R`\xA0\x83 `.R\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x83R\x8B` \x84\x01R\x8A`@\x84\x01R\x89``\x84\x01R\x80`\x80\x84\x01R\x88`\xA0\x84\x01R`\xC0\x83 `NR`B`, _R\x87`\xFF\x16` R\x86`@R\x85``R` \x80`\x80_`\x01Z\xFA\x8C=Q\x14a\x06/Wc\xDD\xAF\xBA\xEF_R`\x04`\x1C\xFD[\x01\x90Uc\x03\xFA\xF4\xF9`\xA5\x1B\x89\x17`@R`4`, \x88\x90U\x88\x8A\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` ``\x85\x01\xA3`@RPP_``RPPPPPPPV[_n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA2\x19`\x01`\x01`\xA0\x1B\x03\x83\x16\x01a\x06\xACWP_\x19a\x03lV[P` Rc\x7F^\x9F `\x0C\x90\x81R_\x91\x90\x91R`4\x90 T\x90V[a\x04\xA4a\x06\xD3\x84a\x06\xE2V[a\x06\xDC\x84a\x06\xE2V[\x83a\tQV[_`\x01`\x01`\xA0\x1B\x03\x82\x16\x80`\xA0a\x06\xF9\x82a\t\xB7V[\x90\x1B\x18\x93\x92PPPV[_\x83``\x1Bn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA33\x14a\x07XW3` Rc\x7F^\x9F \x81\x17`\x0CR`4`\x0C \x80T\x80\x19\x15a\x07UW\x80\x85\x11\x15a\x07OWc\x13\xBE%+_R`\x04`\x1C\xFD[\x84\x81\x03\x82U[PP[c\x87\xA2\x11\xA2\x81\x17`\x0CR` `\x0C \x80T\x80\x85\x11\x15a\x07~Wc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x84\x81\x03\x82UPP\x83_R` `\x0C \x83\x81T\x01\x81UP\x82` R`\x0CQ``\x1C\x81``\x1C_Q` a\x0B\x9C_9_Q\x90_R` \x80\xA3PP`\x01\x93\x92PPPV[h\x054\\\xDFw\xEBh\xF4LT\x81\x81\x01\x81\x81\x10\x15a\x07\xE2Wc\xE5\xCF\xE9W_R`\x04`\x1C\xFD[\x80h\x054\\\xDFw\xEBh\xF4LUPPc\x87\xA2\x11\xA2`\x0CR\x81_R` `\x0C \x81\x81T\x01\x81UP\x80` R`\x0CQ``\x1C__Q` a\x0B\x9C_9_Q\x90_R` \x80\xA3PPV[c\x87\xA2\x11\xA2`\x0CR\x81_R` `\x0C \x80T\x80\x83\x11\x15a\x08OWc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x82\x90\x03\x90Uh\x054\\\xDFw\xEBh\xF4L\x80T\x82\x90\x03\x90U_\x81\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16_Q` a\x0B\x9C_9_Q\x90_R` \x83\xA3PPV[_c\x87\xA2\x11\xA2`\x0CR3_R` `\x0C \x80T\x80\x84\x11\x15a\x08\xB1Wc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x83\x81\x03\x82UPP\x82_R` `\x0C \x82\x81T\x01\x81UP\x81` R`\x0CQ``\x1C3_Q` a\x0B\x9C_9_Q\x90_R` \x80\xA3P`\x01\x92\x91PPV[n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA2\x19`\x01`\x01`\xA0\x1B\x03\x83\x16\x01a\t\x12WPPPV[\x81` Rc\x7F^\x9F `\x0CR\x82_R`4`\x0C \x80T\x80\x19\x15a\tJW\x80\x83\x11\x15a\tDWc\x13\xBE%+_R`\x04`\x1C\xFD[\x82\x81\x03\x82U[PPPPPV[\x82``\x1Bc\x87\xA2\x11\xA2\x81\x17`\x0CR` `\x0C \x80T\x80\x84\x11\x15a\t{Wc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x83\x81\x03\x82UPP\x82_R` `\x0C \x82\x81T\x01\x81UP\x81` R`\x0CQ``\x1C\x81``\x1C_Q` a\x0B\x9C_9_Q\x90_R` \x80\xA3PPPPV[`@Q6_\x8276\x81 ` R`\x10Q\x82\x18`\x10R`\x88_ \x90P\x80`\x10R`\xBC\x19p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Q\x82\t`\x80\x1C`\x07\x16a\t\xFAWP_[\x91\x90PV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\xFAW__\xFD[__`@\x83\x85\x03\x12\x15a\n[W__\xFD[a\nd\x83a\n4V[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a\n\x84W__\xFD[a\n\x8D\x84a\n4V[\x92Pa\n\x9B` \x85\x01a\n4V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_` \x82\x84\x03\x12\x15a\n\xBCW__\xFD[a\x04\x82\x82a\n4V[_______`\xE0\x88\x8A\x03\x12\x15a\n\xDBW__\xFD[a\n\xE4\x88a\n4V[\x96Pa\n\xF2` \x89\x01a\n4V[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015`\xFF\x81\x16\x81\x14a\x0B\x15W__\xFD[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x0BCW__\xFD[a\x0BL\x83a\n4V[\x91Pa\x0BZ` \x84\x01a\n4V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0BwW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0B\x95WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \x0CE\xAC\xC3\xC6\xA9\xD1X\x90\xEB\xBA\x87 \xCB c\xE2|m\xAA\xB30y8k\xB3\xDC\xAE\xCF\xA6L\xFBdsolcC\0\x08\x1E\x003\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-YES_WETH/YES_USDC pair should exist\"\x9AQ'\xDB\xA5I\xED\xF6\xC4Y\x17\xCD\xAC\xDE\xEB\xDC\xB0\xF5z\xD3=-\n\xA6\xE8\xD1*!i)\x02\xA2dipfsX\"\x12 @\xAF\xF2\xE2\x91Z5\x19}\x1F\xA2\t\xB4\x93Bq\xC2\xC2V\xF4b\x15\xFB\x8E-\x03\xE2\x83\xFAv\xE8\x8DdsolcC\0\x08\x1E\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610187575f3560e01c80637dc0d1d0116100d9578063b0464fdc11610093578063c09cec771161006e578063c09cec771461032c578063e20c9f711461033f578063fa7626d414610347578063fb47e3a214610354575f5ffd5b8063b0464fdc14610304578063b5508aa91461030c578063ba414fa614610314575f5ffd5b80637dc0d1d01461027457806385226c81146102875780638c809fbf1461029c578063916a17c6146102b457806399d8fae3146102c9578063a4234b4e146102e4575f5ffd5b80633e413bee116101445780633fc8cef31161011f5780633fc8cef3146102225780635af7071a1461023557806366d9a9a0146102575780637140dc731461026c575f5ffd5b80633e413bee146101ff5780633e5e3c23146102125780633f7286f41461021a575f5ffd5b80630a9254e41461018b5780630d23581714610195578063137da30c146101c55780631d828835146101cd5780631ed7831c146101d55780632ade3880146101ea575b5f5ffd5b610193610367565b005b6025546101a8906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b610193610ddc565b61019361104e565b6101dd611492565b6040516101bc91906137a1565b6101f26114f2565b6040516101bc919061381a565b6022546101a8906001600160a01b031681565b6101dd61162e565b6101dd61168c565b6021546101a8906001600160a01b031681565b6102495f5160206177345f395f51905f5281565b6040519081526020016101bc565b61025f6116ea565b6040516101bc919061391b565b61019361184e565b6020546101a8906001600160a01b031681565b61028f61226c565b6040516101bc9190613999565b601f546101a89061010090046001600160a01b031681565b6102bc612337565b6040516101bc91906139f0565b6101a8735c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f81565b6102ef636b36ec8081565b60405163ffffffff90911681526020016101bc565b6102bc612418565b61028f6124f9565b61031c6125c4565b60405190151581526020016101bc565b6024546101a8906001600160a01b031681565b6101dd61267e565b601f5461031c9060ff1681565b6023546101a8906001600160a01b031681565b60405163f877cb1960e01b815260206004820152600b60248201526a11551217d49410d7d5549360aa1b60448201525f5160206176f15f395f51905f52906371ee464d90829063f877cb19906064015f60405180830381865afa1580156103d0573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526103f79190810190613a7b565b630112a8806040518363ffffffff1660e01b8152600401610419929190613b2e565b6020604051808303815f875af1158015610435573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104599190613b4f565b506040516104669061377a565b604051809103905ff08015801561047f573d5f5f3e3d5ffd5b50601f60016101000a8154816001600160a01b0302191690836001600160a01b031602179055506040516104b290613787565b604051809103905ff0801580156104cb573d5f5f3e3d5ffd5b50602080546001600160a01b0319166001600160a01b03929092169190911790556040516012906104fb90613794565b6060808252600d908201526c2bb930b83832b21022ba3432b960991b608082015260a060208201819052600490820152630ae8aa8960e31b60c082015260ff909116604082015260e001604051809103905ff08015801561055e573d5f5f3e3d5ffd5b50602180546001600160a01b0319166001600160a01b039290921691909117905560405160069061058e90613794565b6060808252600890820152672aa9a21021b7b4b760c11b608082015260a060208201819052600490820152635553444360e01b60c082015260ff909116604082015260e001604051809103905ff0801580156105ec573d5f5f3e3d5ffd5b50602280546001600160a01b0319166001600160a01b039283161790556021546023546040516340c10f1960e01b8152918316926340c10f1992610641929091169068056bc75e2d6310000090600401613b66565b5f604051808303815f87803b158015610658575f5ffd5b505af115801561066a573d5f5f3e3d5ffd5b50506021546024546040516340c10f1960e01b81526001600160a01b0392831694506340c10f1993506106ae929091169068056bc75e2d6310000090600401613b66565b5f604051808303815f87803b1580156106c5575f5ffd5b505af11580156106d7573d5f5f3e3d5ffd5b50506021546025546040516340c10f1960e01b81526001600160a01b0392831694506340c10f19935061071b9290911690683635c9adc5dea0000090600401613b66565b5f604051808303815f87803b158015610732575f5ffd5b505af1158015610744573d5f5f3e3d5ffd5b50506022546023546040516340c10f1960e01b81526001600160a01b0392831694506340c10f199350610784929091169064174876e80090600401613b66565b5f604051808303815f87803b15801561079b575f5ffd5b505af11580156107ad573d5f5f3e3d5ffd5b50506022546024546040516340c10f1960e01b81526001600160a01b0392831694506340c10f1993506107ed929091169064174876e80090600401613b66565b5f604051808303815f87803b158015610804575f5ffd5b505af1158015610816573d5f5f3e3d5ffd5b50506022546025546040516340c10f1960e01b81526001600160a01b0392831694506340c10f199350610857929091169065048c2739500090600401613b66565b5f604051808303815f87803b15801561086e575f5ffd5b505af1158015610880573d5f5f3e3d5ffd5b505060235460405163ca669fa760e01b81526001600160a01b0390911660048201525f5160206176f15f395f51905f52925063ca669fa791506024015f604051808303815f87803b1580156108d3575f5ffd5b505af11580156108e5573d5f5f3e3d5ffd5b5050602154601f5460405163095ea7b360e01b81526001600160a01b03928316945063095ea7b3935061092792610100909204909116905f1990600401613b66565b6020604051808303815f875af1158015610943573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109679190613b7f565b5060235460405163ca669fa760e01b81526001600160a01b0390911660048201525f5160206176f15f395f51905f529063ca669fa7906024015f604051808303815f87803b1580156109b7575f5ffd5b505af11580156109c9573d5f5f3e3d5ffd5b5050602254601f5460405163095ea7b360e01b81526001600160a01b03928316945063095ea7b39350610a0b92610100909204909116905f1990600401613b66565b6020604051808303815f875af1158015610a27573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a4b9190613b7f565b506024805460405163ca669fa760e01b81526001600160a01b0390911660048201525f5160206176f15f395f51905f529163ca669fa791015f604051808303815f87803b158015610a9a575f5ffd5b505af1158015610aac573d5f5f3e3d5ffd5b5050602154601f5460405163095ea7b360e01b81526001600160a01b03928316945063095ea7b39350610aee92610100909204909116905f1990600401613b66565b6020604051808303815f875af1158015610b0a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b2e9190613b7f565b506024805460405163ca669fa760e01b81526001600160a01b0390911660048201525f5160206176f15f395f51905f529163ca669fa791015f604051808303815f87803b158015610b7d575f5ffd5b505af1158015610b8f573d5f5f3e3d5ffd5b5050602254601f5460405163095ea7b360e01b81526001600160a01b03928316945063095ea7b39350610bd192610100909204909116905f1990600401613b66565b6020604051808303815f875af1158015610bed573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c119190613b7f565b5060255460405163ca669fa760e01b81526001600160a01b0390911660048201525f5160206176f15f395f51905f529063ca669fa7906024015f604051808303815f87803b158015610c61575f5ffd5b505af1158015610c73573d5f5f3e3d5ffd5b5050602154601f5460405163095ea7b360e01b81526001600160a01b03928316945063095ea7b39350610cb592610100909204909116905f1990600401613b66565b6020604051808303815f875af1158015610cd1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cf59190613b7f565b5060255460405163ca669fa760e01b81526001600160a01b0390911660048201525f5160206176f15f395f51905f529063ca669fa7906024015f604051808303815f87803b158015610d45575f5ffd5b505af1158015610d57573d5f5f3e3d5ffd5b5050602254601f5460405163095ea7b360e01b81526001600160a01b03928316945063095ea7b39350610d9992610100909204909116905f1990600401613b66565b6020604051808303815f875af1158015610db5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610dd99190613b7f565b50565b60408051610100810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e0810191909152601f5460205460405163c2a33c1d60e01b81526001600160a01b0361010090930483169263c2a33c1d92610e69925f5160206177345f395f51905f5292636b36ec80921690600401613ba5565b5f604051808303815f87803b158015610e80575f5ffd5b505af1158015610e92573d5f5f3e3d5ffd5b505060208054604051610ec894505f5160206177345f395f51905f529350636b36ec80926001600160a01b039092169101613ba5565b60408051601f198184030181529181528151602090920191909120808352601f54602154925163a3def92360e01b81526001600160a01b0361010090920482169363a3def92393610f20939190911691600401613b66565b60408051808303815f875af1158015610f3b573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f5f9190613be5565b6001600160a01b039081166040808501919091529181166020840152601f546022548451935163a3def92360e01b815261010090920483169363a3def92393610fad93921691600401613b66565b60408051808303815f875af1158015610fc8573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610fec9190613be5565b6001600160a01b039081166080840152166060820181905281516020830151611014926126dc565b61102681602001518260600151612885565b6001600160a01b031660a082015261103d81612b84565b610dd9815f01518260600151612e74565b601f5460205460405163c2a33c1d60e01b81526001600160a01b0361010090930483169263c2a33c1d9261109b925f5160206177345f395f51905f5292636b36ec80921690600401613ba5565b5f604051808303815f87803b1580156110b2575f5ffd5b505af11580156110c4573d5f5f3e3d5ffd5b5050602080546040515f94506110fa93505f5160206177345f395f51905f5292636b36ec80926001600160a01b03169101613ba5565b60408051601f19818403018152908290528051602090910120601f5460215463a3def92360e01b84529193505f9283926001600160a01b0361010090930483169263a3def9239261115392909116908790600401613b66565b60408051808303815f875af115801561116e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111929190613be5565b601f5460225460405163a3def92360e01b81529395509193505f9283926001600160a01b0361010090930483169263a3def923926111d892909116908990600401613b66565b60408051808303815f875af11580156111f3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112179190613be5565b6040516364e329cb60e11b81526001600160a01b0380881660048301528616602482015291935091505f90735c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f9063c9c65396906044016020604051808303815f875af115801561127d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112a19190613c16565b6040516364e329cb60e11b81526001600160a01b038086166004830152841660248201529091505f90735c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f9063c9c65396906044016020604051808303815f875af1158015611305573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113299190613c16565b6040516364e329cb60e11b81526001600160a01b038089166004830152861660248201529091505f90735c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f9063c9c65396906044016020604051808303815f875af115801561138d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113b19190613c16565b90506114075f6001600160a01b0316846001600160a01b031614156040518060400160405280601d81526020017f5945532f4e4f205745544820706169722073686f756c64206578697374000000815250613246565b60408051808201909152601d81527f5945532f4e4f205553444320706169722073686f756c642065786973740000006020820152611451906001600160a01b038416151590613246565b6114885f6001600160a01b0316826001600160a01b0316141560405180606001604052806023815260200161771160239139613246565b5050505050505050565b606060168054806020026020016040519081016040528092919081815260200182805480156114e857602002820191905f5260205f20905b81546001600160a01b031681526001909101906020018083116114ca575b5050505050905090565b6060601e805480602002602001604051908101604052809291908181526020015f905b82821015611625575f84815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b8282101561160e578382905f5260205f2001805461158390613c2f565b80601f01602080910402602001604051908101604052809291908181526020018280546115af90613c2f565b80156115fa5780601f106115d1576101008083540402835291602001916115fa565b820191905f5260205f20905b8154815290600101906020018083116115dd57829003601f168201915b505050505081526020019060010190611566565b505050508152505081526020019060010190611515565b50505050905090565b606060188054806020026020016040519081016040528092919081815260200182805480156114e857602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116114ca575050505050905090565b606060178054806020026020016040519081016040528092919081815260200182805480156114e857602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116114ca575050505050905090565b6060601b805480602002602001604051908101604052809291908181526020015f905b82821015611625578382905f5260205f2090600202016040518060400160405290815f8201805461173d90613c2f565b80601f016020809104026020016040519081016040528092919081815260200182805461176990613c2f565b80156117b45780601f1061178b576101008083540402835291602001916117b4565b820191905f5260205f20905b81548152906001019060200180831161179757829003601f168201915b505050505081526020016001820180548060200260200160405190810160405280929190818152602001828054801561183657602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116117f85790505b5050505050815250508152602001906001019061170d565b601f5460205460405163c2a33c1d60e01b81526001600160a01b0361010090930483169263c2a33c1d9261189b925f5160206177345f395f51905f5292636b36ec80921690600401613ba5565b5f604051808303815f87803b1580156118b2575f5ffd5b505af11580156118c4573d5f5f3e3d5ffd5b5050602080546040515f94506118fa93505f5160206177345f395f51905f5292636b36ec80926001600160a01b03169101613ba5565b60408051601f19818403018152908290528051602090910120601f5460215463a3def92360e01b84529193505f9283926001600160a01b0361010090930483169263a3def9239261195392909116908790600401613b66565b60408051808303815f875af115801561196e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119929190613be5565b601f5460225460405163a3def92360e01b81529395509193505f9283926001600160a01b0361010090930483169263a3def923926119d892909116908990600401613b66565b60408051808303815f875af11580156119f3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a179190613be5565b6025546040516303223eab60e11b81526001600160a01b03909116600482015291935091505f5160206176f15f395f51905f52906306447d56906024015f604051808303815f87803b158015611a6b575f5ffd5b505af1158015611a7d573d5f5f3e3d5ffd5b5050601f54602154604051639ba730a960e01b81526101009092046001600160a01b039081169450639ba730a99350611ac892911690680ad78ebc5ac6200000908a90600401613c67565b5f604051808303815f87803b158015611adf575f5ffd5b505af1158015611af1573d5f5f3e3d5ffd5b5050601f54602254604051639ba730a960e01b81526101009092046001600160a01b039081169450639ba730a99350611b389291169064e8d4a51000908a90600401613c67565b5f604051808303815f87803b158015611b4f575f5ffd5b505af1158015611b61573d5f5f3e3d5ffd5b505050505f5160206176f15f395f51905f526001600160a01b03166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015611baa575f5ffd5b505af1158015611bbc573d5f5f3e3d5ffd5b50506040516364e329cb60e11b81526001600160a01b038088166004830152851660248201525f9250735c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f915063c9c65396906044016020604051808303815f875af1158015611c21573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c459190613c16565b6025546040516303223eab60e11b81526001600160a01b0390911660048201529091505f5160206176f15f395f51905f52906306447d56906024015f604051808303815f87803b158015611c97575f5ffd5b505af1158015611ca9573d5f5f3e3d5ffd5b505060405163a9059cbb60e01b81526001600160a01b038816925063a9059cbb9150611ce490849068056bc75e2d6310000090600401613b66565b6020604051808303815f875af1158015611d00573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d249190613b7f565b5060405163a9059cbb60e01b81526001600160a01b0384169063a9059cbb90611d5890849064ba43b7400090600401613b66565b6020604051808303815f875af1158015611d74573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d989190613b7f565b506025546040516335313c2160e11b81526001600160a01b03918216600482015290821690636a627842906024016020604051808303815f875af1158015611de2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e069190613b4f565b505f5160206176f15f395f51905f526001600160a01b03166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015611e4c575f5ffd5b505af1158015611e5e573d5f5f3e3d5ffd5b50506040516364e329cb60e11b81526001600160a01b038088166004830152851660248201525f9250735c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f915063c9c65396906044016020604051808303815f875af1158015611ec3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ee79190613c16565b6025546040516303223eab60e11b81526001600160a01b0390911660048201529091505f5160206176f15f395f51905f52906306447d56906024015f604051808303815f87803b158015611f39575f5ffd5b505af1158015611f4b573d5f5f3e3d5ffd5b505060405163a9059cbb60e01b81526001600160a01b038816925063a9059cbb9150611f8690849068056bc75e2d6310000090600401613b66565b6020604051808303815f875af1158015611fa2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611fc69190613b7f565b5060405163a9059cbb60e01b81526001600160a01b0384169063a9059cbb90611ffa9084906445d964b80090600401613b66565b6020604051808303815f875af1158015612016573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061203a9190613b7f565b506025546040516335313c2160e11b81526001600160a01b03918216600482015290821690636a627842906024016020604051808303815f875af1158015612084573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120a89190613b4f565b505f5160206176f15f395f51905f526001600160a01b03166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b1580156120ee575f5ffd5b505af1158015612100573d5f5f3e3d5ffd5b505050505f5f836001600160a01b0316630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa158015612142573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121669190613c9e565b50915091505f5f846001600160a01b0316630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa1580156121a9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121cd9190613c9e565b509150915061221e5f6001600160a01b0316876001600160a01b0316141560405180604001604052806015815260200174165154c81c185a5c881cda1bdd5b1908195e1a5cdd605a1b815250613246565b6040805180820190915260148152731393c81c185a5c881cda1bdd5b1908195e1a5cdd60621b602082015261225f906001600160a01b038716151590613246565b5050505050505050505050565b6060601a805480602002602001604051908101604052809291908181526020015f905b82821015611625578382905f5260205f200180546122ac90613c2f565b80601f01602080910402602001604051908101604052809291908181526020018280546122d890613c2f565b80156123235780601f106122fa57610100808354040283529160200191612323565b820191905f5260205f20905b81548152906001019060200180831161230657829003601f168201915b50505050508152602001906001019061228f565b6060601d805480602002602001604051908101604052809291908181526020015f905b82821015611625575f8481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561240057602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116123c25790505b5050505050815250508152602001906001019061235a565b6060601c805480602002602001604051908101604052809291908181526020015f905b82821015611625575f8481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156124e157602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116124a35790505b5050505050815250508152602001906001019061243b565b60606019805480602002602001604051908101604052809291908181526020015f905b82821015611625578382905f5260205f2001805461253990613c2f565b80601f016020809104026020016040519081016040528092919081815260200182805461256590613c2f565b80156125b05780601f10612587576101008083540402835291602001916125b0565b820191905f5260205f20905b81548152906001019060200180831161259357829003601f168201915b50505050508152602001906001019061251c565b6008545f9060ff16156125db575060085460ff1690565b604051630667f9d760e41b81525f905f5160206176f15f395f51905f529063667f9d7090612638907f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d906519985a5b195960d21b90600401613b66565b602060405180830381865afa158015612653573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126779190613b4f565b1415905090565b606060158054806020026020016040519081016040528092919081815260200182805480156114e857602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116114ca575050505050905090565b6025546040516303223eab60e11b81526001600160a01b0390911660048201525f5160206176f15f395f51905f52906306447d56906024015f604051808303815f87803b15801561272b575f5ffd5b505af115801561273d573d5f5f3e3d5ffd5b5050601f54602154604051639ba730a960e01b81526101009092046001600160a01b039081169450639ba730a9935061278892911690680ad78ebc5ac6200000908890600401613c67565b5f604051808303815f87803b15801561279f575f5ffd5b505af11580156127b1573d5f5f3e3d5ffd5b5050601f54602254604051639ba730a960e01b81526101009092046001600160a01b039081169450639ba730a993506127f89291169064e8d4a51000908890600401613c67565b5f604051808303815f87803b15801561280f575f5ffd5b505af1158015612821573d5f5f3e3d5ffd5b505050505f5160206176f15f395f51905f526001600160a01b03166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b15801561286a575f5ffd5b505af115801561287c573d5f5f3e3d5ffd5b50505050505050565b6040516364e329cb60e11b81526001600160a01b038084166004830152821660248201525f908190735c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f9063c9c65396906044016020604051808303815f875af11580156128e8573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061290c9190613c16565b90506001600160a01b0381166129605760405162461bcd60e51b815260206004820152601460248201527314185a5c8818dc99585d1a5bdb8819985a5b195960621b60448201526064015b60405180910390fd5b6025546040516303223eab60e11b81526001600160a01b0390911660048201525f5160206176f15f395f51905f52906306447d56906024015f604051808303815f87803b1580156129af575f5ffd5b505af11580156129c1573d5f5f3e3d5ffd5b505060405163a9059cbb60e01b81526001600160a01b038716925063a9059cbb91506129fc90849068056bc75e2d6310000090600401613b66565b6020604051808303815f875af1158015612a18573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612a3c9190613b7f565b5060405163a9059cbb60e01b81526001600160a01b0384169063a9059cbb90612a7090849064ba43b7400090600401613b66565b6020604051808303815f875af1158015612a8c573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ab09190613b7f565b506025546040516335313c2160e11b81526001600160a01b03918216600482015290821690636a627842906024016020604051808303815f875af1158015612afa573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b1e9190613b4f565b505f5160206176f15f395f51905f526001600160a01b03166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015612b64575f5ffd5b505af1158015612b76573d5f5f3e3d5ffd5b509293505050505b92915050565b60235460405163ca669fa760e01b81526001600160a01b0390911660048201525f5160206176f15f395f51905f529063ca669fa7906024015f604051808303815f87803b158015612bd3575f5ffd5b505af1158015612be5573d5f5f3e3d5ffd5b5050601f546021548451604051639ba730a960e01b81526101009093046001600160a01b039081169550639ba730a99450612c3093921691678ac7230489e800009190600401613c67565b5f604051808303815f87803b158015612c47575f5ffd5b505af1158015612c59573d5f5f3e3d5ffd5b5050505060208101516023546040516370a0823160e01b81526001600160a01b0391821660048201529116906370a0823190602401602060405180830381865afa158015612ca9573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ccd9190613b4f565b60c082015260608101516023546040516370a0823160e01b81526001600160a01b0391821660048201529116906370a0823190602401602060405180830381865afa158015612d1e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d429190613b4f565b60e082015260a081015160208201516060830151612d619291906132a8565b60208101516023546040516370a0823160e01b81526001600160a01b039182166004820152612df49291909116906370a0823190602401602060405180830381865afa158015612db3573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612dd79190613b4f565b670de0b6b3a76400008360c00151612def9190613cfe565b61361c565b60608101516023546040516370a0823160e01b81526001600160a01b039182166004820152610dd99291909116906370a0823190602401602060405180830381865afa158015612e46573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612e6a9190613b4f565b8260e00151613654565b6020546040516317ee366f60e11b815260048101849052600160248201526001600160a01b0390911690632fdc6cde906044015f604051808303815f87803b158015612ebe575f5ffd5b505af1158015612ed0573d5f5f3e3d5ffd5b5050601f54604051635c23bdf560e01b8152600481018690526101009091046001600160a01b03169250635c23bdf591506024015f604051808303815f87803b158015612f1b575f5ffd5b505af1158015612f2d573d5f5f3e3d5ffd5b50506022546023546040516370a0823160e01b81526001600160a01b0391821660048201525f9450911691506370a0823190602401602060405180830381865afa158015612f7d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612fa19190613b4f565b6023546040516370a0823160e01b81526001600160a01b0391821660048201529192505f91908416906370a0823190602401602060405180830381865afa158015612fee573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130129190613b4f565b6023546040516303223eab60e11b81526001600160a01b0390911660048201529091505f5160206176f15f395f51905f52906306447d56906024015f604051808303815f87803b158015613064575f5ffd5b505af1158015613076573d5f5f3e3d5ffd5b5050601f5460405163095ea7b360e01b81526001600160a01b03808816945063095ea7b393506130b192610100900416908590600401613b66565b6020604051808303815f875af11580156130cd573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906130f19190613b7f565b50601f546040516301e9a69560e41b81526101009091046001600160a01b031690631e9a6950906131289086908590600401613b66565b6020604051808303815f875af1158015613144573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906131689190613b4f565b505f5160206176f15f395f51905f526001600160a01b03166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b1580156131ae575f5ffd5b505af11580156131c0573d5f5f3e3d5ffd5b50506022546023546040516370a0823160e01b81526001600160a01b0391821660048201526132409450911691506370a0823190602401602060405180830381865afa158015613212573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906132369190613b4f565b612def8385613d11565b50505050565b60405163a34edc0360e01b81525f5160206176f15f395f51905f529063a34edc03906132789085908590600401613d24565b5f6040518083038186803b15801561328e575f5ffd5b505afa1580156132a0573d5f5f3e3d5ffd5b505050505050565b5f5f846001600160a01b0316630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa1580156132e6573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061330a9190613c9e565b50915091505f856001600160a01b0316630dfe16816040518163ffffffff1660e01b8152600401602060405180830381865afa15801561334c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906133709190613c16565b90505f5f866001600160a01b0316836001600160a01b0316146133a657836001600160701b0316856001600160701b03166133bb565b846001600160701b0316846001600160701b03165b9092509050670de0b6b3a76400005f6133d582858561368c565b6023546040516303223eab60e11b81526001600160a01b0390911660048201529091505f5160206176f15f395f51905f52906306447d56906024015f604051808303815f87803b158015613427575f5ffd5b505af1158015613439573d5f5f3e3d5ffd5b505060405163a9059cbb60e01b81526001600160a01b038c16925063a9059cbb915061346b908d908690600401613b66565b6020604051808303815f875af1158015613487573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906134ab9190613b7f565b50886001600160a01b0316856001600160a01b03160361353f5760235460405163022c0d9f60e01b81525f60048201819052602482018490526001600160a01b039283166044830152608060648301526084820152908b169063022c0d9f9060a4015f604051808303815f87803b158015613524575f5ffd5b505af1158015613536573d5f5f3e3d5ffd5b505050506135b5565b60235460405163022c0d9f60e01b8152600481018390525f602482018190526001600160a01b039283166044830152608060648301526084820152908b169063022c0d9f9060a4015f604051808303815f87803b15801561359e575f5ffd5b505af11580156135b0573d5f5f3e3d5ffd5b505050505b5f5160206176f15f395f51905f526001600160a01b03166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b1580156135fa575f5ffd5b505af115801561360c573d5f5f3e3d5ffd5b5050505050505050505050505050565b60405163260a5b1560e21b815260048101839052602481018290525f5160206176f15f395f51905f52906398296c5490604401613278565b604051636d83fe6960e11b815260048101839052602481018290525f5160206176f15f395f51905f529063db07fcd290604401613278565b5f5f84116136dc5760405162461bcd60e51b815260206004820152601960248201527f494e53554646494349454e545f494e5055545f414d4f554e54000000000000006044820152606401612957565b5f831180156136ea57505f82115b61372f5760405162461bcd60e51b8152602060048201526016602482015275494e53554646494349454e545f4c495155494449545960501b6044820152606401612957565b5f61373c856103e5613d46565b90505f6137498483613d46565b90505f82613759876103e8613d46565b6137639190613d11565b905061376f8183613d5d565b979650505050505050565b61259680613d7d83390190565b61050a8061631383390190565b610ed48061681d83390190565b602080825282518282018190525f918401906040840190835b818110156137e15783516001600160a01b03168352602093840193909201916001016137ba565b509095945050505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015612b7657603f19878603018452815180516001600160a01b03168652602090810151604082880181905281519088018190529101906060600582901b8801810191908801905f5b818110156138bd57605f198a85030183526138a78486516137ec565b602095860195909450929092019160010161388b565b509197505050602094850194929092019150600101613840565b5f8151808452602084019350602083015f5b828110156139115781516001600160e01b0319168652602095860195909101906001016138e9565b5093949350505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015612b7657603f19878603018452815180516040875261396760408801826137ec565b905060208201519150868103602088015261398281836138d7565b965050506020938401939190910190600101613941565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015612b7657603f198786030184526139db8583516137ec565b945060209384019391909101906001016139bf565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015612b7657868503603f19018452815180516001600160a01b03168652602090810151604091870182905290613a51908701826138d7565b9550506020938401939190910190600101613a16565b634e487b7160e01b5f52604160045260245ffd5b5f60208284031215613a8b575f5ffd5b815167ffffffffffffffff811115613aa1575f5ffd5b8201601f81018413613ab1575f5ffd5b805167ffffffffffffffff811115613acb57613acb613a67565b604051601f8201601f19908116603f0116810167ffffffffffffffff81118282101715613afa57613afa613a67565b604052818152828201602001861015613b11575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b604081525f613b4060408301856137ec565b90508260208301529392505050565b5f60208284031215613b5f575f5ffd5b5051919050565b6001600160a01b03929092168252602082015260400190565b5f60208284031215613b8f575f5ffd5b81518015158114613b9e575f5ffd5b9392505050565b92835263ffffffff9190911660208301526001600160a01b0316604082015260600190565b80516001600160a01b0381168114613be0575f5ffd5b919050565b5f5f60408385031215613bf6575f5ffd5b613bff83613bca565b9150613c0d60208401613bca565b90509250929050565b5f60208284031215613c26575f5ffd5b613b9e82613bca565b600181811c90821680613c4357607f821691505b602082108103613c6157634e487b7160e01b5f52602260045260245ffd5b50919050565b6001600160a01b039390931683526020830191909152604082015260600190565b80516001600160701b0381168114613be0575f5ffd5b5f5f5f60608486031215613cb0575f5ffd5b613cb984613c88565b9250613cc760208501613c88565b9150604084015163ffffffff81168114613cdf575f5ffd5b809150509250925092565b634e487b7160e01b5f52601160045260245ffd5b81810381811115612b7e57612b7e613cea565b80820180821115612b7e57612b7e613cea565b8215158152604060208201525f613d3e60408301846137ec565b949350505050565b8082028115828204841417612b7e57612b7e613cea565b5f82613d7757634e487b7160e01b5f52601260045260245ffd5b50049056fe6080604052348015600e575f5ffd5b5061257a8061001c5f395ff3fe608060405234801561000f575f5ffd5b506004361061009b575f3560e01c80639ba730a9116100635780639ba730a91461017a578063a3def9231461018d578063c2a33c1d146101c0578063d4b06fb8146101d3578063e973955014610202575f5ffd5b80631e9a69501461009f57806341055a76146100c55780635c23bdf5146100da5780637183d24a146100ed5780637564912b14610110575b5f5ffd5b6100b26100ad3660046111df565b610215565b6040519081526020015b60405180910390f35b6100d86100d3366004611209565b61062a565b005b6100d86100e836600461123b565b6108e1565b6101006100fb366004611252565b610a46565b60405190151581526020016100bc565b61015061011e36600461123b565b5f602081905290815260409020805460019091015463ffffffff82169164010000000090046001600160a01b03169083565b6040805163ffffffff90941684526001600160a01b039092166020840152908201526060016100bc565b6100d8610188366004611209565b610bf1565b6101a061019b3660046111df565b610dcd565b604080516001600160a01b039384168152929091166020830152016100bc565b6100d86101ce366004611274565b610f0f565b6101f56101e136600461123b565b60016020525f908152604090205460ff1681565b6040516100bc91906112cf565b6101a06102103660046111df565b61104c565b5f5f604051602001610230906259455360e81b815260030190565b60405160208183030381529060405280519060200120846001600160a01b03166395d89b416040518163ffffffff1660e01b81526004015f60405180830381865afa158015610281573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526102a89190810190611309565b6040516020016102b891906113d3565b604051602081830303815290604052805190602001201490505f846001600160a01b031663c0474d0b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561030e573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061033291906113de565b90505f856001600160a01b0316634800d97f6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610371573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061039591906113f5565b90506103a2838284611070565b6001600160a01b0316866001600160a01b0316146103d3576040516340fe50fd60e01b815260040160405180910390fd5b60025f8381526001602052604090205460ff1660048111156103f7576103f76112bb565b1480156104015750825b1561040e5784935061049e565b60035f8381526001602052604090205460ff166004811115610432576104326112bb565b14801561043d575082155b1561044a5784935061049e565b60045f8381526001602052604090205460ff16600481111561046e5761046e6112bb565b036104855761047e600286611410565b935061049e565b604051637256f64560e11b815260040160405180910390fd5b6040516323b872dd60e01b81526001600160a01b038716906323b872dd906104ce90339030908a9060040161142f565b6020604051808303815f875af11580156104ea573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061050e9190611453565b50604051632770a7eb60e21b8152306004820152602481018690526001600160a01b03871690639dc29fac906044015f604051808303815f87803b158015610554575f5ffd5b505af1158015610566573d5f5f3e3d5ffd5b505060405163a9059cbb60e01b8152336004820152602481018790526001600160a01b038416925063a9059cbb91506044016020604051808303815f875af11580156105b4573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105d89190611453565b50604080518681526020810186905283916001600160a01b0389169133917fae1d804a86b6ba2a7725d9824d4ccb4e7f9a55e7a1bf4379c26692ce5ad00665910160405180910390a450505092915050565b60015f8281526001602052604090205460ff16600481111561064e5761064e6112bb565b1461066c5760405163239dd4ad60e11b815260040160405180910390fd5b5f5f610678858461104c565b6040516323b872dd60e01b815291935091506001600160a01b038316906323b872dd906106ad9033903090899060040161142f565b6020604051808303815f875af11580156106c9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106ed9190611453565b506040516323b872dd60e01b81526001600160a01b038216906323b872dd9061071e9033903090899060040161142f565b6020604051808303815f875af115801561073a573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061075e9190611453565b50604051632770a7eb60e21b8152306004820152602481018590526001600160a01b03831690639dc29fac906044015f604051808303815f87803b1580156107a4575f5ffd5b505af11580156107b6573d5f5f3e3d5ffd5b5050604051632770a7eb60e21b8152306004820152602481018790526001600160a01b0384169250639dc29fac91506044015f604051808303815f87803b1580156107ff575f5ffd5b505af1158015610811573d5f5f3e3d5ffd5b505060405163a9059cbb60e01b8152336004820152602481018790526001600160a01b038816925063a9059cbb91506044016020604051808303815f875af115801561085f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108839190611453565b50604080518581526001600160a01b03848116602083015283811692820192909252849187169033907f9a853bfbdd6034e0e7553bb8ec7789b2603dcccb5759cf390a7a601c0b6da003906060015b60405180910390a45050505050565b60015f8281526001602052604090205460ff166004811115610905576109056112bb565b146109235760405163239dd4ad60e11b815260040160405180910390fd5b5f8181526020819052604081205463ffffffff164210610945575060046109d8565b5f828152602081905260408082205490516301f46b2960e11b8152600481018590526401000000009091046001600160a01b0316906303e8d652906024016020604051808303815f875af115801561099f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109c39190611453565b9050806109d15760036109d4565b60025b9150505b5f8281526001602081905260409091208054839260ff1990911690836004811115610a0557610a056112bb565b0217905550817ff34984473148051bc1bdf1be6ecc462d7b228d591058a8a27977b84770b738b982604051610a3a91906112cf565b60405180910390a25050565b5f5f826001600160a01b031663c0474d0b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610a84573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610aa891906113de565b90505f836001600160a01b0316634800d97f6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610ae7573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b0b91906113f5565b90505f604051602001610b27906259455360e81b815260030190565b60405160208183030381529060405280519060200120856001600160a01b03166395d89b416040518163ffffffff1660e01b81526004015f60405180830381865afa158015610b78573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610b9f9190810190611309565b604051602001610baf91906113d3565b60405160208183030381529060405280519060200120149050610bd3818385611070565b6001600160a01b0316856001600160a01b0316149350505050919050565b60015f8281526001602052604090205460ff166004811115610c1557610c156112bb565b14610c335760405163239dd4ad60e11b815260040160405180910390fd5b6040516323b872dd60e01b81526001600160a01b038416906323b872dd90610c639033903090879060040161142f565b6020604051808303815f875af1158015610c7f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ca39190611453565b505f5f610cb0858461104c565b6040516340c10f1960e01b81523360048201526024810187905291935091506001600160a01b038316906340c10f19906044015f604051808303815f87803b158015610cfa575f5ffd5b505af1158015610d0c573d5f5f3e3d5ffd5b50506040516340c10f1960e01b8152336004820152602481018790526001600160a01b03841692506340c10f1991506044015f604051808303815f87803b158015610d55575f5ffd5b505af1158015610d67573d5f5f3e3d5ffd5b5050505082856001600160a01b0316336001600160a01b03167f8fb8a329bc68853568e0dd27c6aee2e361a1f07892cf65ab49cc2f30cd21c3c58786866040516108d2939291909283526001600160a01b03918216602084015216604082015260600190565b5f5f828484604051610dde906111bb565b608080825260039082018190526259657360e81b60a083015260c0602083018190528201526259455360e81b60e08201526001600160a01b0390921660408301526060820152610100018190604051809103905ff5905080158015610e45573d5f5f3e3d5ffd5b509150828484604051610e57906111bb565b60808082526002908201819052614e6f60f01b60a083015260c060208301819052820152614e4f60f01b60e08201526001600160a01b0390921660408301526060820152610100018190604051809103905ff5905080158015610ebc573d5f5f3e3d5ffd5b50604080516001600160a01b0385811682528381166020830152825193945087169286927f9e431caef5753e3e939653fc056c910725980029b3653db5a36bebca71d6bc2c928290030190a39250929050565b604080516020810185905263ffffffff8416918101919091526001600160a01b03821660608201525f9060800160408051601f19818403018152919052805160209091012090505f5f8281526001602052604090205460ff166004811115610f7957610f796112bb565b14610f97576040516337a5bd9b60e11b815260040160405180910390fd5b5f818152600160208181526040808420805460ff191684179055805160608101825263ffffffff8881168083526001600160a01b038981168487018181528587018e81528b8b528a89529987902095518654915195166001600160c01b0319909116176401000000009490921693909302178355955191909401558051938452519192879285927f094eba69d7ca9dfafa34c896067fc8e19395a610e47deb8b25b21b87bec32a34928290030190a450505050565b5f5f61105a60018585611070565b91506110675f8585611070565b90509250929050565b5f5f60405180602001611082906111bb565b601f1982820381018352601f90910116604052856110ba57604051806040016040528060028152602001614e6f60f01b8152506110d7565b6040518060400160405280600381526020016259657360e81b8152505b866110fc57604051806040016040528060028152602001614e4f60f01b815250611119565b6040518060400160405280600381526020016259455360e81b8152505b868660405160200161112e94939291906114a0565b60408051601f198184030181529082905261114c92916020016114e1565b60408051808303601f1901815282825280516020918201206001600160f81b0319828501523060601b6bffffffffffffffffffffffff19166021850152603584019690965260558084019690965281518084039096018652607590920190528351930192909220949350505050565b611047806114fe83390190565b6001600160a01b03811681146111dc575f5ffd5b50565b5f5f604083850312156111f0575f5ffd5b82356111fb816111c8565b946020939093013593505050565b5f5f5f6060848603121561121b575f5ffd5b8335611226816111c8565b95602085013595506040909401359392505050565b5f6020828403121561124b575f5ffd5b5035919050565b5f60208284031215611262575f5ffd5b813561126d816111c8565b9392505050565b5f5f5f60608486031215611286575f5ffd5b83359250602084013563ffffffff811681146112a0575f5ffd5b915060408401356112b0816111c8565b809150509250925092565b634e487b7160e01b5f52602160045260245ffd5b60208101600583106112ef57634e487b7160e01b5f52602160045260245ffd5b91905290565b634e487b7160e01b5f52604160045260245ffd5b5f60208284031215611319575f5ffd5b815167ffffffffffffffff81111561132f575f5ffd5b8201601f8101841361133f575f5ffd5b805167ffffffffffffffff811115611359576113596112f5565b604051601f8201601f19908116603f0116810167ffffffffffffffff81118282101715611388576113886112f5565b60405281815282820160200186101561139f575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b5f81518060208401855e5f93019283525090919050565b5f61126d82846113bc565b5f602082840312156113ee575f5ffd5b5051919050565b5f60208284031215611405575f5ffd5b815161126d816111c8565b5f8261142a57634e487b7160e01b5f52601260045260245ffd5b500490565b6001600160a01b039384168152919092166020820152604081019190915260600190565b5f60208284031215611463575f5ffd5b8151801515811461126d575f5ffd5b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b608081525f6114b26080830187611472565b82810360208401526114c48187611472565b6001600160a01b0395909516604084015250506060015292915050565b5f6114f56114ef83866113bc565b846113bc565b94935050505056fe60c060405234801561000f575f5ffd5b5060405161104738038061104783398101604081905261002e91610143565b5f6100398582610252565b5060016100468482610252565b506001600160a01b03821660805260a08190526100623361006b565b5050505061030c565b6001600160a01b0316638b78c6d819819055805f7f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08180a350565b634e487b7160e01b5f52604160045260245ffd5b5f82601f8301126100c9575f5ffd5b81516001600160401b038111156100e2576100e26100a6565b604051601f8201601f19908116603f011681016001600160401b0381118282101715610110576101106100a6565b604052818152838201602001851015610127575f5ffd5b8160208501602083015e5f918101602001919091529392505050565b5f5f5f5f60808587031215610156575f5ffd5b84516001600160401b0381111561016b575f5ffd5b610177878288016100ba565b602087015190955090506001600160401b03811115610194575f5ffd5b6101a0878288016100ba565b604087015190945090506001600160a01b03811681146101be575f5ffd5b6060959095015193969295505050565b600181811c908216806101e257607f821691505b60208210810361020057634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561024d57805f5260205f20601f840160051c8101602085101561022b5750805b601f840160051c820191505b8181101561024a575f8155600101610237565b50505b505050565b81516001600160401b0381111561026b5761026b6100a6565b61027f8161027984546101ce565b84610206565b6020601f8211600181146102b1575f831561029a5750848201515b5f19600385901b1c1916600184901b17845561024a565b5f84815260208120601f198516915b828110156102e057878501518255602094850194600190920191016102c0565b50848210156102fd57868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b60805160a051610d1a61032d5f395f61036e01525f6102470152610d1a5ff3fe60806040526004361061013c575f3560e01c8063715018a6116100b3578063c0474d0b1161006d578063c0474d0b1461035d578063d505accf14610390578063dd62ed3e146103af578063f04e283e146103ce578063f2fde38b146103e1578063fee81cf4146103f4575f5ffd5b8063715018a6146102ba5780637ecebe00146102c25780638da5cb5b146102f357806395d89b411461030b5780639dc29fac1461031f578063a9059cbb1461033e575f5ffd5b8063313ce56711610104578063313ce567146101e85780633644e5151461020357806340c10f19146102175780634800d97f1461023657806354d1f13d1461028157806370a0823114610289575f5ffd5b806306fdde0314610140578063095ea7b31461016a57806318160ddd1461019957806323b872dd146101bf57806325692962146101de575b5f5ffd5b34801561014b575f5ffd5b50610154610425565b6040516101619190610b1c565b60405180910390f35b348015610175575f5ffd5b50610189610184366004610b6c565b6104b4565b6040519015158152602001610161565b3480156101a4575f5ffd5b506805345cdf77eb68f44c545b604051908152602001610161565b3480156101ca575f5ffd5b506101896101d9366004610b94565b610534565b6101e66105f0565b005b3480156101f3575f5ffd5b5060405160128152602001610161565b34801561020e575f5ffd5b506101b161063d565b348015610222575f5ffd5b506101e6610231366004610b6c565b6106b9565b348015610241575f5ffd5b506102697f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610161565b6101e66106cf565b348015610294575f5ffd5b506101b16102a3366004610bce565b6387a211a2600c9081525f91909152602090205490565b6101e6610708565b3480156102cd575f5ffd5b506101b16102dc366004610bce565b6338377508600c9081525f91909152602090205490565b3480156102fe575f5ffd5b50638b78c6d81954610269565b348015610316575f5ffd5b5061015461071b565b34801561032a575f5ffd5b506101e6610339366004610b6c565b61072a565b348015610349575f5ffd5b50610189610358366004610b6c565b61073c565b348015610368575f5ffd5b506101b17f000000000000000000000000000000000000000000000000000000000000000081565b34801561039b575f5ffd5b506101e66103aa366004610bee565b6107a0565b3480156103ba575f5ffd5b506101b16103c9366004610c5b565b610954565b6101e66103dc366004610bce565b610998565b6101e66103ef366004610bce565b6109d5565b3480156103ff575f5ffd5b506101b161040e366004610bce565b63389a75e1600c9081525f91909152602090205490565b60605f805461043390610c8c565b80601f016020809104026020016040519081016040528092919081815260200182805461045f90610c8c565b80156104aa5780601f10610481576101008083540402835291602001916104aa565b820191905f5260205f20905b81548152906001019060200180831161048d57829003601f168201915b5050505050905090565b5f6001600160a01b0383166e22d473030f116ddee9f6b43ac78ba318821915176104e557633f68539a5f526004601cfd5b82602052637f5e9f20600c52335f52816034600c2055815f52602c5160601c337f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560205fa35060015b92915050565b5f8360601b6e22d473030f116ddee9f6b43ac78ba333146105895733602052637f5e9f208117600c526034600c2080548019156105865780851115610580576313be252b5f526004601cfd5b84810382555b50505b6387a211a28117600c526020600c208054808511156105af5763f4d678b85f526004601cfd5b84810382555050835f526020600c208381540181555082602052600c5160601c8160601c5f516020610cc55f395f51905f52602080a3505060019392505050565b5f6202a30067ffffffffffffffff164201905063389a75e1600c52335f52806020600c2055337fdbf36a107da19e49527a7176a1babf963b4b0ff8cde35ee35d6cd8f1f9ac7e1d5f5fa250565b5f80610647610425565b805190602001209050604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f815260208101929092527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc69082015246606082015230608082015260a09020919050565b6106c16109fb565b6106cb8282610a15565b5050565b63389a75e1600c52335f525f6020600c2055337ffa7b8eab7da67f412cc9575ed43464468f9bfbae89d1675917346ca6d8fe3c925f5fa2565b6107106109fb565b6107195f610a7e565b565b60606001805461043390610c8c565b6107326109fb565b6106cb8282610abb565b5f6387a211a2600c52335f526020600c208054808411156107645763f4d678b85f526004601cfd5b83810382555050825f526020600c208281540181555081602052600c5160601c335f516020610cc55f395f51905f52602080a350600192915050565b6001600160a01b0386166e22d473030f116ddee9f6b43ac78ba318851915176107d057633f68539a5f526004601cfd5b5f6107d9610425565b8051906020012090507fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc64286101561081857631a15a3cc5f526004601cfd5b6040518960601b60601c99508860601b60601c985065383775081901600e52895f526020600c2080547f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f835284602084015283604084015246606084015230608084015260a08320602e527f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c983528b60208401528a60408401528960608401528060808401528860a084015260c08320604e526042602c205f528760ff16602052866040528560605260208060805f60015afa8c3d51146109005763ddafbaef5f526004601cfd5b0190556303faf4f960a51b89176040526034602c20889055888a7f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925602060608501a360405250505f60605250505050505050565b5f6e22d473030f116ddee9f6b43ac78ba2196001600160a01b0383160161097d57505f1961052e565b50602052637f5e9f20600c9081525f91909152603490205490565b6109a06109fb565b63389a75e1600c52805f526020600c2080544211156109c657636f5e88185f526004601cfd5b5f90556109d281610a7e565b50565b6109dd6109fb565b8060601b6109f257637448fbae5f526004601cfd5b6109d281610a7e565b638b78c6d819543314610719576382b429005f526004601cfd5b6805345cdf77eb68f44c5481810181811015610a385763e5cfe9575f526004601cfd5b806805345cdf77eb68f44c5550506387a211a2600c52815f526020600c208181540181555080602052600c5160601c5f5f516020610cc55f395f51905f52602080a35050565b638b78c6d81980546001600160a01b039092169182907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a355565b6387a211a2600c52815f526020600c20805480831115610ae25763f4d678b85f526004601cfd5b82900390556805345cdf77eb68f44c805482900390555f8181526001600160a01b0383165f516020610cc55f395f51905f52602083a35050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b80356001600160a01b0381168114610b67575f5ffd5b919050565b5f5f60408385031215610b7d575f5ffd5b610b8683610b51565b946020939093013593505050565b5f5f5f60608486031215610ba6575f5ffd5b610baf84610b51565b9250610bbd60208501610b51565b929592945050506040919091013590565b5f60208284031215610bde575f5ffd5b610be782610b51565b9392505050565b5f5f5f5f5f5f5f60e0888a031215610c04575f5ffd5b610c0d88610b51565b9650610c1b60208901610b51565b95506040880135945060608801359350608088013560ff81168114610c3e575f5ffd5b9699959850939692959460a0840135945060c09093013592915050565b5f5f60408385031215610c6c575f5ffd5b610c7583610b51565b9150610c8360208401610b51565b90509250929050565b600181811c90821680610ca057607f821691505b602082108103610cbe57634e487b7160e01b5f52602260045260245ffd5b5091905056feddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa2646970667358221220f2558ad4460ca00ebebab8d02372dd6b027f19de7bd9d732ad74c5cf24635f4564736f6c634300081e0033a2646970667358221220919de7b4590df56bc11aeb304fa29d7b44145f3508341dc0a20c84eeab358d1564736f6c634300081e00336080604052348015600e575f5ffd5b50601633601a565b6055565b6001600160a01b0316638b78c6d819819055805f7f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08180a350565b6104a8806100625f395ff3fe60806040526004361061009a575f3560e01c80638da5cb5b116100625780638da5cb5b1461010b578063d4b06fb814610136578063f04e283e14610164578063f2fde38b14610177578063f636e0f61461018a578063fee81cf4146101b8575f5ffd5b806303e8d6521461009e57806325692962146100d25780632fdc6cde146100dc57806354d1f13d146100fb578063715018a614610103575b5f5ffd5b3480156100a9575f5ffd5b506100bd6100b83660046103fc565b6101f7565b60405190151581526020015b60405180910390f35b6100da61023a565b005b3480156100e7575f5ffd5b506100da6100f6366004610413565b610287565b6100da6102f6565b6100da61032f565b348015610116575f5ffd5b50638b78c6d819546040516001600160a01b0390911681526020016100c9565b348015610141575f5ffd5b506100bd6101503660046103fc565b5f6020819052908152604090205460ff1681565b6100da610172366004610445565b610342565b6100da610185366004610445565b61037f565b348015610195575f5ffd5b506100bd6101a43660046103fc565b60016020525f908152604090205460ff1681565b3480156101c3575f5ffd5b506101e96101d2366004610445565b63389a75e1600c9081525f91909152602090205490565b6040519081526020016100c9565b5f8181526001602052604081205460ff1661022557604051634aec5ac760e01b815260040160405180910390fd5b505f9081526020819052604090205460ff1690565b5f6202a30067ffffffffffffffff164201905063389a75e1600c52335f52806020600c2055337fdbf36a107da19e49527a7176a1babf963b4b0ff8cde35ee35d6cd8f1f9ac7e1d5f5fa250565b61028f6103a5565b5f82815260208181526040808320805485151560ff19918216811790925560018085529483902080549091169094179093555191825283917f57c39a2480accb4077522f2253a3915afdc67e1ca5883a57957ed61f2f04d023910160405180910390a25050565b63389a75e1600c52335f525f6020600c2055337ffa7b8eab7da67f412cc9575ed43464468f9bfbae89d1675917346ca6d8fe3c925f5fa2565b6103376103a5565b6103405f6103bf565b565b61034a6103a5565b63389a75e1600c52805f526020600c20805442111561037057636f5e88185f526004601cfd5b5f905561037c816103bf565b50565b6103876103a5565b8060601b61039c57637448fbae5f526004601cfd5b61037c816103bf565b638b78c6d819543314610340576382b429005f526004601cfd5b638b78c6d81980546001600160a01b039092169182907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a355565b5f6020828403121561040c575f5ffd5b5035919050565b5f5f60408385031215610424575f5ffd5b823591506020830135801515811461043a575f5ffd5b809150509250929050565b5f60208284031215610455575f5ffd5b81356001600160a01b038116811461046b575f5ffd5b939250505056fea2646970667358221220758a44ba0589aa1e9c5c5158ff502f3cbb50162cde6c15d8d597e83a6706cb0264736f6c634300081e003360a060405234801561000f575f5ffd5b50604051610ed4380380610ed483398101604081905261002e91610109565b5f610039848261020a565b506001610046838261020a565b506002805460ff191660ff929092169190911790555080516020909101206080526102c4565b634e487b7160e01b5f52604160045260245ffd5b5f82601f83011261008f575f5ffd5b81516001600160401b038111156100a8576100a861006c565b604051601f8201601f19908116603f011681016001600160401b03811182821017156100d6576100d661006c565b6040528181528382016020018510156100ed575f5ffd5b8160208501602083015e5f918101602001919091529392505050565b5f5f5f6060848603121561011b575f5ffd5b83516001600160401b03811115610130575f5ffd5b61013c86828701610080565b602086015190945090506001600160401b03811115610159575f5ffd5b61016586828701610080565b925050604084015160ff8116811461017b575f5ffd5b809150509250925092565b600181811c9082168061019a57607f821691505b6020821081036101b857634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561020557805f5260205f20601f840160051c810160208510156101e35750805b601f840160051c820191505b81811015610202575f81556001016101ef565b50505b505050565b81516001600160401b038111156102235761022361006c565b610237816102318454610186565b846101be565b6020601f821160018114610269575f83156102525750848201515b5f19600385901b1c1916600184901b178455610202565b5f84815260208120601f198516915b828110156102985787850151825560209485019460019092019101610278565b50848210156102b557868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b608051610bf16102e35f395f818161039901526104db0152610bf15ff3fe608060405234801561000f575f5ffd5b50600436106100fb575f3560e01c80637ecebe0011610093578063d30ed3b311610063578063d30ed3b314610217578063d505accf1461022a578063dd62ed3e1461023d578063f83d179114610250575f5ffd5b80637ecebe00146101c457806395d89b41146101e95780639dc29fac146101f1578063a9059cbb14610204575f5ffd5b8063313ce567116100ce578063313ce5671461016d5780633644e5151461018257806340c10f191461018a57806370a082311461019f575f5ffd5b806306fdde03146100ff578063095ea7b31461011d57806318160ddd1461014057806323b872dd1461015a575b5f5ffd5b610107610263565b60405161011491906109ff565b60405180910390f35b61013061012b366004610a4a565b6102f2565b6040519015158152602001610114565b6805345cdf77eb68f44c545b604051908152602001610114565b610130610168366004610a72565b610372565b60025460405160ff9091168152602001610114565b61014c610396565b61019d610198366004610a4a565b610438565b005b61014c6101ad366004610aac565b6387a211a2600c9081525f91909152602090205490565b61014c6101d2366004610aac565b6338377508600c9081525f91909152602090205490565b61010761044e565b61019d6101ff366004610a4a565b61045d565b610130610212366004610a4a565b61046f565b61019d610225366004610a72565b610489565b61019d610238366004610ac5565b6104a9565b61014c61024b366004610b32565b610683565b61019d61025e366004610a72565b6106c7565b60605f805461027190610b63565b80601f016020809104026020016040519081016040528092919081815260200182805461029d90610b63565b80156102e85780601f106102bf576101008083540402835291602001916102e8565b820191905f5260205f20905b8154815290600101906020018083116102cb57829003601f168201915b5050505050905090565b5f6001600160a01b0383166e22d473030f116ddee9f6b43ac78ba3188219151761032357633f68539a5f526004601cfd5b82602052637f5e9f20600c52335f52816034600c2055815f52602c5160601c337f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560205fa35060015b92915050565b5f61038e61037f856106e2565b610388856106e2565b84610703565b949350505050565b5f7f0000000000000000000000000000000000000000000000000000000000000000806103cf576103c5610263565b8051906020012090505b604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f815260208101929092527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc69082015246606082015230608082015260a09020919050565b61044a610444836106e2565b826107bf565b5050565b60606001805461027190610b63565b61044a610469836106e2565b82610828565b5f61048261047c846106e2565b83610889565b9392505050565b6104a4610495846106e2565b61049e846106e2565b836108ed565b505050565b6001600160a01b0386166e22d473030f116ddee9f6b43ac78ba318851915176104d957633f68539a5f526004601cfd5b7f00000000000000000000000000000000000000000000000000000000000000008061051157610507610263565b8051906020012090505b7fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc64286101561054757631a15a3cc5f526004601cfd5b6040518960601b60601c99508860601b60601c985065383775081901600e52895f526020600c2080547f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f835284602084015283604084015246606084015230608084015260a08320602e527f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c983528b60208401528a60408401528960608401528060808401528860a084015260c08320604e526042602c205f528760ff16602052866040528560605260208060805f60015afa8c3d511461062f5763ddafbaef5f526004601cfd5b0190556303faf4f960a51b89176040526034602c20889055888a7f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925602060608501a360405250505f60605250505050505050565b5f6e22d473030f116ddee9f6b43ac78ba2196001600160a01b038316016106ac57505f1961036c565b50602052637f5e9f20600c9081525f91909152603490205490565b6104a46106d3846106e2565b6106dc846106e2565b83610951565b5f6001600160a01b0382168060a06106f9826109b7565b901b189392505050565b5f8360601b6e22d473030f116ddee9f6b43ac78ba333146107585733602052637f5e9f208117600c526034600c208054801915610755578085111561074f576313be252b5f526004601cfd5b84810382555b50505b6387a211a28117600c526020600c2080548085111561077e5763f4d678b85f526004601cfd5b84810382555050835f526020600c208381540181555082602052600c5160601c8160601c5f516020610b9c5f395f51905f52602080a3505060019392505050565b6805345cdf77eb68f44c54818101818110156107e25763e5cfe9575f526004601cfd5b806805345cdf77eb68f44c5550506387a211a2600c52815f526020600c208181540181555080602052600c5160601c5f5f516020610b9c5f395f51905f52602080a35050565b6387a211a2600c52815f526020600c2080548083111561084f5763f4d678b85f526004601cfd5b82900390556805345cdf77eb68f44c805482900390555f8181526001600160a01b0383165f516020610b9c5f395f51905f52602083a35050565b5f6387a211a2600c52335f526020600c208054808411156108b15763f4d678b85f526004601cfd5b83810382555050825f526020600c208281540181555081602052600c5160601c335f516020610b9c5f395f51905f52602080a350600192915050565b6e22d473030f116ddee9f6b43ac78ba2196001600160a01b0383160161091257505050565b81602052637f5e9f20600c52825f526034600c20805480191561094a5780831115610944576313be252b5f526004601cfd5b82810382555b5050505050565b8260601b6387a211a28117600c526020600c2080548084111561097b5763f4d678b85f526004601cfd5b83810382555050825f526020600c208281540181555081602052600c5160601c8160601c5f516020610b9c5f395f51905f52602080a350505050565b604051365f8237368120602052601051821860105260885f2090508060105260bc19700100000000000000000000000000000051820960801c6007166109fa57505f5b919050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b80356001600160a01b03811681146109fa575f5ffd5b5f5f60408385031215610a5b575f5ffd5b610a6483610a34565b946020939093013593505050565b5f5f5f60608486031215610a84575f5ffd5b610a8d84610a34565b9250610a9b60208501610a34565b929592945050506040919091013590565b5f60208284031215610abc575f5ffd5b61048282610a34565b5f5f5f5f5f5f5f60e0888a031215610adb575f5ffd5b610ae488610a34565b9650610af260208901610a34565b95506040880135945060608801359350608088013560ff81168114610b15575f5ffd5b9699959850939692959460a0840135945060c09093013592915050565b5f5f60408385031215610b43575f5ffd5b610b4c83610a34565b9150610b5a60208401610a34565b90509250929050565b600181811c90821680610b7757607f821691505b602082108103610b9557634e487b7160e01b5f52602260045260245ffd5b5091905056feddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa26469706673582212200c45acc3c6a9d15890ebba8720cb2063e27c6daab33079386bb3dcaecfa64cfb64736f6c634300081e00330000000000000000000000007109709ecfa91a80626ff3989d68f67f5b1dd12d5945535f574554482f5945535f5553444320706169722073686f756c64206578697374229a5127dba549edf6c45917cdacdeebdcb0f57ad33d2d0aa6e8d12a21692902a264697066735822122040aff2e2915a35197d1fa209b4934271c2c256f46215fb8e2d03e283fa76e88d64736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x87W_5`\xE0\x1C\x80c}\xC0\xD1\xD0\x11a\0\xD9W\x80c\xB0FO\xDC\x11a\0\x93W\x80c\xC0\x9C\xECw\x11a\0nW\x80c\xC0\x9C\xECw\x14a\x03,W\x80c\xE2\x0C\x9Fq\x14a\x03?W\x80c\xFAv&\xD4\x14a\x03GW\x80c\xFBG\xE3\xA2\x14a\x03TW__\xFD[\x80c\xB0FO\xDC\x14a\x03\x04W\x80c\xB5P\x8A\xA9\x14a\x03\x0CW\x80c\xBAAO\xA6\x14a\x03\x14W__\xFD[\x80c}\xC0\xD1\xD0\x14a\x02tW\x80c\x85\"l\x81\x14a\x02\x87W\x80c\x8C\x80\x9F\xBF\x14a\x02\x9CW\x80c\x91j\x17\xC6\x14a\x02\xB4W\x80c\x99\xD8\xFA\xE3\x14a\x02\xC9W\x80c\xA4#KN\x14a\x02\xE4W__\xFD[\x80c>A;\xEE\x11a\x01DW\x80c?\xC8\xCE\xF3\x11a\x01\x1FW\x80c?\xC8\xCE\xF3\x14a\x02\"W\x80cZ\xF7\x07\x1A\x14a\x025W\x80cf\xD9\xA9\xA0\x14a\x02WW\x80cq@\xDCs\x14a\x02lW__\xFD[\x80c>A;\xEE\x14a\x01\xFFW\x80c>^<#\x14a\x02\x12W\x80c?r\x86\xF4\x14a\x02\x1AW__\xFD[\x80c\n\x92T\xE4\x14a\x01\x8BW\x80c\r#X\x17\x14a\x01\x95W\x80c\x13}\xA3\x0C\x14a\x01\xC5W\x80c\x1D\x82\x885\x14a\x01\xCDW\x80c\x1E\xD7\x83\x1C\x14a\x01\xD5W\x80c*\xDE8\x80\x14a\x01\xEAW[__\xFD[a\x01\x93a\x03gV[\0[`%Ta\x01\xA8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x93a\r\xDCV[a\x01\x93a\x10NV[a\x01\xDDa\x14\x92V[`@Qa\x01\xBC\x91\x90a7\xA1V[a\x01\xF2a\x14\xF2V[`@Qa\x01\xBC\x91\x90a8\x1AV[`\"Ta\x01\xA8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xDDa\x16.V[a\x01\xDDa\x16\x8CV[`!Ta\x01\xA8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02I_Q` aw4_9_Q\x90_R\x81V[`@Q\x90\x81R` \x01a\x01\xBCV[a\x02_a\x16\xEAV[`@Qa\x01\xBC\x91\x90a9\x1BV[a\x01\x93a\x18NV[` Ta\x01\xA8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\x8Fa\"lV[`@Qa\x01\xBC\x91\x90a9\x99V[`\x1FTa\x01\xA8\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xBCa#7V[`@Qa\x01\xBC\x91\x90a9\xF0V[a\x01\xA8s\\i\xBE\xE7\x01\xEF\x81J+j>\xDDK\x16R\xCB\x9C\xC5\xAAo\x81V[a\x02\xEFck6\xEC\x80\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xBCV[a\x02\xBCa$\x18V[a\x02\x8Fa$\xF9V[a\x03\x1Ca%\xC4V[`@Q\x90\x15\x15\x81R` \x01a\x01\xBCV[`$Ta\x01\xA8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xDDa&~V[`\x1FTa\x03\x1C\x90`\xFF\x16\x81V[`#Ta\x01\xA8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Qc\xF8w\xCB\x19`\xE0\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01Rj\x11U\x12\x17\xD4\x94\x10\xD7\xD5T\x93`\xAA\x1B`D\x82\x01R_Q` av\xF1_9_Q\x90_R\x90cq\xEEFM\x90\x82\x90c\xF8w\xCB\x19\x90`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xD0W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\xF7\x91\x90\x81\x01\x90a:{V[c\x01\x12\xA8\x80`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x19\x92\x91\x90a;.V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x045W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04Y\x91\x90a;OV[P`@Qa\x04f\x90a7zV[`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x04\x7FW=__>=_\xFD[P`\x1F`\x01a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`@Qa\x04\xB2\x90a7\x87V[`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x04\xCBW=__>=_\xFD[P` \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@Q`\x12\x90a\x04\xFB\x90a7\x94V[``\x80\x82R`\r\x90\x82\x01Rl+\xB90\xB882\xB2\x10\"\xBA42\xB9`\x99\x1B`\x80\x82\x01R`\xA0` \x82\x01\x81\x90R`\x04\x90\x82\x01Rc\n\xE8\xAA\x89`\xE3\x1B`\xC0\x82\x01R`\xFF\x90\x91\x16`@\x82\x01R`\xE0\x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x05^W=__>=_\xFD[P`!\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@Q`\x06\x90a\x05\x8E\x90a7\x94V[``\x80\x82R`\x08\x90\x82\x01Rg*\xA9\xA2\x10!\xB7\xB4\xB7`\xC1\x1B`\x80\x82\x01R`\xA0` \x82\x01\x81\x90R`\x04\x90\x82\x01RcUSDC`\xE0\x1B`\xC0\x82\x01R`\xFF\x90\x91\x16`@\x82\x01R`\xE0\x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x05\xECW=__>=_\xFD[P`\"\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`!T`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R\x91\x83\x16\x92c@\xC1\x0F\x19\x92a\x06A\x92\x90\x91\x16\x90h\x05k\xC7^-c\x10\0\0\x90`\x04\x01a;fV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06XW__\xFD[PZ\xF1\x15\x80\x15a\x06jW=__>=_\xFD[PP`!T`$T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc@\xC1\x0F\x19\x93Pa\x06\xAE\x92\x90\x91\x16\x90h\x05k\xC7^-c\x10\0\0\x90`\x04\x01a;fV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06\xC5W__\xFD[PZ\xF1\x15\x80\x15a\x06\xD7W=__>=_\xFD[PP`!T`%T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc@\xC1\x0F\x19\x93Pa\x07\x1B\x92\x90\x91\x16\x90h65\xC9\xAD\xC5\xDE\xA0\0\0\x90`\x04\x01a;fV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x072W__\xFD[PZ\xF1\x15\x80\x15a\x07DW=__>=_\xFD[PP`\"T`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc@\xC1\x0F\x19\x93Pa\x07\x84\x92\x90\x91\x16\x90d\x17Hv\xE8\0\x90`\x04\x01a;fV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07\x9BW__\xFD[PZ\xF1\x15\x80\x15a\x07\xADW=__>=_\xFD[PP`\"T`$T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc@\xC1\x0F\x19\x93Pa\x07\xED\x92\x90\x91\x16\x90d\x17Hv\xE8\0\x90`\x04\x01a;fV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\x04W__\xFD[PZ\xF1\x15\x80\x15a\x08\x16W=__>=_\xFD[PP`\"T`%T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc@\xC1\x0F\x19\x93Pa\x08W\x92\x90\x91\x16\x90e\x04\x8C'9P\0\x90`\x04\x01a;fV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08nW__\xFD[PZ\xF1\x15\x80\x15a\x08\x80W=__>=_\xFD[PP`#T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` av\xF1_9_Q\x90_R\x92Pc\xCAf\x9F\xA7\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\xD3W__\xFD[PZ\xF1\x15\x80\x15a\x08\xE5W=__>=_\xFD[PP`!T`\x1FT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\t^\xA7\xB3\x93Pa\t'\x92a\x01\0\x90\x92\x04\x90\x91\x16\x90_\x19\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\tCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tg\x91\x90a;\x7FV[P`#T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` av\xF1_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\t\xB7W__\xFD[PZ\xF1\x15\x80\x15a\t\xC9W=__>=_\xFD[PP`\"T`\x1FT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\t^\xA7\xB3\x93Pa\n\x0B\x92a\x01\0\x90\x92\x04\x90\x91\x16\x90_\x19\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\n'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nK\x91\x90a;\x7FV[P`$\x80T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` av\xF1_9_Q\x90_R\x91c\xCAf\x9F\xA7\x91\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n\x9AW__\xFD[PZ\xF1\x15\x80\x15a\n\xACW=__>=_\xFD[PP`!T`\x1FT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\t^\xA7\xB3\x93Pa\n\xEE\x92a\x01\0\x90\x92\x04\x90\x91\x16\x90_\x19\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0B\nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B.\x91\x90a;\x7FV[P`$\x80T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` av\xF1_9_Q\x90_R\x91c\xCAf\x9F\xA7\x91\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0B}W__\xFD[PZ\xF1\x15\x80\x15a\x0B\x8FW=__>=_\xFD[PP`\"T`\x1FT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\t^\xA7\xB3\x93Pa\x0B\xD1\x92a\x01\0\x90\x92\x04\x90\x91\x16\x90_\x19\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0B\xEDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x11\x91\x90a;\x7FV[P`%T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` av\xF1_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0CaW__\xFD[PZ\xF1\x15\x80\x15a\x0CsW=__>=_\xFD[PP`!T`\x1FT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\t^\xA7\xB3\x93Pa\x0C\xB5\x92a\x01\0\x90\x92\x04\x90\x91\x16\x90_\x19\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0C\xD1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xF5\x91\x90a;\x7FV[P`%T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` av\xF1_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\rEW__\xFD[PZ\xF1\x15\x80\x15a\rWW=__>=_\xFD[PP`\"T`\x1FT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x94Pc\t^\xA7\xB3\x93Pa\r\x99\x92a\x01\0\x90\x92\x04\x90\x91\x16\x90_\x19\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\r\xB5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xD9\x91\x90a;\x7FV[PV[`@\x80Qa\x01\0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R`\x1FT` T`@Qc\xC2\xA3<\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x93\x04\x83\x16\x92c\xC2\xA3<\x1D\x92a\x0Ei\x92_Q` aw4_9_Q\x90_R\x92ck6\xEC\x80\x92\x16\x90`\x04\x01a;\xA5V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0E\x80W__\xFD[PZ\xF1\x15\x80\x15a\x0E\x92W=__>=_\xFD[PP` \x80T`@Qa\x0E\xC8\x94P_Q` aw4_9_Q\x90_R\x93Pck6\xEC\x80\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x01a;\xA5V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x90\x92\x01\x91\x90\x91 \x80\x83R`\x1FT`!T\x92Qc\xA3\xDE\xF9#`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x92\x04\x82\x16\x93c\xA3\xDE\xF9#\x93a\x0F \x93\x91\x90\x91\x16\x91`\x04\x01a;fV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0F;W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F_\x91\x90a;\xE5V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x80\x85\x01\x91\x90\x91R\x91\x81\x16` \x84\x01R`\x1FT`\"T\x84Q\x93Qc\xA3\xDE\xF9#`\xE0\x1B\x81Ra\x01\0\x90\x92\x04\x83\x16\x93c\xA3\xDE\xF9#\x93a\x0F\xAD\x93\x92\x16\x91`\x04\x01a;fV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0F\xC8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xEC\x91\x90a;\xE5V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x84\x01R\x16``\x82\x01\x81\x90R\x81Q` \x83\x01Qa\x10\x14\x92a&\xDCV[a\x10&\x81` \x01Q\x82``\x01Qa(\x85V[`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x82\x01Ra\x10=\x81a+\x84V[a\r\xD9\x81_\x01Q\x82``\x01Qa.tV[`\x1FT` T`@Qc\xC2\xA3<\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x93\x04\x83\x16\x92c\xC2\xA3<\x1D\x92a\x10\x9B\x92_Q` aw4_9_Q\x90_R\x92ck6\xEC\x80\x92\x16\x90`\x04\x01a;\xA5V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10\xB2W__\xFD[PZ\xF1\x15\x80\x15a\x10\xC4W=__>=_\xFD[PP` \x80T`@Q_\x94Pa\x10\xFA\x93P_Q` aw4_9_Q\x90_R\x92ck6\xEC\x80\x92`\x01`\x01`\xA0\x1B\x03\x16\x91\x01a;\xA5V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x1FT`!Tc\xA3\xDE\xF9#`\xE0\x1B\x84R\x91\x93P_\x92\x83\x92`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x93\x04\x83\x16\x92c\xA3\xDE\xF9#\x92a\x11S\x92\x90\x91\x16\x90\x87\x90`\x04\x01a;fV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x11nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x92\x91\x90a;\xE5V[`\x1FT`\"T`@Qc\xA3\xDE\xF9#`\xE0\x1B\x81R\x93\x95P\x91\x93P_\x92\x83\x92`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x93\x04\x83\x16\x92c\xA3\xDE\xF9#\x92a\x11\xD8\x92\x90\x91\x16\x90\x89\x90`\x04\x01a;fV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x11\xF3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x17\x91\x90a;\xE5V[`@Qcd\xE3)\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x04\x83\x01R\x86\x16`$\x82\x01R\x91\x93P\x91P_\x90s\\i\xBE\xE7\x01\xEF\x81J+j>\xDDK\x16R\xCB\x9C\xC5\xAAo\x90c\xC9\xC6S\x96\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x12}W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xA1\x91\x90a<\x16V[`@Qcd\xE3)\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x04\x83\x01R\x84\x16`$\x82\x01R\x90\x91P_\x90s\\i\xBE\xE7\x01\xEF\x81J+j>\xDDK\x16R\xCB\x9C\xC5\xAAo\x90c\xC9\xC6S\x96\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13\x05W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13)\x91\x90a<\x16V[`@Qcd\xE3)\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x89\x16`\x04\x83\x01R\x86\x16`$\x82\x01R\x90\x91P_\x90s\\i\xBE\xE7\x01\xEF\x81J+j>\xDDK\x16R\xCB\x9C\xC5\xAAo\x90c\xC9\xC6S\x96\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13\x8DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xB1\x91\x90a<\x16V[\x90Pa\x14\x07_`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15`@Q\x80`@\x01`@R\x80`\x1D\x81R` \x01\x7FYES/NO WETH pair should exist\0\0\0\x81RPa2FV[`@\x80Q\x80\x82\x01\x90\x91R`\x1D\x81R\x7FYES/NO USDC pair should exist\0\0\0` \x82\x01Ra\x14Q\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x15\x15\x90a2FV[a\x14\x88_`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14\x15`@Q\x80``\x01`@R\x80`#\x81R` \x01aw\x11`#\x919a2FV[PPPPPPPPV[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x14\xE8W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x14\xCAW[PPPPP\x90P\x90V[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x16%W_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x16\x0EW\x83\x82\x90_R` _ \x01\x80Ta\x15\x83\x90a</V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\xAF\x90a</V[\x80\x15a\x15\xFAW\x80`\x1F\x10a\x15\xD1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\xFAV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\xDDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x15fV[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x15\x15V[PPPP\x90P\x90V[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x14\xE8W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x14\xCAWPPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x14\xE8W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x14\xCAWPPPPP\x90P\x90V[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x16%W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x17=\x90a</V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x17i\x90a</V[\x80\x15a\x17\xB4W\x80`\x1F\x10a\x17\x8BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x17\xB4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x17\x97W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x186W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x17\xF8W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x17\rV[`\x1FT` T`@Qc\xC2\xA3<\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x93\x04\x83\x16\x92c\xC2\xA3<\x1D\x92a\x18\x9B\x92_Q` aw4_9_Q\x90_R\x92ck6\xEC\x80\x92\x16\x90`\x04\x01a;\xA5V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x18\xB2W__\xFD[PZ\xF1\x15\x80\x15a\x18\xC4W=__>=_\xFD[PP` \x80T`@Q_\x94Pa\x18\xFA\x93P_Q` aw4_9_Q\x90_R\x92ck6\xEC\x80\x92`\x01`\x01`\xA0\x1B\x03\x16\x91\x01a;\xA5V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x1FT`!Tc\xA3\xDE\xF9#`\xE0\x1B\x84R\x91\x93P_\x92\x83\x92`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x93\x04\x83\x16\x92c\xA3\xDE\xF9#\x92a\x19S\x92\x90\x91\x16\x90\x87\x90`\x04\x01a;fV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x92\x91\x90a;\xE5V[`\x1FT`\"T`@Qc\xA3\xDE\xF9#`\xE0\x1B\x81R\x93\x95P\x91\x93P_\x92\x83\x92`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x93\x04\x83\x16\x92c\xA3\xDE\xF9#\x92a\x19\xD8\x92\x90\x91\x16\x90\x89\x90`\x04\x01a;fV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19\xF3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x17\x91\x90a;\xE5V[`%T`@Qc\x03\">\xAB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x91\x93P\x91P_Q` av\xF1_9_Q\x90_R\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1AkW__\xFD[PZ\xF1\x15\x80\x15a\x1A}W=__>=_\xFD[PP`\x1FT`!T`@Qc\x9B\xA70\xA9`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc\x9B\xA70\xA9\x93Pa\x1A\xC8\x92\x91\x16\x90h\n\xD7\x8E\xBCZ\xC6 \0\0\x90\x8A\x90`\x04\x01a<gV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1A\xDFW__\xFD[PZ\xF1\x15\x80\x15a\x1A\xF1W=__>=_\xFD[PP`\x1FT`\"T`@Qc\x9B\xA70\xA9`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc\x9B\xA70\xA9\x93Pa\x1B8\x92\x91\x16\x90d\xE8\xD4\xA5\x10\0\x90\x8A\x90`\x04\x01a<gV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1BOW__\xFD[PZ\xF1\x15\x80\x15a\x1BaW=__>=_\xFD[PPPP_Q` av\xF1_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1B\xAAW__\xFD[PZ\xF1\x15\x80\x15a\x1B\xBCW=__>=_\xFD[PP`@Qcd\xE3)\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x04\x83\x01R\x85\x16`$\x82\x01R_\x92Ps\\i\xBE\xE7\x01\xEF\x81J+j>\xDDK\x16R\xCB\x9C\xC5\xAAo\x91Pc\xC9\xC6S\x96\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1C!W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1CE\x91\x90a<\x16V[`%T`@Qc\x03\">\xAB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x91P_Q` av\xF1_9_Q\x90_R\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1C\x97W__\xFD[PZ\xF1\x15\x80\x15a\x1C\xA9W=__>=_\xFD[PP`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x16\x92Pc\xA9\x05\x9C\xBB\x91Pa\x1C\xE4\x90\x84\x90h\x05k\xC7^-c\x10\0\0\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1D\0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D$\x91\x90a;\x7FV[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xA9\x05\x9C\xBB\x90a\x1DX\x90\x84\x90d\xBAC\xB7@\0\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1DtW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x98\x91\x90a;\x7FV[P`%T`@Qc51<!`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x82\x16\x90cjbxB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1D\xE2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x06\x91\x90a;OV[P_Q` av\xF1_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1ELW__\xFD[PZ\xF1\x15\x80\x15a\x1E^W=__>=_\xFD[PP`@Qcd\xE3)\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x04\x83\x01R\x85\x16`$\x82\x01R_\x92Ps\\i\xBE\xE7\x01\xEF\x81J+j>\xDDK\x16R\xCB\x9C\xC5\xAAo\x91Pc\xC9\xC6S\x96\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1E\xC3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xE7\x91\x90a<\x16V[`%T`@Qc\x03\">\xAB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x91P_Q` av\xF1_9_Q\x90_R\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1F9W__\xFD[PZ\xF1\x15\x80\x15a\x1FKW=__>=_\xFD[PP`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x16\x92Pc\xA9\x05\x9C\xBB\x91Pa\x1F\x86\x90\x84\x90h\x05k\xC7^-c\x10\0\0\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1F\xA2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xC6\x91\x90a;\x7FV[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xA9\x05\x9C\xBB\x90a\x1F\xFA\x90\x84\x90dE\xD9d\xB8\0\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a \x16W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a :\x91\x90a;\x7FV[P`%T`@Qc51<!`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x82\x16\x90cjbxB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a \x84W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xA8\x91\x90a;OV[P_Q` av\xF1_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a \xEEW__\xFD[PZ\xF1\x15\x80\x15a!\0W=__>=_\xFD[PPPP__\x83`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!f\x91\x90a<\x9EV[P\x91P\x91P__\x84`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\xA9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xCD\x91\x90a<\x9EV[P\x91P\x91Pa\"\x1E_`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14\x15`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x16QT\xC8\x1C\x18Z\\\x88\x1C\xDA\x1B\xDD[\x19\x08\x19^\x1A\\\xDD`Z\x1B\x81RPa2FV[`@\x80Q\x80\x82\x01\x90\x91R`\x14\x81Rs\x13\x93\xC8\x1C\x18Z\\\x88\x1C\xDA\x1B\xDD[\x19\x08\x19^\x1A\\\xDD`b\x1B` \x82\x01Ra\"_\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x15\x15\x90a2FV[PPPPPPPPPPPV[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x16%W\x83\x82\x90_R` _ \x01\x80Ta\"\xAC\x90a</V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\"\xD8\x90a</V[\x80\x15a##W\x80`\x1F\x10a\"\xFAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a##V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a#\x06W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\"\x8FV[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x16%W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a$\0W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a#\xC2W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a#ZV[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x16%W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a$\xE1W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a$\xA3W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a$;V[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x16%W\x83\x82\x90_R` _ \x01\x80Ta%9\x90a</V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta%e\x90a</V[\x80\x15a%\xB0W\x80`\x1F\x10a%\x87Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a%\xB0V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a%\x93W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a%\x1CV[`\x08T_\x90`\xFF\x16\x15a%\xDBWP`\x08T`\xFF\x16\x90V[`@Qc\x06g\xF9\xD7`\xE4\x1B\x81R_\x90_Q` av\xF1_9_Q\x90_R\x90cf\x7F\x9Dp\x90a&8\x90\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90e\x19\x98Z[\x19Y`\xD2\x1B\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&SW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&w\x91\x90a;OV[\x14\x15\x90P\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x14\xE8W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x14\xCAWPPPPP\x90P\x90V[`%T`@Qc\x03\">\xAB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` av\xF1_9_Q\x90_R\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a'+W__\xFD[PZ\xF1\x15\x80\x15a'=W=__>=_\xFD[PP`\x1FT`!T`@Qc\x9B\xA70\xA9`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc\x9B\xA70\xA9\x93Pa'\x88\x92\x91\x16\x90h\n\xD7\x8E\xBCZ\xC6 \0\0\x90\x88\x90`\x04\x01a<gV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a'\x9FW__\xFD[PZ\xF1\x15\x80\x15a'\xB1W=__>=_\xFD[PP`\x1FT`\"T`@Qc\x9B\xA70\xA9`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc\x9B\xA70\xA9\x93Pa'\xF8\x92\x91\x16\x90d\xE8\xD4\xA5\x10\0\x90\x88\x90`\x04\x01a<gV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a(\x0FW__\xFD[PZ\xF1\x15\x80\x15a(!W=__>=_\xFD[PPPP_Q` av\xF1_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a(jW__\xFD[PZ\xF1\x15\x80\x15a(|W=__>=_\xFD[PPPPPPPV[`@Qcd\xE3)\xCB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R\x82\x16`$\x82\x01R_\x90\x81\x90s\\i\xBE\xE7\x01\xEF\x81J+j>\xDDK\x16R\xCB\x9C\xC5\xAAo\x90c\xC9\xC6S\x96\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a(\xE8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\x0C\x91\x90a<\x16V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a)`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x14\x18Z\\\x88\x18\xDC\x99X]\x1A[\xDB\x88\x19\x98Z[\x19Y`b\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`%T`@Qc\x03\">\xAB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` av\xF1_9_Q\x90_R\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a)\xAFW__\xFD[PZ\xF1\x15\x80\x15a)\xC1W=__>=_\xFD[PP`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16\x92Pc\xA9\x05\x9C\xBB\x91Pa)\xFC\x90\x84\x90h\x05k\xC7^-c\x10\0\0\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a*\x18W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*<\x91\x90a;\x7FV[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xA9\x05\x9C\xBB\x90a*p\x90\x84\x90d\xBAC\xB7@\0\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a*\x8CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xB0\x91\x90a;\x7FV[P`%T`@Qc51<!`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x82\x16\x90cjbxB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a*\xFAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\x1E\x91\x90a;OV[P_Q` av\xF1_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a+dW__\xFD[PZ\xF1\x15\x80\x15a+vW=__>=_\xFD[P\x92\x93PPPP[\x92\x91PPV[`#T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` av\xF1_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a+\xD3W__\xFD[PZ\xF1\x15\x80\x15a+\xE5W=__>=_\xFD[PP`\x1FT`!T\x84Q`@Qc\x9B\xA70\xA9`\xE0\x1B\x81Ra\x01\0\x90\x93\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x95Pc\x9B\xA70\xA9\x94Pa,0\x93\x92\x16\x91g\x8A\xC7#\x04\x89\xE8\0\0\x91\x90`\x04\x01a<gV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a,GW__\xFD[PZ\xF1\x15\x80\x15a,YW=__>=_\xFD[PPPP` \x81\x01Q`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xA9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xCD\x91\x90a;OV[`\xC0\x82\x01R``\x81\x01Q`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\x1EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-B\x91\x90a;OV[`\xE0\x82\x01R`\xA0\x81\x01Q` \x82\x01Q``\x83\x01Qa-a\x92\x91\x90a2\xA8V[` \x81\x01Q`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra-\xF4\x92\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\xB3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\xD7\x91\x90a;OV[g\r\xE0\xB6\xB3\xA7d\0\0\x83`\xC0\x01Qa-\xEF\x91\x90a<\xFEV[a6\x1CV[``\x81\x01Q`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\r\xD9\x92\x91\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.j\x91\x90a;OV[\x82`\xE0\x01Qa6TV[` T`@Qc\x17\xEE6o`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c/\xDCl\xDE\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a.\xBEW__\xFD[PZ\xF1\x15\x80\x15a.\xD0W=__>=_\xFD[PP`\x1FT`@Qc\\#\xBD\xF5`\xE0\x1B\x81R`\x04\x81\x01\x86\x90Ra\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x92Pc\\#\xBD\xF5\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/\x1BW__\xFD[PZ\xF1\x15\x80\x15a/-W=__>=_\xFD[PP`\"T`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R_\x94P\x91\x16\x91Pcp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/}W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/\xA1\x91\x90a;OV[`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x92P_\x91\x90\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\xEEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\x12\x91\x90a;OV[`#T`@Qc\x03\">\xAB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x91P_Q` av\xF1_9_Q\x90_R\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a0dW__\xFD[PZ\xF1\x15\x80\x15a0vW=__>=_\xFD[PP`\x1FT`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x88\x16\x94Pc\t^\xA7\xB3\x93Pa0\xB1\x92a\x01\0\x90\x04\x16\x90\x85\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a0\xCDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\xF1\x91\x90a;\x7FV[P`\x1FT`@Qc\x01\xE9\xA6\x95`\xE4\x1B\x81Ra\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90c\x1E\x9AiP\x90a1(\x90\x86\x90\x85\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a1DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1h\x91\x90a;OV[P_Q` av\xF1_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a1\xAEW__\xFD[PZ\xF1\x15\x80\x15a1\xC0W=__>=_\xFD[PP`\"T`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra2@\x94P\x91\x16\x91Pcp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\x12W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a26\x91\x90a;OV[a-\xEF\x83\x85a=\x11V[PPPPV[`@Qc\xA3N\xDC\x03`\xE0\x1B\x81R_Q` av\xF1_9_Q\x90_R\x90c\xA3N\xDC\x03\x90a2x\x90\x85\x90\x85\x90`\x04\x01a=$V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a2\x8EW__\xFD[PZ\xFA\x15\x80\x15a2\xA0W=__>=_\xFD[PPPPPPV[__\x84`\x01`\x01`\xA0\x1B\x03\x16c\t\x02\xF1\xAC`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\xE6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\n\x91\x90a<\x9EV[P\x91P\x91P_\x85`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3LW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3p\x91\x90a<\x16V[\x90P__\x86`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14a3\xA6W\x83`\x01`\x01`p\x1B\x03\x16\x85`\x01`\x01`p\x1B\x03\x16a3\xBBV[\x84`\x01`\x01`p\x1B\x03\x16\x84`\x01`\x01`p\x1B\x03\x16[\x90\x92P\x90Pg\r\xE0\xB6\xB3\xA7d\0\0_a3\xD5\x82\x85\x85a6\x8CV[`#T`@Qc\x03\">\xAB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x91P_Q` av\xF1_9_Q\x90_R\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a4'W__\xFD[PZ\xF1\x15\x80\x15a49W=__>=_\xFD[PP`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8C\x16\x92Pc\xA9\x05\x9C\xBB\x91Pa4k\x90\x8D\x90\x86\x90`\x04\x01a;fV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a4\x87W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\xAB\x91\x90a;\x7FV[P\x88`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x03a5?W`#T`@Qc\x02,\r\x9F`\xE0\x1B\x81R_`\x04\x82\x01\x81\x90R`$\x82\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`D\x83\x01R`\x80`d\x83\x01R`\x84\x82\x01R\x90\x8B\x16\x90c\x02,\r\x9F\x90`\xA4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a5$W__\xFD[PZ\xF1\x15\x80\x15a56W=__>=_\xFD[PPPPa5\xB5V[`#T`@Qc\x02,\r\x9F`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R_`$\x82\x01\x81\x90R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`D\x83\x01R`\x80`d\x83\x01R`\x84\x82\x01R\x90\x8B\x16\x90c\x02,\r\x9F\x90`\xA4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a5\x9EW__\xFD[PZ\xF1\x15\x80\x15a5\xB0W=__>=_\xFD[PPPP[_Q` av\xF1_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a5\xFAW__\xFD[PZ\xF1\x15\x80\x15a6\x0CW=__>=_\xFD[PPPPPPPPPPPPPPV[`@Qc&\n[\x15`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R_Q` av\xF1_9_Q\x90_R\x90c\x98)lT\x90`D\x01a2xV[`@Qcm\x83\xFEi`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R_Q` av\xF1_9_Q\x90_R\x90c\xDB\x07\xFC\xD2\x90`D\x01a2xV[__\x84\x11a6\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FINSUFFICIENT_INPUT_AMOUNT\0\0\0\0\0\0\0`D\x82\x01R`d\x01a)WV[_\x83\x11\x80\x15a6\xEAWP_\x82\x11[a7/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuINSUFFICIENT_LIQUIDITY`P\x1B`D\x82\x01R`d\x01a)WV[_a7<\x85a\x03\xE5a=FV[\x90P_a7I\x84\x83a=FV[\x90P_\x82a7Y\x87a\x03\xE8a=FV[a7c\x91\x90a=\x11V[\x90Pa7o\x81\x83a=]V[\x97\x96PPPPPPPV[a%\x96\x80a=}\x839\x01\x90V[a\x05\n\x80ac\x13\x839\x01\x90V[a\x0E\xD4\x80ah\x1D\x839\x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a7\xE1W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a7\xBAV[P\x90\x95\x94PPPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a+vW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90```\x05\x82\x90\x1B\x88\x01\x81\x01\x91\x90\x88\x01\x90_[\x81\x81\x10\x15a8\xBDW`_\x19\x8A\x85\x03\x01\x83Ra8\xA7\x84\x86Qa7\xECV[` \x95\x86\x01\x95\x90\x94P\x92\x90\x92\x01\x91`\x01\x01a8\x8BV[P\x91\x97PPP` \x94\x85\x01\x94\x92\x90\x92\x01\x91P`\x01\x01a8@V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a9\x11W\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a8\xE9V[P\x93\x94\x93PPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a+vW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`@\x87Ra9g`@\x88\x01\x82a7\xECV[\x90P` \x82\x01Q\x91P\x86\x81\x03` \x88\x01Ra9\x82\x81\x83a8\xD7V[\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a9AV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a+vW`?\x19\x87\x86\x03\x01\x84Ra9\xDB\x85\x83Qa7\xECV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a9\xBFV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a+vW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90a:Q\x90\x87\x01\x82a8\xD7V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a:\x16V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a:\x8BW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\xA1W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a:\xB1W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\xCBWa:\xCBa:gV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a:\xFAWa:\xFAa:gV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a;\x11W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`@\x81R_a;@`@\x83\x01\x85a7\xECV[\x90P\x82` \x83\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a;_W__\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[_` \x82\x84\x03\x12\x15a;\x8FW__\xFD[\x81Q\x80\x15\x15\x81\x14a;\x9EW__\xFD[\x93\x92PPPV[\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x90\x91\x16` \x83\x01R`\x01`\x01`\xA0\x1B\x03\x16`@\x82\x01R``\x01\x90V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a;\xE0W__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a;\xF6W__\xFD[a;\xFF\x83a;\xCAV[\x91Pa<\r` \x84\x01a;\xCAV[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a<&W__\xFD[a;\x9E\x82a;\xCAV[`\x01\x81\x81\x1C\x90\x82\x16\x80a<CW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a<aWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R`@\x82\x01R``\x01\x90V[\x80Q`\x01`\x01`p\x1B\x03\x81\x16\x81\x14a;\xE0W__\xFD[___``\x84\x86\x03\x12\x15a<\xB0W__\xFD[a<\xB9\x84a<\x88V[\x92Pa<\xC7` \x85\x01a<\x88V[\x91P`@\x84\x01Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a<\xDFW__\xFD[\x80\x91PP\x92P\x92P\x92V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a+~Wa+~a<\xEAV[\x80\x82\x01\x80\x82\x11\x15a+~Wa+~a<\xEAV[\x82\x15\x15\x81R`@` \x82\x01R_a=>`@\x83\x01\x84a7\xECV[\x94\x93PPPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a+~Wa+~a<\xEAV[_\x82a=wWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE`\x80`@R4\x80\x15`\x0EW__\xFD[Pa%z\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\x9BW_5`\xE0\x1C\x80c\x9B\xA70\xA9\x11a\0cW\x80c\x9B\xA70\xA9\x14a\x01zW\x80c\xA3\xDE\xF9#\x14a\x01\x8DW\x80c\xC2\xA3<\x1D\x14a\x01\xC0W\x80c\xD4\xB0o\xB8\x14a\x01\xD3W\x80c\xE9s\x95P\x14a\x02\x02W__\xFD[\x80c\x1E\x9AiP\x14a\0\x9FW\x80cA\x05Zv\x14a\0\xC5W\x80c\\#\xBD\xF5\x14a\0\xDAW\x80cq\x83\xD2J\x14a\0\xEDW\x80cud\x91+\x14a\x01\x10W[__\xFD[a\0\xB2a\0\xAD6`\x04a\x11\xDFV[a\x02\x15V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xD8a\0\xD36`\x04a\x12\tV[a\x06*V[\0[a\0\xD8a\0\xE86`\x04a\x12;V[a\x08\xE1V[a\x01\0a\0\xFB6`\x04a\x12RV[a\nFV[`@Q\x90\x15\x15\x81R` \x01a\0\xBCV[a\x01Pa\x01\x1E6`\x04a\x12;V[_` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x82\x16\x91d\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90\x83V[`@\x80Qc\xFF\xFF\xFF\xFF\x90\x94\x16\x84R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x84\x01R\x90\x82\x01R``\x01a\0\xBCV[a\0\xD8a\x01\x886`\x04a\x12\tV[a\x0B\xF1V[a\x01\xA0a\x01\x9B6`\x04a\x11\xDFV[a\r\xCDV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\0\xBCV[a\0\xD8a\x01\xCE6`\x04a\x12tV[a\x0F\x0FV[a\x01\xF5a\x01\xE16`\x04a\x12;V[`\x01` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Qa\0\xBC\x91\x90a\x12\xCFV[a\x01\xA0a\x02\x106`\x04a\x11\xDFV[a\x10LV[__`@Q` \x01a\x020\x90bYES`\xE8\x1B\x81R`\x03\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x81W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\xA8\x91\x90\x81\x01\x90a\x13\tV[`@Q` \x01a\x02\xB8\x91\x90a\x13\xD3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14\x90P_\x84`\x01`\x01`\xA0\x1B\x03\x16c\xC0GM\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x0EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x032\x91\x90a\x13\xDEV[\x90P_\x85`\x01`\x01`\xA0\x1B\x03\x16cH\0\xD9\x7F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03qW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x95\x91\x90a\x13\xF5V[\x90Pa\x03\xA2\x83\x82\x84a\x10pV[`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x14a\x03\xD3W`@Qc@\xFEP\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_\x83\x81R`\x01` R`@\x90 T`\xFF\x16`\x04\x81\x11\x15a\x03\xF7Wa\x03\xF7a\x12\xBBV[\x14\x80\x15a\x04\x01WP\x82[\x15a\x04\x0EW\x84\x93Pa\x04\x9EV[`\x03_\x83\x81R`\x01` R`@\x90 T`\xFF\x16`\x04\x81\x11\x15a\x042Wa\x042a\x12\xBBV[\x14\x80\x15a\x04=WP\x82\x15[\x15a\x04JW\x84\x93Pa\x04\x9EV[`\x04_\x83\x81R`\x01` R`@\x90 T`\xFF\x16`\x04\x81\x11\x15a\x04nWa\x04na\x12\xBBV[\x03a\x04\x85Wa\x04~`\x02\x86a\x14\x10V[\x93Pa\x04\x9EV[`@QcrV\xF6E`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c#\xB8r\xDD\x90a\x04\xCE\x903\x900\x90\x8A\x90`\x04\x01a\x14/V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x04\xEAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x0E\x91\x90a\x14SV[P`@Qc'p\xA7\xEB`\xE2\x1B\x81R0`\x04\x82\x01R`$\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c\x9D\xC2\x9F\xAC\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05TW__\xFD[PZ\xF1\x15\x80\x15a\x05fW=__>=_\xFD[PP`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x92Pc\xA9\x05\x9C\xBB\x91P`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x05\xB4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xD8\x91\x90a\x14SV[P`@\x80Q\x86\x81R` \x81\x01\x86\x90R\x83\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x913\x91\x7F\xAE\x1D\x80J\x86\xB6\xBA*w%\xD9\x82ML\xCBN\x7F\x9AU\xE7\xA1\xBFCy\xC2f\x92\xCEZ\xD0\x06e\x91\x01`@Q\x80\x91\x03\x90\xA4PPP\x92\x91PPV[`\x01_\x82\x81R`\x01` R`@\x90 T`\xFF\x16`\x04\x81\x11\x15a\x06NWa\x06Na\x12\xBBV[\x14a\x06lW`@Qc#\x9D\xD4\xAD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a\x06x\x85\x84a\x10LV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c#\xB8r\xDD\x90a\x06\xAD\x903\x900\x90\x89\x90`\x04\x01a\x14/V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06\xC9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xED\x91\x90a\x14SV[P`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c#\xB8r\xDD\x90a\x07\x1E\x903\x900\x90\x89\x90`\x04\x01a\x14/V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x07:W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07^\x91\x90a\x14SV[P`@Qc'p\xA7\xEB`\xE2\x1B\x81R0`\x04\x82\x01R`$\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\x9D\xC2\x9F\xAC\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07\xA4W__\xFD[PZ\xF1\x15\x80\x15a\x07\xB6W=__>=_\xFD[PP`@Qc'p\xA7\xEB`\xE2\x1B\x81R0`\x04\x82\x01R`$\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x92Pc\x9D\xC2\x9F\xAC\x91P`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x07\xFFW__\xFD[PZ\xF1\x15\x80\x15a\x08\x11W=__>=_\xFD[PP`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16\x92Pc\xA9\x05\x9C\xBB\x91P`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08_W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x83\x91\x90a\x14SV[P`@\x80Q\x85\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16` \x83\x01R\x83\x81\x16\x92\x82\x01\x92\x90\x92R\x84\x91\x87\x16\x903\x90\x7F\x9A\x85;\xFB\xDD`4\xE0\xE7U;\xB8\xECw\x89\xB2`=\xCC\xCBWY\xCF9\nz`\x1C\x0Bm\xA0\x03\x90``\x01[`@Q\x80\x91\x03\x90\xA4PPPPPV[`\x01_\x82\x81R`\x01` R`@\x90 T`\xFF\x16`\x04\x81\x11\x15a\t\x05Wa\t\x05a\x12\xBBV[\x14a\t#W`@Qc#\x9D\xD4\xAD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R` \x81\x90R`@\x81 Tc\xFF\xFF\xFF\xFF\x16B\x10a\tEWP`\x04a\t\xD8V[_\x82\x81R` \x81\x90R`@\x80\x82 T\x90Qc\x01\xF4k)`\xE1\x1B\x81R`\x04\x81\x01\x85\x90Rd\x01\0\0\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90c\x03\xE8\xD6R\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\t\x9FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xC3\x91\x90a\x14SV[\x90P\x80a\t\xD1W`\x03a\t\xD4V[`\x02[\x91PP[_\x82\x81R`\x01` \x81\x90R`@\x90\x91 \x80T\x83\x92`\xFF\x19\x90\x91\x16\x90\x83`\x04\x81\x11\x15a\n\x05Wa\n\x05a\x12\xBBV[\x02\x17\x90UP\x81\x7F\xF3I\x84G1H\x05\x1B\xC1\xBD\xF1\xBEn\xCCF-{\"\x8DY\x10X\xA8\xA2yw\xB8Gp\xB78\xB9\x82`@Qa\n:\x91\x90a\x12\xCFV[`@Q\x80\x91\x03\x90\xA2PPV[__\x82`\x01`\x01`\xA0\x1B\x03\x16c\xC0GM\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x84W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xA8\x91\x90a\x13\xDEV[\x90P_\x83`\x01`\x01`\xA0\x1B\x03\x16cH\0\xD9\x7F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xE7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x0B\x91\x90a\x13\xF5V[\x90P_`@Q` \x01a\x0B'\x90bYES`\xE8\x1B\x81R`\x03\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BxW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0B\x9F\x91\x90\x81\x01\x90a\x13\tV[`@Q` \x01a\x0B\xAF\x91\x90a\x13\xD3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14\x90Pa\x0B\xD3\x81\x83\x85a\x10pV[`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x14\x93PPPP\x91\x90PV[`\x01_\x82\x81R`\x01` R`@\x90 T`\xFF\x16`\x04\x81\x11\x15a\x0C\x15Wa\x0C\x15a\x12\xBBV[\x14a\x0C3W`@Qc#\x9D\xD4\xAD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c#\xB8r\xDD\x90a\x0Cc\x903\x900\x90\x87\x90`\x04\x01a\x14/V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0C\x7FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xA3\x91\x90a\x14SV[P__a\x0C\xB0\x85\x84a\x10LV[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x87\x90R\x91\x93P\x91P`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c@\xC1\x0F\x19\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\xFAW__\xFD[PZ\xF1\x15\x80\x15a\r\x0CW=__>=_\xFD[PP`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x92Pc@\xC1\x0F\x19\x91P`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\rUW__\xFD[PZ\xF1\x15\x80\x15a\rgW=__>=_\xFD[PPPP\x82\x85`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8F\xB8\xA3)\xBCh\x855h\xE0\xDD'\xC6\xAE\xE2\xE3a\xA1\xF0x\x92\xCFe\xABI\xCC/0\xCD!\xC3\xC5\x87\x86\x86`@Qa\x08\xD2\x93\x92\x91\x90\x92\x83R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[__\x82\x84\x84`@Qa\r\xDE\x90a\x11\xBBV[`\x80\x80\x82R`\x03\x90\x82\x01\x81\x90RbYes`\xE8\x1B`\xA0\x83\x01R`\xC0` \x83\x01\x81\x90R\x82\x01RbYES`\xE8\x1B`\xE0\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`@\x83\x01R``\x82\x01Ra\x01\0\x01\x81\x90`@Q\x80\x91\x03\x90_\xF5\x90P\x80\x15\x80\x15a\x0EEW=__>=_\xFD[P\x91P\x82\x84\x84`@Qa\x0EW\x90a\x11\xBBV[`\x80\x80\x82R`\x02\x90\x82\x01\x81\x90RaNo`\xF0\x1B`\xA0\x83\x01R`\xC0` \x83\x01\x81\x90R\x82\x01RaNO`\xF0\x1B`\xE0\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`@\x83\x01R``\x82\x01Ra\x01\0\x01\x81\x90`@Q\x80\x91\x03\x90_\xF5\x90P\x80\x15\x80\x15a\x0E\xBCW=__>=_\xFD[P`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x83\x81\x16` \x83\x01R\x82Q\x93\x94P\x87\x16\x92\x86\x92\x7F\x9EC\x1C\xAE\xF5u>>\x93\x96S\xFC\x05l\x91\x07%\x98\0)\xB3e=\xB5\xA3k\xEB\xCAq\xD6\xBC,\x92\x82\x90\x03\x01\x90\xA3\x92P\x92\x90PV[`@\x80Q` \x81\x01\x85\x90Rc\xFF\xFF\xFF\xFF\x84\x16\x91\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x82\x16``\x82\x01R_\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x90P__\x82\x81R`\x01` R`@\x90 T`\xFF\x16`\x04\x81\x11\x15a\x0FyWa\x0Fya\x12\xBBV[\x14a\x0F\x97W`@Qc7\xA5\xBD\x9B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x01` \x81\x81R`@\x80\x84 \x80T`\xFF\x19\x16\x84\x17\x90U\x80Q``\x81\x01\x82Rc\xFF\xFF\xFF\xFF\x88\x81\x16\x80\x83R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x84\x87\x01\x81\x81R\x85\x87\x01\x8E\x81R\x8B\x8BR\x8A\x89R\x99\x87\x90 \x95Q\x86T\x91Q\x95\x16`\x01`\x01`\xC0\x1B\x03\x19\x90\x91\x16\x17d\x01\0\0\0\0\x94\x90\x92\x16\x93\x90\x93\x02\x17\x83U\x95Q\x91\x90\x94\x01U\x80Q\x93\x84RQ\x91\x92\x87\x92\x85\x92\x7F\tN\xBAi\xD7\xCA\x9D\xFA\xFA4\xC8\x96\x06\x7F\xC8\xE1\x93\x95\xA6\x10\xE4}\xEB\x8B%\xB2\x1B\x87\xBE\xC3*4\x92\x82\x90\x03\x01\x90\xA4PPPPV[__a\x10Z`\x01\x85\x85a\x10pV[\x91Pa\x10g_\x85\x85a\x10pV[\x90P\x92P\x92\x90PV[__`@Q\x80` \x01a\x10\x82\x90a\x11\xBBV[`\x1F\x19\x82\x82\x03\x81\x01\x83R`\x1F\x90\x91\x01\x16`@R\x85a\x10\xBAW`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aNo`\xF0\x1B\x81RPa\x10\xD7V[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bYes`\xE8\x1B\x81RP[\x86a\x10\xFCW`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aNO`\xF0\x1B\x81RPa\x11\x19V[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bYES`\xE8\x1B\x81RP[\x86\x86`@Q` \x01a\x11.\x94\x93\x92\x91\x90a\x14\xA0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x11L\x92\x91` \x01a\x14\xE1V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\x01`\x01`\xF8\x1B\x03\x19\x82\x85\x01R0``\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`!\x85\x01R`5\x84\x01\x96\x90\x96R`U\x80\x84\x01\x96\x90\x96R\x81Q\x80\x84\x03\x90\x96\x01\x86R`u\x90\x92\x01\x90R\x83Q\x93\x01\x92\x90\x92 \x94\x93PPPPV[a\x10G\x80a\x14\xFE\x839\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x11\xDCW__\xFD[PV[__`@\x83\x85\x03\x12\x15a\x11\xF0W__\xFD[\x825a\x11\xFB\x81a\x11\xC8V[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a\x12\x1BW__\xFD[\x835a\x12&\x81a\x11\xC8V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x12KW__\xFD[P5\x91\x90PV[_` \x82\x84\x03\x12\x15a\x12bW__\xFD[\x815a\x12m\x81a\x11\xC8V[\x93\x92PPPV[___``\x84\x86\x03\x12\x15a\x12\x86W__\xFD[\x835\x92P` \x84\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x12\xA0W__\xFD[\x91P`@\x84\x015a\x12\xB0\x81a\x11\xC8V[\x80\x91PP\x92P\x92P\x92V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[` \x81\x01`\x05\x83\x10a\x12\xEFWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x91\x90R\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x13\x19W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13/W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x13?W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13YWa\x13Ya\x12\xF5V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x13\x88Wa\x13\x88a\x12\xF5V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x13\x9FW__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\x12m\x82\x84a\x13\xBCV[_` \x82\x84\x03\x12\x15a\x13\xEEW__\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15a\x14\x05W__\xFD[\x81Qa\x12m\x81a\x11\xC8V[_\x82a\x14*WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[_` \x82\x84\x03\x12\x15a\x14cW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x12mW__\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x80\x81R_a\x14\xB2`\x80\x83\x01\x87a\x14rV[\x82\x81\x03` \x84\x01Ra\x14\xC4\x81\x87a\x14rV[`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16`@\x84\x01RPP``\x01R\x92\x91PPV[_a\x14\xF5a\x14\xEF\x83\x86a\x13\xBCV[\x84a\x13\xBCV[\x94\x93PPPPV\xFE`\xC0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x10G8\x03\x80a\x10G\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01CV[_a\09\x85\x82a\x02RV[P`\x01a\0F\x84\x82a\x02RV[P`\x01`\x01`\xA0\x1B\x03\x82\x16`\x80R`\xA0\x81\x90Ra\0b3a\0kV[PPPPa\x03\x0CV[`\x01`\x01`\xA0\x1B\x03\x16c\x8Bx\xC6\xD8\x19\x81\x90U\x80_\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x81\x80\xA3PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a\0\xC9W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\0\xE2Wa\0\xE2a\0\xA6V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x01\x10Wa\x01\x10a\0\xA6V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\x01'W__\xFD[\x81` \x85\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[____`\x80\x85\x87\x03\x12\x15a\x01VW__\xFD[\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01kW__\xFD[a\x01w\x87\x82\x88\x01a\0\xBAV[` \x87\x01Q\x90\x95P\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\x94W__\xFD[a\x01\xA0\x87\x82\x88\x01a\0\xBAV[`@\x87\x01Q\x90\x94P\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xBEW__\xFD[``\x95\x90\x95\x01Q\x93\x96\x92\x95PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01\xE2W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x02\0WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x02MW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x02+WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x02JW_\x81U`\x01\x01a\x027V[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02kWa\x02ka\0\xA6V[a\x02\x7F\x81a\x02y\x84Ta\x01\xCEV[\x84a\x02\x06V[` `\x1F\x82\x11`\x01\x81\x14a\x02\xB1W_\x83\x15a\x02\x9AWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x02JV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x02\xE0W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x02\xC0V[P\x84\x82\x10\x15a\x02\xFDW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80Q`\xA0Qa\r\x1Aa\x03-_9_a\x03n\x01R_a\x02G\x01Ra\r\x1A_\xF3\xFE`\x80`@R`\x046\x10a\x01<W_5`\xE0\x1C\x80cqP\x18\xA6\x11a\0\xB3W\x80c\xC0GM\x0B\x11a\0mW\x80c\xC0GM\x0B\x14a\x03]W\x80c\xD5\x05\xAC\xCF\x14a\x03\x90W\x80c\xDDb\xED>\x14a\x03\xAFW\x80c\xF0N(>\x14a\x03\xCEW\x80c\xF2\xFD\xE3\x8B\x14a\x03\xE1W\x80c\xFE\xE8\x1C\xF4\x14a\x03\xF4W__\xFD[\x80cqP\x18\xA6\x14a\x02\xBAW\x80c~\xCE\xBE\0\x14a\x02\xC2W\x80c\x8D\xA5\xCB[\x14a\x02\xF3W\x80c\x95\xD8\x9BA\x14a\x03\x0BW\x80c\x9D\xC2\x9F\xAC\x14a\x03\x1FW\x80c\xA9\x05\x9C\xBB\x14a\x03>W__\xFD[\x80c1<\xE5g\x11a\x01\x04W\x80c1<\xE5g\x14a\x01\xE8W\x80c6D\xE5\x15\x14a\x02\x03W\x80c@\xC1\x0F\x19\x14a\x02\x17W\x80cH\0\xD9\x7F\x14a\x026W\x80cT\xD1\xF1=\x14a\x02\x81W\x80cp\xA0\x821\x14a\x02\x89W__\xFD[\x80c\x06\xFD\xDE\x03\x14a\x01@W\x80c\t^\xA7\xB3\x14a\x01jW\x80c\x18\x16\r\xDD\x14a\x01\x99W\x80c#\xB8r\xDD\x14a\x01\xBFW\x80c%i)b\x14a\x01\xDEW[__\xFD[4\x80\x15a\x01KW__\xFD[Pa\x01Ta\x04%V[`@Qa\x01a\x91\x90a\x0B\x1CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01uW__\xFD[Pa\x01\x89a\x01\x846`\x04a\x0BlV[a\x04\xB4V[`@Q\x90\x15\x15\x81R` \x01a\x01aV[4\x80\x15a\x01\xA4W__\xFD[Ph\x054\\\xDFw\xEBh\xF4LT[`@Q\x90\x81R` \x01a\x01aV[4\x80\x15a\x01\xCAW__\xFD[Pa\x01\x89a\x01\xD96`\x04a\x0B\x94V[a\x054V[a\x01\xE6a\x05\xF0V[\0[4\x80\x15a\x01\xF3W__\xFD[P`@Q`\x12\x81R` \x01a\x01aV[4\x80\x15a\x02\x0EW__\xFD[Pa\x01\xB1a\x06=V[4\x80\x15a\x02\"W__\xFD[Pa\x01\xE6a\x0216`\x04a\x0BlV[a\x06\xB9V[4\x80\x15a\x02AW__\xFD[Pa\x02i\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01aV[a\x01\xE6a\x06\xCFV[4\x80\x15a\x02\x94W__\xFD[Pa\x01\xB1a\x02\xA36`\x04a\x0B\xCEV[c\x87\xA2\x11\xA2`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[a\x01\xE6a\x07\x08V[4\x80\x15a\x02\xCDW__\xFD[Pa\x01\xB1a\x02\xDC6`\x04a\x0B\xCEV[c87u\x08`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[4\x80\x15a\x02\xFEW__\xFD[Pc\x8Bx\xC6\xD8\x19Ta\x02iV[4\x80\x15a\x03\x16W__\xFD[Pa\x01Ta\x07\x1BV[4\x80\x15a\x03*W__\xFD[Pa\x01\xE6a\x0396`\x04a\x0BlV[a\x07*V[4\x80\x15a\x03IW__\xFD[Pa\x01\x89a\x03X6`\x04a\x0BlV[a\x07<V[4\x80\x15a\x03hW__\xFD[Pa\x01\xB1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\x9BW__\xFD[Pa\x01\xE6a\x03\xAA6`\x04a\x0B\xEEV[a\x07\xA0V[4\x80\x15a\x03\xBAW__\xFD[Pa\x01\xB1a\x03\xC96`\x04a\x0C[V[a\tTV[a\x01\xE6a\x03\xDC6`\x04a\x0B\xCEV[a\t\x98V[a\x01\xE6a\x03\xEF6`\x04a\x0B\xCEV[a\t\xD5V[4\x80\x15a\x03\xFFW__\xFD[Pa\x01\xB1a\x04\x0E6`\x04a\x0B\xCEV[c8\x9Au\xE1`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[``_\x80Ta\x043\x90a\x0C\x8CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04_\x90a\x0C\x8CV[\x80\x15a\x04\xAAW\x80`\x1F\x10a\x04\x81Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xAAV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x8DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[_`\x01`\x01`\xA0\x1B\x03\x83\x16n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x18\x82\x19\x15\x17a\x04\xE5Wc?hS\x9A_R`\x04`\x1C\xFD[\x82` Rc\x7F^\x9F `\x0CR3_R\x81`4`\x0C U\x81_R`,Q``\x1C3\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` _\xA3P`\x01[\x92\x91PPV[_\x83``\x1Bn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA33\x14a\x05\x89W3` Rc\x7F^\x9F \x81\x17`\x0CR`4`\x0C \x80T\x80\x19\x15a\x05\x86W\x80\x85\x11\x15a\x05\x80Wc\x13\xBE%+_R`\x04`\x1C\xFD[\x84\x81\x03\x82U[PP[c\x87\xA2\x11\xA2\x81\x17`\x0CR` `\x0C \x80T\x80\x85\x11\x15a\x05\xAFWc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x84\x81\x03\x82UPP\x83_R` `\x0C \x83\x81T\x01\x81UP\x82` R`\x0CQ``\x1C\x81``\x1C_Q` a\x0C\xC5_9_Q\x90_R` \x80\xA3PP`\x01\x93\x92PPPV[_b\x02\xA3\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x01\x90Pc8\x9Au\xE1`\x0CR3_R\x80` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D__\xA2PV[_\x80a\x06Ga\x04%V[\x80Q\x90` \x01 \x90P`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R` \x81\x01\x92\x90\x92R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6\x90\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 \x91\x90PV[a\x06\xC1a\t\xFBV[a\x06\xCB\x82\x82a\n\x15V[PPV[c8\x9Au\xE1`\x0CR3_R_` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92__\xA2V[a\x07\x10a\t\xFBV[a\x07\x19_a\n~V[V[```\x01\x80Ta\x043\x90a\x0C\x8CV[a\x072a\t\xFBV[a\x06\xCB\x82\x82a\n\xBBV[_c\x87\xA2\x11\xA2`\x0CR3_R` `\x0C \x80T\x80\x84\x11\x15a\x07dWc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x83\x81\x03\x82UPP\x82_R` `\x0C \x82\x81T\x01\x81UP\x81` R`\x0CQ``\x1C3_Q` a\x0C\xC5_9_Q\x90_R` \x80\xA3P`\x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x86\x16n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x18\x85\x19\x15\x17a\x07\xD0Wc?hS\x9A_R`\x04`\x1C\xFD[_a\x07\xD9a\x04%V[\x80Q\x90` \x01 \x90P\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6B\x86\x10\x15a\x08\x18Wc\x1A\x15\xA3\xCC_R`\x04`\x1C\xFD[`@Q\x89``\x1B``\x1C\x99P\x88``\x1B``\x1C\x98Pe87u\x08\x19\x01`\x0ER\x89_R` `\x0C \x80T\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x83R\x84` \x84\x01R\x83`@\x84\x01RF``\x84\x01R0`\x80\x84\x01R`\xA0\x83 `.R\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x83R\x8B` \x84\x01R\x8A`@\x84\x01R\x89``\x84\x01R\x80`\x80\x84\x01R\x88`\xA0\x84\x01R`\xC0\x83 `NR`B`, _R\x87`\xFF\x16` R\x86`@R\x85``R` \x80`\x80_`\x01Z\xFA\x8C=Q\x14a\t\0Wc\xDD\xAF\xBA\xEF_R`\x04`\x1C\xFD[\x01\x90Uc\x03\xFA\xF4\xF9`\xA5\x1B\x89\x17`@R`4`, \x88\x90U\x88\x8A\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` ``\x85\x01\xA3`@RPP_``RPPPPPPPV[_n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA2\x19`\x01`\x01`\xA0\x1B\x03\x83\x16\x01a\t}WP_\x19a\x05.V[P` Rc\x7F^\x9F `\x0C\x90\x81R_\x91\x90\x91R`4\x90 T\x90V[a\t\xA0a\t\xFBV[c8\x9Au\xE1`\x0CR\x80_R` `\x0C \x80TB\x11\x15a\t\xC6Wco^\x88\x18_R`\x04`\x1C\xFD[_\x90Ua\t\xD2\x81a\n~V[PV[a\t\xDDa\t\xFBV[\x80``\x1Ba\t\xF2WctH\xFB\xAE_R`\x04`\x1C\xFD[a\t\xD2\x81a\n~V[c\x8Bx\xC6\xD8\x19T3\x14a\x07\x19Wc\x82\xB4)\0_R`\x04`\x1C\xFD[h\x054\\\xDFw\xEBh\xF4LT\x81\x81\x01\x81\x81\x10\x15a\n8Wc\xE5\xCF\xE9W_R`\x04`\x1C\xFD[\x80h\x054\\\xDFw\xEBh\xF4LUPPc\x87\xA2\x11\xA2`\x0CR\x81_R` `\x0C \x81\x81T\x01\x81UP\x80` R`\x0CQ``\x1C__Q` a\x0C\xC5_9_Q\x90_R` \x80\xA3PPV[c\x8Bx\xC6\xD8\x19\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3UV[c\x87\xA2\x11\xA2`\x0CR\x81_R` `\x0C \x80T\x80\x83\x11\x15a\n\xE2Wc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x82\x90\x03\x90Uh\x054\\\xDFw\xEBh\xF4L\x80T\x82\x90\x03\x90U_\x81\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16_Q` a\x0C\xC5_9_Q\x90_R` \x83\xA3PPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0BgW__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x0B}W__\xFD[a\x0B\x86\x83a\x0BQV[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a\x0B\xA6W__\xFD[a\x0B\xAF\x84a\x0BQV[\x92Pa\x0B\xBD` \x85\x01a\x0BQV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_` \x82\x84\x03\x12\x15a\x0B\xDEW__\xFD[a\x0B\xE7\x82a\x0BQV[\x93\x92PPPV[_______`\xE0\x88\x8A\x03\x12\x15a\x0C\x04W__\xFD[a\x0C\r\x88a\x0BQV[\x96Pa\x0C\x1B` \x89\x01a\x0BQV[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015`\xFF\x81\x16\x81\x14a\x0C>W__\xFD[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x0ClW__\xFD[a\x0Cu\x83a\x0BQV[\x91Pa\x0C\x83` \x84\x01a\x0BQV[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0C\xA0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0C\xBEWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \xF2U\x8A\xD4F\x0C\xA0\x0E\xBE\xBA\xB8\xD0#r\xDDk\x02\x7F\x19\xDE{\xD9\xD72\xADt\xC5\xCF$c_EdsolcC\0\x08\x1E\x003\xA2dipfsX\"\x12 \x91\x9D\xE7\xB4Y\r\xF5k\xC1\x1A\xEB0O\xA2\x9D{D\x14_5\x084\x1D\xC0\xA2\x0C\x84\xEE\xAB5\x8D\x15dsolcC\0\x08\x1E\x003`\x80`@R4\x80\x15`\x0EW__\xFD[P`\x163`\x1AV[`UV[`\x01`\x01`\xA0\x1B\x03\x16c\x8Bx\xC6\xD8\x19\x81\x90U\x80_\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x81\x80\xA3PV[a\x04\xA8\x80a\0b_9_\xF3\xFE`\x80`@R`\x046\x10a\0\x9AW_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0bW\x80c\x8D\xA5\xCB[\x14a\x01\x0BW\x80c\xD4\xB0o\xB8\x14a\x016W\x80c\xF0N(>\x14a\x01dW\x80c\xF2\xFD\xE3\x8B\x14a\x01wW\x80c\xF66\xE0\xF6\x14a\x01\x8AW\x80c\xFE\xE8\x1C\xF4\x14a\x01\xB8W__\xFD[\x80c\x03\xE8\xD6R\x14a\0\x9EW\x80c%i)b\x14a\0\xD2W\x80c/\xDCl\xDE\x14a\0\xDCW\x80cT\xD1\xF1=\x14a\0\xFBW\x80cqP\x18\xA6\x14a\x01\x03W[__\xFD[4\x80\x15a\0\xA9W__\xFD[Pa\0\xBDa\0\xB86`\x04a\x03\xFCV[a\x01\xF7V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xDAa\x02:V[\0[4\x80\x15a\0\xE7W__\xFD[Pa\0\xDAa\0\xF66`\x04a\x04\x13V[a\x02\x87V[a\0\xDAa\x02\xF6V[a\0\xDAa\x03/V[4\x80\x15a\x01\x16W__\xFD[Pc\x8Bx\xC6\xD8\x19T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xC9V[4\x80\x15a\x01AW__\xFD[Pa\0\xBDa\x01P6`\x04a\x03\xFCV[_` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[a\0\xDAa\x01r6`\x04a\x04EV[a\x03BV[a\0\xDAa\x01\x856`\x04a\x04EV[a\x03\x7FV[4\x80\x15a\x01\x95W__\xFD[Pa\0\xBDa\x01\xA46`\x04a\x03\xFCV[`\x01` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\x01\xC3W__\xFD[Pa\x01\xE9a\x01\xD26`\x04a\x04EV[c8\x9Au\xE1`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[`@Q\x90\x81R` \x01a\0\xC9V[_\x81\x81R`\x01` R`@\x81 T`\xFF\x16a\x02%W`@QcJ\xECZ\xC7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x90\x81R` \x81\x90R`@\x90 T`\xFF\x16\x90V[_b\x02\xA3\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x01\x90Pc8\x9Au\xE1`\x0CR3_R\x80` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D__\xA2PV[a\x02\x8Fa\x03\xA5V[_\x82\x81R` \x81\x81R`@\x80\x83 \x80T\x85\x15\x15`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x01\x80\x85R\x94\x83\x90 \x80T\x90\x91\x16\x90\x94\x17\x90\x93UQ\x91\x82R\x83\x91\x7FW\xC3\x9A$\x80\xAC\xCB@wR/\"S\xA3\x91Z\xFD\xC6~\x1C\xA5\x88:W\x95~\xD6\x1F/\x04\xD0#\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[c8\x9Au\xE1`\x0CR3_R_` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92__\xA2V[a\x037a\x03\xA5V[a\x03@_a\x03\xBFV[V[a\x03Ja\x03\xA5V[c8\x9Au\xE1`\x0CR\x80_R` `\x0C \x80TB\x11\x15a\x03pWco^\x88\x18_R`\x04`\x1C\xFD[_\x90Ua\x03|\x81a\x03\xBFV[PV[a\x03\x87a\x03\xA5V[\x80``\x1Ba\x03\x9CWctH\xFB\xAE_R`\x04`\x1C\xFD[a\x03|\x81a\x03\xBFV[c\x8Bx\xC6\xD8\x19T3\x14a\x03@Wc\x82\xB4)\0_R`\x04`\x1C\xFD[c\x8Bx\xC6\xD8\x19\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3UV[_` \x82\x84\x03\x12\x15a\x04\x0CW__\xFD[P5\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x04$W__\xFD[\x825\x91P` \x83\x015\x80\x15\x15\x81\x14a\x04:W__\xFD[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\x04UW__\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04kW__\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 u\x8AD\xBA\x05\x89\xAA\x1E\x9C\\QX\xFFP/<\xBBP\x16,\xDEl\x15\xD8\xD5\x97\xE8:g\x06\xCB\x02dsolcC\0\x08\x1E\x003`\xA0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x0E\xD48\x03\x80a\x0E\xD4\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01\tV[_a\09\x84\x82a\x02\nV[P`\x01a\0F\x83\x82a\x02\nV[P`\x02\x80T`\xFF\x19\x16`\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UP\x80Q` \x90\x91\x01 `\x80Ra\x02\xC4V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a\0\x8FW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\0\xA8Wa\0\xA8a\0lV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\0\xD6Wa\0\xD6a\0lV[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\0\xEDW__\xFD[\x81` \x85\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[___``\x84\x86\x03\x12\x15a\x01\x1BW__\xFD[\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x010W__\xFD[a\x01<\x86\x82\x87\x01a\0\x80V[` \x86\x01Q\x90\x94P\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01YW__\xFD[a\x01e\x86\x82\x87\x01a\0\x80V[\x92PP`@\x84\x01Q`\xFF\x81\x16\x81\x14a\x01{W__\xFD[\x80\x91PP\x92P\x92P\x92V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01\x9AW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x01\xB8WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x02\x05W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x01\xE3WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x02\x02W_\x81U`\x01\x01a\x01\xEFV[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02#Wa\x02#a\0lV[a\x027\x81a\x021\x84Ta\x01\x86V[\x84a\x01\xBEV[` `\x1F\x82\x11`\x01\x81\x14a\x02iW_\x83\x15a\x02RWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x02\x02V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x02\x98W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x02xV[P\x84\x82\x10\x15a\x02\xB5W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80Qa\x0B\xF1a\x02\xE3_9_\x81\x81a\x03\x99\x01Ra\x04\xDB\x01Ra\x0B\xF1_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xFBW_5`\xE0\x1C\x80c~\xCE\xBE\0\x11a\0\x93W\x80c\xD3\x0E\xD3\xB3\x11a\0cW\x80c\xD3\x0E\xD3\xB3\x14a\x02\x17W\x80c\xD5\x05\xAC\xCF\x14a\x02*W\x80c\xDDb\xED>\x14a\x02=W\x80c\xF8=\x17\x91\x14a\x02PW__\xFD[\x80c~\xCE\xBE\0\x14a\x01\xC4W\x80c\x95\xD8\x9BA\x14a\x01\xE9W\x80c\x9D\xC2\x9F\xAC\x14a\x01\xF1W\x80c\xA9\x05\x9C\xBB\x14a\x02\x04W__\xFD[\x80c1<\xE5g\x11a\0\xCEW\x80c1<\xE5g\x14a\x01mW\x80c6D\xE5\x15\x14a\x01\x82W\x80c@\xC1\x0F\x19\x14a\x01\x8AW\x80cp\xA0\x821\x14a\x01\x9FW__\xFD[\x80c\x06\xFD\xDE\x03\x14a\0\xFFW\x80c\t^\xA7\xB3\x14a\x01\x1DW\x80c\x18\x16\r\xDD\x14a\x01@W\x80c#\xB8r\xDD\x14a\x01ZW[__\xFD[a\x01\x07a\x02cV[`@Qa\x01\x14\x91\x90a\t\xFFV[`@Q\x80\x91\x03\x90\xF3[a\x010a\x01+6`\x04a\nJV[a\x02\xF2V[`@Q\x90\x15\x15\x81R` \x01a\x01\x14V[h\x054\\\xDFw\xEBh\xF4LT[`@Q\x90\x81R` \x01a\x01\x14V[a\x010a\x01h6`\x04a\nrV[a\x03rV[`\x02T`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\x14V[a\x01La\x03\x96V[a\x01\x9Da\x01\x986`\x04a\nJV[a\x048V[\0[a\x01La\x01\xAD6`\x04a\n\xACV[c\x87\xA2\x11\xA2`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[a\x01La\x01\xD26`\x04a\n\xACV[c87u\x08`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[a\x01\x07a\x04NV[a\x01\x9Da\x01\xFF6`\x04a\nJV[a\x04]V[a\x010a\x02\x126`\x04a\nJV[a\x04oV[a\x01\x9Da\x02%6`\x04a\nrV[a\x04\x89V[a\x01\x9Da\x0286`\x04a\n\xC5V[a\x04\xA9V[a\x01La\x02K6`\x04a\x0B2V[a\x06\x83V[a\x01\x9Da\x02^6`\x04a\nrV[a\x06\xC7V[``_\x80Ta\x02q\x90a\x0BcV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\x9D\x90a\x0BcV[\x80\x15a\x02\xE8W\x80`\x1F\x10a\x02\xBFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xE8V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xCBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[_`\x01`\x01`\xA0\x1B\x03\x83\x16n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x18\x82\x19\x15\x17a\x03#Wc?hS\x9A_R`\x04`\x1C\xFD[\x82` Rc\x7F^\x9F `\x0CR3_R\x81`4`\x0C U\x81_R`,Q``\x1C3\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` _\xA3P`\x01[\x92\x91PPV[_a\x03\x8Ea\x03\x7F\x85a\x06\xE2V[a\x03\x88\x85a\x06\xE2V[\x84a\x07\x03V[\x94\x93PPPPV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80a\x03\xCFWa\x03\xC5a\x02cV[\x80Q\x90` \x01 \x90P[`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R` \x81\x01\x92\x90\x92R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6\x90\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 \x91\x90PV[a\x04Ja\x04D\x83a\x06\xE2V[\x82a\x07\xBFV[PPV[```\x01\x80Ta\x02q\x90a\x0BcV[a\x04Ja\x04i\x83a\x06\xE2V[\x82a\x08(V[_a\x04\x82a\x04|\x84a\x06\xE2V[\x83a\x08\x89V[\x93\x92PPPV[a\x04\xA4a\x04\x95\x84a\x06\xE2V[a\x04\x9E\x84a\x06\xE2V[\x83a\x08\xEDV[PPPV[`\x01`\x01`\xA0\x1B\x03\x86\x16n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x18\x85\x19\x15\x17a\x04\xD9Wc?hS\x9A_R`\x04`\x1C\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80a\x05\x11Wa\x05\x07a\x02cV[\x80Q\x90` \x01 \x90P[\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6B\x86\x10\x15a\x05GWc\x1A\x15\xA3\xCC_R`\x04`\x1C\xFD[`@Q\x89``\x1B``\x1C\x99P\x88``\x1B``\x1C\x98Pe87u\x08\x19\x01`\x0ER\x89_R` `\x0C \x80T\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x83R\x84` \x84\x01R\x83`@\x84\x01RF``\x84\x01R0`\x80\x84\x01R`\xA0\x83 `.R\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x83R\x8B` \x84\x01R\x8A`@\x84\x01R\x89``\x84\x01R\x80`\x80\x84\x01R\x88`\xA0\x84\x01R`\xC0\x83 `NR`B`, _R\x87`\xFF\x16` R\x86`@R\x85``R` \x80`\x80_`\x01Z\xFA\x8C=Q\x14a\x06/Wc\xDD\xAF\xBA\xEF_R`\x04`\x1C\xFD[\x01\x90Uc\x03\xFA\xF4\xF9`\xA5\x1B\x89\x17`@R`4`, \x88\x90U\x88\x8A\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` ``\x85\x01\xA3`@RPP_``RPPPPPPPV[_n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA2\x19`\x01`\x01`\xA0\x1B\x03\x83\x16\x01a\x06\xACWP_\x19a\x03lV[P` Rc\x7F^\x9F `\x0C\x90\x81R_\x91\x90\x91R`4\x90 T\x90V[a\x04\xA4a\x06\xD3\x84a\x06\xE2V[a\x06\xDC\x84a\x06\xE2V[\x83a\tQV[_`\x01`\x01`\xA0\x1B\x03\x82\x16\x80`\xA0a\x06\xF9\x82a\t\xB7V[\x90\x1B\x18\x93\x92PPPV[_\x83``\x1Bn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA33\x14a\x07XW3` Rc\x7F^\x9F \x81\x17`\x0CR`4`\x0C \x80T\x80\x19\x15a\x07UW\x80\x85\x11\x15a\x07OWc\x13\xBE%+_R`\x04`\x1C\xFD[\x84\x81\x03\x82U[PP[c\x87\xA2\x11\xA2\x81\x17`\x0CR` `\x0C \x80T\x80\x85\x11\x15a\x07~Wc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x84\x81\x03\x82UPP\x83_R` `\x0C \x83\x81T\x01\x81UP\x82` R`\x0CQ``\x1C\x81``\x1C_Q` a\x0B\x9C_9_Q\x90_R` \x80\xA3PP`\x01\x93\x92PPPV[h\x054\\\xDFw\xEBh\xF4LT\x81\x81\x01\x81\x81\x10\x15a\x07\xE2Wc\xE5\xCF\xE9W_R`\x04`\x1C\xFD[\x80h\x054\\\xDFw\xEBh\xF4LUPPc\x87\xA2\x11\xA2`\x0CR\x81_R` `\x0C \x81\x81T\x01\x81UP\x80` R`\x0CQ``\x1C__Q` a\x0B\x9C_9_Q\x90_R` \x80\xA3PPV[c\x87\xA2\x11\xA2`\x0CR\x81_R` `\x0C \x80T\x80\x83\x11\x15a\x08OWc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x82\x90\x03\x90Uh\x054\\\xDFw\xEBh\xF4L\x80T\x82\x90\x03\x90U_\x81\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16_Q` a\x0B\x9C_9_Q\x90_R` \x83\xA3PPV[_c\x87\xA2\x11\xA2`\x0CR3_R` `\x0C \x80T\x80\x84\x11\x15a\x08\xB1Wc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x83\x81\x03\x82UPP\x82_R` `\x0C \x82\x81T\x01\x81UP\x81` R`\x0CQ``\x1C3_Q` a\x0B\x9C_9_Q\x90_R` \x80\xA3P`\x01\x92\x91PPV[n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA2\x19`\x01`\x01`\xA0\x1B\x03\x83\x16\x01a\t\x12WPPPV[\x81` Rc\x7F^\x9F `\x0CR\x82_R`4`\x0C \x80T\x80\x19\x15a\tJW\x80\x83\x11\x15a\tDWc\x13\xBE%+_R`\x04`\x1C\xFD[\x82\x81\x03\x82U[PPPPPV[\x82``\x1Bc\x87\xA2\x11\xA2\x81\x17`\x0CR` `\x0C \x80T\x80\x84\x11\x15a\t{Wc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x83\x81\x03\x82UPP\x82_R` `\x0C \x82\x81T\x01\x81UP\x81` R`\x0CQ``\x1C\x81``\x1C_Q` a\x0B\x9C_9_Q\x90_R` \x80\xA3PPPPV[`@Q6_\x8276\x81 ` R`\x10Q\x82\x18`\x10R`\x88_ \x90P\x80`\x10R`\xBC\x19p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Q\x82\t`\x80\x1C`\x07\x16a\t\xFAWP_[\x91\x90PV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\xFAW__\xFD[__`@\x83\x85\x03\x12\x15a\n[W__\xFD[a\nd\x83a\n4V[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a\n\x84W__\xFD[a\n\x8D\x84a\n4V[\x92Pa\n\x9B` \x85\x01a\n4V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_` \x82\x84\x03\x12\x15a\n\xBCW__\xFD[a\x04\x82\x82a\n4V[_______`\xE0\x88\x8A\x03\x12\x15a\n\xDBW__\xFD[a\n\xE4\x88a\n4V[\x96Pa\n\xF2` \x89\x01a\n4V[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015`\xFF\x81\x16\x81\x14a\x0B\x15W__\xFD[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x0BCW__\xFD[a\x0BL\x83a\n4V[\x91Pa\x0BZ` \x84\x01a\n4V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0BwW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0B\x95WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \x0CE\xAC\xC3\xC6\xA9\xD1X\x90\xEB\xBA\x87 \xCB c\xE2|m\xAA\xB30y8k\xB3\xDC\xAE\xCF\xA6L\xFBdsolcC\0\x08\x1E\x003\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-YES_WETH/YES_USDC pair should exist\"\x9AQ'\xDB\xA5I\xED\xF6\xC4Y\x17\xCD\xAC\xDE\xEB\xDC\xB0\xF5z\xD3=-\n\xA6\xE8\xD1*!i)\x02\xA2dipfsX\"\x12 @\xAF\xF2\xE2\x91Z5\x19}\x1F\xA2\t\xB4\x93Bq\xC2\xC2V\xF4b\x15\xFB\x8E-\x03\xE2\x83\xFAv\xE8\x8DdsolcC\0\x08\x1E\x003",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log(string)` and selector `0x41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f50`.
```solidity
event log(string);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                65u8, 48u8, 79u8, 172u8, 217u8, 50u8, 61u8, 117u8, 177u8, 27u8, 205u8,
                214u8, 9u8, 203u8, 56u8, 239u8, 255u8, 253u8, 176u8, 87u8, 16u8, 247u8,
                202u8, 240u8, 233u8, 177u8, 108u8, 109u8, 157u8, 112u8, 159u8, 80u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_address(address)` and selector `0x7ae74c527414ae135fd97047b12921a5ec3911b804197855d67e25c7b75ee6f3`.
```solidity
event log_address(address);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_address {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_address {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_address(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                122u8, 231u8, 76u8, 82u8, 116u8, 20u8, 174u8, 19u8, 95u8, 217u8, 112u8,
                71u8, 177u8, 41u8, 33u8, 165u8, 236u8, 57u8, 17u8, 184u8, 4u8, 25u8,
                120u8, 85u8, 214u8, 126u8, 37u8, 199u8, 183u8, 94u8, 230u8, 243u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_address {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_address> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_address) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_array(uint256[])` and selector `0xfb102865d50addddf69da9b5aa1bced66c80cf869a5c8d0471a467e18ce9cab1`.
```solidity
event log_array(uint256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_0 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_array_0 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                251u8, 16u8, 40u8, 101u8, 213u8, 10u8, 221u8, 221u8, 246u8, 157u8, 169u8,
                181u8, 170u8, 27u8, 206u8, 214u8, 108u8, 128u8, 207u8, 134u8, 154u8,
                92u8, 141u8, 4u8, 113u8, 164u8, 103u8, 225u8, 140u8, 233u8, 202u8, 177u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_array_0 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_0> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_0) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_array(int256[])` and selector `0x890a82679b470f2bd82816ed9b161f97d8b967f37fa3647c21d5bf39749e2dd5`.
```solidity
event log_array(int256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_1 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::I256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_array_1 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Int<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(int256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                137u8, 10u8, 130u8, 103u8, 155u8, 71u8, 15u8, 43u8, 216u8, 40u8, 22u8,
                237u8, 155u8, 22u8, 31u8, 151u8, 216u8, 185u8, 103u8, 243u8, 127u8,
                163u8, 100u8, 124u8, 33u8, 213u8, 191u8, 57u8, 116u8, 158u8, 45u8, 213u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Int<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_array_1 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_1> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_1) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_array(address[])` and selector `0x40e1840f5769073d61bd01372d9b75baa9842d5629a0c99ff103be1178a8e9e2`.
```solidity
event log_array(address[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_2 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_array_2 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                64u8, 225u8, 132u8, 15u8, 87u8, 105u8, 7u8, 61u8, 97u8, 189u8, 1u8, 55u8,
                45u8, 155u8, 117u8, 186u8, 169u8, 132u8, 45u8, 86u8, 41u8, 160u8, 201u8,
                159u8, 241u8, 3u8, 190u8, 17u8, 120u8, 168u8, 233u8, 226u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_array_2 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_2> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_2) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_bytes(bytes)` and selector `0x23b62ad0584d24a75f0bf3560391ef5659ec6db1269c56e11aa241d637f19b20`.
```solidity
event log_bytes(bytes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_bytes {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_bytes {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_bytes(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                35u8, 182u8, 42u8, 208u8, 88u8, 77u8, 36u8, 167u8, 95u8, 11u8, 243u8,
                86u8, 3u8, 145u8, 239u8, 86u8, 89u8, 236u8, 109u8, 177u8, 38u8, 156u8,
                86u8, 225u8, 26u8, 162u8, 65u8, 214u8, 55u8, 241u8, 155u8, 32u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_bytes {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_bytes> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_bytes) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_bytes32(bytes32)` and selector `0xe81699b85113eea1c73e10588b2b035e55893369632173afd43feb192fac64e3`.
```solidity
event log_bytes32(bytes32);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_bytes32 {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_bytes32 {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_bytes32(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                232u8, 22u8, 153u8, 184u8, 81u8, 19u8, 238u8, 161u8, 199u8, 62u8, 16u8,
                88u8, 139u8, 43u8, 3u8, 94u8, 85u8, 137u8, 51u8, 105u8, 99u8, 33u8,
                115u8, 175u8, 212u8, 63u8, 235u8, 25u8, 47u8, 172u8, 100u8, 227u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_bytes32 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_bytes32> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_bytes32) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_int(int256)` and selector `0x0eb5d52624c8d28ada9fc55a8c502ed5aa3fbe2fb6e91b71b5f376882b1d2fb8`.
```solidity
event log_int(int256);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_int {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::I256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_int {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Int<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_int(int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                14u8, 181u8, 213u8, 38u8, 36u8, 200u8, 210u8, 138u8, 218u8, 159u8, 197u8,
                90u8, 140u8, 80u8, 46u8, 213u8, 170u8, 63u8, 190u8, 47u8, 182u8, 233u8,
                27u8, 113u8, 181u8, 243u8, 118u8, 136u8, 43u8, 29u8, 47u8, 184u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_address(string,address)` and selector `0x9c4e8541ca8f0dc1c413f9108f66d82d3cecb1bddbce437a61caa3175c4cc96f`.
```solidity
event log_named_address(string key, address val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_address {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_address {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_address(string,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                156u8, 78u8, 133u8, 65u8, 202u8, 143u8, 13u8, 193u8, 196u8, 19u8, 249u8,
                16u8, 143u8, 102u8, 216u8, 45u8, 60u8, 236u8, 177u8, 189u8, 219u8, 206u8,
                67u8, 122u8, 97u8, 202u8, 163u8, 23u8, 92u8, 76u8, 201u8, 111u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.val,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_address {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_address> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_address) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_array(string,uint256[])` and selector `0x00aaa39c9ffb5f567a4534380c737075702e1f7f14107fc95328e3b56c0325fb`.
```solidity
event log_named_array(string key, uint256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_0 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_array_0 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                0u8, 170u8, 163u8, 156u8, 159u8, 251u8, 95u8, 86u8, 122u8, 69u8, 52u8,
                56u8, 12u8, 115u8, 112u8, 117u8, 112u8, 46u8, 31u8, 127u8, 20u8, 16u8,
                127u8, 201u8, 83u8, 40u8, 227u8, 181u8, 108u8, 3u8, 37u8, 251u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_array_0 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_0> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_0) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_array(string,int256[])` and selector `0xa73eda09662f46dde729be4611385ff34fe6c44fbbc6f7e17b042b59a3445b57`.
```solidity
event log_named_array(string key, int256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_1 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::I256,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_array_1 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Int<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,int256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                167u8, 62u8, 218u8, 9u8, 102u8, 47u8, 70u8, 221u8, 231u8, 41u8, 190u8,
                70u8, 17u8, 56u8, 95u8, 243u8, 79u8, 230u8, 196u8, 79u8, 187u8, 198u8,
                247u8, 225u8, 123u8, 4u8, 43u8, 89u8, 163u8, 68u8, 91u8, 87u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Int<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_array_1 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_1> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_1) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_array(string,address[])` and selector `0x3bcfb2ae2e8d132dd1fce7cf278a9a19756a9fceabe470df3bdabb4bc577d1bd`.
```solidity
event log_named_array(string key, address[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_2 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_array_2 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                59u8, 207u8, 178u8, 174u8, 46u8, 141u8, 19u8, 45u8, 209u8, 252u8, 231u8,
                207u8, 39u8, 138u8, 154u8, 25u8, 117u8, 106u8, 159u8, 206u8, 171u8,
                228u8, 112u8, 223u8, 59u8, 218u8, 187u8, 75u8, 197u8, 119u8, 209u8, 189u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_array_2 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_2> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_2) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_bytes(string,bytes)` and selector `0xd26e16cad4548705e4c9e2d94f98ee91c289085ee425594fd5635fa2964ccf18`.
```solidity
event log_named_bytes(string key, bytes val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_bytes {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_bytes {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_bytes(string,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                210u8, 110u8, 22u8, 202u8, 212u8, 84u8, 135u8, 5u8, 228u8, 201u8, 226u8,
                217u8, 79u8, 152u8, 238u8, 145u8, 194u8, 137u8, 8u8, 94u8, 228u8, 37u8,
                89u8, 79u8, 213u8, 99u8, 95u8, 162u8, 150u8, 76u8, 207u8, 24u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.val,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_bytes {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_bytes> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_bytes) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_bytes32(string,bytes32)` and selector `0xafb795c9c61e4fe7468c386f925d7a5429ecad9c0495ddb8d38d690614d32f99`.
```solidity
event log_named_bytes32(string key, bytes32 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_bytes32 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_bytes32 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_bytes32(string,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                175u8, 183u8, 149u8, 201u8, 198u8, 30u8, 79u8, 231u8, 70u8, 140u8, 56u8,
                111u8, 146u8, 93u8, 122u8, 84u8, 41u8, 236u8, 173u8, 156u8, 4u8, 149u8,
                221u8, 184u8, 211u8, 141u8, 105u8, 6u8, 20u8, 211u8, 47u8, 153u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_bytes32 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_bytes32> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_bytes32) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_decimal_int(string,int256,uint256)` and selector `0x5da6ce9d51151ba10c09a559ef24d520b9dac5c5b8810ae8434e4d0d86411a95`.
```solidity
event log_named_decimal_int(string key, int256 val, uint256 decimals);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_decimal_int {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::I256,
        #[allow(missing_docs)]
        pub decimals: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_decimal_int {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Int<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_decimal_int(string,int256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                93u8, 166u8, 206u8, 157u8, 81u8, 21u8, 27u8, 161u8, 12u8, 9u8, 165u8,
                89u8, 239u8, 36u8, 213u8, 32u8, 185u8, 218u8, 197u8, 197u8, 184u8, 129u8,
                10u8, 232u8, 67u8, 78u8, 77u8, 13u8, 134u8, 65u8, 26u8, 149u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    key: data.0,
                    val: data.1,
                    decimals: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_decimal_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_decimal_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_decimal_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_decimal_uint(string,uint256,uint256)` and selector `0xeb8ba43ced7537421946bd43e828b8b2b8428927aa8f801c13d934bf11aca57b`.
```solidity
event log_named_decimal_uint(string key, uint256 val, uint256 decimals);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_decimal_uint {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub decimals: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_decimal_uint {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_decimal_uint(string,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                235u8, 139u8, 164u8, 60u8, 237u8, 117u8, 55u8, 66u8, 25u8, 70u8, 189u8,
                67u8, 232u8, 40u8, 184u8, 178u8, 184u8, 66u8, 137u8, 39u8, 170u8, 143u8,
                128u8, 28u8, 19u8, 217u8, 52u8, 191u8, 17u8, 172u8, 165u8, 123u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    key: data.0,
                    val: data.1,
                    decimals: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_decimal_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_decimal_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_decimal_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_int(string,int256)` and selector `0x2fe632779174374378442a8e978bccfbdcc1d6b2b0d81f7e8eb776ab2286f168`.
```solidity
event log_named_int(string key, int256 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_int {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::I256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_int {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Int<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_int(string,int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8, 230u8, 50u8, 119u8, 145u8, 116u8, 55u8, 67u8, 120u8, 68u8, 42u8,
                142u8, 151u8, 139u8, 204u8, 251u8, 220u8, 193u8, 214u8, 178u8, 176u8,
                216u8, 31u8, 126u8, 142u8, 183u8, 118u8, 171u8, 34u8, 134u8, 241u8, 104u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_string(string,string)` and selector `0x280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf3583`.
```solidity
event log_named_string(string key, string val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_string {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_string {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_string(string,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                40u8, 15u8, 68u8, 70u8, 178u8, 138u8, 19u8, 114u8, 65u8, 125u8, 218u8,
                101u8, 141u8, 48u8, 185u8, 91u8, 41u8, 146u8, 177u8, 42u8, 201u8, 199u8,
                243u8, 120u8, 83u8, 95u8, 41u8, 169u8, 122u8, 207u8, 53u8, 131u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.val,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_string {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_string> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_string) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_named_uint(string,uint256)` and selector `0xb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a8`.
```solidity
event log_named_uint(string key, uint256 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_uint {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_uint {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_uint(string,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                178u8, 222u8, 47u8, 190u8, 128u8, 26u8, 13u8, 246u8, 192u8, 203u8, 221u8,
                253u8, 68u8, 139u8, 163u8, 196u8, 29u8, 72u8, 160u8, 64u8, 202u8, 53u8,
                197u8, 108u8, 129u8, 150u8, 239u8, 15u8, 202u8, 231u8, 33u8, 168u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_named_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_string(string)` and selector `0x0b2e13ff20ac7b474198655583edf70dedd2c1dc980e329c4fbb2fc0748b796b`.
```solidity
event log_string(string);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_string {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_string {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_string(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                11u8, 46u8, 19u8, 255u8, 32u8, 172u8, 123u8, 71u8, 65u8, 152u8, 101u8,
                85u8, 131u8, 237u8, 247u8, 13u8, 237u8, 210u8, 193u8, 220u8, 152u8, 14u8,
                50u8, 156u8, 79u8, 187u8, 47u8, 192u8, 116u8, 139u8, 121u8, 107u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_string {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_string> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_string) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `log_uint(uint256)` and selector `0x2cab9790510fd8bdfbd2115288db33fec66691d476efc5427cfd4c0969301755`.
```solidity
event log_uint(uint256);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_uint {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_uint {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_uint(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                44u8, 171u8, 151u8, 144u8, 81u8, 15u8, 216u8, 189u8, 251u8, 210u8, 17u8,
                82u8, 136u8, 219u8, 51u8, 254u8, 198u8, 102u8, 145u8, 212u8, 118u8,
                239u8, 197u8, 66u8, 124u8, 253u8, 76u8, 9u8, 105u8, 48u8, 23u8, 85u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for log_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `logs(bytes)` and selector `0xe7950ede0394b9f2ce4a5a1bf5a7e1852411f7e6661b4308c913c4bfd11027e4`.
```solidity
event logs(bytes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct logs {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for logs {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "logs(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                231u8, 149u8, 14u8, 222u8, 3u8, 148u8, 185u8, 242u8, 206u8, 74u8, 90u8,
                27u8, 245u8, 167u8, 225u8, 133u8, 36u8, 17u8, 247u8, 230u8, 102u8, 27u8,
                67u8, 8u8, 201u8, 19u8, 196u8, 191u8, 209u8, 16u8, 39u8, 228u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for logs {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&logs> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &logs) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `IS_TEST()` and selector `0xfa7626d4`.
```solidity
function IS_TEST() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_TESTCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`IS_TEST()`](IS_TESTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_TESTReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<IS_TESTCall> for UnderlyingRustTuple<'_> {
                fn from(value: IS_TESTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for IS_TESTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<IS_TESTReturn> for UnderlyingRustTuple<'_> {
                fn from(value: IS_TESTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for IS_TESTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for IS_TESTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IS_TEST()";
            const SELECTOR: [u8; 4] = [250u8, 118u8, 38u8, 212u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: IS_TESTReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: IS_TESTReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `QUESTION_HASH()` and selector `0x5af7071a`.
```solidity
function QUESTION_HASH() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QUESTION_HASHCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`QUESTION_HASH()`](QUESTION_HASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QUESTION_HASHReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<QUESTION_HASHCall> for UnderlyingRustTuple<'_> {
                fn from(value: QUESTION_HASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for QUESTION_HASHCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<QUESTION_HASHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: QUESTION_HASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for QUESTION_HASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for QUESTION_HASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "QUESTION_HASH()";
            const SELECTOR: [u8; 4] = [90u8, 247u8, 7u8, 26u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: QUESTION_HASHReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: QUESTION_HASHReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `RESOLUTION_DEADLINE()` and selector `0xa4234b4e`.
```solidity
function RESOLUTION_DEADLINE() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RESOLUTION_DEADLINECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`RESOLUTION_DEADLINE()`](RESOLUTION_DEADLINECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RESOLUTION_DEADLINEReturn {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<RESOLUTION_DEADLINECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: RESOLUTION_DEADLINECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for RESOLUTION_DEADLINECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<RESOLUTION_DEADLINEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: RESOLUTION_DEADLINEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for RESOLUTION_DEADLINEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for RESOLUTION_DEADLINECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RESOLUTION_DEADLINE()";
            const SELECTOR: [u8; 4] = [164u8, 35u8, 75u8, 78u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: RESOLUTION_DEADLINEReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: RESOLUTION_DEADLINEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `UNISWAP_V2_FACTORY()` and selector `0x99d8fae3`.
```solidity
function UNISWAP_V2_FACTORY() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UNISWAP_V2_FACTORYCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`UNISWAP_V2_FACTORY()`](UNISWAP_V2_FACTORYCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UNISWAP_V2_FACTORYReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UNISWAP_V2_FACTORYCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: UNISWAP_V2_FACTORYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for UNISWAP_V2_FACTORYCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UNISWAP_V2_FACTORYReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: UNISWAP_V2_FACTORYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for UNISWAP_V2_FACTORYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for UNISWAP_V2_FACTORYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UNISWAP_V2_FACTORY()";
            const SELECTOR: [u8; 4] = [153u8, 216u8, 250u8, 227u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: UNISWAP_V2_FACTORYReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: UNISWAP_V2_FACTORYReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `alice()` and selector `0xfb47e3a2`.
```solidity
function alice() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct aliceCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`alice()`](aliceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct aliceReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<aliceCall> for UnderlyingRustTuple<'_> {
                fn from(value: aliceCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for aliceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<aliceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: aliceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for aliceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for aliceCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "alice()";
            const SELECTOR: [u8; 4] = [251u8, 71u8, 227u8, 162u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: aliceReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: aliceReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `bob()` and selector `0xc09cec77`.
```solidity
function bob() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bobCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`bob()`](bobCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bobReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<bobCall> for UnderlyingRustTuple<'_> {
                fn from(value: bobCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bobCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<bobReturn> for UnderlyingRustTuple<'_> {
                fn from(value: bobReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bobReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bobCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "bob()";
            const SELECTOR: [u8; 4] = [192u8, 156u8, 236u8, 119u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: bobReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: bobReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `excludeArtifacts()` and selector `0xb5508aa9`.
```solidity
function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeArtifactsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`excludeArtifacts()`](excludeArtifactsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeArtifactsReturn {
        #[allow(missing_docs)]
        pub excludedArtifacts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeArtifactsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeArtifactsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeArtifactsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeArtifactsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeArtifactsReturn) -> Self {
                    (value.excludedArtifacts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeArtifactsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedArtifacts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeArtifactsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::String,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeArtifacts()";
            const SELECTOR: [u8; 4] = [181u8, 80u8, 138u8, 169u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: excludeArtifactsReturn = r.into();
                        r.excludedArtifacts_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: excludeArtifactsReturn = r.into();
                        r.excludedArtifacts_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `excludeContracts()` and selector `0xe20c9f71`.
```solidity
function excludeContracts() external view returns (address[] memory excludedContracts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeContractsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`excludeContracts()`](excludeContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeContractsReturn {
        #[allow(missing_docs)]
        pub excludedContracts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeContractsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeContractsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeContractsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeContractsReturn) -> Self {
                    (value.excludedContracts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeContractsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedContracts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeContractsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeContracts()";
            const SELECTOR: [u8; 4] = [226u8, 12u8, 159u8, 113u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: excludeContractsReturn = r.into();
                        r.excludedContracts_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: excludeContractsReturn = r.into();
                        r.excludedContracts_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `excludeSelectors()` and selector `0xb0464fdc`.
```solidity
function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSelectorsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`excludeSelectors()`](excludeSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSelectorsReturn {
        #[allow(missing_docs)]
        pub excludedSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeSelectorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSelectorsReturn) -> Self {
                    (value.excludedSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeSelectors()";
            const SELECTOR: [u8; 4] = [176u8, 70u8, 79u8, 220u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StdInvariant::FuzzSelector,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: excludeSelectorsReturn = r.into();
                        r.excludedSelectors_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: excludeSelectorsReturn = r.into();
                        r.excludedSelectors_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `excludeSenders()` and selector `0x1ed7831c`.
```solidity
function excludeSenders() external view returns (address[] memory excludedSenders_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSendersCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`excludeSenders()`](excludeSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSendersReturn {
        #[allow(missing_docs)]
        pub excludedSenders_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeSendersCall> for UnderlyingRustTuple<'_> {
                fn from(value: excludeSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeSendersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSendersReturn) -> Self {
                    (value.excludedSenders_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { excludedSenders_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeSendersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeSenders()";
            const SELECTOR: [u8; 4] = [30u8, 215u8, 131u8, 28u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: excludeSendersReturn = r.into();
                        r.excludedSenders_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: excludeSendersReturn = r.into();
                        r.excludedSenders_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `failed()` and selector `0xba414fa6`.
```solidity
function failed() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct failedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`failed()`](failedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct failedReturn {
        #[allow(missing_docs)]
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<failedCall> for UnderlyingRustTuple<'_> {
                fn from(value: failedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for failedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<failedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: failedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for failedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for failedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "failed()";
            const SELECTOR: [u8; 4] = [186u8, 65u8, 79u8, 166u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: failedReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: failedReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `lpProvider()` and selector `0x0d235817`.
```solidity
function lpProvider() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lpProviderCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`lpProvider()`](lpProviderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lpProviderReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lpProviderCall> for UnderlyingRustTuple<'_> {
                fn from(value: lpProviderCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lpProviderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lpProviderReturn> for UnderlyingRustTuple<'_> {
                fn from(value: lpProviderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lpProviderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lpProviderCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lpProvider()";
            const SELECTOR: [u8; 4] = [13u8, 35u8, 88u8, 23u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: lpProviderReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: lpProviderReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `multiVerse()` and selector `0x8c809fbf`.
```solidity
function multiVerse() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct multiVerseCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`multiVerse()`](multiVerseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct multiVerseReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<multiVerseCall> for UnderlyingRustTuple<'_> {
                fn from(value: multiVerseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for multiVerseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<multiVerseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: multiVerseReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for multiVerseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for multiVerseCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "multiVerse()";
            const SELECTOR: [u8; 4] = [140u8, 128u8, 159u8, 191u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: multiVerseReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: multiVerseReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `oracle()` and selector `0x7dc0d1d0`.
```solidity
function oracle() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct oracleCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`oracle()`](oracleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct oracleReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<oracleCall> for UnderlyingRustTuple<'_> {
                fn from(value: oracleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for oracleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<oracleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: oracleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for oracleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for oracleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "oracle()";
            const SELECTOR: [u8; 4] = [125u8, 192u8, 209u8, 208u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: oracleReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: oracleReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setUp()` and selector `0x0a9254e4`.
```solidity
function setUp() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setUpCall;
    ///Container type for the return parameters of the [`setUp()`](setUpCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setUpReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setUpCall> for UnderlyingRustTuple<'_> {
                fn from(value: setUpCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setUpCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setUpReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setUpReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setUpReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setUpReturn {
            fn _tokenize(
                &self,
            ) -> <setUpCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setUpCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setUpReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setUp()";
            const SELECTOR: [u8; 4] = [10u8, 146u8, 84u8, 228u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setUpReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`.
```solidity
function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetArtifactSelectors()`](targetArtifactSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsReturn {
        #[allow(missing_docs)]
        pub targetedArtifactSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetArtifactSelectorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzArtifactSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetArtifactSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactSelectorsReturn) -> Self {
                    (value.targetedArtifactSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedArtifactSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetArtifactSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzArtifactSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetArtifactSelectors()";
            const SELECTOR: [u8; 4] = [102u8, 217u8, 169u8, 160u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StdInvariant::FuzzArtifactSelector,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: targetArtifactSelectorsReturn = r.into();
                        r.targetedArtifactSelectors_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: targetArtifactSelectorsReturn = r.into();
                        r.targetedArtifactSelectors_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetArtifacts()` and selector `0x85226c81`.
```solidity
function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetArtifacts()`](targetArtifactsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactsReturn {
        #[allow(missing_docs)]
        pub targetedArtifacts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetArtifactsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetArtifactsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetArtifactsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactsReturn) -> Self {
                    (value.targetedArtifacts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedArtifacts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetArtifactsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::String,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetArtifacts()";
            const SELECTOR: [u8; 4] = [133u8, 34u8, 108u8, 129u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: targetArtifactsReturn = r.into();
                        r.targetedArtifacts_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: targetArtifactsReturn = r.into();
                        r.targetedArtifacts_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetContracts()` and selector `0x3f7286f4`.
```solidity
function targetContracts() external view returns (address[] memory targetedContracts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetContractsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetContracts()`](targetContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetContractsReturn {
        #[allow(missing_docs)]
        pub targetedContracts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetContractsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetContractsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetContractsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetContractsReturn) -> Self {
                    (value.targetedContracts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetContractsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedContracts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetContractsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetContracts()";
            const SELECTOR: [u8; 4] = [63u8, 114u8, 134u8, 244u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: targetContractsReturn = r.into();
                        r.targetedContracts_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: targetContractsReturn = r.into();
                        r.targetedContracts_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetInterfaces()` and selector `0x2ade3880`.
```solidity
function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetInterfacesCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetInterfaces()`](targetInterfacesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetInterfacesReturn {
        #[allow(missing_docs)]
        pub targetedInterfaces_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetInterfacesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetInterfacesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetInterfacesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzInterface>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetInterfacesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetInterfacesReturn) -> Self {
                    (value.targetedInterfaces_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetInterfacesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedInterfaces_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetInterfacesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzInterface>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetInterfaces()";
            const SELECTOR: [u8; 4] = [42u8, 222u8, 56u8, 128u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StdInvariant::FuzzInterface,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: targetInterfacesReturn = r.into();
                        r.targetedInterfaces_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: targetInterfacesReturn = r.into();
                        r.targetedInterfaces_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetSelectors()` and selector `0x916a17c6`.
```solidity
function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSelectorsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetSelectors()`](targetSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSelectorsReturn {
        #[allow(missing_docs)]
        pub targetedSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetSelectorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetSelectorsReturn) -> Self {
                    (value.targetedSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetSelectors()";
            const SELECTOR: [u8; 4] = [145u8, 106u8, 23u8, 198u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StdInvariant::FuzzSelector,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: targetSelectorsReturn = r.into();
                        r.targetedSelectors_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: targetSelectorsReturn = r.into();
                        r.targetedSelectors_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `targetSenders()` and selector `0x3e5e3c23`.
```solidity
function targetSenders() external view returns (address[] memory targetedSenders_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSendersCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`targetSenders()`](targetSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSendersReturn {
        #[allow(missing_docs)]
        pub targetedSenders_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetSendersCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetSendersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetSendersReturn) -> Self {
                    (value.targetedSenders_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { targetedSenders_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetSendersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetSenders()";
            const SELECTOR: [u8; 4] = [62u8, 94u8, 60u8, 35u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: targetSendersReturn = r.into();
                        r.targetedSenders_
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: targetSendersReturn = r.into();
                        r.targetedSenders_
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testCrossAssetVerseTradingOnUniswapV2()` and selector `0x137da30c`.
```solidity
function testCrossAssetVerseTradingOnUniswapV2() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testCrossAssetVerseTradingOnUniswapV2Call;
    ///Container type for the return parameters of the [`testCrossAssetVerseTradingOnUniswapV2()`](testCrossAssetVerseTradingOnUniswapV2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testCrossAssetVerseTradingOnUniswapV2Return {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<testCrossAssetVerseTradingOnUniswapV2Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: testCrossAssetVerseTradingOnUniswapV2Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testCrossAssetVerseTradingOnUniswapV2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<testCrossAssetVerseTradingOnUniswapV2Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: testCrossAssetVerseTradingOnUniswapV2Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testCrossAssetVerseTradingOnUniswapV2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testCrossAssetVerseTradingOnUniswapV2Return {
            fn _tokenize(
                &self,
            ) -> <testCrossAssetVerseTradingOnUniswapV2Call as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testCrossAssetVerseTradingOnUniswapV2Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testCrossAssetVerseTradingOnUniswapV2Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testCrossAssetVerseTradingOnUniswapV2()";
            const SELECTOR: [u8; 4] = [19u8, 125u8, 163u8, 12u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testCrossAssetVerseTradingOnUniswapV2Return::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testMultipleVersePairsOnUniswap()` and selector `0x1d828835`.
```solidity
function testMultipleVersePairsOnUniswap() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testMultipleVersePairsOnUniswapCall;
    ///Container type for the return parameters of the [`testMultipleVersePairsOnUniswap()`](testMultipleVersePairsOnUniswapCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testMultipleVersePairsOnUniswapReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<testMultipleVersePairsOnUniswapCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testMultipleVersePairsOnUniswapCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testMultipleVersePairsOnUniswapCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<testMultipleVersePairsOnUniswapReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testMultipleVersePairsOnUniswapReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testMultipleVersePairsOnUniswapReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testMultipleVersePairsOnUniswapReturn {
            fn _tokenize(
                &self,
            ) -> <testMultipleVersePairsOnUniswapCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testMultipleVersePairsOnUniswapCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testMultipleVersePairsOnUniswapReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testMultipleVersePairsOnUniswap()";
            const SELECTOR: [u8; 4] = [29u8, 130u8, 136u8, 53u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testMultipleVersePairsOnUniswapReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `testPriceDivergenceBetweenVerses()` and selector `0x7140dc73`.
```solidity
function testPriceDivergenceBetweenVerses() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testPriceDivergenceBetweenVersesCall;
    ///Container type for the return parameters of the [`testPriceDivergenceBetweenVerses()`](testPriceDivergenceBetweenVersesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testPriceDivergenceBetweenVersesReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<testPriceDivergenceBetweenVersesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testPriceDivergenceBetweenVersesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testPriceDivergenceBetweenVersesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<testPriceDivergenceBetweenVersesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testPriceDivergenceBetweenVersesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testPriceDivergenceBetweenVersesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testPriceDivergenceBetweenVersesReturn {
            fn _tokenize(
                &self,
            ) -> <testPriceDivergenceBetweenVersesCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testPriceDivergenceBetweenVersesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testPriceDivergenceBetweenVersesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testPriceDivergenceBetweenVerses()";
            const SELECTOR: [u8; 4] = [113u8, 64u8, 220u8, 115u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testPriceDivergenceBetweenVersesReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `usdc()` and selector `0x3e413bee`.
```solidity
function usdc() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct usdcCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`usdc()`](usdcCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct usdcReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<usdcCall> for UnderlyingRustTuple<'_> {
                fn from(value: usdcCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for usdcCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<usdcReturn> for UnderlyingRustTuple<'_> {
                fn from(value: usdcReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for usdcReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for usdcCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "usdc()";
            const SELECTOR: [u8; 4] = [62u8, 65u8, 59u8, 238u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: usdcReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: usdcReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `weth()` and selector `0x3fc8cef3`.
```solidity
function weth() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct wethCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`weth()`](wethCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct wethReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<wethCall> for UnderlyingRustTuple<'_> {
                fn from(value: wethCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for wethCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<wethReturn> for UnderlyingRustTuple<'_> {
                fn from(value: wethReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for wethReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for wethCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "weth()";
            const SELECTOR: [u8; 4] = [63u8, 200u8, 206u8, 243u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: wethReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                    .map(|r| {
                        let r: wethReturn = r.into();
                        r._0
                    })
            }
        }
    };
    ///Container for all the [`VerseAMMTest`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum VerseAMMTestCalls {
        #[allow(missing_docs)]
        IS_TEST(IS_TESTCall),
        #[allow(missing_docs)]
        QUESTION_HASH(QUESTION_HASHCall),
        #[allow(missing_docs)]
        RESOLUTION_DEADLINE(RESOLUTION_DEADLINECall),
        #[allow(missing_docs)]
        UNISWAP_V2_FACTORY(UNISWAP_V2_FACTORYCall),
        #[allow(missing_docs)]
        alice(aliceCall),
        #[allow(missing_docs)]
        bob(bobCall),
        #[allow(missing_docs)]
        excludeArtifacts(excludeArtifactsCall),
        #[allow(missing_docs)]
        excludeContracts(excludeContractsCall),
        #[allow(missing_docs)]
        excludeSelectors(excludeSelectorsCall),
        #[allow(missing_docs)]
        excludeSenders(excludeSendersCall),
        #[allow(missing_docs)]
        failed(failedCall),
        #[allow(missing_docs)]
        lpProvider(lpProviderCall),
        #[allow(missing_docs)]
        multiVerse(multiVerseCall),
        #[allow(missing_docs)]
        oracle(oracleCall),
        #[allow(missing_docs)]
        setUp(setUpCall),
        #[allow(missing_docs)]
        targetArtifactSelectors(targetArtifactSelectorsCall),
        #[allow(missing_docs)]
        targetArtifacts(targetArtifactsCall),
        #[allow(missing_docs)]
        targetContracts(targetContractsCall),
        #[allow(missing_docs)]
        targetInterfaces(targetInterfacesCall),
        #[allow(missing_docs)]
        targetSelectors(targetSelectorsCall),
        #[allow(missing_docs)]
        targetSenders(targetSendersCall),
        #[allow(missing_docs)]
        testCrossAssetVerseTradingOnUniswapV2(testCrossAssetVerseTradingOnUniswapV2Call),
        #[allow(missing_docs)]
        testMultipleVersePairsOnUniswap(testMultipleVersePairsOnUniswapCall),
        #[allow(missing_docs)]
        testPriceDivergenceBetweenVerses(testPriceDivergenceBetweenVersesCall),
        #[allow(missing_docs)]
        usdc(usdcCall),
        #[allow(missing_docs)]
        weth(wethCall),
    }
    impl VerseAMMTestCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [10u8, 146u8, 84u8, 228u8],
            [13u8, 35u8, 88u8, 23u8],
            [19u8, 125u8, 163u8, 12u8],
            [29u8, 130u8, 136u8, 53u8],
            [30u8, 215u8, 131u8, 28u8],
            [42u8, 222u8, 56u8, 128u8],
            [62u8, 65u8, 59u8, 238u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [63u8, 200u8, 206u8, 243u8],
            [90u8, 247u8, 7u8, 26u8],
            [102u8, 217u8, 169u8, 160u8],
            [113u8, 64u8, 220u8, 115u8],
            [125u8, 192u8, 209u8, 208u8],
            [133u8, 34u8, 108u8, 129u8],
            [140u8, 128u8, 159u8, 191u8],
            [145u8, 106u8, 23u8, 198u8],
            [153u8, 216u8, 250u8, 227u8],
            [164u8, 35u8, 75u8, 78u8],
            [176u8, 70u8, 79u8, 220u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [192u8, 156u8, 236u8, 119u8],
            [226u8, 12u8, 159u8, 113u8],
            [250u8, 118u8, 38u8, 212u8],
            [251u8, 71u8, 227u8, 162u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(setUp),
            ::core::stringify!(lpProvider),
            ::core::stringify!(testCrossAssetVerseTradingOnUniswapV2),
            ::core::stringify!(testMultipleVersePairsOnUniswap),
            ::core::stringify!(excludeSenders),
            ::core::stringify!(targetInterfaces),
            ::core::stringify!(usdc),
            ::core::stringify!(targetSenders),
            ::core::stringify!(targetContracts),
            ::core::stringify!(weth),
            ::core::stringify!(QUESTION_HASH),
            ::core::stringify!(targetArtifactSelectors),
            ::core::stringify!(testPriceDivergenceBetweenVerses),
            ::core::stringify!(oracle),
            ::core::stringify!(targetArtifacts),
            ::core::stringify!(multiVerse),
            ::core::stringify!(targetSelectors),
            ::core::stringify!(UNISWAP_V2_FACTORY),
            ::core::stringify!(RESOLUTION_DEADLINE),
            ::core::stringify!(excludeSelectors),
            ::core::stringify!(excludeArtifacts),
            ::core::stringify!(failed),
            ::core::stringify!(bob),
            ::core::stringify!(excludeContracts),
            ::core::stringify!(IS_TEST),
            ::core::stringify!(alice),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <setUpCall as alloy_sol_types::SolCall>::SIGNATURE,
            <lpProviderCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testCrossAssetVerseTradingOnUniswapV2Call as alloy_sol_types::SolCall>::SIGNATURE,
            <testMultipleVersePairsOnUniswapCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetInterfacesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <usdcCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetContractsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <wethCall as alloy_sol_types::SolCall>::SIGNATURE,
            <QUESTION_HASHCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testPriceDivergenceBetweenVersesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <oracleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetArtifactsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <multiVerseCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <UNISWAP_V2_FACTORYCall as alloy_sol_types::SolCall>::SIGNATURE,
            <RESOLUTION_DEADLINECall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeArtifactsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <failedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <bobCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeContractsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <IS_TESTCall as alloy_sol_types::SolCall>::SIGNATURE,
            <aliceCall as alloy_sol_types::SolCall>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for VerseAMMTestCalls {
        const NAME: &'static str = "VerseAMMTestCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 26usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::QUESTION_HASH(_) => {
                    <QUESTION_HASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::RESOLUTION_DEADLINE(_) => {
                    <RESOLUTION_DEADLINECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::UNISWAP_V2_FACTORY(_) => {
                    <UNISWAP_V2_FACTORYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::alice(_) => <aliceCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::bob(_) => <bobCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::excludeArtifacts(_) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeContracts(_) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeSelectors(_) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeSenders(_) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::failed(_) => <failedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::lpProvider(_) => {
                    <lpProviderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::multiVerse(_) => {
                    <multiVerseCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::oracle(_) => <oracleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::setUp(_) => <setUpCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::targetArtifactSelectors(_) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetArtifacts(_) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetContracts(_) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetInterfaces(_) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSelectors(_) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSenders(_) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testCrossAssetVerseTradingOnUniswapV2(_) => {
                    <testCrossAssetVerseTradingOnUniswapV2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testMultipleVersePairsOnUniswap(_) => {
                    <testMultipleVersePairsOnUniswapCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testPriceDivergenceBetweenVerses(_) => {
                    <testPriceDivergenceBetweenVersesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::usdc(_) => <usdcCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::weth(_) => <wethCall as alloy_sol_types::SolCall>::SELECTOR,
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<VerseAMMTestCalls>] = &[
                {
                    fn setUp(data: &[u8]) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(VerseAMMTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn lpProvider(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <lpProviderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseAMMTestCalls::lpProvider)
                    }
                    lpProvider
                },
                {
                    fn testCrossAssetVerseTradingOnUniswapV2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <testCrossAssetVerseTradingOnUniswapV2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                VerseAMMTestCalls::testCrossAssetVerseTradingOnUniswapV2,
                            )
                    }
                    testCrossAssetVerseTradingOnUniswapV2
                },
                {
                    fn testMultipleVersePairsOnUniswap(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <testMultipleVersePairsOnUniswapCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseAMMTestCalls::testMultipleVersePairsOnUniswap)
                    }
                    testMultipleVersePairsOnUniswap
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseAMMTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseAMMTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn usdc(data: &[u8]) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <usdcCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(VerseAMMTestCalls::usdc)
                    }
                    usdc
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseAMMTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseAMMTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn weth(data: &[u8]) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <wethCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(VerseAMMTestCalls::weth)
                    }
                    weth
                },
                {
                    fn QUESTION_HASH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <QUESTION_HASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseAMMTestCalls::QUESTION_HASH)
                    }
                    QUESTION_HASH
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseAMMTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn testPriceDivergenceBetweenVerses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <testPriceDivergenceBetweenVersesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseAMMTestCalls::testPriceDivergenceBetweenVerses)
                    }
                    testPriceDivergenceBetweenVerses
                },
                {
                    fn oracle(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <oracleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(VerseAMMTestCalls::oracle)
                    }
                    oracle
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseAMMTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn multiVerse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <multiVerseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseAMMTestCalls::multiVerse)
                    }
                    multiVerse
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseAMMTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn UNISWAP_V2_FACTORY(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <UNISWAP_V2_FACTORYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseAMMTestCalls::UNISWAP_V2_FACTORY)
                    }
                    UNISWAP_V2_FACTORY
                },
                {
                    fn RESOLUTION_DEADLINE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <RESOLUTION_DEADLINECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseAMMTestCalls::RESOLUTION_DEADLINE)
                    }
                    RESOLUTION_DEADLINE
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseAMMTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseAMMTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(VerseAMMTestCalls::failed)
                    }
                    failed
                },
                {
                    fn bob(data: &[u8]) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <bobCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(VerseAMMTestCalls::bob)
                    }
                    bob
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseAMMTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(VerseAMMTestCalls::IS_TEST)
                    }
                    IS_TEST
                },
                {
                    fn alice(data: &[u8]) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <aliceCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(VerseAMMTestCalls::alice)
                    }
                    alice
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<VerseAMMTestCalls>] = &[
                {
                    fn setUp(data: &[u8]) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn lpProvider(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <lpProviderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::lpProvider)
                    }
                    lpProvider
                },
                {
                    fn testCrossAssetVerseTradingOnUniswapV2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <testCrossAssetVerseTradingOnUniswapV2Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                VerseAMMTestCalls::testCrossAssetVerseTradingOnUniswapV2,
                            )
                    }
                    testCrossAssetVerseTradingOnUniswapV2
                },
                {
                    fn testMultipleVersePairsOnUniswap(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <testMultipleVersePairsOnUniswapCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::testMultipleVersePairsOnUniswap)
                    }
                    testMultipleVersePairsOnUniswap
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn usdc(data: &[u8]) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <usdcCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::usdc)
                    }
                    usdc
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn weth(data: &[u8]) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <wethCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::weth)
                    }
                    weth
                },
                {
                    fn QUESTION_HASH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <QUESTION_HASHCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::QUESTION_HASH)
                    }
                    QUESTION_HASH
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn testPriceDivergenceBetweenVerses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <testPriceDivergenceBetweenVersesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::testPriceDivergenceBetweenVerses)
                    }
                    testPriceDivergenceBetweenVerses
                },
                {
                    fn oracle(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <oracleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::oracle)
                    }
                    oracle
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn multiVerse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <multiVerseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::multiVerse)
                    }
                    multiVerse
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn UNISWAP_V2_FACTORY(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <UNISWAP_V2_FACTORYCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::UNISWAP_V2_FACTORY)
                    }
                    UNISWAP_V2_FACTORY
                },
                {
                    fn RESOLUTION_DEADLINE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <RESOLUTION_DEADLINECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::RESOLUTION_DEADLINE)
                    }
                    RESOLUTION_DEADLINE
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::failed)
                    }
                    failed
                },
                {
                    fn bob(data: &[u8]) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <bobCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::bob)
                    }
                    bob
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::IS_TEST)
                    }
                    IS_TEST
                },
                {
                    fn alice(data: &[u8]) -> alloy_sol_types::Result<VerseAMMTestCalls> {
                        <aliceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseAMMTestCalls::alice)
                    }
                    alice
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::QUESTION_HASH(inner) => {
                    <QUESTION_HASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::RESOLUTION_DEADLINE(inner) => {
                    <RESOLUTION_DEADLINECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UNISWAP_V2_FACTORY(inner) => {
                    <UNISWAP_V2_FACTORYCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::alice(inner) => {
                    <aliceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::bob(inner) => {
                    <bobCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeSelectors(inner) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::lpProvider(inner) => {
                    <lpProviderCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::multiVerse(inner) => {
                    <multiVerseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::oracle(inner) => {
                    <oracleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setUp(inner) => {
                    <setUpCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::targetArtifactSelectors(inner) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetArtifacts(inner) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetContracts(inner) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetInterfaces(inner) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testCrossAssetVerseTradingOnUniswapV2(inner) => {
                    <testCrossAssetVerseTradingOnUniswapV2Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testMultipleVersePairsOnUniswap(inner) => {
                    <testMultipleVersePairsOnUniswapCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testPriceDivergenceBetweenVerses(inner) => {
                    <testPriceDivergenceBetweenVersesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::usdc(inner) => {
                    <usdcCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::weth(inner) => {
                    <wethCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::QUESTION_HASH(inner) => {
                    <QUESTION_HASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::RESOLUTION_DEADLINE(inner) => {
                    <RESOLUTION_DEADLINECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UNISWAP_V2_FACTORY(inner) => {
                    <UNISWAP_V2_FACTORYCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::alice(inner) => {
                    <aliceCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::bob(inner) => {
                    <bobCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeSelectors(inner) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::lpProvider(inner) => {
                    <lpProviderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::multiVerse(inner) => {
                    <multiVerseCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::oracle(inner) => {
                    <oracleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setUp(inner) => {
                    <setUpCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::targetArtifactSelectors(inner) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetArtifacts(inner) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetContracts(inner) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetInterfaces(inner) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testCrossAssetVerseTradingOnUniswapV2(inner) => {
                    <testCrossAssetVerseTradingOnUniswapV2Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testMultipleVersePairsOnUniswap(inner) => {
                    <testMultipleVersePairsOnUniswapCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testPriceDivergenceBetweenVerses(inner) => {
                    <testPriceDivergenceBetweenVersesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::usdc(inner) => {
                    <usdcCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::weth(inner) => {
                    <wethCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`VerseAMMTest`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum VerseAMMTestEvents {
        #[allow(missing_docs)]
        log(log),
        #[allow(missing_docs)]
        log_address(log_address),
        #[allow(missing_docs)]
        log_array_0(log_array_0),
        #[allow(missing_docs)]
        log_array_1(log_array_1),
        #[allow(missing_docs)]
        log_array_2(log_array_2),
        #[allow(missing_docs)]
        log_bytes(log_bytes),
        #[allow(missing_docs)]
        log_bytes32(log_bytes32),
        #[allow(missing_docs)]
        log_int(log_int),
        #[allow(missing_docs)]
        log_named_address(log_named_address),
        #[allow(missing_docs)]
        log_named_array_0(log_named_array_0),
        #[allow(missing_docs)]
        log_named_array_1(log_named_array_1),
        #[allow(missing_docs)]
        log_named_array_2(log_named_array_2),
        #[allow(missing_docs)]
        log_named_bytes(log_named_bytes),
        #[allow(missing_docs)]
        log_named_bytes32(log_named_bytes32),
        #[allow(missing_docs)]
        log_named_decimal_int(log_named_decimal_int),
        #[allow(missing_docs)]
        log_named_decimal_uint(log_named_decimal_uint),
        #[allow(missing_docs)]
        log_named_int(log_named_int),
        #[allow(missing_docs)]
        log_named_string(log_named_string),
        #[allow(missing_docs)]
        log_named_uint(log_named_uint),
        #[allow(missing_docs)]
        log_string(log_string),
        #[allow(missing_docs)]
        log_uint(log_uint),
        #[allow(missing_docs)]
        logs(logs),
    }
    impl VerseAMMTestEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                0u8, 170u8, 163u8, 156u8, 159u8, 251u8, 95u8, 86u8, 122u8, 69u8, 52u8,
                56u8, 12u8, 115u8, 112u8, 117u8, 112u8, 46u8, 31u8, 127u8, 20u8, 16u8,
                127u8, 201u8, 83u8, 40u8, 227u8, 181u8, 108u8, 3u8, 37u8, 251u8,
            ],
            [
                11u8, 46u8, 19u8, 255u8, 32u8, 172u8, 123u8, 71u8, 65u8, 152u8, 101u8,
                85u8, 131u8, 237u8, 247u8, 13u8, 237u8, 210u8, 193u8, 220u8, 152u8, 14u8,
                50u8, 156u8, 79u8, 187u8, 47u8, 192u8, 116u8, 139u8, 121u8, 107u8,
            ],
            [
                14u8, 181u8, 213u8, 38u8, 36u8, 200u8, 210u8, 138u8, 218u8, 159u8, 197u8,
                90u8, 140u8, 80u8, 46u8, 213u8, 170u8, 63u8, 190u8, 47u8, 182u8, 233u8,
                27u8, 113u8, 181u8, 243u8, 118u8, 136u8, 43u8, 29u8, 47u8, 184u8,
            ],
            [
                35u8, 182u8, 42u8, 208u8, 88u8, 77u8, 36u8, 167u8, 95u8, 11u8, 243u8,
                86u8, 3u8, 145u8, 239u8, 86u8, 89u8, 236u8, 109u8, 177u8, 38u8, 156u8,
                86u8, 225u8, 26u8, 162u8, 65u8, 214u8, 55u8, 241u8, 155u8, 32u8,
            ],
            [
                40u8, 15u8, 68u8, 70u8, 178u8, 138u8, 19u8, 114u8, 65u8, 125u8, 218u8,
                101u8, 141u8, 48u8, 185u8, 91u8, 41u8, 146u8, 177u8, 42u8, 201u8, 199u8,
                243u8, 120u8, 83u8, 95u8, 41u8, 169u8, 122u8, 207u8, 53u8, 131u8,
            ],
            [
                44u8, 171u8, 151u8, 144u8, 81u8, 15u8, 216u8, 189u8, 251u8, 210u8, 17u8,
                82u8, 136u8, 219u8, 51u8, 254u8, 198u8, 102u8, 145u8, 212u8, 118u8,
                239u8, 197u8, 66u8, 124u8, 253u8, 76u8, 9u8, 105u8, 48u8, 23u8, 85u8,
            ],
            [
                47u8, 230u8, 50u8, 119u8, 145u8, 116u8, 55u8, 67u8, 120u8, 68u8, 42u8,
                142u8, 151u8, 139u8, 204u8, 251u8, 220u8, 193u8, 214u8, 178u8, 176u8,
                216u8, 31u8, 126u8, 142u8, 183u8, 118u8, 171u8, 34u8, 134u8, 241u8, 104u8,
            ],
            [
                59u8, 207u8, 178u8, 174u8, 46u8, 141u8, 19u8, 45u8, 209u8, 252u8, 231u8,
                207u8, 39u8, 138u8, 154u8, 25u8, 117u8, 106u8, 159u8, 206u8, 171u8,
                228u8, 112u8, 223u8, 59u8, 218u8, 187u8, 75u8, 197u8, 119u8, 209u8, 189u8,
            ],
            [
                64u8, 225u8, 132u8, 15u8, 87u8, 105u8, 7u8, 61u8, 97u8, 189u8, 1u8, 55u8,
                45u8, 155u8, 117u8, 186u8, 169u8, 132u8, 45u8, 86u8, 41u8, 160u8, 201u8,
                159u8, 241u8, 3u8, 190u8, 17u8, 120u8, 168u8, 233u8, 226u8,
            ],
            [
                65u8, 48u8, 79u8, 172u8, 217u8, 50u8, 61u8, 117u8, 177u8, 27u8, 205u8,
                214u8, 9u8, 203u8, 56u8, 239u8, 255u8, 253u8, 176u8, 87u8, 16u8, 247u8,
                202u8, 240u8, 233u8, 177u8, 108u8, 109u8, 157u8, 112u8, 159u8, 80u8,
            ],
            [
                93u8, 166u8, 206u8, 157u8, 81u8, 21u8, 27u8, 161u8, 12u8, 9u8, 165u8,
                89u8, 239u8, 36u8, 213u8, 32u8, 185u8, 218u8, 197u8, 197u8, 184u8, 129u8,
                10u8, 232u8, 67u8, 78u8, 77u8, 13u8, 134u8, 65u8, 26u8, 149u8,
            ],
            [
                122u8, 231u8, 76u8, 82u8, 116u8, 20u8, 174u8, 19u8, 95u8, 217u8, 112u8,
                71u8, 177u8, 41u8, 33u8, 165u8, 236u8, 57u8, 17u8, 184u8, 4u8, 25u8,
                120u8, 85u8, 214u8, 126u8, 37u8, 199u8, 183u8, 94u8, 230u8, 243u8,
            ],
            [
                137u8, 10u8, 130u8, 103u8, 155u8, 71u8, 15u8, 43u8, 216u8, 40u8, 22u8,
                237u8, 155u8, 22u8, 31u8, 151u8, 216u8, 185u8, 103u8, 243u8, 127u8,
                163u8, 100u8, 124u8, 33u8, 213u8, 191u8, 57u8, 116u8, 158u8, 45u8, 213u8,
            ],
            [
                156u8, 78u8, 133u8, 65u8, 202u8, 143u8, 13u8, 193u8, 196u8, 19u8, 249u8,
                16u8, 143u8, 102u8, 216u8, 45u8, 60u8, 236u8, 177u8, 189u8, 219u8, 206u8,
                67u8, 122u8, 97u8, 202u8, 163u8, 23u8, 92u8, 76u8, 201u8, 111u8,
            ],
            [
                167u8, 62u8, 218u8, 9u8, 102u8, 47u8, 70u8, 221u8, 231u8, 41u8, 190u8,
                70u8, 17u8, 56u8, 95u8, 243u8, 79u8, 230u8, 196u8, 79u8, 187u8, 198u8,
                247u8, 225u8, 123u8, 4u8, 43u8, 89u8, 163u8, 68u8, 91u8, 87u8,
            ],
            [
                175u8, 183u8, 149u8, 201u8, 198u8, 30u8, 79u8, 231u8, 70u8, 140u8, 56u8,
                111u8, 146u8, 93u8, 122u8, 84u8, 41u8, 236u8, 173u8, 156u8, 4u8, 149u8,
                221u8, 184u8, 211u8, 141u8, 105u8, 6u8, 20u8, 211u8, 47u8, 153u8,
            ],
            [
                178u8, 222u8, 47u8, 190u8, 128u8, 26u8, 13u8, 246u8, 192u8, 203u8, 221u8,
                253u8, 68u8, 139u8, 163u8, 196u8, 29u8, 72u8, 160u8, 64u8, 202u8, 53u8,
                197u8, 108u8, 129u8, 150u8, 239u8, 15u8, 202u8, 231u8, 33u8, 168u8,
            ],
            [
                210u8, 110u8, 22u8, 202u8, 212u8, 84u8, 135u8, 5u8, 228u8, 201u8, 226u8,
                217u8, 79u8, 152u8, 238u8, 145u8, 194u8, 137u8, 8u8, 94u8, 228u8, 37u8,
                89u8, 79u8, 213u8, 99u8, 95u8, 162u8, 150u8, 76u8, 207u8, 24u8,
            ],
            [
                231u8, 149u8, 14u8, 222u8, 3u8, 148u8, 185u8, 242u8, 206u8, 74u8, 90u8,
                27u8, 245u8, 167u8, 225u8, 133u8, 36u8, 17u8, 247u8, 230u8, 102u8, 27u8,
                67u8, 8u8, 201u8, 19u8, 196u8, 191u8, 209u8, 16u8, 39u8, 228u8,
            ],
            [
                232u8, 22u8, 153u8, 184u8, 81u8, 19u8, 238u8, 161u8, 199u8, 62u8, 16u8,
                88u8, 139u8, 43u8, 3u8, 94u8, 85u8, 137u8, 51u8, 105u8, 99u8, 33u8,
                115u8, 175u8, 212u8, 63u8, 235u8, 25u8, 47u8, 172u8, 100u8, 227u8,
            ],
            [
                235u8, 139u8, 164u8, 60u8, 237u8, 117u8, 55u8, 66u8, 25u8, 70u8, 189u8,
                67u8, 232u8, 40u8, 184u8, 178u8, 184u8, 66u8, 137u8, 39u8, 170u8, 143u8,
                128u8, 28u8, 19u8, 217u8, 52u8, 191u8, 17u8, 172u8, 165u8, 123u8,
            ],
            [
                251u8, 16u8, 40u8, 101u8, 213u8, 10u8, 221u8, 221u8, 246u8, 157u8, 169u8,
                181u8, 170u8, 27u8, 206u8, 214u8, 108u8, 128u8, 207u8, 134u8, 154u8,
                92u8, 141u8, 4u8, 113u8, 164u8, 103u8, 225u8, 140u8, 233u8, 202u8, 177u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(log_named_array_0),
            ::core::stringify!(log_string),
            ::core::stringify!(log_int),
            ::core::stringify!(log_bytes),
            ::core::stringify!(log_named_string),
            ::core::stringify!(log_uint),
            ::core::stringify!(log_named_int),
            ::core::stringify!(log_named_array_2),
            ::core::stringify!(log_array_2),
            ::core::stringify!(log),
            ::core::stringify!(log_named_decimal_int),
            ::core::stringify!(log_address),
            ::core::stringify!(log_array_1),
            ::core::stringify!(log_named_address),
            ::core::stringify!(log_named_array_1),
            ::core::stringify!(log_named_bytes32),
            ::core::stringify!(log_named_uint),
            ::core::stringify!(log_named_bytes),
            ::core::stringify!(logs),
            ::core::stringify!(log_bytes32),
            ::core::stringify!(log_named_decimal_uint),
            ::core::stringify!(log_array_0),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <log_named_array_0 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_string as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_int as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_bytes as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_string as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_uint as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_int as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_array_2 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_array_2 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_decimal_int as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_address as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_array_1 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_address as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_array_1 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_uint as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_bytes as alloy_sol_types::SolEvent>::SIGNATURE,
            <logs as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_named_decimal_uint as alloy_sol_types::SolEvent>::SIGNATURE,
            <log_array_0 as alloy_sol_types::SolEvent>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for VerseAMMTestEvents {
        const NAME: &'static str = "VerseAMMTestEvents";
        const COUNT: usize = 22usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<log as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::log)
                }
                Some(<log_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_address as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_address)
                }
                Some(<log_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_array_0)
                }
                Some(<log_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_array_1)
                }
                Some(<log_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_array_2)
                }
                Some(<log_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_bytes)
                }
                Some(<log_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_bytes32)
                }
                Some(<log_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_int as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::log_int)
                }
                Some(
                    <log_named_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_address as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_address)
                }
                Some(
                    <log_named_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_array_0)
                }
                Some(
                    <log_named_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_array_1)
                }
                Some(
                    <log_named_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_array_2)
                }
                Some(<log_named_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_bytes as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_bytes)
                }
                Some(
                    <log_named_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_bytes32)
                }
                Some(
                    <log_named_decimal_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_decimal_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_decimal_int)
                }
                Some(
                    <log_named_decimal_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_decimal_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_decimal_uint)
                }
                Some(<log_named_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_int)
                }
                Some(<log_named_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_string as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_string)
                }
                Some(<log_named_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_named_uint)
                }
                Some(<log_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_string as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::log_string)
                }
                Some(<log_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_uint as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::log_uint)
                }
                Some(<logs as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <logs as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::logs)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for VerseAMMTestEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::log(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_address(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_address(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_decimal_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_decimal_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_string(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_string(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::logs(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::log(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_address(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_address(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_decimal_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_decimal_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_string(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_string(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::logs(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`VerseAMMTest`](self) contract instance.

See the [wrapper's documentation](`VerseAMMTestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> VerseAMMTestInstance<P, N> {
        VerseAMMTestInstance::<P, N>::new(address, __provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<VerseAMMTestInstance<P, N>>,
    > {
        VerseAMMTestInstance::<P, N>::deploy(__provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
        VerseAMMTestInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`VerseAMMTest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`VerseAMMTest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct VerseAMMTestInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for VerseAMMTestInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("VerseAMMTestInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > VerseAMMTestInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`VerseAMMTest`](self) contract instance.

See the [wrapper's documentation](`VerseAMMTestInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            __provider: P,
        ) -> alloy_contract::Result<VerseAMMTestInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> VerseAMMTestInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> VerseAMMTestInstance<P, N> {
            VerseAMMTestInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > VerseAMMTestInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`IS_TEST`] function.
        pub fn IS_TEST(&self) -> alloy_contract::SolCallBuilder<&P, IS_TESTCall, N> {
            self.call_builder(&IS_TESTCall)
        }
        ///Creates a new call builder for the [`QUESTION_HASH`] function.
        pub fn QUESTION_HASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, QUESTION_HASHCall, N> {
            self.call_builder(&QUESTION_HASHCall)
        }
        ///Creates a new call builder for the [`RESOLUTION_DEADLINE`] function.
        pub fn RESOLUTION_DEADLINE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, RESOLUTION_DEADLINECall, N> {
            self.call_builder(&RESOLUTION_DEADLINECall)
        }
        ///Creates a new call builder for the [`UNISWAP_V2_FACTORY`] function.
        pub fn UNISWAP_V2_FACTORY(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, UNISWAP_V2_FACTORYCall, N> {
            self.call_builder(&UNISWAP_V2_FACTORYCall)
        }
        ///Creates a new call builder for the [`alice`] function.
        pub fn alice(&self) -> alloy_contract::SolCallBuilder<&P, aliceCall, N> {
            self.call_builder(&aliceCall)
        }
        ///Creates a new call builder for the [`bob`] function.
        pub fn bob(&self) -> alloy_contract::SolCallBuilder<&P, bobCall, N> {
            self.call_builder(&bobCall)
        }
        ///Creates a new call builder for the [`excludeArtifacts`] function.
        pub fn excludeArtifacts(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, excludeArtifactsCall, N> {
            self.call_builder(&excludeArtifactsCall)
        }
        ///Creates a new call builder for the [`excludeContracts`] function.
        pub fn excludeContracts(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, excludeContractsCall, N> {
            self.call_builder(&excludeContractsCall)
        }
        ///Creates a new call builder for the [`excludeSelectors`] function.
        pub fn excludeSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, excludeSelectorsCall, N> {
            self.call_builder(&excludeSelectorsCall)
        }
        ///Creates a new call builder for the [`excludeSenders`] function.
        pub fn excludeSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, excludeSendersCall, N> {
            self.call_builder(&excludeSendersCall)
        }
        ///Creates a new call builder for the [`failed`] function.
        pub fn failed(&self) -> alloy_contract::SolCallBuilder<&P, failedCall, N> {
            self.call_builder(&failedCall)
        }
        ///Creates a new call builder for the [`lpProvider`] function.
        pub fn lpProvider(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, lpProviderCall, N> {
            self.call_builder(&lpProviderCall)
        }
        ///Creates a new call builder for the [`multiVerse`] function.
        pub fn multiVerse(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, multiVerseCall, N> {
            self.call_builder(&multiVerseCall)
        }
        ///Creates a new call builder for the [`oracle`] function.
        pub fn oracle(&self) -> alloy_contract::SolCallBuilder<&P, oracleCall, N> {
            self.call_builder(&oracleCall)
        }
        ///Creates a new call builder for the [`setUp`] function.
        pub fn setUp(&self) -> alloy_contract::SolCallBuilder<&P, setUpCall, N> {
            self.call_builder(&setUpCall)
        }
        ///Creates a new call builder for the [`targetArtifactSelectors`] function.
        pub fn targetArtifactSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, targetArtifactSelectorsCall, N> {
            self.call_builder(&targetArtifactSelectorsCall)
        }
        ///Creates a new call builder for the [`targetArtifacts`] function.
        pub fn targetArtifacts(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, targetArtifactsCall, N> {
            self.call_builder(&targetArtifactsCall)
        }
        ///Creates a new call builder for the [`targetContracts`] function.
        pub fn targetContracts(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, targetContractsCall, N> {
            self.call_builder(&targetContractsCall)
        }
        ///Creates a new call builder for the [`targetInterfaces`] function.
        pub fn targetInterfaces(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, targetInterfacesCall, N> {
            self.call_builder(&targetInterfacesCall)
        }
        ///Creates a new call builder for the [`targetSelectors`] function.
        pub fn targetSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, targetSelectorsCall, N> {
            self.call_builder(&targetSelectorsCall)
        }
        ///Creates a new call builder for the [`targetSenders`] function.
        pub fn targetSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, targetSendersCall, N> {
            self.call_builder(&targetSendersCall)
        }
        ///Creates a new call builder for the [`testCrossAssetVerseTradingOnUniswapV2`] function.
        pub fn testCrossAssetVerseTradingOnUniswapV2(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testCrossAssetVerseTradingOnUniswapV2Call,
            N,
        > {
            self.call_builder(&testCrossAssetVerseTradingOnUniswapV2Call)
        }
        ///Creates a new call builder for the [`testMultipleVersePairsOnUniswap`] function.
        pub fn testMultipleVersePairsOnUniswap(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testMultipleVersePairsOnUniswapCall, N> {
            self.call_builder(&testMultipleVersePairsOnUniswapCall)
        }
        ///Creates a new call builder for the [`testPriceDivergenceBetweenVerses`] function.
        pub fn testPriceDivergenceBetweenVerses(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            testPriceDivergenceBetweenVersesCall,
            N,
        > {
            self.call_builder(&testPriceDivergenceBetweenVersesCall)
        }
        ///Creates a new call builder for the [`usdc`] function.
        pub fn usdc(&self) -> alloy_contract::SolCallBuilder<&P, usdcCall, N> {
            self.call_builder(&usdcCall)
        }
        ///Creates a new call builder for the [`weth`] function.
        pub fn weth(&self) -> alloy_contract::SolCallBuilder<&P, wethCall, N> {
            self.call_builder(&wethCall)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > VerseAMMTestInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`log`] event.
        pub fn log_filter(&self) -> alloy_contract::Event<&P, log, N> {
            self.event_filter::<log>()
        }
        ///Creates a new event filter for the [`log_address`] event.
        pub fn log_address_filter(&self) -> alloy_contract::Event<&P, log_address, N> {
            self.event_filter::<log_address>()
        }
        ///Creates a new event filter for the [`log_array_0`] event.
        pub fn log_array_0_filter(&self) -> alloy_contract::Event<&P, log_array_0, N> {
            self.event_filter::<log_array_0>()
        }
        ///Creates a new event filter for the [`log_array_1`] event.
        pub fn log_array_1_filter(&self) -> alloy_contract::Event<&P, log_array_1, N> {
            self.event_filter::<log_array_1>()
        }
        ///Creates a new event filter for the [`log_array_2`] event.
        pub fn log_array_2_filter(&self) -> alloy_contract::Event<&P, log_array_2, N> {
            self.event_filter::<log_array_2>()
        }
        ///Creates a new event filter for the [`log_bytes`] event.
        pub fn log_bytes_filter(&self) -> alloy_contract::Event<&P, log_bytes, N> {
            self.event_filter::<log_bytes>()
        }
        ///Creates a new event filter for the [`log_bytes32`] event.
        pub fn log_bytes32_filter(&self) -> alloy_contract::Event<&P, log_bytes32, N> {
            self.event_filter::<log_bytes32>()
        }
        ///Creates a new event filter for the [`log_int`] event.
        pub fn log_int_filter(&self) -> alloy_contract::Event<&P, log_int, N> {
            self.event_filter::<log_int>()
        }
        ///Creates a new event filter for the [`log_named_address`] event.
        pub fn log_named_address_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_address, N> {
            self.event_filter::<log_named_address>()
        }
        ///Creates a new event filter for the [`log_named_array_0`] event.
        pub fn log_named_array_0_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_array_0, N> {
            self.event_filter::<log_named_array_0>()
        }
        ///Creates a new event filter for the [`log_named_array_1`] event.
        pub fn log_named_array_1_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_array_1, N> {
            self.event_filter::<log_named_array_1>()
        }
        ///Creates a new event filter for the [`log_named_array_2`] event.
        pub fn log_named_array_2_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_array_2, N> {
            self.event_filter::<log_named_array_2>()
        }
        ///Creates a new event filter for the [`log_named_bytes`] event.
        pub fn log_named_bytes_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_bytes, N> {
            self.event_filter::<log_named_bytes>()
        }
        ///Creates a new event filter for the [`log_named_bytes32`] event.
        pub fn log_named_bytes32_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_bytes32, N> {
            self.event_filter::<log_named_bytes32>()
        }
        ///Creates a new event filter for the [`log_named_decimal_int`] event.
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_decimal_int, N> {
            self.event_filter::<log_named_decimal_int>()
        }
        ///Creates a new event filter for the [`log_named_decimal_uint`] event.
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_decimal_uint, N> {
            self.event_filter::<log_named_decimal_uint>()
        }
        ///Creates a new event filter for the [`log_named_int`] event.
        pub fn log_named_int_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_int, N> {
            self.event_filter::<log_named_int>()
        }
        ///Creates a new event filter for the [`log_named_string`] event.
        pub fn log_named_string_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_string, N> {
            self.event_filter::<log_named_string>()
        }
        ///Creates a new event filter for the [`log_named_uint`] event.
        pub fn log_named_uint_filter(
            &self,
        ) -> alloy_contract::Event<&P, log_named_uint, N> {
            self.event_filter::<log_named_uint>()
        }
        ///Creates a new event filter for the [`log_string`] event.
        pub fn log_string_filter(&self) -> alloy_contract::Event<&P, log_string, N> {
            self.event_filter::<log_string>()
        }
        ///Creates a new event filter for the [`log_uint`] event.
        pub fn log_uint_filter(&self) -> alloy_contract::Event<&P, log_uint, N> {
            self.event_filter::<log_uint>()
        }
        ///Creates a new event filter for the [`logs`] event.
        pub fn logs_filter(&self) -> alloy_contract::Event<&P, logs, N> {
            self.event_filter::<logs>()
        }
    }
}
