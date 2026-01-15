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

interface VerseTest {
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
    function MARKET_HASH() external view returns (bytes32);
    function alice() external view returns (address);
    function asset() external view returns (address);
    function bob() external view returns (address);
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external view returns (bool);
    function noVerse() external view returns (address);
    function owner() external view returns (address);
    function setUp() external;
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function testBurnAsNonOwner() external;
    function testBurnAsOwner() external;
    function testConstructor() external;
    function testERC20Approve() external;
    function testERC20BasicFunctionality() external;
    function testFuzzApproveTransferFrom(uint256 mintAmount, uint256 approveAmount, uint256 transferAmount) external;
    function testFuzzMintBurn(address recipient, uint256 mintAmount, uint256 burnAmount) external;
    function testFuzzTransfer(uint256 mintAmount, uint256 transferAmount) external;
    function testMintAsNonOwner() external;
    function testMintAsOwner() external;
    function testMultipleMintsBurns() external;
    function testOwnershipTransfer() external;
    function testRenounceOwnership() external;
    function yesVerse() external view returns (address);
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
    "name": "MARKET_HASH",
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
    "name": "asset",
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
    "name": "noVerse",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract Verse"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "owner",
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
    "name": "testBurnAsNonOwner",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testBurnAsOwner",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testConstructor",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testERC20Approve",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testERC20BasicFunctionality",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testFuzzApproveTransferFrom",
    "inputs": [
      {
        "name": "mintAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "approveAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "transferAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testFuzzMintBurn",
    "inputs": [
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "mintAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "burnAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testFuzzTransfer",
    "inputs": [
      {
        "name": "mintAmount",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "transferAmount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testMintAsNonOwner",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testMintAsOwner",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testMultipleMintsBurns",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testOwnershipTransfer",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testRenounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "yesVerse",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract Verse"
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
pub mod VerseTest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x600c8054600160ff199182168117909255601f8054909116909117905560c0604052600560809081526437bbb732b960d91b60a05261003d906100f6565b602280546001600160a01b0319166001600160a01b0392909216919091179055604080518082019091526005815264616c69636560d81b6020820152610082906100f6565b602380546001600160a01b0319166001600160a01b03929092169190911790556040805180820190915260038152623137b160e91b60208201526100c5906100f6565b602480546001600160a01b0319166001600160a01b03929092169190911790553480156100f0575f5ffd5b5061029c565b5f61010082610107565b5092915050565b5f5f8260405160200161011a9190610215565b60408051808303601f190181529082905280516020909101206001625e79b760e01b03198252600482018190529150737109709ecfa91a80626ff3989d68f67f5b1dd12d9063ffa1864990602401602060405180830381865afa158015610183573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906101a7919061022b565b6040516318caf8e360e31b8152909250737109709ecfa91a80626ff3989d68f67f5b1dd12d9063c657c718906101e39085908790600401610258565b5f604051808303815f87803b1580156101fa575f5ffd5b505af115801561020c573d5f5f3e3d5ffd5b50505050915091565b5f82518060208501845e5f920191825250919050565b5f6020828403121561023b575f5ffd5b81516001600160a01b0381168114610251575f5ffd5b9392505050565b60018060a01b0383168152604060208201525f82518060408401528060208501606085015e5f606082850101526060601f19601f8301168401019150509392505050565b615efb806102a95f395ff3fe608060405234801561000f575f5ffd5b50600436106101f2575f3560e01c806385226c8111610114578063c0474d0b116100a9578063e115c64211610079578063e115c642146103b6578063e20c9f71146103c9578063f09d1d44146103d1578063fa7626d4146103d9578063fb47e3a2146103e6575f5ffd5b8063c0474d0b14610371578063c09cec7714610393578063c2e9f2e4146103a6578063d883ba03146103ae575f5ffd5b8063b0464fdc116100e4578063b0464fdc14610336578063b5508aa91461033e578063ba414fa614610346578063bf8452f51461035e575f5ffd5b806385226c81146102f15780638da5cb5b14610306578063916a17c6146103195780639a22b0541461032e575f5ffd5b80633c08bd5f1161018a578063575fc5d51161015a578063575fc5d5146102ae57806366d9a9a0146102b65780636e40056f146102cb57806374742c08146102de575f5ffd5b80633c08bd5f1461027e5780633e5e3c23146102865780633f7286f41461028e5780634f19b94114610296575f5ffd5b80632ade3880116101c55780632ade38801461022e5780632b903d9b1461024357806338d52e0f1461024b5780633b8ddf7714610276575f5ffd5b8063081c69e4146101f65780630a9254e414610200578063105cda16146102085780631ed7831c14610210575b5f5ffd5b6101fe6103f9565b005b6101fe61052e565b6101fe6107b1565b610218610a81565b6040516102259190613987565b60405180910390f35b610236610ae1565b6040516102259190613a00565b6101fe610c1d565b60215461025e906001600160a01b031681565b6040516001600160a01b039091168152602001610225565b6101fe611286565b6101fe6114c9565b6102186116a5565b610218611703565b601f5461025e9061010090046001600160a01b031681565b6101fe611761565b6102be611b5f565b6040516102259190613b0d565b6101fe6102d9366004613b8b565b611cc3565b60205461025e906001600160a01b031681565b6102f9611f8f565b6040516102259190613bab565b60225461025e906001600160a01b031681565b61032161205a565b6040516102259190613c02565b6101fe61213b565b61032161256e565b6102f961264f565b61034e61271a565b6040519015158152602001610225565b6101fe61036c366004613c79565b6127d4565b6103855f516020615ea65f395f51905f5281565b604051908152602001610225565b60245461025e906001600160a01b031681565b6101fe6129f0565b6101fe612ebb565b6101fe6103c4366004613cb6565b613025565b610218613308565b6101fe613366565b601f5461034e9060ff1681565b60235461025e906001600160a01b031681565b60235460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f529063ca669fa7906024015b5f604051808303815f87803b158015610449575f5ffd5b505af115801561045b573d5f5f3e3d5ffd5b505050505f516020615e865f395f51905f526001600160a01b031663f48448146040518163ffffffff1660e01b81526004015f604051808303815f87803b1580156104a4575f5ffd5b505af11580156104b6573d5f5f3e3d5ffd5b5050601f546023546040516340c10f1960e01b81526101009092046001600160a01b0390811694506340c10f1993506104ff9291169068056bc75e2d6310000090600401613ce8565b5f604051808303815f87803b158015610516575f5ffd5b505af1158015610528573d5f5f3e3d5ffd5b50505050565b601260405161053c9061396d565b6060808252600a908201526915195cdd08105cdcd95d60b21b608082015260a06020820181905260049082015263151154d560e21b60c082015260ff909116604082015260e001604051809103905ff08015801561059c573d5f5f3e3d5ffd5b50602180546001600160a01b0319166001600160a01b039283161790556022546040516303223eab60e11b8152911660048201525f516020615e865f395f51905f52906306447d56906024015f604051808303815f87803b1580156105ff575f5ffd5b505af1158015610611573d5f5f3e3d5ffd5b50506021546040516001600160a01b0390911692505f516020615ea65f395f51905f52915061063f9061397a565b608080825260039082018190526259657360e81b60a083015260c0602083018190528201526259455360e81b60e08201526001600160a01b039092166040830152606082015261010001604051809103905ff0801580156106a2573d5f5f3e3d5ffd5b50601f8054610100600160a81b0319166101006001600160a01b03938416021790556021546040519116905f516020615ea65f395f51905f52906106e59061397a565b60808082526002908201819052614e6f60f01b60a083015260c060208301819052820152614e4f60f01b60e08201526001600160a01b039092166040830152606082015261010001604051809103905ff080158015610746573d5f5f3e3d5ffd5b5060205f6101000a8154816001600160a01b0302191690836001600160a01b031602179055505f516020615e865f395f51905f526001600160a01b03166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015610516575f5ffd5b60225460405163ca669fa760e01b81526001600160a01b03909116600482015268056bc75e2d63100000906801a055690d9db80000905f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b158015610816575f5ffd5b505af1158015610828573d5f5f3e3d5ffd5b5050601f546023546040516340c10f1960e01b81526101009092046001600160a01b0390811694506340c10f199350610868929116908690600401613ce8565b5f604051808303815f87803b15801561087f575f5ffd5b505af1158015610891573d5f5f3e3d5ffd5b505060235460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f52925063ca669fa791506024015f604051808303815f87803b1580156108e4575f5ffd5b505af11580156108f6573d5f5f3e3d5ffd5b5050601f5460245460405163a9059cbb60e01b81526101009092046001600160a01b03908116945063a9059cbb9350610936929116908590600401613ce8565b6020604051808303815f875af1158015610952573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109769190613d01565b50601f546023546040516370a0823160e01b81526001600160a01b0391821660048201526109ff926101009004909116906370a08231906024015b602060405180830381865afa1580156109cc573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109f09190613d20565b6109fa8385613d4b565b6134e9565b601f54602480546040516370a0823160e01b81526001600160a01b039182166004820152610a7d936101009004909116916370a0823191015b602060405180830381865afa158015610a53573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a779190613d20565b826134e9565b5050565b60606016805480602002602001604051908101604052809291908181526020018280548015610ad757602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311610ab9575b5050505050905090565b6060601e805480602002602001604051908101604052809291908181526020015f905b82821015610c14575f84815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b82821015610bfd578382905f5260205f20018054610b7290613d64565b80601f0160208091040260200160405190810160405280929190818152602001828054610b9e90613d64565b8015610be95780601f10610bc057610100808354040283529160200191610be9565b820191905f5260205f20905b815481529060010190602001808311610bcc57829003601f168201915b505050505081526020019060010190610b55565b505050508152505081526020019060010190610b04565b50505050905090565b604080516003808252608082019092525f91602082016060803683370190505090506802b5e3af16b1880000815f81518110610c5b57610c5b613db0565b6020026020010181815250506801a055690d9db8000081600181518110610c8457610c84613db0565b6020026020010181815250506801158e460913d0000081600281518110610cad57610cad613db0565b6020908102919091010152604080516003808252608082019092525f91816020016020820280368337505060235482519293506001600160a01b0316918391505f90610cfb57610cfb613db0565b6001600160a01b039283166020918202929092010152602454825191169082906001908110610d2c57610d2c613db0565b60200260200101906001600160a01b031690816001600160a01b031681525050610d7460405180604001604052806007815260200166636861726c696560c81b81525061354d565b81600281518110610d8757610d87613db0565b6001600160a01b0392831660209182029290920101526022546040516303223eab60e11b8152911660048201525f516020615e865f395f51905f52906306447d56906024015f604051808303815f87803b158015610de3575f5ffd5b505af1158015610df5573d5f5f3e3d5ffd5b505f925050505b8151811015610eb357601f60019054906101000a90046001600160a01b03166001600160a01b03166340c10f19838381518110610e3b57610e3b613db0565b6020026020010151858481518110610e5557610e55613db0565b60200260200101516040518363ffffffff1660e01b8152600401610e7a929190613ce8565b5f604051808303815f87803b158015610e91575f5ffd5b505af1158015610ea3573d5f5f3e3d5ffd5b505060019092019150610dfc9050565b505f805b8251811015610fb957610f8a601f60019054906101000a90046001600160a01b03166001600160a01b03166370a08231858481518110610ef957610ef9613db0565b60200260200101516040518263ffffffff1660e01b8152600401610f2c91906001600160a01b0391909116815260200190565b602060405180830381865afa158015610f47573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f6b9190613d20565b858381518110610f7d57610f7d613db0565b60200260200101516134e9565b838181518110610f9c57610f9c613db0565b602002602001015182610faf9190613dc4565b9150600101610eb7565b5061100e601f60019054906101000a90046001600160a01b03166001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610a53573d5f5f3e3d5ffd5b678ac7230489e800005f5b83518110156110b757601f60019054906101000a90046001600160a01b03166001600160a01b0316639dc29fac85838151811061105857611058613db0565b6020026020010151846040518363ffffffff1660e01b815260040161107e929190613ce8565b5f604051808303815f87803b158015611095575f5ffd5b505af11580156110a7573d5f5f3e3d5ffd5b5050600190920191506110199050565b505f516020615e865f395f51905f526001600160a01b03166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b1580156110fd575f5ffd5b505af115801561110f573d5f5f3e3d5ffd5b505f925050505b83518110156111f7576111ef601f60019054906101000a90046001600160a01b03166001600160a01b03166370a0823186848151811061115857611158613db0565b60200260200101516040518263ffffffff1660e01b815260040161118b91906001600160a01b0391909116815260200190565b602060405180830381865afa1580156111a6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111ca9190613d20565b838784815181106111dd576111dd613db0565b60200260200101516109fa9190613d4b565b600101611116565b50610528601f60019054906101000a90046001600160a01b03166001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561124c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112709190613d20565b845161127c9084613dd7565b6109fa9085613d4b565b60225460405163ca669fa760e01b81526001600160a01b03909116600482015268056bc75e2d631000009068022b1c8c1227a00000905f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b1580156112eb575f5ffd5b505af11580156112fd573d5f5f3e3d5ffd5b5050601f546023546040516340c10f1960e01b81526101009092046001600160a01b0390811694506340c10f19935061133d929116908690600401613ce8565b5f604051808303815f87803b158015611354575f5ffd5b505af1158015611366573d5f5f3e3d5ffd5b505060225460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f52925063ca669fa791506024015f604051808303815f87803b1580156113b9575f5ffd5b505af11580156113cb573d5f5f3e3d5ffd5b5050601f54602354604051632770a7eb60e21b81526101009092046001600160a01b039081169450639dc29fac935061140b929116908590600401613ce8565b5f604051808303815f87803b158015611422575f5ffd5b505af1158015611434573d5f5f3e3d5ffd5b5050601f546023546040516370a0823160e01b81526001600160a01b03918216600482015261147594506101009092041691506370a08231906024016109b1565b610a7d601f60019054906101000a90046001600160a01b03166001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156109cc573d5f5f3e3d5ffd5b60225460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b158015611518575f5ffd5b505af115801561152a573d5f5f3e3d5ffd5b5050601f546023546040516340c10f1960e01b81526101009092046001600160a01b0390811694506340c10f1993506115739291169068056bc75e2d6310000090600401613ce8565b5f604051808303815f87803b15801561158a575f5ffd5b505af115801561159c573d5f5f3e3d5ffd5b505060235460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f52925063ca669fa791506024015f604051808303815f87803b1580156115ef575f5ffd5b505af1158015611601573d5f5f3e3d5ffd5b505050505f516020615e865f395f51905f526001600160a01b031663f48448146040518163ffffffff1660e01b81526004015f604051808303815f87803b15801561164a575f5ffd5b505af115801561165c573d5f5f3e3d5ffd5b5050601f54602354604051632770a7eb60e21b81526101009092046001600160a01b039081169450639dc29fac93506104ff929116906802b5e3af16b188000090600401613ce8565b60606018805480602002602001604051908101604052809291908181526020018280548015610ad757602002820191905f5260205f209081546001600160a01b03168152600190910190602001808311610ab9575050505050905090565b60606017805480602002602001604051908101604052809291908181526020018280548015610ad757602002820191905f5260205f209081546001600160a01b03168152600190910190602001808311610ab9575050505050905090565b5f61178b604051806040016040528060088152602001673732bba7bbb732b960c11b81525061354d565b60225460405163ca669fa760e01b81526001600160a01b0390911660048201529091505f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b1580156117dd575f5ffd5b505af11580156117ef573d5f5f3e3d5ffd5b5050601f5460405163f2fde38b60e01b81526001600160a01b038581166004830152610100909204909116925063f2fde38b91506024015f604051808303815f87803b15801561183d575f5ffd5b505af115801561184f573d5f5f3e3d5ffd5b505050506118d1601f60019054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156118a7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118cb9190613dee565b8261355e565b60225460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b158015611920575f5ffd5b505af1158015611932573d5f5f3e3d5ffd5b505050505f516020615e865f395f51905f526001600160a01b031663f48448146040518163ffffffff1660e01b81526004015f604051808303815f87803b15801561197b575f5ffd5b505af115801561198d573d5f5f3e3d5ffd5b5050601f546023546040516340c10f1960e01b81526101009092046001600160a01b0390811694506340c10f1993506119d69291169068056bc75e2d6310000090600401613ce8565b5f604051808303815f87803b1580156119ed575f5ffd5b505af11580156119ff573d5f5f3e3d5ffd5b505060405163ca669fa760e01b81526001600160a01b03841660048201525f516020615e865f395f51905f52925063ca669fa791506024015f604051808303815f87803b158015611a4e575f5ffd5b505af1158015611a60573d5f5f3e3d5ffd5b5050601f546023546040516340c10f1960e01b81526101009092046001600160a01b0390811694506340c10f199350611aa99291169068056bc75e2d6310000090600401613ce8565b5f604051808303815f87803b158015611ac0575f5ffd5b505af1158015611ad2573d5f5f3e3d5ffd5b5050601f546023546040516370a0823160e01b81526001600160a01b039182166004820152611b5c94506101009092041691506370a0823190602401602060405180830381865afa158015611b29573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b4d9190613d20565b68056bc75e2d631000006134e9565b50565b6060601b805480602002602001604051908101604052809291908181526020015f905b82821015610c14578382905f5260205f2090600202016040518060400160405290815f82018054611bb290613d64565b80601f0160208091040260200160405190810160405280929190818152602001828054611bde90613d64565b8015611c295780601f10611c0057610100808354040283529160200191611c29565b820191905f5260205f20905b815481529060010190602001808311611c0c57829003601f168201915b5050505050815260200160018201805480602002602001604051908101604052809291908181526020018280548015611cab57602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b03191681526020019060040190602082600301049283019260010382029150808411611c6d5790505b50505050508152505081526020019060010190611b82565b611cd68260016001600160801b0361359f565b9150611ce3815f8461359f565b60225460405163ca669fa760e01b81526001600160a01b0390911660048201529091505f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b158015611d35575f5ffd5b505af1158015611d47573d5f5f3e3d5ffd5b5050601f546023546040516340c10f1960e01b81526101009092046001600160a01b0390811694506340c10f199350611d87929116908690600401613ce8565b5f604051808303815f87803b158015611d9e575f5ffd5b505af1158015611db0573d5f5f3e3d5ffd5b505060235460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f52925063ca669fa791506024015f604051808303815f87803b158015611e03575f5ffd5b505af1158015611e15573d5f5f3e3d5ffd5b5050601f5460245460405163a9059cbb60e01b81526101009092046001600160a01b03908116945063a9059cbb9350611e55929116908590600401613ce8565b6020604051808303815f875af1158015611e71573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e959190613d01565b50601f546023546040516370a0823160e01b81526001600160a01b039182166004820152611ed4926101009004909116906370a08231906024016109b1565b601f54602480546040516370a0823160e01b81526001600160a01b039182166004820152611f11936101009004909116916370a082319101610a38565b610a7d601f60019054906101000a90046001600160a01b03166001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611f65573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f899190613d20565b836134e9565b6060601a805480602002602001604051908101604052809291908181526020015f905b82821015610c14578382905f5260205f20018054611fcf90613d64565b80601f0160208091040260200160405190810160405280929190818152602001828054611ffb90613d64565b80156120465780601f1061201d57610100808354040283529160200191612046565b820191905f5260205f20905b81548152906001019060200180831161202957829003601f168201915b505050505081526020019060010190611fb2565b6060601d805480602002602001604051908101604052809291908181526020015f905b82821015610c14575f8481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561212357602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116120e55790505b5050505050815250508152602001906001019061207d565b60225460405163ca669fa760e01b81526001600160a01b03909116600482015268056bc75e2d63100000906802b5e3af16b1880000906801a055690d9db80000905f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b1580156121ab575f5ffd5b505af11580156121bd573d5f5f3e3d5ffd5b5050601f546023546040516340c10f1960e01b81526101009092046001600160a01b0390811694506340c10f1993506121fd929116908790600401613ce8565b5f604051808303815f87803b158015612214575f5ffd5b505af1158015612226573d5f5f3e3d5ffd5b505060235460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f52925063ca669fa791506024015f604051808303815f87803b158015612279575f5ffd5b505af115801561228b573d5f5f3e3d5ffd5b5050601f5460245460405163095ea7b360e01b81526101009092046001600160a01b03908116945063095ea7b393506122cb929116908690600401613ce8565b6020604051808303815f875af11580156122e7573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061230b9190613d01565b50601f5460235460248054604051636eb1769f60e11b81526001600160a01b0393841660048201529083169181019190915261236f9261010090049091169063dd62ed3e906044015b602060405180830381865afa158015611f65573d5f5f3e3d5ffd5b6024805460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f529163ca669fa791015b5f604051808303815f87803b1580156123be575f5ffd5b505af11580156123d0573d5f5f3e3d5ffd5b5050601f54602354602480546040516323b872dd60e01b81526001600160a01b03938416600482015290831691810191909152604481018690526101009092041692506323b872dd91506064016020604051808303815f875af1158015612439573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061245d9190613d01565b50601f546023546040516370a0823160e01b81526001600160a01b0391821660048201526124e0926101009004909116906370a0823190602401602060405180830381865afa1580156124b2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906124d69190613d20565b6109fa8386613d4b565b601f54602480546040516370a0823160e01b81526001600160a01b03918216600482015261251d936101009004909116916370a082319101610a38565b601f5460235460248054604051636eb1769f60e11b81526001600160a01b039384166004820152908316918101919091526125699261010090049091169063dd62ed3e906044016109b1565b505050565b6060601c805480602002602001604051908101604052809291908181526020015f905b82821015610c14575f8481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561263757602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116125f95790505b50505050508152505081526020019060010190612591565b60606019805480602002602001604051908101604052809291908181526020015f905b82821015610c14578382905f5260205f2001805461268f90613d64565b80601f01602080910402602001604051908101604052809291908181526020018280546126bb90613d64565b80156127065780601f106126dd57610100808354040283529160200191612706565b820191905f5260205f20905b8154815290600101906020018083116126e957829003601f168201915b505050505081526020019060010190612672565b6008545f9060ff1615612731575060085460ff1690565b604051630667f9d760e41b81525f905f516020615e865f395f51905f529063667f9d709061278e907f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d906519985a5b195960d21b90600401613ce8565b602060405180830381865afa1580156127a9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906127cd9190613d20565b1415905090565b6127e78360016001600160801b0361359f565b92506127f4825f8561359f565b9150612801815f8461359f565b60225460405163ca669fa760e01b81526001600160a01b0390911660048201529091505f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b158015612853575f5ffd5b505af1158015612865573d5f5f3e3d5ffd5b5050601f546023546040516340c10f1960e01b81526101009092046001600160a01b0390811694506340c10f1993506128a5929116908790600401613ce8565b5f604051808303815f87803b1580156128bc575f5ffd5b505af11580156128ce573d5f5f3e3d5ffd5b505060235460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f52925063ca669fa791506024015f604051808303815f87803b158015612921575f5ffd5b505af1158015612933573d5f5f3e3d5ffd5b5050601f5460245460405163095ea7b360e01b81526101009092046001600160a01b03908116945063095ea7b39350612973929116908690600401613ce8565b6020604051808303815f875af115801561298f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129b39190613d01565b506024805460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f529163ca669fa791016123a7565b612a8b601f60019054906101000a90046001600160a01b03166001600160a01b03166306fdde036040518163ffffffff1660e01b81526004015f60405180830381865afa158015612a43573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612a6a9190810190613e09565b6040518060400160405280600381526020016259657360e81b8152506135e2565b612b26601f60019054906101000a90046001600160a01b03166001600160a01b03166395d89b416040518163ffffffff1660e01b81526004015f60405180830381865afa158015612ade573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612b059190810190613e09565b6040518060400160405280600381526020016259455360e81b8152506135e2565b612baf601f60019054906101000a90046001600160a01b03166001600160a01b0316634800d97f6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612b7a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b9e9190613dee565b6021546001600160a01b031661355e565b612c3a601f60019054906101000a90046001600160a01b03166001600160a01b031663c0474d0b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612c03573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612c279190613d20565b5f516020615ea65f395f51905f52613614565b612cc3601f60019054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612c8e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612cb29190613dee565b6022546001600160a01b031661355e565b602054604080516306fdde0360e01b81529051612d52926001600160a01b0316916306fdde03916004808301925f9291908290030181865afa158015612d0b573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612d329190810190613e09565b604051806040016040528060028152602001614e6f60f01b8152506135e2565b602054604080516395d89b4160e01b81529051612de1926001600160a01b0316916395d89b41916004808301925f9291908290030181865afa158015612d9a573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612dc19190810190613e09565b604051806040016040528060028152602001614e4f60f01b8152506135e2565b6020805460408051634800d97f60e01b81529051612e29936001600160a01b0390931692634800d97f92600480820193918290030181865afa158015612b7a573d5f5f3e3d5ffd5b602080546040805163c0474d0b60e01b81529051612e71936001600160a01b039093169263c0474d0b92600480820193918290030181865afa158015612c03573d5f5f3e3d5ffd5b6020805460408051638da5cb5b60e01b81529051612eb9936001600160a01b0390931692638da5cb5b92600480820193918290030181865afa158015612c8e573d5f5f3e3d5ffd5b565b60225460405163ca669fa760e01b81526001600160a01b03909116600482015268056bc75e2d63100000905f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b158015612f15575f5ffd5b505af1158015612f27573d5f5f3e3d5ffd5b5050601f546023546040516340c10f1960e01b81526101009092046001600160a01b0390811694506340c10f199350612f67929116908590600401613ce8565b5f604051808303815f87803b158015612f7e575f5ffd5b505af1158015612f90573d5f5f3e3d5ffd5b5050601f546023546040516370a0823160e01b81526001600160a01b039182166004820152612fd194506101009092041691506370a0823190602401610a38565b611b5c601f60019054906101000a90046001600160a01b03166001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610a53573d5f5f3e3d5ffd5b604051632631f2b160e11b81526001600160a01b038416151560048201525f516020615e865f395f51905f5290634c63e562906024015f6040518083038186803b158015613071575f5ffd5b505afa158015613083573d5f5f3e3d5ffd5b5050505061309c8260016001600160801b03801661359f565b91506130a9815f8461359f565b60225460405163ca669fa760e01b81526001600160a01b0390911660048201529091505f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b1580156130fb575f5ffd5b505af115801561310d573d5f5f3e3d5ffd5b5050601f546040516340c10f1960e01b81526101009091046001600160a01b031692506340c10f1991506131479086908690600401613ce8565b5f604051808303815f87803b15801561315e575f5ffd5b505af1158015613170573d5f5f3e3d5ffd5b5050601f546040516370a0823160e01b81526001600160a01b0387811660048301526131b0945061010090920490911691506370a0823190602401612354565b60225460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b1580156131ff575f5ffd5b505af1158015613211573d5f5f3e3d5ffd5b5050601f54604051632770a7eb60e21b81526101009091046001600160a01b03169250639dc29fac915061324b9086908590600401613ce8565b5f604051808303815f87803b158015613262575f5ffd5b505af1158015613274573d5f5f3e3d5ffd5b5050601f546040516370a0823160e01b81526001600160a01b0387811660048301526132b4945061010090920490911691506370a08231906024016109b1565b612569601f60019054906101000a90046001600160a01b03166001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156109cc573d5f5f3e3d5ffd5b60606015805480602002602001604051908101604052809291908181526020018280548015610ad757602002820191905f5260205f209081546001600160a01b03168152600190910190602001808311610ab9575050505050905090565b60225460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b1580156133b5575f5ffd5b505af11580156133c7573d5f5f3e3d5ffd5b50505050601f60019054906101000a90046001600160a01b03166001600160a01b031663715018a66040518163ffffffff1660e01b81526004015f604051808303815f87803b158015613418575f5ffd5b505af115801561342a573d5f5f3e3d5ffd5b505050506134ac601f60019054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613482573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906134a69190613dee565b5f61355e565b60225460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f529063ca669fa790602401610432565b60405163260a5b1560e21b815260048101839052602481018290525f516020615e865f395f51905f52906398296c54906044015b5f6040518083038186803b158015613533575f5ffd5b505afa158015613545573d5f5f3e3d5ffd5b505050505050565b5f6135578261364c565b5092915050565b6040516328a9b0fb60e11b81526001600160a01b038084166004830152821660248201525f516020615e865f395f51905f529063515361f69060440161351d565b5f6135ab84848461374c565b90506135db6040518060400160405280600c81526020016b109bdd5b99081c995cdd5b1d60a21b81525082613909565b9392505050565b60405163f320d96360e01b81525f516020615e865f395f51905f529063f320d9639061351d9085908590600401613ebc565b604051637c84c69b60e01b815260048101839052602481018290525f516020615e865f395f51905f5290637c84c69b9060440161351d565b5f5f8260405160200161365f9190613ee9565b60408051808303601f190181529082905280516020909101206001625e79b760e01b031982526004820181905291505f516020615e865f395f51905f529063ffa1864990602401602060405180830381865afa1580156136c1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906136e59190613dee565b6040516318caf8e360e31b81529092505f516020615e865f395f51905f529063c657c7189061371a9085908790600401613eff565b5f604051808303815f87803b158015613731575f5ffd5b505af1158015613743573d5f5f3e3d5ffd5b50505050915091565b5f818311156137c75760405162461bcd60e51b815260206004820152603e60248201527f5374645574696c7320626f756e642875696e743235362c75696e743235362c7560448201527f696e74323536293a204d6178206973206c657373207468616e206d696e2e0000606482015260840160405180910390fd5b8284101580156137d75750818411155b156137e35750826135db565b5f6137ee8484613d4b565b6137f9906001613dc4565b90506003851115801561380b57508481115b156138225761381a8585613dc4565b9150506135db565b61382e60035f19613d4b565b85101580156138465750613843855f19613d4b565b81115b1561386057613856855f19613d4b565b61381a9084613d4b565b828511156138b3575f6138738487613d4b565b90505f6138808383613f2a565b9050805f03613894578493505050506135db565b60016138a08288613dc4565b6138aa9190613d4b565b93505050613901565b83851015613901575f6138c68686613d4b565b90505f6138d38383613f2a565b9050805f036138e7578593505050506135db565b6138f18186613d4b565b6138fc906001613dc4565b935050505b509392505050565b610a7d828260405160240161391f929190613f49565b60408051601f198184030181529190526020810180516001600160e01b0316632d839cb360e21b179052611b5c8180516a636f6e736f6c652e6c6f67602083015f808483855afa5050505050565b610ed480613f6b83390190565b61104780614e3f83390190565b602080825282518282018190525f918401906040840190835b818110156139c75783516001600160a01b03168352602093840193909201916001016139a0565b509095945050505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015613abd57603f19878603018452815180516001600160a01b03168652602090810151604082880181905281519088018190529101906060600582901b8801810191908801905f5b81811015613aa357605f198a8503018352613a8d8486516139d2565b6020958601959094509290920191600101613a71565b509197505050602094850194929092019150600101613a26565b50929695505050505050565b5f8151808452602084019350602083015f5b82811015613b035781516001600160e01b031916865260209586019590910190600101613adb565b5093949350505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015613abd57603f198786030184528151805160408752613b5960408801826139d2565b9050602082015191508681036020880152613b748183613ac9565b965050506020938401939190910190600101613b33565b5f5f60408385031215613b9c575f5ffd5b50508035926020909101359150565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015613abd57603f19878603018452613bed8583516139d2565b94506020938401939190910190600101613bd1565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015613abd57868503603f19018452815180516001600160a01b03168652602090810151604091870182905290613c6390870182613ac9565b9550506020938401939190910190600101613c28565b5f5f5f60608486031215613c8b575f5ffd5b505081359360208301359350604090920135919050565b6001600160a01b0381168114611b5c575f5ffd5b5f5f5f60608486031215613cc8575f5ffd5b8335613cd381613ca2565b95602085013595506040909401359392505050565b6001600160a01b03929092168252602082015260400190565b5f60208284031215613d11575f5ffd5b815180151581146135db575f5ffd5b5f60208284031215613d30575f5ffd5b5051919050565b634e487b7160e01b5f52601160045260245ffd5b81810381811115613d5e57613d5e613d37565b92915050565b600181811c90821680613d7857607f821691505b602082108103613d9657634e487b7160e01b5f52602260045260245ffd5b50919050565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b80820180821115613d5e57613d5e613d37565b8082028115828204841417613d5e57613d5e613d37565b5f60208284031215613dfe575f5ffd5b81516135db81613ca2565b5f60208284031215613e19575f5ffd5b815167ffffffffffffffff811115613e2f575f5ffd5b8201601f81018413613e3f575f5ffd5b805167ffffffffffffffff811115613e5957613e59613d9c565b604051601f8201601f19908116603f0116810167ffffffffffffffff81118282101715613e8857613e88613d9c565b604052818152828201602001861015613e9f575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b604081525f613ece60408301856139d2565b8281036020840152613ee081856139d2565b95945050505050565b5f82518060208501845e5f920191825250919050565b6001600160a01b03831681526040602082018190525f90613f22908301846139d2565b949350505050565b5f82613f4457634e487b7160e01b5f52601260045260245ffd5b500690565b604081525f613f5b60408301856139d2565b9050826020830152939250505056fe60a060405234801561000f575f5ffd5b50604051610ed4380380610ed483398101604081905261002e91610109565b5f610039848261020a565b506001610046838261020a565b506002805460ff191660ff929092169190911790555080516020909101206080526102c4565b634e487b7160e01b5f52604160045260245ffd5b5f82601f83011261008f575f5ffd5b81516001600160401b038111156100a8576100a861006c565b604051601f8201601f19908116603f011681016001600160401b03811182821017156100d6576100d661006c565b6040528181528382016020018510156100ed575f5ffd5b8160208501602083015e5f918101602001919091529392505050565b5f5f5f6060848603121561011b575f5ffd5b83516001600160401b03811115610130575f5ffd5b61013c86828701610080565b602086015190945090506001600160401b03811115610159575f5ffd5b61016586828701610080565b925050604084015160ff8116811461017b575f5ffd5b809150509250925092565b600181811c9082168061019a57607f821691505b6020821081036101b857634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561020557805f5260205f20601f840160051c810160208510156101e35750805b601f840160051c820191505b81811015610202575f81556001016101ef565b50505b505050565b81516001600160401b038111156102235761022361006c565b610237816102318454610186565b846101be565b6020601f821160018114610269575f83156102525750848201515b5f19600385901b1c1916600184901b178455610202565b5f84815260208120601f198516915b828110156102985787850151825560209485019460019092019101610278565b50848210156102b557868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b608051610bf16102e35f395f818161039901526104db0152610bf15ff3fe608060405234801561000f575f5ffd5b50600436106100fb575f3560e01c80637ecebe0011610093578063d30ed3b311610063578063d30ed3b314610217578063d505accf1461022a578063dd62ed3e1461023d578063f83d179114610250575f5ffd5b80637ecebe00146101c457806395d89b41146101e95780639dc29fac146101f1578063a9059cbb14610204575f5ffd5b8063313ce567116100ce578063313ce5671461016d5780633644e5151461018257806340c10f191461018a57806370a082311461019f575f5ffd5b806306fdde03146100ff578063095ea7b31461011d57806318160ddd1461014057806323b872dd1461015a575b5f5ffd5b610107610263565b60405161011491906109ff565b60405180910390f35b61013061012b366004610a4a565b6102f2565b6040519015158152602001610114565b6805345cdf77eb68f44c545b604051908152602001610114565b610130610168366004610a72565b610372565b60025460405160ff9091168152602001610114565b61014c610396565b61019d610198366004610a4a565b610438565b005b61014c6101ad366004610aac565b6387a211a2600c9081525f91909152602090205490565b61014c6101d2366004610aac565b6338377508600c9081525f91909152602090205490565b61010761044e565b61019d6101ff366004610a4a565b61045d565b610130610212366004610a4a565b61046f565b61019d610225366004610a72565b610489565b61019d610238366004610ac5565b6104a9565b61014c61024b366004610b32565b610683565b61019d61025e366004610a72565b6106c7565b60605f805461027190610b63565b80601f016020809104026020016040519081016040528092919081815260200182805461029d90610b63565b80156102e85780601f106102bf576101008083540402835291602001916102e8565b820191905f5260205f20905b8154815290600101906020018083116102cb57829003601f168201915b5050505050905090565b5f6001600160a01b0383166e22d473030f116ddee9f6b43ac78ba3188219151761032357633f68539a5f526004601cfd5b82602052637f5e9f20600c52335f52816034600c2055815f52602c5160601c337f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560205fa35060015b92915050565b5f61038e61037f856106e2565b610388856106e2565b84610703565b949350505050565b5f7f0000000000000000000000000000000000000000000000000000000000000000806103cf576103c5610263565b8051906020012090505b604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f815260208101929092527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc69082015246606082015230608082015260a09020919050565b61044a610444836106e2565b826107bf565b5050565b60606001805461027190610b63565b61044a610469836106e2565b82610828565b5f61048261047c846106e2565b83610889565b9392505050565b6104a4610495846106e2565b61049e846106e2565b836108ed565b505050565b6001600160a01b0386166e22d473030f116ddee9f6b43ac78ba318851915176104d957633f68539a5f526004601cfd5b7f00000000000000000000000000000000000000000000000000000000000000008061051157610507610263565b8051906020012090505b7fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc64286101561054757631a15a3cc5f526004601cfd5b6040518960601b60601c99508860601b60601c985065383775081901600e52895f526020600c2080547f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f835284602084015283604084015246606084015230608084015260a08320602e527f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c983528b60208401528a60408401528960608401528060808401528860a084015260c08320604e526042602c205f528760ff16602052866040528560605260208060805f60015afa8c3d511461062f5763ddafbaef5f526004601cfd5b0190556303faf4f960a51b89176040526034602c20889055888a7f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925602060608501a360405250505f60605250505050505050565b5f6e22d473030f116ddee9f6b43ac78ba2196001600160a01b038316016106ac57505f1961036c565b50602052637f5e9f20600c9081525f91909152603490205490565b6104a46106d3846106e2565b6106dc846106e2565b83610951565b5f6001600160a01b0382168060a06106f9826109b7565b901b189392505050565b5f8360601b6e22d473030f116ddee9f6b43ac78ba333146107585733602052637f5e9f208117600c526034600c208054801915610755578085111561074f576313be252b5f526004601cfd5b84810382555b50505b6387a211a28117600c526020600c2080548085111561077e5763f4d678b85f526004601cfd5b84810382555050835f526020600c208381540181555082602052600c5160601c8160601c5f516020610b9c5f395f51905f52602080a3505060019392505050565b6805345cdf77eb68f44c54818101818110156107e25763e5cfe9575f526004601cfd5b806805345cdf77eb68f44c5550506387a211a2600c52815f526020600c208181540181555080602052600c5160601c5f5f516020610b9c5f395f51905f52602080a35050565b6387a211a2600c52815f526020600c2080548083111561084f5763f4d678b85f526004601cfd5b82900390556805345cdf77eb68f44c805482900390555f8181526001600160a01b0383165f516020610b9c5f395f51905f52602083a35050565b5f6387a211a2600c52335f526020600c208054808411156108b15763f4d678b85f526004601cfd5b83810382555050825f526020600c208281540181555081602052600c5160601c335f516020610b9c5f395f51905f52602080a350600192915050565b6e22d473030f116ddee9f6b43ac78ba2196001600160a01b0383160161091257505050565b81602052637f5e9f20600c52825f526034600c20805480191561094a5780831115610944576313be252b5f526004601cfd5b82810382555b5050505050565b8260601b6387a211a28117600c526020600c2080548084111561097b5763f4d678b85f526004601cfd5b83810382555050825f526020600c208281540181555081602052600c5160601c8160601c5f516020610b9c5f395f51905f52602080a350505050565b604051365f8237368120602052601051821860105260885f2090508060105260bc19700100000000000000000000000000000051820960801c6007166109fa57505f5b919050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b80356001600160a01b03811681146109fa575f5ffd5b5f5f60408385031215610a5b575f5ffd5b610a6483610a34565b946020939093013593505050565b5f5f5f60608486031215610a84575f5ffd5b610a8d84610a34565b9250610a9b60208501610a34565b929592945050506040919091013590565b5f60208284031215610abc575f5ffd5b61048282610a34565b5f5f5f5f5f5f5f60e0888a031215610adb575f5ffd5b610ae488610a34565b9650610af260208901610a34565b95506040880135945060608801359350608088013560ff81168114610b15575f5ffd5b9699959850939692959460a0840135945060c09093013592915050565b5f5f60408385031215610b43575f5ffd5b610b4c83610a34565b9150610b5a60208401610a34565b90509250929050565b600181811c90821680610b7757607f821691505b602082108103610b9557634e487b7160e01b5f52602260045260245ffd5b5091905056feddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa26469706673582212200c45acc3c6a9d15890ebba8720cb2063e27c6daab33079386bb3dcaecfa64cfb64736f6c634300081e003360c060405234801561000f575f5ffd5b5060405161104738038061104783398101604081905261002e91610143565b5f6100398582610252565b5060016100468482610252565b506001600160a01b03821660805260a08190526100623361006b565b5050505061030c565b6001600160a01b0316638b78c6d819819055805f7f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08180a350565b634e487b7160e01b5f52604160045260245ffd5b5f82601f8301126100c9575f5ffd5b81516001600160401b038111156100e2576100e26100a6565b604051601f8201601f19908116603f011681016001600160401b0381118282101715610110576101106100a6565b604052818152838201602001851015610127575f5ffd5b8160208501602083015e5f918101602001919091529392505050565b5f5f5f5f60808587031215610156575f5ffd5b84516001600160401b0381111561016b575f5ffd5b610177878288016100ba565b602087015190955090506001600160401b03811115610194575f5ffd5b6101a0878288016100ba565b604087015190945090506001600160a01b03811681146101be575f5ffd5b6060959095015193969295505050565b600181811c908216806101e257607f821691505b60208210810361020057634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561024d57805f5260205f20601f840160051c8101602085101561022b5750805b601f840160051c820191505b8181101561024a575f8155600101610237565b50505b505050565b81516001600160401b0381111561026b5761026b6100a6565b61027f8161027984546101ce565b84610206565b6020601f8211600181146102b1575f831561029a5750848201515b5f19600385901b1c1916600184901b17845561024a565b5f84815260208120601f198516915b828110156102e057878501518255602094850194600190920191016102c0565b50848210156102fd57868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b60805160a051610d1a61032d5f395f61036e01525f6102470152610d1a5ff3fe60806040526004361061013c575f3560e01c8063715018a6116100b3578063c0474d0b1161006d578063c0474d0b1461035d578063d505accf14610390578063dd62ed3e146103af578063f04e283e146103ce578063f2fde38b146103e1578063fee81cf4146103f4575f5ffd5b8063715018a6146102ba5780637ecebe00146102c25780638da5cb5b146102f357806395d89b411461030b5780639dc29fac1461031f578063a9059cbb1461033e575f5ffd5b8063313ce56711610104578063313ce567146101e85780633644e5151461020357806340c10f19146102175780634800d97f1461023657806354d1f13d1461028157806370a0823114610289575f5ffd5b806306fdde0314610140578063095ea7b31461016a57806318160ddd1461019957806323b872dd146101bf57806325692962146101de575b5f5ffd5b34801561014b575f5ffd5b50610154610425565b6040516101619190610b1c565b60405180910390f35b348015610175575f5ffd5b50610189610184366004610b6c565b6104b4565b6040519015158152602001610161565b3480156101a4575f5ffd5b506805345cdf77eb68f44c545b604051908152602001610161565b3480156101ca575f5ffd5b506101896101d9366004610b94565b610534565b6101e66105f0565b005b3480156101f3575f5ffd5b5060405160128152602001610161565b34801561020e575f5ffd5b506101b161063d565b348015610222575f5ffd5b506101e6610231366004610b6c565b6106b9565b348015610241575f5ffd5b506102697f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610161565b6101e66106cf565b348015610294575f5ffd5b506101b16102a3366004610bce565b6387a211a2600c9081525f91909152602090205490565b6101e6610708565b3480156102cd575f5ffd5b506101b16102dc366004610bce565b6338377508600c9081525f91909152602090205490565b3480156102fe575f5ffd5b50638b78c6d81954610269565b348015610316575f5ffd5b5061015461071b565b34801561032a575f5ffd5b506101e6610339366004610b6c565b61072a565b348015610349575f5ffd5b50610189610358366004610b6c565b61073c565b348015610368575f5ffd5b506101b17f000000000000000000000000000000000000000000000000000000000000000081565b34801561039b575f5ffd5b506101e66103aa366004610bee565b6107a0565b3480156103ba575f5ffd5b506101b16103c9366004610c5b565b610954565b6101e66103dc366004610bce565b610998565b6101e66103ef366004610bce565b6109d5565b3480156103ff575f5ffd5b506101b161040e366004610bce565b63389a75e1600c9081525f91909152602090205490565b60605f805461043390610c8c565b80601f016020809104026020016040519081016040528092919081815260200182805461045f90610c8c565b80156104aa5780601f10610481576101008083540402835291602001916104aa565b820191905f5260205f20905b81548152906001019060200180831161048d57829003601f168201915b5050505050905090565b5f6001600160a01b0383166e22d473030f116ddee9f6b43ac78ba318821915176104e557633f68539a5f526004601cfd5b82602052637f5e9f20600c52335f52816034600c2055815f52602c5160601c337f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560205fa35060015b92915050565b5f8360601b6e22d473030f116ddee9f6b43ac78ba333146105895733602052637f5e9f208117600c526034600c2080548019156105865780851115610580576313be252b5f526004601cfd5b84810382555b50505b6387a211a28117600c526020600c208054808511156105af5763f4d678b85f526004601cfd5b84810382555050835f526020600c208381540181555082602052600c5160601c8160601c5f516020610cc55f395f51905f52602080a3505060019392505050565b5f6202a30067ffffffffffffffff164201905063389a75e1600c52335f52806020600c2055337fdbf36a107da19e49527a7176a1babf963b4b0ff8cde35ee35d6cd8f1f9ac7e1d5f5fa250565b5f80610647610425565b805190602001209050604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f815260208101929092527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc69082015246606082015230608082015260a09020919050565b6106c16109fb565b6106cb8282610a15565b5050565b63389a75e1600c52335f525f6020600c2055337ffa7b8eab7da67f412cc9575ed43464468f9bfbae89d1675917346ca6d8fe3c925f5fa2565b6107106109fb565b6107195f610a7e565b565b60606001805461043390610c8c565b6107326109fb565b6106cb8282610abb565b5f6387a211a2600c52335f526020600c208054808411156107645763f4d678b85f526004601cfd5b83810382555050825f526020600c208281540181555081602052600c5160601c335f516020610cc55f395f51905f52602080a350600192915050565b6001600160a01b0386166e22d473030f116ddee9f6b43ac78ba318851915176107d057633f68539a5f526004601cfd5b5f6107d9610425565b8051906020012090507fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc64286101561081857631a15a3cc5f526004601cfd5b6040518960601b60601c99508860601b60601c985065383775081901600e52895f526020600c2080547f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f835284602084015283604084015246606084015230608084015260a08320602e527f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c983528b60208401528a60408401528960608401528060808401528860a084015260c08320604e526042602c205f528760ff16602052866040528560605260208060805f60015afa8c3d51146109005763ddafbaef5f526004601cfd5b0190556303faf4f960a51b89176040526034602c20889055888a7f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925602060608501a360405250505f60605250505050505050565b5f6e22d473030f116ddee9f6b43ac78ba2196001600160a01b0383160161097d57505f1961052e565b50602052637f5e9f20600c9081525f91909152603490205490565b6109a06109fb565b63389a75e1600c52805f526020600c2080544211156109c657636f5e88185f526004601cfd5b5f90556109d281610a7e565b50565b6109dd6109fb565b8060601b6109f257637448fbae5f526004601cfd5b6109d281610a7e565b638b78c6d819543314610719576382b429005f526004601cfd5b6805345cdf77eb68f44c5481810181811015610a385763e5cfe9575f526004601cfd5b806805345cdf77eb68f44c5550506387a211a2600c52815f526020600c208181540181555080602052600c5160601c5f5f516020610cc55f395f51905f52602080a35050565b638b78c6d81980546001600160a01b039092169182907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a355565b6387a211a2600c52815f526020600c20805480831115610ae25763f4d678b85f526004601cfd5b82900390556805345cdf77eb68f44c805482900390555f8181526001600160a01b0383165f516020610cc55f395f51905f52602083a35050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b80356001600160a01b0381168114610b67575f5ffd5b919050565b5f5f60408385031215610b7d575f5ffd5b610b8683610b51565b946020939093013593505050565b5f5f5f60608486031215610ba6575f5ffd5b610baf84610b51565b9250610bbd60208501610b51565b929592945050506040919091013590565b5f60208284031215610bde575f5ffd5b610be782610b51565b9392505050565b5f5f5f5f5f5f5f60e0888a031215610c04575f5ffd5b610c0d88610b51565b9650610c1b60208901610b51565b95506040880135945060608801359350608088013560ff81168114610c3e575f5ffd5b9699959850939692959460a0840135945060c09093013592915050565b5f5f60408385031215610c6c575f5ffd5b610c7583610b51565b9150610c8360208401610b51565b90509250929050565b600181811c90821680610ca057607f821691505b602082108103610cbe57634e487b7160e01b5f52602260045260245ffd5b5091905056feddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa2646970667358221220f2558ad4460ca00ebebab8d02372dd6b027f19de7bd9d732ad74c5cf24635f4564736f6c634300081e00330000000000000000000000007109709ecfa91a80626ff3989d68f67f5b1dd12ded225bd385902e728b5dfa2f4bcd102bd0f4bb7ca0be24b587b96ebfd5fc2a64a2646970667358221220a9b7b20668fbda7df982ae4eb85b18057391ab06e4507ff202d1b1af3e3b6a2f64736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x0C\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x1F\x80T\x90\x91\x16\x90\x91\x17\x90U`\xC0`@R`\x05`\x80\x90\x81Rd7\xBB\xB72\xB9`\xD9\x1B`\xA0Ra\0=\x90a\0\xF6V[`\"\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80Q\x80\x82\x01\x90\x91R`\x05\x81Rdalice`\xD8\x1B` \x82\x01Ra\0\x82\x90a\0\xF6V[`#\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@\x80Q\x80\x82\x01\x90\x91R`\x03\x81Rb17\xB1`\xE9\x1B` \x82\x01Ra\0\xC5\x90a\0\xF6V[`$\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U4\x80\x15a\0\xF0W__\xFD[Pa\x02\x9CV[_a\x01\0\x82a\x01\x07V[P\x92\x91PPV[__\x82`@Q` \x01a\x01\x1A\x91\x90a\x02\x15V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01b^y\xB7`\xE0\x1B\x03\x19\x82R`\x04\x82\x01\x81\x90R\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xFF\xA1\x86I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x83W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xA7\x91\x90a\x02+V[`@Qc\x18\xCA\xF8\xE3`\xE3\x1B\x81R\x90\x92Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xC6W\xC7\x18\x90a\x01\xE3\x90\x85\x90\x87\x90`\x04\x01a\x02XV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x01\xFAW__\xFD[PZ\xF1\x15\x80\x15a\x02\x0CW=__>=_\xFD[PPPP\x91P\x91V[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15a\x02;W__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02QW__\xFD[\x93\x92PPPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R_\x82Q\x80`@\x84\x01R\x80` \x85\x01``\x85\x01^_``\x82\x85\x01\x01R```\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x93\x92PPPV[a^\xFB\x80a\x02\xA9_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\xF2W_5`\xE0\x1C\x80c\x85\"l\x81\x11a\x01\x14W\x80c\xC0GM\x0B\x11a\0\xA9W\x80c\xE1\x15\xC6B\x11a\0yW\x80c\xE1\x15\xC6B\x14a\x03\xB6W\x80c\xE2\x0C\x9Fq\x14a\x03\xC9W\x80c\xF0\x9D\x1DD\x14a\x03\xD1W\x80c\xFAv&\xD4\x14a\x03\xD9W\x80c\xFBG\xE3\xA2\x14a\x03\xE6W__\xFD[\x80c\xC0GM\x0B\x14a\x03qW\x80c\xC0\x9C\xECw\x14a\x03\x93W\x80c\xC2\xE9\xF2\xE4\x14a\x03\xA6W\x80c\xD8\x83\xBA\x03\x14a\x03\xAEW__\xFD[\x80c\xB0FO\xDC\x11a\0\xE4W\x80c\xB0FO\xDC\x14a\x036W\x80c\xB5P\x8A\xA9\x14a\x03>W\x80c\xBAAO\xA6\x14a\x03FW\x80c\xBF\x84R\xF5\x14a\x03^W__\xFD[\x80c\x85\"l\x81\x14a\x02\xF1W\x80c\x8D\xA5\xCB[\x14a\x03\x06W\x80c\x91j\x17\xC6\x14a\x03\x19W\x80c\x9A\"\xB0T\x14a\x03.W__\xFD[\x80c<\x08\xBD_\x11a\x01\x8AW\x80cW_\xC5\xD5\x11a\x01ZW\x80cW_\xC5\xD5\x14a\x02\xAEW\x80cf\xD9\xA9\xA0\x14a\x02\xB6W\x80cn@\x05o\x14a\x02\xCBW\x80ctt,\x08\x14a\x02\xDEW__\xFD[\x80c<\x08\xBD_\x14a\x02~W\x80c>^<#\x14a\x02\x86W\x80c?r\x86\xF4\x14a\x02\x8EW\x80cO\x19\xB9A\x14a\x02\x96W__\xFD[\x80c*\xDE8\x80\x11a\x01\xC5W\x80c*\xDE8\x80\x14a\x02.W\x80c+\x90=\x9B\x14a\x02CW\x80c8\xD5.\x0F\x14a\x02KW\x80c;\x8D\xDFw\x14a\x02vW__\xFD[\x80c\x08\x1Ci\xE4\x14a\x01\xF6W\x80c\n\x92T\xE4\x14a\x02\0W\x80c\x10\\\xDA\x16\x14a\x02\x08W\x80c\x1E\xD7\x83\x1C\x14a\x02\x10W[__\xFD[a\x01\xFEa\x03\xF9V[\0[a\x01\xFEa\x05.V[a\x01\xFEa\x07\xB1V[a\x02\x18a\n\x81V[`@Qa\x02%\x91\x90a9\x87V[`@Q\x80\x91\x03\x90\xF3[a\x026a\n\xE1V[`@Qa\x02%\x91\x90a:\0V[a\x01\xFEa\x0C\x1DV[`!Ta\x02^\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02%V[a\x01\xFEa\x12\x86V[a\x01\xFEa\x14\xC9V[a\x02\x18a\x16\xA5V[a\x02\x18a\x17\x03V[`\x1FTa\x02^\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xFEa\x17aV[a\x02\xBEa\x1B_V[`@Qa\x02%\x91\x90a;\rV[a\x01\xFEa\x02\xD96`\x04a;\x8BV[a\x1C\xC3V[` Ta\x02^\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xF9a\x1F\x8FV[`@Qa\x02%\x91\x90a;\xABV[`\"Ta\x02^\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03!a ZV[`@Qa\x02%\x91\x90a<\x02V[a\x01\xFEa!;V[a\x03!a%nV[a\x02\xF9a&OV[a\x03Na'\x1AV[`@Q\x90\x15\x15\x81R` \x01a\x02%V[a\x01\xFEa\x03l6`\x04a<yV[a'\xD4V[a\x03\x85_Q` a^\xA6_9_Q\x90_R\x81V[`@Q\x90\x81R` \x01a\x02%V[`$Ta\x02^\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xFEa)\xF0V[a\x01\xFEa.\xBBV[a\x01\xFEa\x03\xC46`\x04a<\xB6V[a0%V[a\x02\x18a3\x08V[a\x01\xFEa3fV[`\x1FTa\x03N\x90`\xFF\x16\x81V[`#Ta\x02^\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`#T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x04IW__\xFD[PZ\xF1\x15\x80\x15a\x04[W=__>=_\xFD[PPPP_Q` a^\x86_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\xF4\x84H\x14`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x04\xA4W__\xFD[PZ\xF1\x15\x80\x15a\x04\xB6W=__>=_\xFD[PP`\x1FT`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc@\xC1\x0F\x19\x93Pa\x04\xFF\x92\x91\x16\x90h\x05k\xC7^-c\x10\0\0\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\x16W__\xFD[PZ\xF1\x15\x80\x15a\x05(W=__>=_\xFD[PPPPV[`\x12`@Qa\x05<\x90a9mV[``\x80\x82R`\n\x90\x82\x01Ri\x15\x19\\\xDD\x08\x10\\\xDC\xD9]`\xB2\x1B`\x80\x82\x01R`\xA0` \x82\x01\x81\x90R`\x04\x90\x82\x01Rc\x15\x11T\xD5`\xE2\x1B`\xC0\x82\x01R`\xFF\x90\x91\x16`@\x82\x01R`\xE0\x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x05\x9CW=__>=_\xFD[P`!\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\"T`@Qc\x03\">\xAB`\xE1\x1B\x81R\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\xFFW__\xFD[PZ\xF1\x15\x80\x15a\x06\x11W=__>=_\xFD[PP`!T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92P_Q` a^\xA6_9_Q\x90_R\x91Pa\x06?\x90a9zV[`\x80\x80\x82R`\x03\x90\x82\x01\x81\x90RbYes`\xE8\x1B`\xA0\x83\x01R`\xC0` \x83\x01\x81\x90R\x82\x01RbYES`\xE8\x1B`\xE0\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`@\x83\x01R``\x82\x01Ra\x01\0\x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x06\xA2W=__>=_\xFD[P`\x1F\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x16a\x01\0`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x02\x17\x90U`!T`@Q\x91\x16\x90_Q` a^\xA6_9_Q\x90_R\x90a\x06\xE5\x90a9zV[`\x80\x80\x82R`\x02\x90\x82\x01\x81\x90RaNo`\xF0\x1B`\xA0\x83\x01R`\xC0` \x83\x01\x81\x90R\x82\x01RaNO`\xF0\x1B`\xE0\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`@\x83\x01R``\x82\x01Ra\x01\0\x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x07FW=__>=_\xFD[P` _a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP_Q` a^\x86_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\x16W__\xFD[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rh\x05k\xC7^-c\x10\0\0\x90h\x01\xA0Ui\r\x9D\xB8\0\0\x90_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\x16W__\xFD[PZ\xF1\x15\x80\x15a\x08(W=__>=_\xFD[PP`\x1FT`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc@\xC1\x0F\x19\x93Pa\x08h\x92\x91\x16\x90\x86\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\x7FW__\xFD[PZ\xF1\x15\x80\x15a\x08\x91W=__>=_\xFD[PP`#T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x92Pc\xCAf\x9F\xA7\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\xE4W__\xFD[PZ\xF1\x15\x80\x15a\x08\xF6W=__>=_\xFD[PP`\x1FT`$T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc\xA9\x05\x9C\xBB\x93Pa\t6\x92\x91\x16\x90\x85\x90`\x04\x01a<\xE8V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\tRW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tv\x91\x90a=\x01V[P`\x1FT`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\t\xFF\x92a\x01\0\x90\x04\x90\x91\x16\x90cp\xA0\x821\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xCCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xF0\x91\x90a= V[a\t\xFA\x83\x85a=KV[a4\xE9V[`\x1FT`$\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\n}\x93a\x01\0\x90\x04\x90\x91\x16\x91cp\xA0\x821\x91\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nSW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nw\x91\x90a= V[\x82a4\xE9V[PPV[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\n\xD7W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\n\xB9W[PPPPP\x90P\x90V[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\x14W_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x0B\xFDW\x83\x82\x90_R` _ \x01\x80Ta\x0Br\x90a=dV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\x9E\x90a=dV[\x80\x15a\x0B\xE9W\x80`\x1F\x10a\x0B\xC0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xE9V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xCCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0BUV[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0B\x04V[PPPP\x90P\x90V[`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R_\x91` \x82\x01``\x806\x837\x01\x90PP\x90Ph\x02\xB5\xE3\xAF\x16\xB1\x88\0\0\x81_\x81Q\x81\x10a\x0C[Wa\x0C[a=\xB0V[` \x02` \x01\x01\x81\x81RPPh\x01\xA0Ui\r\x9D\xB8\0\0\x81`\x01\x81Q\x81\x10a\x0C\x84Wa\x0C\x84a=\xB0V[` \x02` \x01\x01\x81\x81RPPh\x01\x15\x8EF\t\x13\xD0\0\0\x81`\x02\x81Q\x81\x10a\x0C\xADWa\x0C\xADa=\xB0V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R_\x91\x81` \x01` \x82\x02\x806\x837PP`#T\x82Q\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91\x83\x91P_\x90a\x0C\xFBWa\x0C\xFBa=\xB0V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`$T\x82Q\x91\x16\x90\x82\x90`\x01\x90\x81\x10a\r,Wa\r,a=\xB0V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPa\rt`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fcharlie`\xC8\x1B\x81RPa5MV[\x81`\x02\x81Q\x81\x10a\r\x87Wa\r\x87a=\xB0V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`\"T`@Qc\x03\">\xAB`\xE1\x1B\x81R\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\r\xE3W__\xFD[PZ\xF1\x15\x80\x15a\r\xF5W=__>=_\xFD[P_\x92PPP[\x81Q\x81\x10\x15a\x0E\xB3W`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x19\x83\x83\x81Q\x81\x10a\x0E;Wa\x0E;a=\xB0V[` \x02` \x01\x01Q\x85\x84\x81Q\x81\x10a\x0EUWa\x0EUa=\xB0V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Ez\x92\x91\x90a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0E\x91W__\xFD[PZ\xF1\x15\x80\x15a\x0E\xA3W=__>=_\xFD[PP`\x01\x90\x92\x01\x91Pa\r\xFC\x90PV[P_\x80[\x82Q\x81\x10\x15a\x0F\xB9Wa\x0F\x8A`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x821\x85\x84\x81Q\x81\x10a\x0E\xF9Wa\x0E\xF9a=\xB0V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F,\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FGW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Fk\x91\x90a= V[\x85\x83\x81Q\x81\x10a\x0F}Wa\x0F}a=\xB0V[` \x02` \x01\x01Qa4\xE9V[\x83\x81\x81Q\x81\x10a\x0F\x9CWa\x0F\x9Ca=\xB0V[` \x02` \x01\x01Q\x82a\x0F\xAF\x91\x90a=\xC4V[\x91P`\x01\x01a\x0E\xB7V[Pa\x10\x0E`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nSW=__>=_\xFD[g\x8A\xC7#\x04\x89\xE8\0\0_[\x83Q\x81\x10\x15a\x10\xB7W`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x9D\xC2\x9F\xAC\x85\x83\x81Q\x81\x10a\x10XWa\x10Xa=\xB0V[` \x02` \x01\x01Q\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10~\x92\x91\x90a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10\x95W__\xFD[PZ\xF1\x15\x80\x15a\x10\xA7W=__>=_\xFD[PP`\x01\x90\x92\x01\x91Pa\x10\x19\x90PV[P_Q` a^\x86_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10\xFDW__\xFD[PZ\xF1\x15\x80\x15a\x11\x0FW=__>=_\xFD[P_\x92PPP[\x83Q\x81\x10\x15a\x11\xF7Wa\x11\xEF`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x821\x86\x84\x81Q\x81\x10a\x11XWa\x11Xa=\xB0V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\x8B\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xA6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xCA\x91\x90a= V[\x83\x87\x84\x81Q\x81\x10a\x11\xDDWa\x11\xDDa=\xB0V[` \x02` \x01\x01Qa\t\xFA\x91\x90a=KV[`\x01\x01a\x11\x16V[Pa\x05(`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12LW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12p\x91\x90a= V[\x84Qa\x12|\x90\x84a=\xD7V[a\t\xFA\x90\x85a=KV[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rh\x05k\xC7^-c\x10\0\0\x90h\x02+\x1C\x8C\x12'\xA0\0\0\x90_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x12\xEBW__\xFD[PZ\xF1\x15\x80\x15a\x12\xFDW=__>=_\xFD[PP`\x1FT`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc@\xC1\x0F\x19\x93Pa\x13=\x92\x91\x16\x90\x86\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x13TW__\xFD[PZ\xF1\x15\x80\x15a\x13fW=__>=_\xFD[PP`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x92Pc\xCAf\x9F\xA7\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x13\xB9W__\xFD[PZ\xF1\x15\x80\x15a\x13\xCBW=__>=_\xFD[PP`\x1FT`#T`@Qc'p\xA7\xEB`\xE2\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc\x9D\xC2\x9F\xAC\x93Pa\x14\x0B\x92\x91\x16\x90\x85\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x14\"W__\xFD[PZ\xF1\x15\x80\x15a\x144W=__>=_\xFD[PP`\x1FT`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\x14u\x94Pa\x01\0\x90\x92\x04\x16\x91Pcp\xA0\x821\x90`$\x01a\t\xB1V[a\n}`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xCCW=__>=_\xFD[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x15\x18W__\xFD[PZ\xF1\x15\x80\x15a\x15*W=__>=_\xFD[PP`\x1FT`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc@\xC1\x0F\x19\x93Pa\x15s\x92\x91\x16\x90h\x05k\xC7^-c\x10\0\0\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x15\x8AW__\xFD[PZ\xF1\x15\x80\x15a\x15\x9CW=__>=_\xFD[PP`#T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x92Pc\xCAf\x9F\xA7\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x15\xEFW__\xFD[PZ\xF1\x15\x80\x15a\x16\x01W=__>=_\xFD[PPPP_Q` a^\x86_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\xF4\x84H\x14`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x16JW__\xFD[PZ\xF1\x15\x80\x15a\x16\\W=__>=_\xFD[PP`\x1FT`#T`@Qc'p\xA7\xEB`\xE2\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc\x9D\xC2\x9F\xAC\x93Pa\x04\xFF\x92\x91\x16\x90h\x02\xB5\xE3\xAF\x16\xB1\x88\0\0\x90`\x04\x01a<\xE8V[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\n\xD7W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\n\xB9WPPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\n\xD7W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\n\xB9WPPPPP\x90P\x90V[_a\x17\x8B`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g72\xBB\xA7\xBB\xB72\xB9`\xC1\x1B\x81RPa5MV[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x91P_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x17\xDDW__\xFD[PZ\xF1\x15\x80\x15a\x17\xEFW=__>=_\xFD[PP`\x1FT`@Qc\xF2\xFD\xE3\x8B`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01Ra\x01\0\x90\x92\x04\x90\x91\x16\x92Pc\xF2\xFD\xE3\x8B\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x18=W__\xFD[PZ\xF1\x15\x80\x15a\x18OW=__>=_\xFD[PPPPa\x18\xD1`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xA7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xCB\x91\x90a=\xEEV[\x82a5^V[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x19 W__\xFD[PZ\xF1\x15\x80\x15a\x192W=__>=_\xFD[PPPP_Q` a^\x86_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\xF4\x84H\x14`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x19{W__\xFD[PZ\xF1\x15\x80\x15a\x19\x8DW=__>=_\xFD[PP`\x1FT`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc@\xC1\x0F\x19\x93Pa\x19\xD6\x92\x91\x16\x90h\x05k\xC7^-c\x10\0\0\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x19\xEDW__\xFD[PZ\xF1\x15\x80\x15a\x19\xFFW=__>=_\xFD[PP`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x92Pc\xCAf\x9F\xA7\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1ANW__\xFD[PZ\xF1\x15\x80\x15a\x1A`W=__>=_\xFD[PP`\x1FT`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc@\xC1\x0F\x19\x93Pa\x1A\xA9\x92\x91\x16\x90h\x05k\xC7^-c\x10\0\0\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1A\xC0W__\xFD[PZ\xF1\x15\x80\x15a\x1A\xD2W=__>=_\xFD[PP`\x1FT`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\x1B\\\x94Pa\x01\0\x90\x92\x04\x16\x91Pcp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B)W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1BM\x91\x90a= V[h\x05k\xC7^-c\x10\0\0a4\xE9V[PV[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\x14W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x1B\xB2\x90a=dV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1B\xDE\x90a=dV[\x80\x15a\x1C)W\x80`\x1F\x10a\x1C\0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1C)V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1C\x0CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1C\xABW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x1CmW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1B\x82V[a\x1C\xD6\x82`\x01`\x01`\x01`\x80\x1B\x03a5\x9FV[\x91Pa\x1C\xE3\x81_\x84a5\x9FV[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x91P_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1D5W__\xFD[PZ\xF1\x15\x80\x15a\x1DGW=__>=_\xFD[PP`\x1FT`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc@\xC1\x0F\x19\x93Pa\x1D\x87\x92\x91\x16\x90\x86\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1D\x9EW__\xFD[PZ\xF1\x15\x80\x15a\x1D\xB0W=__>=_\xFD[PP`#T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x92Pc\xCAf\x9F\xA7\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1E\x03W__\xFD[PZ\xF1\x15\x80\x15a\x1E\x15W=__>=_\xFD[PP`\x1FT`$T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc\xA9\x05\x9C\xBB\x93Pa\x1EU\x92\x91\x16\x90\x85\x90`\x04\x01a<\xE8V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1EqW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x95\x91\x90a=\x01V[P`\x1FT`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\x1E\xD4\x92a\x01\0\x90\x04\x90\x91\x16\x90cp\xA0\x821\x90`$\x01a\t\xB1V[`\x1FT`$\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\x1F\x11\x93a\x01\0\x90\x04\x90\x91\x16\x91cp\xA0\x821\x91\x01a\n8V[a\n}`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1FeW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\x89\x91\x90a= V[\x83a4\xE9V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\x14W\x83\x82\x90_R` _ \x01\x80Ta\x1F\xCF\x90a=dV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1F\xFB\x90a=dV[\x80\x15a FW\x80`\x1F\x10a \x1DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a )W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1F\xB2V[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\x14W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a!#W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a \xE5W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a }V[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rh\x05k\xC7^-c\x10\0\0\x90h\x02\xB5\xE3\xAF\x16\xB1\x88\0\0\x90h\x01\xA0Ui\r\x9D\xB8\0\0\x90_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a!\xABW__\xFD[PZ\xF1\x15\x80\x15a!\xBDW=__>=_\xFD[PP`\x1FT`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc@\xC1\x0F\x19\x93Pa!\xFD\x92\x91\x16\x90\x87\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\"\x14W__\xFD[PZ\xF1\x15\x80\x15a\"&W=__>=_\xFD[PP`#T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x92Pc\xCAf\x9F\xA7\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\"yW__\xFD[PZ\xF1\x15\x80\x15a\"\x8BW=__>=_\xFD[PP`\x1FT`$T`@Qc\t^\xA7\xB3`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc\t^\xA7\xB3\x93Pa\"\xCB\x92\x91\x16\x90\x86\x90`\x04\x01a<\xE8V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"\xE7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x0B\x91\x90a=\x01V[P`\x1FT`#T`$\x80T`@Qcn\xB1v\x9F`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x90\x83\x16\x91\x81\x01\x91\x90\x91Ra#o\x92a\x01\0\x90\x04\x90\x91\x16\x90c\xDDb\xED>\x90`D\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1FeW=__>=_\xFD[`$\x80T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x91c\xCAf\x9F\xA7\x91\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a#\xBEW__\xFD[PZ\xF1\x15\x80\x15a#\xD0W=__>=_\xFD[PP`\x1FT`#T`$\x80T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x90\x83\x16\x91\x81\x01\x91\x90\x91R`D\x81\x01\x86\x90Ra\x01\0\x90\x92\x04\x16\x92Pc#\xB8r\xDD\x91P`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a$9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$]\x91\x90a=\x01V[P`\x1FT`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra$\xE0\x92a\x01\0\x90\x04\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\xB2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xD6\x91\x90a= V[a\t\xFA\x83\x86a=KV[`\x1FT`$\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra%\x1D\x93a\x01\0\x90\x04\x90\x91\x16\x91cp\xA0\x821\x91\x01a\n8V[`\x1FT`#T`$\x80T`@Qcn\xB1v\x9F`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x90\x83\x16\x91\x81\x01\x91\x90\x91Ra%i\x92a\x01\0\x90\x04\x90\x91\x16\x90c\xDDb\xED>\x90`D\x01a\t\xB1V[PPPV[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\x14W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a&7W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a%\xF9W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a%\x91V[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\x14W\x83\x82\x90_R` _ \x01\x80Ta&\x8F\x90a=dV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta&\xBB\x90a=dV[\x80\x15a'\x06W\x80`\x1F\x10a&\xDDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a'\x06V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a&\xE9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a&rV[`\x08T_\x90`\xFF\x16\x15a'1WP`\x08T`\xFF\x16\x90V[`@Qc\x06g\xF9\xD7`\xE4\x1B\x81R_\x90_Q` a^\x86_9_Q\x90_R\x90cf\x7F\x9Dp\x90a'\x8E\x90\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90e\x19\x98Z[\x19Y`\xD2\x1B\x90`\x04\x01a<\xE8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\xA9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xCD\x91\x90a= V[\x14\x15\x90P\x90V[a'\xE7\x83`\x01`\x01`\x01`\x80\x1B\x03a5\x9FV[\x92Pa'\xF4\x82_\x85a5\x9FV[\x91Pa(\x01\x81_\x84a5\x9FV[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x91P_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a(SW__\xFD[PZ\xF1\x15\x80\x15a(eW=__>=_\xFD[PP`\x1FT`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc@\xC1\x0F\x19\x93Pa(\xA5\x92\x91\x16\x90\x87\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a(\xBCW__\xFD[PZ\xF1\x15\x80\x15a(\xCEW=__>=_\xFD[PP`#T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x92Pc\xCAf\x9F\xA7\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a)!W__\xFD[PZ\xF1\x15\x80\x15a)3W=__>=_\xFD[PP`\x1FT`$T`@Qc\t^\xA7\xB3`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc\t^\xA7\xB3\x93Pa)s\x92\x91\x16\x90\x86\x90`\x04\x01a<\xE8V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a)\x8FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xB3\x91\x90a=\x01V[P`$\x80T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x91c\xCAf\x9F\xA7\x91\x01a#\xA7V[a*\x8B`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x06\xFD\xDE\x03`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*CW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra*j\x91\x90\x81\x01\x90a>\tV[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bYes`\xE8\x1B\x81RPa5\xE2V[a+&`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\xDEW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra+\x05\x91\x90\x81\x01\x90a>\tV[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bYES`\xE8\x1B\x81RPa5\xE2V[a+\xAF`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cH\0\xD9\x7F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+zW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\x9E\x91\x90a=\xEEV[`!T`\x01`\x01`\xA0\x1B\x03\x16a5^V[a,:`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xC0GM\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\x03W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,'\x91\x90a= V[_Q` a^\xA6_9_Q\x90_Ra6\x14V[a,\xC3`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\x8EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xB2\x91\x90a=\xEEV[`\"T`\x01`\x01`\xA0\x1B\x03\x16a5^V[` T`@\x80Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x90Qa-R\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x06\xFD\xDE\x03\x91`\x04\x80\x83\x01\x92_\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a-\x0BW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra-2\x91\x90\x81\x01\x90a>\tV[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aNo`\xF0\x1B\x81RPa5\xE2V[` T`@\x80Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x90Qa-\xE1\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x83\x01\x92_\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a-\x9AW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra-\xC1\x91\x90\x81\x01\x90a>\tV[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aNO`\xF0\x1B\x81RPa5\xE2V[` \x80T`@\x80QcH\0\xD9\x7F`\xE0\x1B\x81R\x90Qa.)\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92cH\0\xD9\x7F\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a+zW=__>=_\xFD[` \x80T`@\x80Qc\xC0GM\x0B`\xE0\x1B\x81R\x90Qa.q\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\xC0GM\x0B\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a,\x03W=__>=_\xFD[` \x80T`@\x80Qc\x8D\xA5\xCB[`\xE0\x1B\x81R\x90Qa.\xB9\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\x8D\xA5\xCB[\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a,\x8EW=__>=_\xFD[V[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rh\x05k\xC7^-c\x10\0\0\x90_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/\x15W__\xFD[PZ\xF1\x15\x80\x15a/'W=__>=_\xFD[PP`\x1FT`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc@\xC1\x0F\x19\x93Pa/g\x92\x91\x16\x90\x85\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/~W__\xFD[PZ\xF1\x15\x80\x15a/\x90W=__>=_\xFD[PP`\x1FT`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra/\xD1\x94Pa\x01\0\x90\x92\x04\x16\x91Pcp\xA0\x821\x90`$\x01a\n8V[a\x1B\\`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nSW=__>=_\xFD[`@Qc&1\xF2\xB1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x15\x15`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x90cLc\xE5b\x90`$\x01_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a0qW__\xFD[PZ\xFA\x15\x80\x15a0\x83W=__>=_\xFD[PPPPa0\x9C\x82`\x01`\x01`\x01`\x80\x1B\x03\x80\x16a5\x9FV[\x91Pa0\xA9\x81_\x84a5\x9FV[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x91P_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a0\xFBW__\xFD[PZ\xF1\x15\x80\x15a1\rW=__>=_\xFD[PP`\x1FT`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x92Pc@\xC1\x0F\x19\x91Pa1G\x90\x86\x90\x86\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a1^W__\xFD[PZ\xF1\x15\x80\x15a1pW=__>=_\xFD[PP`\x1FT`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01Ra1\xB0\x94Pa\x01\0\x90\x92\x04\x90\x91\x16\x91Pcp\xA0\x821\x90`$\x01a#TV[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a1\xFFW__\xFD[PZ\xF1\x15\x80\x15a2\x11W=__>=_\xFD[PP`\x1FT`@Qc'p\xA7\xEB`\xE2\x1B\x81Ra\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x92Pc\x9D\xC2\x9F\xAC\x91Pa2K\x90\x86\x90\x85\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a2bW__\xFD[PZ\xF1\x15\x80\x15a2tW=__>=_\xFD[PP`\x1FT`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01Ra2\xB4\x94Pa\x01\0\x90\x92\x04\x90\x91\x16\x91Pcp\xA0\x821\x90`$\x01a\t\xB1V[a%i`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xCCW=__>=_\xFD[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\n\xD7W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\n\xB9WPPPPP\x90P\x90V[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a3\xB5W__\xFD[PZ\xF1\x15\x80\x15a3\xC7W=__>=_\xFD[PPPP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cqP\x18\xA6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a4\x18W__\xFD[PZ\xF1\x15\x80\x15a4*W=__>=_\xFD[PPPPa4\xAC`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4\x82W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\xA6\x91\x90a=\xEEV[_a5^V[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01a\x042V[`@Qc&\n[\x15`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R_Q` a^\x86_9_Q\x90_R\x90c\x98)lT\x90`D\x01[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a53W__\xFD[PZ\xFA\x15\x80\x15a5EW=__>=_\xFD[PPPPPPV[_a5W\x82a6LV[P\x92\x91PPV[`@Qc(\xA9\xB0\xFB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R\x82\x16`$\x82\x01R_Q` a^\x86_9_Q\x90_R\x90cQSa\xF6\x90`D\x01a5\x1DV[_a5\xAB\x84\x84\x84a7LV[\x90Pa5\xDB`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x10\x9B\xDD[\x99\x08\x1C\x99\\\xDD[\x1D`\xA2\x1B\x81RP\x82a9\tV[\x93\x92PPPV[`@Qc\xF3 \xD9c`\xE0\x1B\x81R_Q` a^\x86_9_Q\x90_R\x90c\xF3 \xD9c\x90a5\x1D\x90\x85\x90\x85\x90`\x04\x01a>\xBCV[`@Qc|\x84\xC6\x9B`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R_Q` a^\x86_9_Q\x90_R\x90c|\x84\xC6\x9B\x90`D\x01a5\x1DV[__\x82`@Q` \x01a6_\x91\x90a>\xE9V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01b^y\xB7`\xE0\x1B\x03\x19\x82R`\x04\x82\x01\x81\x90R\x91P_Q` a^\x86_9_Q\x90_R\x90c\xFF\xA1\x86I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\xC1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\xE5\x91\x90a=\xEEV[`@Qc\x18\xCA\xF8\xE3`\xE3\x1B\x81R\x90\x92P_Q` a^\x86_9_Q\x90_R\x90c\xC6W\xC7\x18\x90a7\x1A\x90\x85\x90\x87\x90`\x04\x01a>\xFFV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a71W__\xFD[PZ\xF1\x15\x80\x15a7CW=__>=_\xFD[PPPP\x91P\x91V[_\x81\x83\x11\x15a7\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a7\xD7WP\x81\x84\x11\x15[\x15a7\xE3WP\x82a5\xDBV[_a7\xEE\x84\x84a=KV[a7\xF9\x90`\x01a=\xC4V[\x90P`\x03\x85\x11\x15\x80\x15a8\x0BWP\x84\x81\x11[\x15a8\"Wa8\x1A\x85\x85a=\xC4V[\x91PPa5\xDBV[a8.`\x03_\x19a=KV[\x85\x10\x15\x80\x15a8FWPa8C\x85_\x19a=KV[\x81\x11[\x15a8`Wa8V\x85_\x19a=KV[a8\x1A\x90\x84a=KV[\x82\x85\x11\x15a8\xB3W_a8s\x84\x87a=KV[\x90P_a8\x80\x83\x83a?*V[\x90P\x80_\x03a8\x94W\x84\x93PPPPa5\xDBV[`\x01a8\xA0\x82\x88a=\xC4V[a8\xAA\x91\x90a=KV[\x93PPPa9\x01V[\x83\x85\x10\x15a9\x01W_a8\xC6\x86\x86a=KV[\x90P_a8\xD3\x83\x83a?*V[\x90P\x80_\x03a8\xE7W\x85\x93PPPPa5\xDBV[a8\xF1\x81\x86a=KV[a8\xFC\x90`\x01a=\xC4V[\x93PPP[P\x93\x92PPPV[a\n}\x82\x82`@Q`$\x01a9\x1F\x92\x91\x90a?IV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\x1B\\\x81\x80Qjconsole.log` \x83\x01_\x80\x84\x83\x85Z\xFAPPPPPV[a\x0E\xD4\x80a?k\x839\x01\x90V[a\x10G\x80aN?\x839\x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a9\xC7W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a9\xA0V[P\x90\x95\x94PPPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a:\xBDW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90```\x05\x82\x90\x1B\x88\x01\x81\x01\x91\x90\x88\x01\x90_[\x81\x81\x10\x15a:\xA3W`_\x19\x8A\x85\x03\x01\x83Ra:\x8D\x84\x86Qa9\xD2V[` \x95\x86\x01\x95\x90\x94P\x92\x90\x92\x01\x91`\x01\x01a:qV[P\x91\x97PPP` \x94\x85\x01\x94\x92\x90\x92\x01\x91P`\x01\x01a:&V[P\x92\x96\x95PPPPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a;\x03W\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a:\xDBV[P\x93\x94\x93PPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a:\xBDW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`@\x87Ra;Y`@\x88\x01\x82a9\xD2V[\x90P` \x82\x01Q\x91P\x86\x81\x03` \x88\x01Ra;t\x81\x83a:\xC9V[\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a;3V[__`@\x83\x85\x03\x12\x15a;\x9CW__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a:\xBDW`?\x19\x87\x86\x03\x01\x84Ra;\xED\x85\x83Qa9\xD2V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a;\xD1V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a:\xBDW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90a<c\x90\x87\x01\x82a:\xC9V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a<(V[___``\x84\x86\x03\x12\x15a<\x8BW__\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1B\\W__\xFD[___``\x84\x86\x03\x12\x15a<\xC8W__\xFD[\x835a<\xD3\x81a<\xA2V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[_` \x82\x84\x03\x12\x15a=\x11W__\xFD[\x81Q\x80\x15\x15\x81\x14a5\xDBW__\xFD[_` \x82\x84\x03\x12\x15a=0W__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a=^Wa=^a=7V[\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a=xW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a=\x96WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a=^Wa=^a=7V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a=^Wa=^a=7V[_` \x82\x84\x03\x12\x15a=\xFEW__\xFD[\x81Qa5\xDB\x81a<\xA2V[_` \x82\x84\x03\x12\x15a>\x19W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>/W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a>?W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>YWa>Ya=\x9CV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a>\x88Wa>\x88a=\x9CV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a>\x9FW__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`@\x81R_a>\xCE`@\x83\x01\x85a9\xD2V[\x82\x81\x03` \x84\x01Ra>\xE0\x81\x85a9\xD2V[\x95\x94PPPPPV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a?\"\x90\x83\x01\x84a9\xD2V[\x94\x93PPPPV[_\x82a?DWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x06\x90V[`@\x81R_a?[`@\x83\x01\x85a9\xD2V[\x90P\x82` \x83\x01R\x93\x92PPPV\xFE`\xA0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x0E\xD48\x03\x80a\x0E\xD4\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01\tV[_a\09\x84\x82a\x02\nV[P`\x01a\0F\x83\x82a\x02\nV[P`\x02\x80T`\xFF\x19\x16`\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UP\x80Q` \x90\x91\x01 `\x80Ra\x02\xC4V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a\0\x8FW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\0\xA8Wa\0\xA8a\0lV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\0\xD6Wa\0\xD6a\0lV[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\0\xEDW__\xFD[\x81` \x85\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[___``\x84\x86\x03\x12\x15a\x01\x1BW__\xFD[\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x010W__\xFD[a\x01<\x86\x82\x87\x01a\0\x80V[` \x86\x01Q\x90\x94P\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01YW__\xFD[a\x01e\x86\x82\x87\x01a\0\x80V[\x92PP`@\x84\x01Q`\xFF\x81\x16\x81\x14a\x01{W__\xFD[\x80\x91PP\x92P\x92P\x92V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01\x9AW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x01\xB8WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x02\x05W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x01\xE3WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x02\x02W_\x81U`\x01\x01a\x01\xEFV[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02#Wa\x02#a\0lV[a\x027\x81a\x021\x84Ta\x01\x86V[\x84a\x01\xBEV[` `\x1F\x82\x11`\x01\x81\x14a\x02iW_\x83\x15a\x02RWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x02\x02V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x02\x98W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x02xV[P\x84\x82\x10\x15a\x02\xB5W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80Qa\x0B\xF1a\x02\xE3_9_\x81\x81a\x03\x99\x01Ra\x04\xDB\x01Ra\x0B\xF1_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xFBW_5`\xE0\x1C\x80c~\xCE\xBE\0\x11a\0\x93W\x80c\xD3\x0E\xD3\xB3\x11a\0cW\x80c\xD3\x0E\xD3\xB3\x14a\x02\x17W\x80c\xD5\x05\xAC\xCF\x14a\x02*W\x80c\xDDb\xED>\x14a\x02=W\x80c\xF8=\x17\x91\x14a\x02PW__\xFD[\x80c~\xCE\xBE\0\x14a\x01\xC4W\x80c\x95\xD8\x9BA\x14a\x01\xE9W\x80c\x9D\xC2\x9F\xAC\x14a\x01\xF1W\x80c\xA9\x05\x9C\xBB\x14a\x02\x04W__\xFD[\x80c1<\xE5g\x11a\0\xCEW\x80c1<\xE5g\x14a\x01mW\x80c6D\xE5\x15\x14a\x01\x82W\x80c@\xC1\x0F\x19\x14a\x01\x8AW\x80cp\xA0\x821\x14a\x01\x9FW__\xFD[\x80c\x06\xFD\xDE\x03\x14a\0\xFFW\x80c\t^\xA7\xB3\x14a\x01\x1DW\x80c\x18\x16\r\xDD\x14a\x01@W\x80c#\xB8r\xDD\x14a\x01ZW[__\xFD[a\x01\x07a\x02cV[`@Qa\x01\x14\x91\x90a\t\xFFV[`@Q\x80\x91\x03\x90\xF3[a\x010a\x01+6`\x04a\nJV[a\x02\xF2V[`@Q\x90\x15\x15\x81R` \x01a\x01\x14V[h\x054\\\xDFw\xEBh\xF4LT[`@Q\x90\x81R` \x01a\x01\x14V[a\x010a\x01h6`\x04a\nrV[a\x03rV[`\x02T`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\x14V[a\x01La\x03\x96V[a\x01\x9Da\x01\x986`\x04a\nJV[a\x048V[\0[a\x01La\x01\xAD6`\x04a\n\xACV[c\x87\xA2\x11\xA2`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[a\x01La\x01\xD26`\x04a\n\xACV[c87u\x08`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[a\x01\x07a\x04NV[a\x01\x9Da\x01\xFF6`\x04a\nJV[a\x04]V[a\x010a\x02\x126`\x04a\nJV[a\x04oV[a\x01\x9Da\x02%6`\x04a\nrV[a\x04\x89V[a\x01\x9Da\x0286`\x04a\n\xC5V[a\x04\xA9V[a\x01La\x02K6`\x04a\x0B2V[a\x06\x83V[a\x01\x9Da\x02^6`\x04a\nrV[a\x06\xC7V[``_\x80Ta\x02q\x90a\x0BcV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\x9D\x90a\x0BcV[\x80\x15a\x02\xE8W\x80`\x1F\x10a\x02\xBFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xE8V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xCBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[_`\x01`\x01`\xA0\x1B\x03\x83\x16n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x18\x82\x19\x15\x17a\x03#Wc?hS\x9A_R`\x04`\x1C\xFD[\x82` Rc\x7F^\x9F `\x0CR3_R\x81`4`\x0C U\x81_R`,Q``\x1C3\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` _\xA3P`\x01[\x92\x91PPV[_a\x03\x8Ea\x03\x7F\x85a\x06\xE2V[a\x03\x88\x85a\x06\xE2V[\x84a\x07\x03V[\x94\x93PPPPV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80a\x03\xCFWa\x03\xC5a\x02cV[\x80Q\x90` \x01 \x90P[`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R` \x81\x01\x92\x90\x92R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6\x90\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 \x91\x90PV[a\x04Ja\x04D\x83a\x06\xE2V[\x82a\x07\xBFV[PPV[```\x01\x80Ta\x02q\x90a\x0BcV[a\x04Ja\x04i\x83a\x06\xE2V[\x82a\x08(V[_a\x04\x82a\x04|\x84a\x06\xE2V[\x83a\x08\x89V[\x93\x92PPPV[a\x04\xA4a\x04\x95\x84a\x06\xE2V[a\x04\x9E\x84a\x06\xE2V[\x83a\x08\xEDV[PPPV[`\x01`\x01`\xA0\x1B\x03\x86\x16n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x18\x85\x19\x15\x17a\x04\xD9Wc?hS\x9A_R`\x04`\x1C\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80a\x05\x11Wa\x05\x07a\x02cV[\x80Q\x90` \x01 \x90P[\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6B\x86\x10\x15a\x05GWc\x1A\x15\xA3\xCC_R`\x04`\x1C\xFD[`@Q\x89``\x1B``\x1C\x99P\x88``\x1B``\x1C\x98Pe87u\x08\x19\x01`\x0ER\x89_R` `\x0C \x80T\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x83R\x84` \x84\x01R\x83`@\x84\x01RF``\x84\x01R0`\x80\x84\x01R`\xA0\x83 `.R\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x83R\x8B` \x84\x01R\x8A`@\x84\x01R\x89``\x84\x01R\x80`\x80\x84\x01R\x88`\xA0\x84\x01R`\xC0\x83 `NR`B`, _R\x87`\xFF\x16` R\x86`@R\x85``R` \x80`\x80_`\x01Z\xFA\x8C=Q\x14a\x06/Wc\xDD\xAF\xBA\xEF_R`\x04`\x1C\xFD[\x01\x90Uc\x03\xFA\xF4\xF9`\xA5\x1B\x89\x17`@R`4`, \x88\x90U\x88\x8A\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` ``\x85\x01\xA3`@RPP_``RPPPPPPPV[_n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA2\x19`\x01`\x01`\xA0\x1B\x03\x83\x16\x01a\x06\xACWP_\x19a\x03lV[P` Rc\x7F^\x9F `\x0C\x90\x81R_\x91\x90\x91R`4\x90 T\x90V[a\x04\xA4a\x06\xD3\x84a\x06\xE2V[a\x06\xDC\x84a\x06\xE2V[\x83a\tQV[_`\x01`\x01`\xA0\x1B\x03\x82\x16\x80`\xA0a\x06\xF9\x82a\t\xB7V[\x90\x1B\x18\x93\x92PPPV[_\x83``\x1Bn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA33\x14a\x07XW3` Rc\x7F^\x9F \x81\x17`\x0CR`4`\x0C \x80T\x80\x19\x15a\x07UW\x80\x85\x11\x15a\x07OWc\x13\xBE%+_R`\x04`\x1C\xFD[\x84\x81\x03\x82U[PP[c\x87\xA2\x11\xA2\x81\x17`\x0CR` `\x0C \x80T\x80\x85\x11\x15a\x07~Wc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x84\x81\x03\x82UPP\x83_R` `\x0C \x83\x81T\x01\x81UP\x82` R`\x0CQ``\x1C\x81``\x1C_Q` a\x0B\x9C_9_Q\x90_R` \x80\xA3PP`\x01\x93\x92PPPV[h\x054\\\xDFw\xEBh\xF4LT\x81\x81\x01\x81\x81\x10\x15a\x07\xE2Wc\xE5\xCF\xE9W_R`\x04`\x1C\xFD[\x80h\x054\\\xDFw\xEBh\xF4LUPPc\x87\xA2\x11\xA2`\x0CR\x81_R` `\x0C \x81\x81T\x01\x81UP\x80` R`\x0CQ``\x1C__Q` a\x0B\x9C_9_Q\x90_R` \x80\xA3PPV[c\x87\xA2\x11\xA2`\x0CR\x81_R` `\x0C \x80T\x80\x83\x11\x15a\x08OWc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x82\x90\x03\x90Uh\x054\\\xDFw\xEBh\xF4L\x80T\x82\x90\x03\x90U_\x81\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16_Q` a\x0B\x9C_9_Q\x90_R` \x83\xA3PPV[_c\x87\xA2\x11\xA2`\x0CR3_R` `\x0C \x80T\x80\x84\x11\x15a\x08\xB1Wc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x83\x81\x03\x82UPP\x82_R` `\x0C \x82\x81T\x01\x81UP\x81` R`\x0CQ``\x1C3_Q` a\x0B\x9C_9_Q\x90_R` \x80\xA3P`\x01\x92\x91PPV[n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA2\x19`\x01`\x01`\xA0\x1B\x03\x83\x16\x01a\t\x12WPPPV[\x81` Rc\x7F^\x9F `\x0CR\x82_R`4`\x0C \x80T\x80\x19\x15a\tJW\x80\x83\x11\x15a\tDWc\x13\xBE%+_R`\x04`\x1C\xFD[\x82\x81\x03\x82U[PPPPPV[\x82``\x1Bc\x87\xA2\x11\xA2\x81\x17`\x0CR` `\x0C \x80T\x80\x84\x11\x15a\t{Wc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x83\x81\x03\x82UPP\x82_R` `\x0C \x82\x81T\x01\x81UP\x81` R`\x0CQ``\x1C\x81``\x1C_Q` a\x0B\x9C_9_Q\x90_R` \x80\xA3PPPPV[`@Q6_\x8276\x81 ` R`\x10Q\x82\x18`\x10R`\x88_ \x90P\x80`\x10R`\xBC\x19p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Q\x82\t`\x80\x1C`\x07\x16a\t\xFAWP_[\x91\x90PV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\xFAW__\xFD[__`@\x83\x85\x03\x12\x15a\n[W__\xFD[a\nd\x83a\n4V[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a\n\x84W__\xFD[a\n\x8D\x84a\n4V[\x92Pa\n\x9B` \x85\x01a\n4V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_` \x82\x84\x03\x12\x15a\n\xBCW__\xFD[a\x04\x82\x82a\n4V[_______`\xE0\x88\x8A\x03\x12\x15a\n\xDBW__\xFD[a\n\xE4\x88a\n4V[\x96Pa\n\xF2` \x89\x01a\n4V[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015`\xFF\x81\x16\x81\x14a\x0B\x15W__\xFD[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x0BCW__\xFD[a\x0BL\x83a\n4V[\x91Pa\x0BZ` \x84\x01a\n4V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0BwW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0B\x95WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \x0CE\xAC\xC3\xC6\xA9\xD1X\x90\xEB\xBA\x87 \xCB c\xE2|m\xAA\xB30y8k\xB3\xDC\xAE\xCF\xA6L\xFBdsolcC\0\x08\x1E\x003`\xC0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x10G8\x03\x80a\x10G\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01CV[_a\09\x85\x82a\x02RV[P`\x01a\0F\x84\x82a\x02RV[P`\x01`\x01`\xA0\x1B\x03\x82\x16`\x80R`\xA0\x81\x90Ra\0b3a\0kV[PPPPa\x03\x0CV[`\x01`\x01`\xA0\x1B\x03\x16c\x8Bx\xC6\xD8\x19\x81\x90U\x80_\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x81\x80\xA3PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a\0\xC9W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\0\xE2Wa\0\xE2a\0\xA6V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x01\x10Wa\x01\x10a\0\xA6V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\x01'W__\xFD[\x81` \x85\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[____`\x80\x85\x87\x03\x12\x15a\x01VW__\xFD[\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01kW__\xFD[a\x01w\x87\x82\x88\x01a\0\xBAV[` \x87\x01Q\x90\x95P\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\x94W__\xFD[a\x01\xA0\x87\x82\x88\x01a\0\xBAV[`@\x87\x01Q\x90\x94P\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xBEW__\xFD[``\x95\x90\x95\x01Q\x93\x96\x92\x95PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01\xE2W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x02\0WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x02MW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x02+WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x02JW_\x81U`\x01\x01a\x027V[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02kWa\x02ka\0\xA6V[a\x02\x7F\x81a\x02y\x84Ta\x01\xCEV[\x84a\x02\x06V[` `\x1F\x82\x11`\x01\x81\x14a\x02\xB1W_\x83\x15a\x02\x9AWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x02JV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x02\xE0W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x02\xC0V[P\x84\x82\x10\x15a\x02\xFDW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80Q`\xA0Qa\r\x1Aa\x03-_9_a\x03n\x01R_a\x02G\x01Ra\r\x1A_\xF3\xFE`\x80`@R`\x046\x10a\x01<W_5`\xE0\x1C\x80cqP\x18\xA6\x11a\0\xB3W\x80c\xC0GM\x0B\x11a\0mW\x80c\xC0GM\x0B\x14a\x03]W\x80c\xD5\x05\xAC\xCF\x14a\x03\x90W\x80c\xDDb\xED>\x14a\x03\xAFW\x80c\xF0N(>\x14a\x03\xCEW\x80c\xF2\xFD\xE3\x8B\x14a\x03\xE1W\x80c\xFE\xE8\x1C\xF4\x14a\x03\xF4W__\xFD[\x80cqP\x18\xA6\x14a\x02\xBAW\x80c~\xCE\xBE\0\x14a\x02\xC2W\x80c\x8D\xA5\xCB[\x14a\x02\xF3W\x80c\x95\xD8\x9BA\x14a\x03\x0BW\x80c\x9D\xC2\x9F\xAC\x14a\x03\x1FW\x80c\xA9\x05\x9C\xBB\x14a\x03>W__\xFD[\x80c1<\xE5g\x11a\x01\x04W\x80c1<\xE5g\x14a\x01\xE8W\x80c6D\xE5\x15\x14a\x02\x03W\x80c@\xC1\x0F\x19\x14a\x02\x17W\x80cH\0\xD9\x7F\x14a\x026W\x80cT\xD1\xF1=\x14a\x02\x81W\x80cp\xA0\x821\x14a\x02\x89W__\xFD[\x80c\x06\xFD\xDE\x03\x14a\x01@W\x80c\t^\xA7\xB3\x14a\x01jW\x80c\x18\x16\r\xDD\x14a\x01\x99W\x80c#\xB8r\xDD\x14a\x01\xBFW\x80c%i)b\x14a\x01\xDEW[__\xFD[4\x80\x15a\x01KW__\xFD[Pa\x01Ta\x04%V[`@Qa\x01a\x91\x90a\x0B\x1CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01uW__\xFD[Pa\x01\x89a\x01\x846`\x04a\x0BlV[a\x04\xB4V[`@Q\x90\x15\x15\x81R` \x01a\x01aV[4\x80\x15a\x01\xA4W__\xFD[Ph\x054\\\xDFw\xEBh\xF4LT[`@Q\x90\x81R` \x01a\x01aV[4\x80\x15a\x01\xCAW__\xFD[Pa\x01\x89a\x01\xD96`\x04a\x0B\x94V[a\x054V[a\x01\xE6a\x05\xF0V[\0[4\x80\x15a\x01\xF3W__\xFD[P`@Q`\x12\x81R` \x01a\x01aV[4\x80\x15a\x02\x0EW__\xFD[Pa\x01\xB1a\x06=V[4\x80\x15a\x02\"W__\xFD[Pa\x01\xE6a\x0216`\x04a\x0BlV[a\x06\xB9V[4\x80\x15a\x02AW__\xFD[Pa\x02i\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01aV[a\x01\xE6a\x06\xCFV[4\x80\x15a\x02\x94W__\xFD[Pa\x01\xB1a\x02\xA36`\x04a\x0B\xCEV[c\x87\xA2\x11\xA2`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[a\x01\xE6a\x07\x08V[4\x80\x15a\x02\xCDW__\xFD[Pa\x01\xB1a\x02\xDC6`\x04a\x0B\xCEV[c87u\x08`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[4\x80\x15a\x02\xFEW__\xFD[Pc\x8Bx\xC6\xD8\x19Ta\x02iV[4\x80\x15a\x03\x16W__\xFD[Pa\x01Ta\x07\x1BV[4\x80\x15a\x03*W__\xFD[Pa\x01\xE6a\x0396`\x04a\x0BlV[a\x07*V[4\x80\x15a\x03IW__\xFD[Pa\x01\x89a\x03X6`\x04a\x0BlV[a\x07<V[4\x80\x15a\x03hW__\xFD[Pa\x01\xB1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\x9BW__\xFD[Pa\x01\xE6a\x03\xAA6`\x04a\x0B\xEEV[a\x07\xA0V[4\x80\x15a\x03\xBAW__\xFD[Pa\x01\xB1a\x03\xC96`\x04a\x0C[V[a\tTV[a\x01\xE6a\x03\xDC6`\x04a\x0B\xCEV[a\t\x98V[a\x01\xE6a\x03\xEF6`\x04a\x0B\xCEV[a\t\xD5V[4\x80\x15a\x03\xFFW__\xFD[Pa\x01\xB1a\x04\x0E6`\x04a\x0B\xCEV[c8\x9Au\xE1`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[``_\x80Ta\x043\x90a\x0C\x8CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04_\x90a\x0C\x8CV[\x80\x15a\x04\xAAW\x80`\x1F\x10a\x04\x81Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xAAV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x8DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[_`\x01`\x01`\xA0\x1B\x03\x83\x16n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x18\x82\x19\x15\x17a\x04\xE5Wc?hS\x9A_R`\x04`\x1C\xFD[\x82` Rc\x7F^\x9F `\x0CR3_R\x81`4`\x0C U\x81_R`,Q``\x1C3\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` _\xA3P`\x01[\x92\x91PPV[_\x83``\x1Bn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA33\x14a\x05\x89W3` Rc\x7F^\x9F \x81\x17`\x0CR`4`\x0C \x80T\x80\x19\x15a\x05\x86W\x80\x85\x11\x15a\x05\x80Wc\x13\xBE%+_R`\x04`\x1C\xFD[\x84\x81\x03\x82U[PP[c\x87\xA2\x11\xA2\x81\x17`\x0CR` `\x0C \x80T\x80\x85\x11\x15a\x05\xAFWc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x84\x81\x03\x82UPP\x83_R` `\x0C \x83\x81T\x01\x81UP\x82` R`\x0CQ``\x1C\x81``\x1C_Q` a\x0C\xC5_9_Q\x90_R` \x80\xA3PP`\x01\x93\x92PPPV[_b\x02\xA3\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x01\x90Pc8\x9Au\xE1`\x0CR3_R\x80` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D__\xA2PV[_\x80a\x06Ga\x04%V[\x80Q\x90` \x01 \x90P`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R` \x81\x01\x92\x90\x92R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6\x90\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 \x91\x90PV[a\x06\xC1a\t\xFBV[a\x06\xCB\x82\x82a\n\x15V[PPV[c8\x9Au\xE1`\x0CR3_R_` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92__\xA2V[a\x07\x10a\t\xFBV[a\x07\x19_a\n~V[V[```\x01\x80Ta\x043\x90a\x0C\x8CV[a\x072a\t\xFBV[a\x06\xCB\x82\x82a\n\xBBV[_c\x87\xA2\x11\xA2`\x0CR3_R` `\x0C \x80T\x80\x84\x11\x15a\x07dWc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x83\x81\x03\x82UPP\x82_R` `\x0C \x82\x81T\x01\x81UP\x81` R`\x0CQ``\x1C3_Q` a\x0C\xC5_9_Q\x90_R` \x80\xA3P`\x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x86\x16n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x18\x85\x19\x15\x17a\x07\xD0Wc?hS\x9A_R`\x04`\x1C\xFD[_a\x07\xD9a\x04%V[\x80Q\x90` \x01 \x90P\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6B\x86\x10\x15a\x08\x18Wc\x1A\x15\xA3\xCC_R`\x04`\x1C\xFD[`@Q\x89``\x1B``\x1C\x99P\x88``\x1B``\x1C\x98Pe87u\x08\x19\x01`\x0ER\x89_R` `\x0C \x80T\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x83R\x84` \x84\x01R\x83`@\x84\x01RF``\x84\x01R0`\x80\x84\x01R`\xA0\x83 `.R\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x83R\x8B` \x84\x01R\x8A`@\x84\x01R\x89``\x84\x01R\x80`\x80\x84\x01R\x88`\xA0\x84\x01R`\xC0\x83 `NR`B`, _R\x87`\xFF\x16` R\x86`@R\x85``R` \x80`\x80_`\x01Z\xFA\x8C=Q\x14a\t\0Wc\xDD\xAF\xBA\xEF_R`\x04`\x1C\xFD[\x01\x90Uc\x03\xFA\xF4\xF9`\xA5\x1B\x89\x17`@R`4`, \x88\x90U\x88\x8A\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` ``\x85\x01\xA3`@RPP_``RPPPPPPPV[_n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA2\x19`\x01`\x01`\xA0\x1B\x03\x83\x16\x01a\t}WP_\x19a\x05.V[P` Rc\x7F^\x9F `\x0C\x90\x81R_\x91\x90\x91R`4\x90 T\x90V[a\t\xA0a\t\xFBV[c8\x9Au\xE1`\x0CR\x80_R` `\x0C \x80TB\x11\x15a\t\xC6Wco^\x88\x18_R`\x04`\x1C\xFD[_\x90Ua\t\xD2\x81a\n~V[PV[a\t\xDDa\t\xFBV[\x80``\x1Ba\t\xF2WctH\xFB\xAE_R`\x04`\x1C\xFD[a\t\xD2\x81a\n~V[c\x8Bx\xC6\xD8\x19T3\x14a\x07\x19Wc\x82\xB4)\0_R`\x04`\x1C\xFD[h\x054\\\xDFw\xEBh\xF4LT\x81\x81\x01\x81\x81\x10\x15a\n8Wc\xE5\xCF\xE9W_R`\x04`\x1C\xFD[\x80h\x054\\\xDFw\xEBh\xF4LUPPc\x87\xA2\x11\xA2`\x0CR\x81_R` `\x0C \x81\x81T\x01\x81UP\x80` R`\x0CQ``\x1C__Q` a\x0C\xC5_9_Q\x90_R` \x80\xA3PPV[c\x8Bx\xC6\xD8\x19\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3UV[c\x87\xA2\x11\xA2`\x0CR\x81_R` `\x0C \x80T\x80\x83\x11\x15a\n\xE2Wc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x82\x90\x03\x90Uh\x054\\\xDFw\xEBh\xF4L\x80T\x82\x90\x03\x90U_\x81\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16_Q` a\x0C\xC5_9_Q\x90_R` \x83\xA3PPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0BgW__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x0B}W__\xFD[a\x0B\x86\x83a\x0BQV[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a\x0B\xA6W__\xFD[a\x0B\xAF\x84a\x0BQV[\x92Pa\x0B\xBD` \x85\x01a\x0BQV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_` \x82\x84\x03\x12\x15a\x0B\xDEW__\xFD[a\x0B\xE7\x82a\x0BQV[\x93\x92PPPV[_______`\xE0\x88\x8A\x03\x12\x15a\x0C\x04W__\xFD[a\x0C\r\x88a\x0BQV[\x96Pa\x0C\x1B` \x89\x01a\x0BQV[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015`\xFF\x81\x16\x81\x14a\x0C>W__\xFD[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x0ClW__\xFD[a\x0Cu\x83a\x0BQV[\x91Pa\x0C\x83` \x84\x01a\x0BQV[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0C\xA0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0C\xBEWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \xF2U\x8A\xD4F\x0C\xA0\x0E\xBE\xBA\xB8\xD0#r\xDDk\x02\x7F\x19\xDE{\xD9\xD72\xADt\xC5\xCF$c_EdsolcC\0\x08\x1E\x003\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\xED\"[\xD3\x85\x90.r\x8B]\xFA/K\xCD\x10+\xD0\xF4\xBB|\xA0\xBE$\xB5\x87\xB9n\xBF\xD5\xFC*d\xA2dipfsX\"\x12 \xA9\xB7\xB2\x06h\xFB\xDA}\xF9\x82\xAEN\xB8[\x18\x05s\x91\xAB\x06\xE4P\x7F\xF2\x02\xD1\xB1\xAF>;j/dsolcC\0\x08\x1E\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106101f2575f3560e01c806385226c8111610114578063c0474d0b116100a9578063e115c64211610079578063e115c642146103b6578063e20c9f71146103c9578063f09d1d44146103d1578063fa7626d4146103d9578063fb47e3a2146103e6575f5ffd5b8063c0474d0b14610371578063c09cec7714610393578063c2e9f2e4146103a6578063d883ba03146103ae575f5ffd5b8063b0464fdc116100e4578063b0464fdc14610336578063b5508aa91461033e578063ba414fa614610346578063bf8452f51461035e575f5ffd5b806385226c81146102f15780638da5cb5b14610306578063916a17c6146103195780639a22b0541461032e575f5ffd5b80633c08bd5f1161018a578063575fc5d51161015a578063575fc5d5146102ae57806366d9a9a0146102b65780636e40056f146102cb57806374742c08146102de575f5ffd5b80633c08bd5f1461027e5780633e5e3c23146102865780633f7286f41461028e5780634f19b94114610296575f5ffd5b80632ade3880116101c55780632ade38801461022e5780632b903d9b1461024357806338d52e0f1461024b5780633b8ddf7714610276575f5ffd5b8063081c69e4146101f65780630a9254e414610200578063105cda16146102085780631ed7831c14610210575b5f5ffd5b6101fe6103f9565b005b6101fe61052e565b6101fe6107b1565b610218610a81565b6040516102259190613987565b60405180910390f35b610236610ae1565b6040516102259190613a00565b6101fe610c1d565b60215461025e906001600160a01b031681565b6040516001600160a01b039091168152602001610225565b6101fe611286565b6101fe6114c9565b6102186116a5565b610218611703565b601f5461025e9061010090046001600160a01b031681565b6101fe611761565b6102be611b5f565b6040516102259190613b0d565b6101fe6102d9366004613b8b565b611cc3565b60205461025e906001600160a01b031681565b6102f9611f8f565b6040516102259190613bab565b60225461025e906001600160a01b031681565b61032161205a565b6040516102259190613c02565b6101fe61213b565b61032161256e565b6102f961264f565b61034e61271a565b6040519015158152602001610225565b6101fe61036c366004613c79565b6127d4565b6103855f516020615ea65f395f51905f5281565b604051908152602001610225565b60245461025e906001600160a01b031681565b6101fe6129f0565b6101fe612ebb565b6101fe6103c4366004613cb6565b613025565b610218613308565b6101fe613366565b601f5461034e9060ff1681565b60235461025e906001600160a01b031681565b60235460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f529063ca669fa7906024015b5f604051808303815f87803b158015610449575f5ffd5b505af115801561045b573d5f5f3e3d5ffd5b505050505f516020615e865f395f51905f526001600160a01b031663f48448146040518163ffffffff1660e01b81526004015f604051808303815f87803b1580156104a4575f5ffd5b505af11580156104b6573d5f5f3e3d5ffd5b5050601f546023546040516340c10f1960e01b81526101009092046001600160a01b0390811694506340c10f1993506104ff9291169068056bc75e2d6310000090600401613ce8565b5f604051808303815f87803b158015610516575f5ffd5b505af1158015610528573d5f5f3e3d5ffd5b50505050565b601260405161053c9061396d565b6060808252600a908201526915195cdd08105cdcd95d60b21b608082015260a06020820181905260049082015263151154d560e21b60c082015260ff909116604082015260e001604051809103905ff08015801561059c573d5f5f3e3d5ffd5b50602180546001600160a01b0319166001600160a01b039283161790556022546040516303223eab60e11b8152911660048201525f516020615e865f395f51905f52906306447d56906024015f604051808303815f87803b1580156105ff575f5ffd5b505af1158015610611573d5f5f3e3d5ffd5b50506021546040516001600160a01b0390911692505f516020615ea65f395f51905f52915061063f9061397a565b608080825260039082018190526259657360e81b60a083015260c0602083018190528201526259455360e81b60e08201526001600160a01b039092166040830152606082015261010001604051809103905ff0801580156106a2573d5f5f3e3d5ffd5b50601f8054610100600160a81b0319166101006001600160a01b03938416021790556021546040519116905f516020615ea65f395f51905f52906106e59061397a565b60808082526002908201819052614e6f60f01b60a083015260c060208301819052820152614e4f60f01b60e08201526001600160a01b039092166040830152606082015261010001604051809103905ff080158015610746573d5f5f3e3d5ffd5b5060205f6101000a8154816001600160a01b0302191690836001600160a01b031602179055505f516020615e865f395f51905f526001600160a01b03166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015610516575f5ffd5b60225460405163ca669fa760e01b81526001600160a01b03909116600482015268056bc75e2d63100000906801a055690d9db80000905f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b158015610816575f5ffd5b505af1158015610828573d5f5f3e3d5ffd5b5050601f546023546040516340c10f1960e01b81526101009092046001600160a01b0390811694506340c10f199350610868929116908690600401613ce8565b5f604051808303815f87803b15801561087f575f5ffd5b505af1158015610891573d5f5f3e3d5ffd5b505060235460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f52925063ca669fa791506024015f604051808303815f87803b1580156108e4575f5ffd5b505af11580156108f6573d5f5f3e3d5ffd5b5050601f5460245460405163a9059cbb60e01b81526101009092046001600160a01b03908116945063a9059cbb9350610936929116908590600401613ce8565b6020604051808303815f875af1158015610952573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109769190613d01565b50601f546023546040516370a0823160e01b81526001600160a01b0391821660048201526109ff926101009004909116906370a08231906024015b602060405180830381865afa1580156109cc573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109f09190613d20565b6109fa8385613d4b565b6134e9565b601f54602480546040516370a0823160e01b81526001600160a01b039182166004820152610a7d936101009004909116916370a0823191015b602060405180830381865afa158015610a53573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a779190613d20565b826134e9565b5050565b60606016805480602002602001604051908101604052809291908181526020018280548015610ad757602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311610ab9575b5050505050905090565b6060601e805480602002602001604051908101604052809291908181526020015f905b82821015610c14575f84815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b82821015610bfd578382905f5260205f20018054610b7290613d64565b80601f0160208091040260200160405190810160405280929190818152602001828054610b9e90613d64565b8015610be95780601f10610bc057610100808354040283529160200191610be9565b820191905f5260205f20905b815481529060010190602001808311610bcc57829003601f168201915b505050505081526020019060010190610b55565b505050508152505081526020019060010190610b04565b50505050905090565b604080516003808252608082019092525f91602082016060803683370190505090506802b5e3af16b1880000815f81518110610c5b57610c5b613db0565b6020026020010181815250506801a055690d9db8000081600181518110610c8457610c84613db0565b6020026020010181815250506801158e460913d0000081600281518110610cad57610cad613db0565b6020908102919091010152604080516003808252608082019092525f91816020016020820280368337505060235482519293506001600160a01b0316918391505f90610cfb57610cfb613db0565b6001600160a01b039283166020918202929092010152602454825191169082906001908110610d2c57610d2c613db0565b60200260200101906001600160a01b031690816001600160a01b031681525050610d7460405180604001604052806007815260200166636861726c696560c81b81525061354d565b81600281518110610d8757610d87613db0565b6001600160a01b0392831660209182029290920101526022546040516303223eab60e11b8152911660048201525f516020615e865f395f51905f52906306447d56906024015f604051808303815f87803b158015610de3575f5ffd5b505af1158015610df5573d5f5f3e3d5ffd5b505f925050505b8151811015610eb357601f60019054906101000a90046001600160a01b03166001600160a01b03166340c10f19838381518110610e3b57610e3b613db0565b6020026020010151858481518110610e5557610e55613db0565b60200260200101516040518363ffffffff1660e01b8152600401610e7a929190613ce8565b5f604051808303815f87803b158015610e91575f5ffd5b505af1158015610ea3573d5f5f3e3d5ffd5b505060019092019150610dfc9050565b505f805b8251811015610fb957610f8a601f60019054906101000a90046001600160a01b03166001600160a01b03166370a08231858481518110610ef957610ef9613db0565b60200260200101516040518263ffffffff1660e01b8152600401610f2c91906001600160a01b0391909116815260200190565b602060405180830381865afa158015610f47573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f6b9190613d20565b858381518110610f7d57610f7d613db0565b60200260200101516134e9565b838181518110610f9c57610f9c613db0565b602002602001015182610faf9190613dc4565b9150600101610eb7565b5061100e601f60019054906101000a90046001600160a01b03166001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610a53573d5f5f3e3d5ffd5b678ac7230489e800005f5b83518110156110b757601f60019054906101000a90046001600160a01b03166001600160a01b0316639dc29fac85838151811061105857611058613db0565b6020026020010151846040518363ffffffff1660e01b815260040161107e929190613ce8565b5f604051808303815f87803b158015611095575f5ffd5b505af11580156110a7573d5f5f3e3d5ffd5b5050600190920191506110199050565b505f516020615e865f395f51905f526001600160a01b03166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b1580156110fd575f5ffd5b505af115801561110f573d5f5f3e3d5ffd5b505f925050505b83518110156111f7576111ef601f60019054906101000a90046001600160a01b03166001600160a01b03166370a0823186848151811061115857611158613db0565b60200260200101516040518263ffffffff1660e01b815260040161118b91906001600160a01b0391909116815260200190565b602060405180830381865afa1580156111a6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111ca9190613d20565b838784815181106111dd576111dd613db0565b60200260200101516109fa9190613d4b565b600101611116565b50610528601f60019054906101000a90046001600160a01b03166001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561124c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112709190613d20565b845161127c9084613dd7565b6109fa9085613d4b565b60225460405163ca669fa760e01b81526001600160a01b03909116600482015268056bc75e2d631000009068022b1c8c1227a00000905f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b1580156112eb575f5ffd5b505af11580156112fd573d5f5f3e3d5ffd5b5050601f546023546040516340c10f1960e01b81526101009092046001600160a01b0390811694506340c10f19935061133d929116908690600401613ce8565b5f604051808303815f87803b158015611354575f5ffd5b505af1158015611366573d5f5f3e3d5ffd5b505060225460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f52925063ca669fa791506024015f604051808303815f87803b1580156113b9575f5ffd5b505af11580156113cb573d5f5f3e3d5ffd5b5050601f54602354604051632770a7eb60e21b81526101009092046001600160a01b039081169450639dc29fac935061140b929116908590600401613ce8565b5f604051808303815f87803b158015611422575f5ffd5b505af1158015611434573d5f5f3e3d5ffd5b5050601f546023546040516370a0823160e01b81526001600160a01b03918216600482015261147594506101009092041691506370a08231906024016109b1565b610a7d601f60019054906101000a90046001600160a01b03166001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156109cc573d5f5f3e3d5ffd5b60225460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b158015611518575f5ffd5b505af115801561152a573d5f5f3e3d5ffd5b5050601f546023546040516340c10f1960e01b81526101009092046001600160a01b0390811694506340c10f1993506115739291169068056bc75e2d6310000090600401613ce8565b5f604051808303815f87803b15801561158a575f5ffd5b505af115801561159c573d5f5f3e3d5ffd5b505060235460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f52925063ca669fa791506024015f604051808303815f87803b1580156115ef575f5ffd5b505af1158015611601573d5f5f3e3d5ffd5b505050505f516020615e865f395f51905f526001600160a01b031663f48448146040518163ffffffff1660e01b81526004015f604051808303815f87803b15801561164a575f5ffd5b505af115801561165c573d5f5f3e3d5ffd5b5050601f54602354604051632770a7eb60e21b81526101009092046001600160a01b039081169450639dc29fac93506104ff929116906802b5e3af16b188000090600401613ce8565b60606018805480602002602001604051908101604052809291908181526020018280548015610ad757602002820191905f5260205f209081546001600160a01b03168152600190910190602001808311610ab9575050505050905090565b60606017805480602002602001604051908101604052809291908181526020018280548015610ad757602002820191905f5260205f209081546001600160a01b03168152600190910190602001808311610ab9575050505050905090565b5f61178b604051806040016040528060088152602001673732bba7bbb732b960c11b81525061354d565b60225460405163ca669fa760e01b81526001600160a01b0390911660048201529091505f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b1580156117dd575f5ffd5b505af11580156117ef573d5f5f3e3d5ffd5b5050601f5460405163f2fde38b60e01b81526001600160a01b038581166004830152610100909204909116925063f2fde38b91506024015f604051808303815f87803b15801561183d575f5ffd5b505af115801561184f573d5f5f3e3d5ffd5b505050506118d1601f60019054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156118a7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118cb9190613dee565b8261355e565b60225460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b158015611920575f5ffd5b505af1158015611932573d5f5f3e3d5ffd5b505050505f516020615e865f395f51905f526001600160a01b031663f48448146040518163ffffffff1660e01b81526004015f604051808303815f87803b15801561197b575f5ffd5b505af115801561198d573d5f5f3e3d5ffd5b5050601f546023546040516340c10f1960e01b81526101009092046001600160a01b0390811694506340c10f1993506119d69291169068056bc75e2d6310000090600401613ce8565b5f604051808303815f87803b1580156119ed575f5ffd5b505af11580156119ff573d5f5f3e3d5ffd5b505060405163ca669fa760e01b81526001600160a01b03841660048201525f516020615e865f395f51905f52925063ca669fa791506024015f604051808303815f87803b158015611a4e575f5ffd5b505af1158015611a60573d5f5f3e3d5ffd5b5050601f546023546040516340c10f1960e01b81526101009092046001600160a01b0390811694506340c10f199350611aa99291169068056bc75e2d6310000090600401613ce8565b5f604051808303815f87803b158015611ac0575f5ffd5b505af1158015611ad2573d5f5f3e3d5ffd5b5050601f546023546040516370a0823160e01b81526001600160a01b039182166004820152611b5c94506101009092041691506370a0823190602401602060405180830381865afa158015611b29573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b4d9190613d20565b68056bc75e2d631000006134e9565b50565b6060601b805480602002602001604051908101604052809291908181526020015f905b82821015610c14578382905f5260205f2090600202016040518060400160405290815f82018054611bb290613d64565b80601f0160208091040260200160405190810160405280929190818152602001828054611bde90613d64565b8015611c295780601f10611c0057610100808354040283529160200191611c29565b820191905f5260205f20905b815481529060010190602001808311611c0c57829003601f168201915b5050505050815260200160018201805480602002602001604051908101604052809291908181526020018280548015611cab57602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b03191681526020019060040190602082600301049283019260010382029150808411611c6d5790505b50505050508152505081526020019060010190611b82565b611cd68260016001600160801b0361359f565b9150611ce3815f8461359f565b60225460405163ca669fa760e01b81526001600160a01b0390911660048201529091505f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b158015611d35575f5ffd5b505af1158015611d47573d5f5f3e3d5ffd5b5050601f546023546040516340c10f1960e01b81526101009092046001600160a01b0390811694506340c10f199350611d87929116908690600401613ce8565b5f604051808303815f87803b158015611d9e575f5ffd5b505af1158015611db0573d5f5f3e3d5ffd5b505060235460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f52925063ca669fa791506024015f604051808303815f87803b158015611e03575f5ffd5b505af1158015611e15573d5f5f3e3d5ffd5b5050601f5460245460405163a9059cbb60e01b81526101009092046001600160a01b03908116945063a9059cbb9350611e55929116908590600401613ce8565b6020604051808303815f875af1158015611e71573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e959190613d01565b50601f546023546040516370a0823160e01b81526001600160a01b039182166004820152611ed4926101009004909116906370a08231906024016109b1565b601f54602480546040516370a0823160e01b81526001600160a01b039182166004820152611f11936101009004909116916370a082319101610a38565b610a7d601f60019054906101000a90046001600160a01b03166001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611f65573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f899190613d20565b836134e9565b6060601a805480602002602001604051908101604052809291908181526020015f905b82821015610c14578382905f5260205f20018054611fcf90613d64565b80601f0160208091040260200160405190810160405280929190818152602001828054611ffb90613d64565b80156120465780601f1061201d57610100808354040283529160200191612046565b820191905f5260205f20905b81548152906001019060200180831161202957829003601f168201915b505050505081526020019060010190611fb2565b6060601d805480602002602001604051908101604052809291908181526020015f905b82821015610c14575f8481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561212357602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116120e55790505b5050505050815250508152602001906001019061207d565b60225460405163ca669fa760e01b81526001600160a01b03909116600482015268056bc75e2d63100000906802b5e3af16b1880000906801a055690d9db80000905f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b1580156121ab575f5ffd5b505af11580156121bd573d5f5f3e3d5ffd5b5050601f546023546040516340c10f1960e01b81526101009092046001600160a01b0390811694506340c10f1993506121fd929116908790600401613ce8565b5f604051808303815f87803b158015612214575f5ffd5b505af1158015612226573d5f5f3e3d5ffd5b505060235460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f52925063ca669fa791506024015f604051808303815f87803b158015612279575f5ffd5b505af115801561228b573d5f5f3e3d5ffd5b5050601f5460245460405163095ea7b360e01b81526101009092046001600160a01b03908116945063095ea7b393506122cb929116908690600401613ce8565b6020604051808303815f875af11580156122e7573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061230b9190613d01565b50601f5460235460248054604051636eb1769f60e11b81526001600160a01b0393841660048201529083169181019190915261236f9261010090049091169063dd62ed3e906044015b602060405180830381865afa158015611f65573d5f5f3e3d5ffd5b6024805460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f529163ca669fa791015b5f604051808303815f87803b1580156123be575f5ffd5b505af11580156123d0573d5f5f3e3d5ffd5b5050601f54602354602480546040516323b872dd60e01b81526001600160a01b03938416600482015290831691810191909152604481018690526101009092041692506323b872dd91506064016020604051808303815f875af1158015612439573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061245d9190613d01565b50601f546023546040516370a0823160e01b81526001600160a01b0391821660048201526124e0926101009004909116906370a0823190602401602060405180830381865afa1580156124b2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906124d69190613d20565b6109fa8386613d4b565b601f54602480546040516370a0823160e01b81526001600160a01b03918216600482015261251d936101009004909116916370a082319101610a38565b601f5460235460248054604051636eb1769f60e11b81526001600160a01b039384166004820152908316918101919091526125699261010090049091169063dd62ed3e906044016109b1565b505050565b6060601c805480602002602001604051908101604052809291908181526020015f905b82821015610c14575f8481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561263757602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116125f95790505b50505050508152505081526020019060010190612591565b60606019805480602002602001604051908101604052809291908181526020015f905b82821015610c14578382905f5260205f2001805461268f90613d64565b80601f01602080910402602001604051908101604052809291908181526020018280546126bb90613d64565b80156127065780601f106126dd57610100808354040283529160200191612706565b820191905f5260205f20905b8154815290600101906020018083116126e957829003601f168201915b505050505081526020019060010190612672565b6008545f9060ff1615612731575060085460ff1690565b604051630667f9d760e41b81525f905f516020615e865f395f51905f529063667f9d709061278e907f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d906519985a5b195960d21b90600401613ce8565b602060405180830381865afa1580156127a9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906127cd9190613d20565b1415905090565b6127e78360016001600160801b0361359f565b92506127f4825f8561359f565b9150612801815f8461359f565b60225460405163ca669fa760e01b81526001600160a01b0390911660048201529091505f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b158015612853575f5ffd5b505af1158015612865573d5f5f3e3d5ffd5b5050601f546023546040516340c10f1960e01b81526101009092046001600160a01b0390811694506340c10f1993506128a5929116908790600401613ce8565b5f604051808303815f87803b1580156128bc575f5ffd5b505af11580156128ce573d5f5f3e3d5ffd5b505060235460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f52925063ca669fa791506024015f604051808303815f87803b158015612921575f5ffd5b505af1158015612933573d5f5f3e3d5ffd5b5050601f5460245460405163095ea7b360e01b81526101009092046001600160a01b03908116945063095ea7b39350612973929116908690600401613ce8565b6020604051808303815f875af115801561298f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129b39190613d01565b506024805460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f529163ca669fa791016123a7565b612a8b601f60019054906101000a90046001600160a01b03166001600160a01b03166306fdde036040518163ffffffff1660e01b81526004015f60405180830381865afa158015612a43573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612a6a9190810190613e09565b6040518060400160405280600381526020016259657360e81b8152506135e2565b612b26601f60019054906101000a90046001600160a01b03166001600160a01b03166395d89b416040518163ffffffff1660e01b81526004015f60405180830381865afa158015612ade573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612b059190810190613e09565b6040518060400160405280600381526020016259455360e81b8152506135e2565b612baf601f60019054906101000a90046001600160a01b03166001600160a01b0316634800d97f6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612b7a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b9e9190613dee565b6021546001600160a01b031661355e565b612c3a601f60019054906101000a90046001600160a01b03166001600160a01b031663c0474d0b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612c03573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612c279190613d20565b5f516020615ea65f395f51905f52613614565b612cc3601f60019054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612c8e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612cb29190613dee565b6022546001600160a01b031661355e565b602054604080516306fdde0360e01b81529051612d52926001600160a01b0316916306fdde03916004808301925f9291908290030181865afa158015612d0b573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612d329190810190613e09565b604051806040016040528060028152602001614e6f60f01b8152506135e2565b602054604080516395d89b4160e01b81529051612de1926001600160a01b0316916395d89b41916004808301925f9291908290030181865afa158015612d9a573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612dc19190810190613e09565b604051806040016040528060028152602001614e4f60f01b8152506135e2565b6020805460408051634800d97f60e01b81529051612e29936001600160a01b0390931692634800d97f92600480820193918290030181865afa158015612b7a573d5f5f3e3d5ffd5b602080546040805163c0474d0b60e01b81529051612e71936001600160a01b039093169263c0474d0b92600480820193918290030181865afa158015612c03573d5f5f3e3d5ffd5b6020805460408051638da5cb5b60e01b81529051612eb9936001600160a01b0390931692638da5cb5b92600480820193918290030181865afa158015612c8e573d5f5f3e3d5ffd5b565b60225460405163ca669fa760e01b81526001600160a01b03909116600482015268056bc75e2d63100000905f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b158015612f15575f5ffd5b505af1158015612f27573d5f5f3e3d5ffd5b5050601f546023546040516340c10f1960e01b81526101009092046001600160a01b0390811694506340c10f199350612f67929116908590600401613ce8565b5f604051808303815f87803b158015612f7e575f5ffd5b505af1158015612f90573d5f5f3e3d5ffd5b5050601f546023546040516370a0823160e01b81526001600160a01b039182166004820152612fd194506101009092041691506370a0823190602401610a38565b611b5c601f60019054906101000a90046001600160a01b03166001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610a53573d5f5f3e3d5ffd5b604051632631f2b160e11b81526001600160a01b038416151560048201525f516020615e865f395f51905f5290634c63e562906024015f6040518083038186803b158015613071575f5ffd5b505afa158015613083573d5f5f3e3d5ffd5b5050505061309c8260016001600160801b03801661359f565b91506130a9815f8461359f565b60225460405163ca669fa760e01b81526001600160a01b0390911660048201529091505f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b1580156130fb575f5ffd5b505af115801561310d573d5f5f3e3d5ffd5b5050601f546040516340c10f1960e01b81526101009091046001600160a01b031692506340c10f1991506131479086908690600401613ce8565b5f604051808303815f87803b15801561315e575f5ffd5b505af1158015613170573d5f5f3e3d5ffd5b5050601f546040516370a0823160e01b81526001600160a01b0387811660048301526131b0945061010090920490911691506370a0823190602401612354565b60225460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b1580156131ff575f5ffd5b505af1158015613211573d5f5f3e3d5ffd5b5050601f54604051632770a7eb60e21b81526101009091046001600160a01b03169250639dc29fac915061324b9086908590600401613ce8565b5f604051808303815f87803b158015613262575f5ffd5b505af1158015613274573d5f5f3e3d5ffd5b5050601f546040516370a0823160e01b81526001600160a01b0387811660048301526132b4945061010090920490911691506370a08231906024016109b1565b612569601f60019054906101000a90046001600160a01b03166001600160a01b03166318160ddd6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156109cc573d5f5f3e3d5ffd5b60606015805480602002602001604051908101604052809291908181526020018280548015610ad757602002820191905f5260205f209081546001600160a01b03168152600190910190602001808311610ab9575050505050905090565b60225460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f529063ca669fa7906024015f604051808303815f87803b1580156133b5575f5ffd5b505af11580156133c7573d5f5f3e3d5ffd5b50505050601f60019054906101000a90046001600160a01b03166001600160a01b031663715018a66040518163ffffffff1660e01b81526004015f604051808303815f87803b158015613418575f5ffd5b505af115801561342a573d5f5f3e3d5ffd5b505050506134ac601f60019054906101000a90046001600160a01b03166001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613482573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906134a69190613dee565b5f61355e565b60225460405163ca669fa760e01b81526001600160a01b0390911660048201525f516020615e865f395f51905f529063ca669fa790602401610432565b60405163260a5b1560e21b815260048101839052602481018290525f516020615e865f395f51905f52906398296c54906044015b5f6040518083038186803b158015613533575f5ffd5b505afa158015613545573d5f5f3e3d5ffd5b505050505050565b5f6135578261364c565b5092915050565b6040516328a9b0fb60e11b81526001600160a01b038084166004830152821660248201525f516020615e865f395f51905f529063515361f69060440161351d565b5f6135ab84848461374c565b90506135db6040518060400160405280600c81526020016b109bdd5b99081c995cdd5b1d60a21b81525082613909565b9392505050565b60405163f320d96360e01b81525f516020615e865f395f51905f529063f320d9639061351d9085908590600401613ebc565b604051637c84c69b60e01b815260048101839052602481018290525f516020615e865f395f51905f5290637c84c69b9060440161351d565b5f5f8260405160200161365f9190613ee9565b60408051808303601f190181529082905280516020909101206001625e79b760e01b031982526004820181905291505f516020615e865f395f51905f529063ffa1864990602401602060405180830381865afa1580156136c1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906136e59190613dee565b6040516318caf8e360e31b81529092505f516020615e865f395f51905f529063c657c7189061371a9085908790600401613eff565b5f604051808303815f87803b158015613731575f5ffd5b505af1158015613743573d5f5f3e3d5ffd5b50505050915091565b5f818311156137c75760405162461bcd60e51b815260206004820152603e60248201527f5374645574696c7320626f756e642875696e743235362c75696e743235362c7560448201527f696e74323536293a204d6178206973206c657373207468616e206d696e2e0000606482015260840160405180910390fd5b8284101580156137d75750818411155b156137e35750826135db565b5f6137ee8484613d4b565b6137f9906001613dc4565b90506003851115801561380b57508481115b156138225761381a8585613dc4565b9150506135db565b61382e60035f19613d4b565b85101580156138465750613843855f19613d4b565b81115b1561386057613856855f19613d4b565b61381a9084613d4b565b828511156138b3575f6138738487613d4b565b90505f6138808383613f2a565b9050805f03613894578493505050506135db565b60016138a08288613dc4565b6138aa9190613d4b565b93505050613901565b83851015613901575f6138c68686613d4b565b90505f6138d38383613f2a565b9050805f036138e7578593505050506135db565b6138f18186613d4b565b6138fc906001613dc4565b935050505b509392505050565b610a7d828260405160240161391f929190613f49565b60408051601f198184030181529190526020810180516001600160e01b0316632d839cb360e21b179052611b5c8180516a636f6e736f6c652e6c6f67602083015f808483855afa5050505050565b610ed480613f6b83390190565b61104780614e3f83390190565b602080825282518282018190525f918401906040840190835b818110156139c75783516001600160a01b03168352602093840193909201916001016139a0565b509095945050505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015613abd57603f19878603018452815180516001600160a01b03168652602090810151604082880181905281519088018190529101906060600582901b8801810191908801905f5b81811015613aa357605f198a8503018352613a8d8486516139d2565b6020958601959094509290920191600101613a71565b509197505050602094850194929092019150600101613a26565b50929695505050505050565b5f8151808452602084019350602083015f5b82811015613b035781516001600160e01b031916865260209586019590910190600101613adb565b5093949350505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015613abd57603f198786030184528151805160408752613b5960408801826139d2565b9050602082015191508681036020880152613b748183613ac9565b965050506020938401939190910190600101613b33565b5f5f60408385031215613b9c575f5ffd5b50508035926020909101359150565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015613abd57603f19878603018452613bed8583516139d2565b94506020938401939190910190600101613bd1565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015613abd57868503603f19018452815180516001600160a01b03168652602090810151604091870182905290613c6390870182613ac9565b9550506020938401939190910190600101613c28565b5f5f5f60608486031215613c8b575f5ffd5b505081359360208301359350604090920135919050565b6001600160a01b0381168114611b5c575f5ffd5b5f5f5f60608486031215613cc8575f5ffd5b8335613cd381613ca2565b95602085013595506040909401359392505050565b6001600160a01b03929092168252602082015260400190565b5f60208284031215613d11575f5ffd5b815180151581146135db575f5ffd5b5f60208284031215613d30575f5ffd5b5051919050565b634e487b7160e01b5f52601160045260245ffd5b81810381811115613d5e57613d5e613d37565b92915050565b600181811c90821680613d7857607f821691505b602082108103613d9657634e487b7160e01b5f52602260045260245ffd5b50919050565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b80820180821115613d5e57613d5e613d37565b8082028115828204841417613d5e57613d5e613d37565b5f60208284031215613dfe575f5ffd5b81516135db81613ca2565b5f60208284031215613e19575f5ffd5b815167ffffffffffffffff811115613e2f575f5ffd5b8201601f81018413613e3f575f5ffd5b805167ffffffffffffffff811115613e5957613e59613d9c565b604051601f8201601f19908116603f0116810167ffffffffffffffff81118282101715613e8857613e88613d9c565b604052818152828201602001861015613e9f575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b604081525f613ece60408301856139d2565b8281036020840152613ee081856139d2565b95945050505050565b5f82518060208501845e5f920191825250919050565b6001600160a01b03831681526040602082018190525f90613f22908301846139d2565b949350505050565b5f82613f4457634e487b7160e01b5f52601260045260245ffd5b500690565b604081525f613f5b60408301856139d2565b9050826020830152939250505056fe60a060405234801561000f575f5ffd5b50604051610ed4380380610ed483398101604081905261002e91610109565b5f610039848261020a565b506001610046838261020a565b506002805460ff191660ff929092169190911790555080516020909101206080526102c4565b634e487b7160e01b5f52604160045260245ffd5b5f82601f83011261008f575f5ffd5b81516001600160401b038111156100a8576100a861006c565b604051601f8201601f19908116603f011681016001600160401b03811182821017156100d6576100d661006c565b6040528181528382016020018510156100ed575f5ffd5b8160208501602083015e5f918101602001919091529392505050565b5f5f5f6060848603121561011b575f5ffd5b83516001600160401b03811115610130575f5ffd5b61013c86828701610080565b602086015190945090506001600160401b03811115610159575f5ffd5b61016586828701610080565b925050604084015160ff8116811461017b575f5ffd5b809150509250925092565b600181811c9082168061019a57607f821691505b6020821081036101b857634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561020557805f5260205f20601f840160051c810160208510156101e35750805b601f840160051c820191505b81811015610202575f81556001016101ef565b50505b505050565b81516001600160401b038111156102235761022361006c565b610237816102318454610186565b846101be565b6020601f821160018114610269575f83156102525750848201515b5f19600385901b1c1916600184901b178455610202565b5f84815260208120601f198516915b828110156102985787850151825560209485019460019092019101610278565b50848210156102b557868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b608051610bf16102e35f395f818161039901526104db0152610bf15ff3fe608060405234801561000f575f5ffd5b50600436106100fb575f3560e01c80637ecebe0011610093578063d30ed3b311610063578063d30ed3b314610217578063d505accf1461022a578063dd62ed3e1461023d578063f83d179114610250575f5ffd5b80637ecebe00146101c457806395d89b41146101e95780639dc29fac146101f1578063a9059cbb14610204575f5ffd5b8063313ce567116100ce578063313ce5671461016d5780633644e5151461018257806340c10f191461018a57806370a082311461019f575f5ffd5b806306fdde03146100ff578063095ea7b31461011d57806318160ddd1461014057806323b872dd1461015a575b5f5ffd5b610107610263565b60405161011491906109ff565b60405180910390f35b61013061012b366004610a4a565b6102f2565b6040519015158152602001610114565b6805345cdf77eb68f44c545b604051908152602001610114565b610130610168366004610a72565b610372565b60025460405160ff9091168152602001610114565b61014c610396565b61019d610198366004610a4a565b610438565b005b61014c6101ad366004610aac565b6387a211a2600c9081525f91909152602090205490565b61014c6101d2366004610aac565b6338377508600c9081525f91909152602090205490565b61010761044e565b61019d6101ff366004610a4a565b61045d565b610130610212366004610a4a565b61046f565b61019d610225366004610a72565b610489565b61019d610238366004610ac5565b6104a9565b61014c61024b366004610b32565b610683565b61019d61025e366004610a72565b6106c7565b60605f805461027190610b63565b80601f016020809104026020016040519081016040528092919081815260200182805461029d90610b63565b80156102e85780601f106102bf576101008083540402835291602001916102e8565b820191905f5260205f20905b8154815290600101906020018083116102cb57829003601f168201915b5050505050905090565b5f6001600160a01b0383166e22d473030f116ddee9f6b43ac78ba3188219151761032357633f68539a5f526004601cfd5b82602052637f5e9f20600c52335f52816034600c2055815f52602c5160601c337f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560205fa35060015b92915050565b5f61038e61037f856106e2565b610388856106e2565b84610703565b949350505050565b5f7f0000000000000000000000000000000000000000000000000000000000000000806103cf576103c5610263565b8051906020012090505b604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f815260208101929092527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc69082015246606082015230608082015260a09020919050565b61044a610444836106e2565b826107bf565b5050565b60606001805461027190610b63565b61044a610469836106e2565b82610828565b5f61048261047c846106e2565b83610889565b9392505050565b6104a4610495846106e2565b61049e846106e2565b836108ed565b505050565b6001600160a01b0386166e22d473030f116ddee9f6b43ac78ba318851915176104d957633f68539a5f526004601cfd5b7f00000000000000000000000000000000000000000000000000000000000000008061051157610507610263565b8051906020012090505b7fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc64286101561054757631a15a3cc5f526004601cfd5b6040518960601b60601c99508860601b60601c985065383775081901600e52895f526020600c2080547f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f835284602084015283604084015246606084015230608084015260a08320602e527f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c983528b60208401528a60408401528960608401528060808401528860a084015260c08320604e526042602c205f528760ff16602052866040528560605260208060805f60015afa8c3d511461062f5763ddafbaef5f526004601cfd5b0190556303faf4f960a51b89176040526034602c20889055888a7f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925602060608501a360405250505f60605250505050505050565b5f6e22d473030f116ddee9f6b43ac78ba2196001600160a01b038316016106ac57505f1961036c565b50602052637f5e9f20600c9081525f91909152603490205490565b6104a46106d3846106e2565b6106dc846106e2565b83610951565b5f6001600160a01b0382168060a06106f9826109b7565b901b189392505050565b5f8360601b6e22d473030f116ddee9f6b43ac78ba333146107585733602052637f5e9f208117600c526034600c208054801915610755578085111561074f576313be252b5f526004601cfd5b84810382555b50505b6387a211a28117600c526020600c2080548085111561077e5763f4d678b85f526004601cfd5b84810382555050835f526020600c208381540181555082602052600c5160601c8160601c5f516020610b9c5f395f51905f52602080a3505060019392505050565b6805345cdf77eb68f44c54818101818110156107e25763e5cfe9575f526004601cfd5b806805345cdf77eb68f44c5550506387a211a2600c52815f526020600c208181540181555080602052600c5160601c5f5f516020610b9c5f395f51905f52602080a35050565b6387a211a2600c52815f526020600c2080548083111561084f5763f4d678b85f526004601cfd5b82900390556805345cdf77eb68f44c805482900390555f8181526001600160a01b0383165f516020610b9c5f395f51905f52602083a35050565b5f6387a211a2600c52335f526020600c208054808411156108b15763f4d678b85f526004601cfd5b83810382555050825f526020600c208281540181555081602052600c5160601c335f516020610b9c5f395f51905f52602080a350600192915050565b6e22d473030f116ddee9f6b43ac78ba2196001600160a01b0383160161091257505050565b81602052637f5e9f20600c52825f526034600c20805480191561094a5780831115610944576313be252b5f526004601cfd5b82810382555b5050505050565b8260601b6387a211a28117600c526020600c2080548084111561097b5763f4d678b85f526004601cfd5b83810382555050825f526020600c208281540181555081602052600c5160601c8160601c5f516020610b9c5f395f51905f52602080a350505050565b604051365f8237368120602052601051821860105260885f2090508060105260bc19700100000000000000000000000000000051820960801c6007166109fa57505f5b919050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b80356001600160a01b03811681146109fa575f5ffd5b5f5f60408385031215610a5b575f5ffd5b610a6483610a34565b946020939093013593505050565b5f5f5f60608486031215610a84575f5ffd5b610a8d84610a34565b9250610a9b60208501610a34565b929592945050506040919091013590565b5f60208284031215610abc575f5ffd5b61048282610a34565b5f5f5f5f5f5f5f60e0888a031215610adb575f5ffd5b610ae488610a34565b9650610af260208901610a34565b95506040880135945060608801359350608088013560ff81168114610b15575f5ffd5b9699959850939692959460a0840135945060c09093013592915050565b5f5f60408385031215610b43575f5ffd5b610b4c83610a34565b9150610b5a60208401610a34565b90509250929050565b600181811c90821680610b7757607f821691505b602082108103610b9557634e487b7160e01b5f52602260045260245ffd5b5091905056feddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa26469706673582212200c45acc3c6a9d15890ebba8720cb2063e27c6daab33079386bb3dcaecfa64cfb64736f6c634300081e003360c060405234801561000f575f5ffd5b5060405161104738038061104783398101604081905261002e91610143565b5f6100398582610252565b5060016100468482610252565b506001600160a01b03821660805260a08190526100623361006b565b5050505061030c565b6001600160a01b0316638b78c6d819819055805f7f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08180a350565b634e487b7160e01b5f52604160045260245ffd5b5f82601f8301126100c9575f5ffd5b81516001600160401b038111156100e2576100e26100a6565b604051601f8201601f19908116603f011681016001600160401b0381118282101715610110576101106100a6565b604052818152838201602001851015610127575f5ffd5b8160208501602083015e5f918101602001919091529392505050565b5f5f5f5f60808587031215610156575f5ffd5b84516001600160401b0381111561016b575f5ffd5b610177878288016100ba565b602087015190955090506001600160401b03811115610194575f5ffd5b6101a0878288016100ba565b604087015190945090506001600160a01b03811681146101be575f5ffd5b6060959095015193969295505050565b600181811c908216806101e257607f821691505b60208210810361020057634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561024d57805f5260205f20601f840160051c8101602085101561022b5750805b601f840160051c820191505b8181101561024a575f8155600101610237565b50505b505050565b81516001600160401b0381111561026b5761026b6100a6565b61027f8161027984546101ce565b84610206565b6020601f8211600181146102b1575f831561029a5750848201515b5f19600385901b1c1916600184901b17845561024a565b5f84815260208120601f198516915b828110156102e057878501518255602094850194600190920191016102c0565b50848210156102fd57868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b60805160a051610d1a61032d5f395f61036e01525f6102470152610d1a5ff3fe60806040526004361061013c575f3560e01c8063715018a6116100b3578063c0474d0b1161006d578063c0474d0b1461035d578063d505accf14610390578063dd62ed3e146103af578063f04e283e146103ce578063f2fde38b146103e1578063fee81cf4146103f4575f5ffd5b8063715018a6146102ba5780637ecebe00146102c25780638da5cb5b146102f357806395d89b411461030b5780639dc29fac1461031f578063a9059cbb1461033e575f5ffd5b8063313ce56711610104578063313ce567146101e85780633644e5151461020357806340c10f19146102175780634800d97f1461023657806354d1f13d1461028157806370a0823114610289575f5ffd5b806306fdde0314610140578063095ea7b31461016a57806318160ddd1461019957806323b872dd146101bf57806325692962146101de575b5f5ffd5b34801561014b575f5ffd5b50610154610425565b6040516101619190610b1c565b60405180910390f35b348015610175575f5ffd5b50610189610184366004610b6c565b6104b4565b6040519015158152602001610161565b3480156101a4575f5ffd5b506805345cdf77eb68f44c545b604051908152602001610161565b3480156101ca575f5ffd5b506101896101d9366004610b94565b610534565b6101e66105f0565b005b3480156101f3575f5ffd5b5060405160128152602001610161565b34801561020e575f5ffd5b506101b161063d565b348015610222575f5ffd5b506101e6610231366004610b6c565b6106b9565b348015610241575f5ffd5b506102697f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610161565b6101e66106cf565b348015610294575f5ffd5b506101b16102a3366004610bce565b6387a211a2600c9081525f91909152602090205490565b6101e6610708565b3480156102cd575f5ffd5b506101b16102dc366004610bce565b6338377508600c9081525f91909152602090205490565b3480156102fe575f5ffd5b50638b78c6d81954610269565b348015610316575f5ffd5b5061015461071b565b34801561032a575f5ffd5b506101e6610339366004610b6c565b61072a565b348015610349575f5ffd5b50610189610358366004610b6c565b61073c565b348015610368575f5ffd5b506101b17f000000000000000000000000000000000000000000000000000000000000000081565b34801561039b575f5ffd5b506101e66103aa366004610bee565b6107a0565b3480156103ba575f5ffd5b506101b16103c9366004610c5b565b610954565b6101e66103dc366004610bce565b610998565b6101e66103ef366004610bce565b6109d5565b3480156103ff575f5ffd5b506101b161040e366004610bce565b63389a75e1600c9081525f91909152602090205490565b60605f805461043390610c8c565b80601f016020809104026020016040519081016040528092919081815260200182805461045f90610c8c565b80156104aa5780601f10610481576101008083540402835291602001916104aa565b820191905f5260205f20905b81548152906001019060200180831161048d57829003601f168201915b5050505050905090565b5f6001600160a01b0383166e22d473030f116ddee9f6b43ac78ba318821915176104e557633f68539a5f526004601cfd5b82602052637f5e9f20600c52335f52816034600c2055815f52602c5160601c337f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560205fa35060015b92915050565b5f8360601b6e22d473030f116ddee9f6b43ac78ba333146105895733602052637f5e9f208117600c526034600c2080548019156105865780851115610580576313be252b5f526004601cfd5b84810382555b50505b6387a211a28117600c526020600c208054808511156105af5763f4d678b85f526004601cfd5b84810382555050835f526020600c208381540181555082602052600c5160601c8160601c5f516020610cc55f395f51905f52602080a3505060019392505050565b5f6202a30067ffffffffffffffff164201905063389a75e1600c52335f52806020600c2055337fdbf36a107da19e49527a7176a1babf963b4b0ff8cde35ee35d6cd8f1f9ac7e1d5f5fa250565b5f80610647610425565b805190602001209050604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f815260208101929092527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc69082015246606082015230608082015260a09020919050565b6106c16109fb565b6106cb8282610a15565b5050565b63389a75e1600c52335f525f6020600c2055337ffa7b8eab7da67f412cc9575ed43464468f9bfbae89d1675917346ca6d8fe3c925f5fa2565b6107106109fb565b6107195f610a7e565b565b60606001805461043390610c8c565b6107326109fb565b6106cb8282610abb565b5f6387a211a2600c52335f526020600c208054808411156107645763f4d678b85f526004601cfd5b83810382555050825f526020600c208281540181555081602052600c5160601c335f516020610cc55f395f51905f52602080a350600192915050565b6001600160a01b0386166e22d473030f116ddee9f6b43ac78ba318851915176107d057633f68539a5f526004601cfd5b5f6107d9610425565b8051906020012090507fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc64286101561081857631a15a3cc5f526004601cfd5b6040518960601b60601c99508860601b60601c985065383775081901600e52895f526020600c2080547f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f835284602084015283604084015246606084015230608084015260a08320602e527f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c983528b60208401528a60408401528960608401528060808401528860a084015260c08320604e526042602c205f528760ff16602052866040528560605260208060805f60015afa8c3d51146109005763ddafbaef5f526004601cfd5b0190556303faf4f960a51b89176040526034602c20889055888a7f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925602060608501a360405250505f60605250505050505050565b5f6e22d473030f116ddee9f6b43ac78ba2196001600160a01b0383160161097d57505f1961052e565b50602052637f5e9f20600c9081525f91909152603490205490565b6109a06109fb565b63389a75e1600c52805f526020600c2080544211156109c657636f5e88185f526004601cfd5b5f90556109d281610a7e565b50565b6109dd6109fb565b8060601b6109f257637448fbae5f526004601cfd5b6109d281610a7e565b638b78c6d819543314610719576382b429005f526004601cfd5b6805345cdf77eb68f44c5481810181811015610a385763e5cfe9575f526004601cfd5b806805345cdf77eb68f44c5550506387a211a2600c52815f526020600c208181540181555080602052600c5160601c5f5f516020610cc55f395f51905f52602080a35050565b638b78c6d81980546001600160a01b039092169182907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a355565b6387a211a2600c52815f526020600c20805480831115610ae25763f4d678b85f526004601cfd5b82900390556805345cdf77eb68f44c805482900390555f8181526001600160a01b0383165f516020610cc55f395f51905f52602083a35050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b80356001600160a01b0381168114610b67575f5ffd5b919050565b5f5f60408385031215610b7d575f5ffd5b610b8683610b51565b946020939093013593505050565b5f5f5f60608486031215610ba6575f5ffd5b610baf84610b51565b9250610bbd60208501610b51565b929592945050506040919091013590565b5f60208284031215610bde575f5ffd5b610be782610b51565b9392505050565b5f5f5f5f5f5f5f60e0888a031215610c04575f5ffd5b610c0d88610b51565b9650610c1b60208901610b51565b95506040880135945060608801359350608088013560ff81168114610c3e575f5ffd5b9699959850939692959460a0840135945060c09093013592915050565b5f5f60408385031215610c6c575f5ffd5b610c7583610b51565b9150610c8360208401610b51565b90509250929050565b600181811c90821680610ca057607f821691505b602082108103610cbe57634e487b7160e01b5f52602260045260245ffd5b5091905056feddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa2646970667358221220f2558ad4460ca00ebebab8d02372dd6b027f19de7bd9d732ad74c5cf24635f4564736f6c634300081e00330000000000000000000000007109709ecfa91a80626ff3989d68f67f5b1dd12ded225bd385902e728b5dfa2f4bcd102bd0f4bb7ca0be24b587b96ebfd5fc2a64a2646970667358221220a9b7b20668fbda7df982ae4eb85b18057391ab06e4507ff202d1b1af3e3b6a2f64736f6c634300081e0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\xF2W_5`\xE0\x1C\x80c\x85\"l\x81\x11a\x01\x14W\x80c\xC0GM\x0B\x11a\0\xA9W\x80c\xE1\x15\xC6B\x11a\0yW\x80c\xE1\x15\xC6B\x14a\x03\xB6W\x80c\xE2\x0C\x9Fq\x14a\x03\xC9W\x80c\xF0\x9D\x1DD\x14a\x03\xD1W\x80c\xFAv&\xD4\x14a\x03\xD9W\x80c\xFBG\xE3\xA2\x14a\x03\xE6W__\xFD[\x80c\xC0GM\x0B\x14a\x03qW\x80c\xC0\x9C\xECw\x14a\x03\x93W\x80c\xC2\xE9\xF2\xE4\x14a\x03\xA6W\x80c\xD8\x83\xBA\x03\x14a\x03\xAEW__\xFD[\x80c\xB0FO\xDC\x11a\0\xE4W\x80c\xB0FO\xDC\x14a\x036W\x80c\xB5P\x8A\xA9\x14a\x03>W\x80c\xBAAO\xA6\x14a\x03FW\x80c\xBF\x84R\xF5\x14a\x03^W__\xFD[\x80c\x85\"l\x81\x14a\x02\xF1W\x80c\x8D\xA5\xCB[\x14a\x03\x06W\x80c\x91j\x17\xC6\x14a\x03\x19W\x80c\x9A\"\xB0T\x14a\x03.W__\xFD[\x80c<\x08\xBD_\x11a\x01\x8AW\x80cW_\xC5\xD5\x11a\x01ZW\x80cW_\xC5\xD5\x14a\x02\xAEW\x80cf\xD9\xA9\xA0\x14a\x02\xB6W\x80cn@\x05o\x14a\x02\xCBW\x80ctt,\x08\x14a\x02\xDEW__\xFD[\x80c<\x08\xBD_\x14a\x02~W\x80c>^<#\x14a\x02\x86W\x80c?r\x86\xF4\x14a\x02\x8EW\x80cO\x19\xB9A\x14a\x02\x96W__\xFD[\x80c*\xDE8\x80\x11a\x01\xC5W\x80c*\xDE8\x80\x14a\x02.W\x80c+\x90=\x9B\x14a\x02CW\x80c8\xD5.\x0F\x14a\x02KW\x80c;\x8D\xDFw\x14a\x02vW__\xFD[\x80c\x08\x1Ci\xE4\x14a\x01\xF6W\x80c\n\x92T\xE4\x14a\x02\0W\x80c\x10\\\xDA\x16\x14a\x02\x08W\x80c\x1E\xD7\x83\x1C\x14a\x02\x10W[__\xFD[a\x01\xFEa\x03\xF9V[\0[a\x01\xFEa\x05.V[a\x01\xFEa\x07\xB1V[a\x02\x18a\n\x81V[`@Qa\x02%\x91\x90a9\x87V[`@Q\x80\x91\x03\x90\xF3[a\x026a\n\xE1V[`@Qa\x02%\x91\x90a:\0V[a\x01\xFEa\x0C\x1DV[`!Ta\x02^\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02%V[a\x01\xFEa\x12\x86V[a\x01\xFEa\x14\xC9V[a\x02\x18a\x16\xA5V[a\x02\x18a\x17\x03V[`\x1FTa\x02^\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xFEa\x17aV[a\x02\xBEa\x1B_V[`@Qa\x02%\x91\x90a;\rV[a\x01\xFEa\x02\xD96`\x04a;\x8BV[a\x1C\xC3V[` Ta\x02^\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xF9a\x1F\x8FV[`@Qa\x02%\x91\x90a;\xABV[`\"Ta\x02^\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03!a ZV[`@Qa\x02%\x91\x90a<\x02V[a\x01\xFEa!;V[a\x03!a%nV[a\x02\xF9a&OV[a\x03Na'\x1AV[`@Q\x90\x15\x15\x81R` \x01a\x02%V[a\x01\xFEa\x03l6`\x04a<yV[a'\xD4V[a\x03\x85_Q` a^\xA6_9_Q\x90_R\x81V[`@Q\x90\x81R` \x01a\x02%V[`$Ta\x02^\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xFEa)\xF0V[a\x01\xFEa.\xBBV[a\x01\xFEa\x03\xC46`\x04a<\xB6V[a0%V[a\x02\x18a3\x08V[a\x01\xFEa3fV[`\x1FTa\x03N\x90`\xFF\x16\x81V[`#Ta\x02^\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`#T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x04IW__\xFD[PZ\xF1\x15\x80\x15a\x04[W=__>=_\xFD[PPPP_Q` a^\x86_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\xF4\x84H\x14`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x04\xA4W__\xFD[PZ\xF1\x15\x80\x15a\x04\xB6W=__>=_\xFD[PP`\x1FT`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc@\xC1\x0F\x19\x93Pa\x04\xFF\x92\x91\x16\x90h\x05k\xC7^-c\x10\0\0\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\x16W__\xFD[PZ\xF1\x15\x80\x15a\x05(W=__>=_\xFD[PPPPV[`\x12`@Qa\x05<\x90a9mV[``\x80\x82R`\n\x90\x82\x01Ri\x15\x19\\\xDD\x08\x10\\\xDC\xD9]`\xB2\x1B`\x80\x82\x01R`\xA0` \x82\x01\x81\x90R`\x04\x90\x82\x01Rc\x15\x11T\xD5`\xE2\x1B`\xC0\x82\x01R`\xFF\x90\x91\x16`@\x82\x01R`\xE0\x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x05\x9CW=__>=_\xFD[P`!\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U`\"T`@Qc\x03\">\xAB`\xE1\x1B\x81R\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\xFFW__\xFD[PZ\xF1\x15\x80\x15a\x06\x11W=__>=_\xFD[PP`!T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92P_Q` a^\xA6_9_Q\x90_R\x91Pa\x06?\x90a9zV[`\x80\x80\x82R`\x03\x90\x82\x01\x81\x90RbYes`\xE8\x1B`\xA0\x83\x01R`\xC0` \x83\x01\x81\x90R\x82\x01RbYES`\xE8\x1B`\xE0\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`@\x83\x01R``\x82\x01Ra\x01\0\x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x06\xA2W=__>=_\xFD[P`\x1F\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x16a\x01\0`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x02\x17\x90U`!T`@Q\x91\x16\x90_Q` a^\xA6_9_Q\x90_R\x90a\x06\xE5\x90a9zV[`\x80\x80\x82R`\x02\x90\x82\x01\x81\x90RaNo`\xF0\x1B`\xA0\x83\x01R`\xC0` \x83\x01\x81\x90R\x82\x01RaNO`\xF0\x1B`\xE0\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`@\x83\x01R``\x82\x01Ra\x01\0\x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x07FW=__>=_\xFD[P` _a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP_Q` a^\x86_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\x16W__\xFD[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rh\x05k\xC7^-c\x10\0\0\x90h\x01\xA0Ui\r\x9D\xB8\0\0\x90_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\x16W__\xFD[PZ\xF1\x15\x80\x15a\x08(W=__>=_\xFD[PP`\x1FT`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc@\xC1\x0F\x19\x93Pa\x08h\x92\x91\x16\x90\x86\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\x7FW__\xFD[PZ\xF1\x15\x80\x15a\x08\x91W=__>=_\xFD[PP`#T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x92Pc\xCAf\x9F\xA7\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08\xE4W__\xFD[PZ\xF1\x15\x80\x15a\x08\xF6W=__>=_\xFD[PP`\x1FT`$T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc\xA9\x05\x9C\xBB\x93Pa\t6\x92\x91\x16\x90\x85\x90`\x04\x01a<\xE8V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\tRW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tv\x91\x90a=\x01V[P`\x1FT`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\t\xFF\x92a\x01\0\x90\x04\x90\x91\x16\x90cp\xA0\x821\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xCCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xF0\x91\x90a= V[a\t\xFA\x83\x85a=KV[a4\xE9V[`\x1FT`$\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\n}\x93a\x01\0\x90\x04\x90\x91\x16\x91cp\xA0\x821\x91\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nSW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nw\x91\x90a= V[\x82a4\xE9V[PPV[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\n\xD7W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\n\xB9W[PPPPP\x90P\x90V[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\x14W_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x0B\xFDW\x83\x82\x90_R` _ \x01\x80Ta\x0Br\x90a=dV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\x9E\x90a=dV[\x80\x15a\x0B\xE9W\x80`\x1F\x10a\x0B\xC0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xE9V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xCCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0BUV[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0B\x04V[PPPP\x90P\x90V[`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R_\x91` \x82\x01``\x806\x837\x01\x90PP\x90Ph\x02\xB5\xE3\xAF\x16\xB1\x88\0\0\x81_\x81Q\x81\x10a\x0C[Wa\x0C[a=\xB0V[` \x02` \x01\x01\x81\x81RPPh\x01\xA0Ui\r\x9D\xB8\0\0\x81`\x01\x81Q\x81\x10a\x0C\x84Wa\x0C\x84a=\xB0V[` \x02` \x01\x01\x81\x81RPPh\x01\x15\x8EF\t\x13\xD0\0\0\x81`\x02\x81Q\x81\x10a\x0C\xADWa\x0C\xADa=\xB0V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R_\x91\x81` \x01` \x82\x02\x806\x837PP`#T\x82Q\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91\x83\x91P_\x90a\x0C\xFBWa\x0C\xFBa=\xB0V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`$T\x82Q\x91\x16\x90\x82\x90`\x01\x90\x81\x10a\r,Wa\r,a=\xB0V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPa\rt`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01fcharlie`\xC8\x1B\x81RPa5MV[\x81`\x02\x81Q\x81\x10a\r\x87Wa\r\x87a=\xB0V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`\"T`@Qc\x03\">\xAB`\xE1\x1B\x81R\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\r\xE3W__\xFD[PZ\xF1\x15\x80\x15a\r\xF5W=__>=_\xFD[P_\x92PPP[\x81Q\x81\x10\x15a\x0E\xB3W`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x19\x83\x83\x81Q\x81\x10a\x0E;Wa\x0E;a=\xB0V[` \x02` \x01\x01Q\x85\x84\x81Q\x81\x10a\x0EUWa\x0EUa=\xB0V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Ez\x92\x91\x90a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0E\x91W__\xFD[PZ\xF1\x15\x80\x15a\x0E\xA3W=__>=_\xFD[PP`\x01\x90\x92\x01\x91Pa\r\xFC\x90PV[P_\x80[\x82Q\x81\x10\x15a\x0F\xB9Wa\x0F\x8A`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x821\x85\x84\x81Q\x81\x10a\x0E\xF9Wa\x0E\xF9a=\xB0V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F,\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FGW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Fk\x91\x90a= V[\x85\x83\x81Q\x81\x10a\x0F}Wa\x0F}a=\xB0V[` \x02` \x01\x01Qa4\xE9V[\x83\x81\x81Q\x81\x10a\x0F\x9CWa\x0F\x9Ca=\xB0V[` \x02` \x01\x01Q\x82a\x0F\xAF\x91\x90a=\xC4V[\x91P`\x01\x01a\x0E\xB7V[Pa\x10\x0E`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nSW=__>=_\xFD[g\x8A\xC7#\x04\x89\xE8\0\0_[\x83Q\x81\x10\x15a\x10\xB7W`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x9D\xC2\x9F\xAC\x85\x83\x81Q\x81\x10a\x10XWa\x10Xa=\xB0V[` \x02` \x01\x01Q\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10~\x92\x91\x90a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10\x95W__\xFD[PZ\xF1\x15\x80\x15a\x10\xA7W=__>=_\xFD[PP`\x01\x90\x92\x01\x91Pa\x10\x19\x90PV[P_Q` a^\x86_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10\xFDW__\xFD[PZ\xF1\x15\x80\x15a\x11\x0FW=__>=_\xFD[P_\x92PPP[\x83Q\x81\x10\x15a\x11\xF7Wa\x11\xEF`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cp\xA0\x821\x86\x84\x81Q\x81\x10a\x11XWa\x11Xa=\xB0V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\x8B\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xA6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xCA\x91\x90a= V[\x83\x87\x84\x81Q\x81\x10a\x11\xDDWa\x11\xDDa=\xB0V[` \x02` \x01\x01Qa\t\xFA\x91\x90a=KV[`\x01\x01a\x11\x16V[Pa\x05(`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12LW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12p\x91\x90a= V[\x84Qa\x12|\x90\x84a=\xD7V[a\t\xFA\x90\x85a=KV[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rh\x05k\xC7^-c\x10\0\0\x90h\x02+\x1C\x8C\x12'\xA0\0\0\x90_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x12\xEBW__\xFD[PZ\xF1\x15\x80\x15a\x12\xFDW=__>=_\xFD[PP`\x1FT`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc@\xC1\x0F\x19\x93Pa\x13=\x92\x91\x16\x90\x86\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x13TW__\xFD[PZ\xF1\x15\x80\x15a\x13fW=__>=_\xFD[PP`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x92Pc\xCAf\x9F\xA7\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x13\xB9W__\xFD[PZ\xF1\x15\x80\x15a\x13\xCBW=__>=_\xFD[PP`\x1FT`#T`@Qc'p\xA7\xEB`\xE2\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc\x9D\xC2\x9F\xAC\x93Pa\x14\x0B\x92\x91\x16\x90\x85\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x14\"W__\xFD[PZ\xF1\x15\x80\x15a\x144W=__>=_\xFD[PP`\x1FT`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\x14u\x94Pa\x01\0\x90\x92\x04\x16\x91Pcp\xA0\x821\x90`$\x01a\t\xB1V[a\n}`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xCCW=__>=_\xFD[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x15\x18W__\xFD[PZ\xF1\x15\x80\x15a\x15*W=__>=_\xFD[PP`\x1FT`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc@\xC1\x0F\x19\x93Pa\x15s\x92\x91\x16\x90h\x05k\xC7^-c\x10\0\0\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x15\x8AW__\xFD[PZ\xF1\x15\x80\x15a\x15\x9CW=__>=_\xFD[PP`#T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x92Pc\xCAf\x9F\xA7\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x15\xEFW__\xFD[PZ\xF1\x15\x80\x15a\x16\x01W=__>=_\xFD[PPPP_Q` a^\x86_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\xF4\x84H\x14`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x16JW__\xFD[PZ\xF1\x15\x80\x15a\x16\\W=__>=_\xFD[PP`\x1FT`#T`@Qc'p\xA7\xEB`\xE2\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc\x9D\xC2\x9F\xAC\x93Pa\x04\xFF\x92\x91\x16\x90h\x02\xB5\xE3\xAF\x16\xB1\x88\0\0\x90`\x04\x01a<\xE8V[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\n\xD7W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\n\xB9WPPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\n\xD7W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\n\xB9WPPPPP\x90P\x90V[_a\x17\x8B`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g72\xBB\xA7\xBB\xB72\xB9`\xC1\x1B\x81RPa5MV[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x91P_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x17\xDDW__\xFD[PZ\xF1\x15\x80\x15a\x17\xEFW=__>=_\xFD[PP`\x1FT`@Qc\xF2\xFD\xE3\x8B`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01Ra\x01\0\x90\x92\x04\x90\x91\x16\x92Pc\xF2\xFD\xE3\x8B\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x18=W__\xFD[PZ\xF1\x15\x80\x15a\x18OW=__>=_\xFD[PPPPa\x18\xD1`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xA7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xCB\x91\x90a=\xEEV[\x82a5^V[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x19 W__\xFD[PZ\xF1\x15\x80\x15a\x192W=__>=_\xFD[PPPP_Q` a^\x86_9_Q\x90_R`\x01`\x01`\xA0\x1B\x03\x16c\xF4\x84H\x14`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x19{W__\xFD[PZ\xF1\x15\x80\x15a\x19\x8DW=__>=_\xFD[PP`\x1FT`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc@\xC1\x0F\x19\x93Pa\x19\xD6\x92\x91\x16\x90h\x05k\xC7^-c\x10\0\0\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x19\xEDW__\xFD[PZ\xF1\x15\x80\x15a\x19\xFFW=__>=_\xFD[PP`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x92Pc\xCAf\x9F\xA7\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1ANW__\xFD[PZ\xF1\x15\x80\x15a\x1A`W=__>=_\xFD[PP`\x1FT`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc@\xC1\x0F\x19\x93Pa\x1A\xA9\x92\x91\x16\x90h\x05k\xC7^-c\x10\0\0\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1A\xC0W__\xFD[PZ\xF1\x15\x80\x15a\x1A\xD2W=__>=_\xFD[PP`\x1FT`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\x1B\\\x94Pa\x01\0\x90\x92\x04\x16\x91Pcp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B)W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1BM\x91\x90a= V[h\x05k\xC7^-c\x10\0\0a4\xE9V[PV[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\x14W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x1B\xB2\x90a=dV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1B\xDE\x90a=dV[\x80\x15a\x1C)W\x80`\x1F\x10a\x1C\0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1C)V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1C\x0CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1C\xABW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x1CmW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1B\x82V[a\x1C\xD6\x82`\x01`\x01`\x01`\x80\x1B\x03a5\x9FV[\x91Pa\x1C\xE3\x81_\x84a5\x9FV[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x91P_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1D5W__\xFD[PZ\xF1\x15\x80\x15a\x1DGW=__>=_\xFD[PP`\x1FT`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc@\xC1\x0F\x19\x93Pa\x1D\x87\x92\x91\x16\x90\x86\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1D\x9EW__\xFD[PZ\xF1\x15\x80\x15a\x1D\xB0W=__>=_\xFD[PP`#T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x92Pc\xCAf\x9F\xA7\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1E\x03W__\xFD[PZ\xF1\x15\x80\x15a\x1E\x15W=__>=_\xFD[PP`\x1FT`$T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc\xA9\x05\x9C\xBB\x93Pa\x1EU\x92\x91\x16\x90\x85\x90`\x04\x01a<\xE8V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1EqW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x95\x91\x90a=\x01V[P`\x1FT`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\x1E\xD4\x92a\x01\0\x90\x04\x90\x91\x16\x90cp\xA0\x821\x90`$\x01a\t\xB1V[`\x1FT`$\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra\x1F\x11\x93a\x01\0\x90\x04\x90\x91\x16\x91cp\xA0\x821\x91\x01a\n8V[a\n}`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1FeW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\x89\x91\x90a= V[\x83a4\xE9V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\x14W\x83\x82\x90_R` _ \x01\x80Ta\x1F\xCF\x90a=dV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1F\xFB\x90a=dV[\x80\x15a FW\x80`\x1F\x10a \x1DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a )W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1F\xB2V[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\x14W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a!#W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a \xE5W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a }V[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rh\x05k\xC7^-c\x10\0\0\x90h\x02\xB5\xE3\xAF\x16\xB1\x88\0\0\x90h\x01\xA0Ui\r\x9D\xB8\0\0\x90_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a!\xABW__\xFD[PZ\xF1\x15\x80\x15a!\xBDW=__>=_\xFD[PP`\x1FT`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc@\xC1\x0F\x19\x93Pa!\xFD\x92\x91\x16\x90\x87\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\"\x14W__\xFD[PZ\xF1\x15\x80\x15a\"&W=__>=_\xFD[PP`#T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x92Pc\xCAf\x9F\xA7\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\"yW__\xFD[PZ\xF1\x15\x80\x15a\"\x8BW=__>=_\xFD[PP`\x1FT`$T`@Qc\t^\xA7\xB3`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc\t^\xA7\xB3\x93Pa\"\xCB\x92\x91\x16\x90\x86\x90`\x04\x01a<\xE8V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\"\xE7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x0B\x91\x90a=\x01V[P`\x1FT`#T`$\x80T`@Qcn\xB1v\x9F`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x90\x83\x16\x91\x81\x01\x91\x90\x91Ra#o\x92a\x01\0\x90\x04\x90\x91\x16\x90c\xDDb\xED>\x90`D\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1FeW=__>=_\xFD[`$\x80T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x91c\xCAf\x9F\xA7\x91\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a#\xBEW__\xFD[PZ\xF1\x15\x80\x15a#\xD0W=__>=_\xFD[PP`\x1FT`#T`$\x80T`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x90\x83\x16\x91\x81\x01\x91\x90\x91R`D\x81\x01\x86\x90Ra\x01\0\x90\x92\x04\x16\x92Pc#\xB8r\xDD\x91P`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a$9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$]\x91\x90a=\x01V[P`\x1FT`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra$\xE0\x92a\x01\0\x90\x04\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\xB2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xD6\x91\x90a= V[a\t\xFA\x83\x86a=KV[`\x1FT`$\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra%\x1D\x93a\x01\0\x90\x04\x90\x91\x16\x91cp\xA0\x821\x91\x01a\n8V[`\x1FT`#T`$\x80T`@Qcn\xB1v\x9F`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x90\x83\x16\x91\x81\x01\x91\x90\x91Ra%i\x92a\x01\0\x90\x04\x90\x91\x16\x90c\xDDb\xED>\x90`D\x01a\t\xB1V[PPPV[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\x14W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a&7W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a%\xF9W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a%\x91V[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\x14W\x83\x82\x90_R` _ \x01\x80Ta&\x8F\x90a=dV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta&\xBB\x90a=dV[\x80\x15a'\x06W\x80`\x1F\x10a&\xDDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a'\x06V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a&\xE9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a&rV[`\x08T_\x90`\xFF\x16\x15a'1WP`\x08T`\xFF\x16\x90V[`@Qc\x06g\xF9\xD7`\xE4\x1B\x81R_\x90_Q` a^\x86_9_Q\x90_R\x90cf\x7F\x9Dp\x90a'\x8E\x90\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90e\x19\x98Z[\x19Y`\xD2\x1B\x90`\x04\x01a<\xE8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\xA9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xCD\x91\x90a= V[\x14\x15\x90P\x90V[a'\xE7\x83`\x01`\x01`\x01`\x80\x1B\x03a5\x9FV[\x92Pa'\xF4\x82_\x85a5\x9FV[\x91Pa(\x01\x81_\x84a5\x9FV[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x91P_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a(SW__\xFD[PZ\xF1\x15\x80\x15a(eW=__>=_\xFD[PP`\x1FT`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc@\xC1\x0F\x19\x93Pa(\xA5\x92\x91\x16\x90\x87\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a(\xBCW__\xFD[PZ\xF1\x15\x80\x15a(\xCEW=__>=_\xFD[PP`#T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x92Pc\xCAf\x9F\xA7\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a)!W__\xFD[PZ\xF1\x15\x80\x15a)3W=__>=_\xFD[PP`\x1FT`$T`@Qc\t^\xA7\xB3`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc\t^\xA7\xB3\x93Pa)s\x92\x91\x16\x90\x86\x90`\x04\x01a<\xE8V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a)\x8FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xB3\x91\x90a=\x01V[P`$\x80T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x91c\xCAf\x9F\xA7\x91\x01a#\xA7V[a*\x8B`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x06\xFD\xDE\x03`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*CW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra*j\x91\x90\x81\x01\x90a>\tV[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bYes`\xE8\x1B\x81RPa5\xE2V[a+&`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\xDEW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra+\x05\x91\x90\x81\x01\x90a>\tV[`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01bYES`\xE8\x1B\x81RPa5\xE2V[a+\xAF`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cH\0\xD9\x7F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+zW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\x9E\x91\x90a=\xEEV[`!T`\x01`\x01`\xA0\x1B\x03\x16a5^V[a,:`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xC0GM\x0B`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\x03W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,'\x91\x90a= V[_Q` a^\xA6_9_Q\x90_Ra6\x14V[a,\xC3`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\x8EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xB2\x91\x90a=\xEEV[`\"T`\x01`\x01`\xA0\x1B\x03\x16a5^V[` T`@\x80Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x90Qa-R\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x06\xFD\xDE\x03\x91`\x04\x80\x83\x01\x92_\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a-\x0BW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra-2\x91\x90\x81\x01\x90a>\tV[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aNo`\xF0\x1B\x81RPa5\xE2V[` T`@\x80Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x90Qa-\xE1\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x95\xD8\x9BA\x91`\x04\x80\x83\x01\x92_\x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a-\x9AW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra-\xC1\x91\x90\x81\x01\x90a>\tV[`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01aNO`\xF0\x1B\x81RPa5\xE2V[` \x80T`@\x80QcH\0\xD9\x7F`\xE0\x1B\x81R\x90Qa.)\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92cH\0\xD9\x7F\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a+zW=__>=_\xFD[` \x80T`@\x80Qc\xC0GM\x0B`\xE0\x1B\x81R\x90Qa.q\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\xC0GM\x0B\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a,\x03W=__>=_\xFD[` \x80T`@\x80Qc\x8D\xA5\xCB[`\xE0\x1B\x81R\x90Qa.\xB9\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92c\x8D\xA5\xCB[\x92`\x04\x80\x82\x01\x93\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a,\x8EW=__>=_\xFD[V[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01Rh\x05k\xC7^-c\x10\0\0\x90_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/\x15W__\xFD[PZ\xF1\x15\x80\x15a/'W=__>=_\xFD[PP`\x1FT`#T`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x92\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94Pc@\xC1\x0F\x19\x93Pa/g\x92\x91\x16\x90\x85\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/~W__\xFD[PZ\xF1\x15\x80\x15a/\x90W=__>=_\xFD[PP`\x1FT`#T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01Ra/\xD1\x94Pa\x01\0\x90\x92\x04\x16\x91Pcp\xA0\x821\x90`$\x01a\n8V[a\x1B\\`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nSW=__>=_\xFD[`@Qc&1\xF2\xB1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x15\x15`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x90cLc\xE5b\x90`$\x01_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a0qW__\xFD[PZ\xFA\x15\x80\x15a0\x83W=__>=_\xFD[PPPPa0\x9C\x82`\x01`\x01`\x01`\x80\x1B\x03\x80\x16a5\x9FV[\x91Pa0\xA9\x81_\x84a5\x9FV[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x91P_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a0\xFBW__\xFD[PZ\xF1\x15\x80\x15a1\rW=__>=_\xFD[PP`\x1FT`@Qc@\xC1\x0F\x19`\xE0\x1B\x81Ra\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x92Pc@\xC1\x0F\x19\x91Pa1G\x90\x86\x90\x86\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a1^W__\xFD[PZ\xF1\x15\x80\x15a1pW=__>=_\xFD[PP`\x1FT`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01Ra1\xB0\x94Pa\x01\0\x90\x92\x04\x90\x91\x16\x91Pcp\xA0\x821\x90`$\x01a#TV[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a1\xFFW__\xFD[PZ\xF1\x15\x80\x15a2\x11W=__>=_\xFD[PP`\x1FT`@Qc'p\xA7\xEB`\xE2\x1B\x81Ra\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x92Pc\x9D\xC2\x9F\xAC\x91Pa2K\x90\x86\x90\x85\x90`\x04\x01a<\xE8V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a2bW__\xFD[PZ\xF1\x15\x80\x15a2tW=__>=_\xFD[PP`\x1FT`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01Ra2\xB4\x94Pa\x01\0\x90\x92\x04\x90\x91\x16\x91Pcp\xA0\x821\x90`$\x01a\t\xB1V[a%i`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xCCW=__>=_\xFD[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\n\xD7W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\n\xB9WPPPPP\x90P\x90V[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a3\xB5W__\xFD[PZ\xF1\x15\x80\x15a3\xC7W=__>=_\xFD[PPPP`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cqP\x18\xA6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a4\x18W__\xFD[PZ\xF1\x15\x80\x15a4*W=__>=_\xFD[PPPPa4\xAC`\x1F`\x01\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4\x82W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\xA6\x91\x90a=\xEEV[_a5^V[`\"T`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R_Q` a^\x86_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01a\x042V[`@Qc&\n[\x15`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R_Q` a^\x86_9_Q\x90_R\x90c\x98)lT\x90`D\x01[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a53W__\xFD[PZ\xFA\x15\x80\x15a5EW=__>=_\xFD[PPPPPPV[_a5W\x82a6LV[P\x92\x91PPV[`@Qc(\xA9\xB0\xFB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R\x82\x16`$\x82\x01R_Q` a^\x86_9_Q\x90_R\x90cQSa\xF6\x90`D\x01a5\x1DV[_a5\xAB\x84\x84\x84a7LV[\x90Pa5\xDB`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x10\x9B\xDD[\x99\x08\x1C\x99\\\xDD[\x1D`\xA2\x1B\x81RP\x82a9\tV[\x93\x92PPPV[`@Qc\xF3 \xD9c`\xE0\x1B\x81R_Q` a^\x86_9_Q\x90_R\x90c\xF3 \xD9c\x90a5\x1D\x90\x85\x90\x85\x90`\x04\x01a>\xBCV[`@Qc|\x84\xC6\x9B`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R_Q` a^\x86_9_Q\x90_R\x90c|\x84\xC6\x9B\x90`D\x01a5\x1DV[__\x82`@Q` \x01a6_\x91\x90a>\xE9V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 `\x01b^y\xB7`\xE0\x1B\x03\x19\x82R`\x04\x82\x01\x81\x90R\x91P_Q` a^\x86_9_Q\x90_R\x90c\xFF\xA1\x86I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\xC1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\xE5\x91\x90a=\xEEV[`@Qc\x18\xCA\xF8\xE3`\xE3\x1B\x81R\x90\x92P_Q` a^\x86_9_Q\x90_R\x90c\xC6W\xC7\x18\x90a7\x1A\x90\x85\x90\x87\x90`\x04\x01a>\xFFV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a71W__\xFD[PZ\xF1\x15\x80\x15a7CW=__>=_\xFD[PPPP\x91P\x91V[_\x81\x83\x11\x15a7\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FStdUtils bound(uint256,uint256,u`D\x82\x01R\x7Fint256): Max is less than min.\0\0`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x10\x15\x80\x15a7\xD7WP\x81\x84\x11\x15[\x15a7\xE3WP\x82a5\xDBV[_a7\xEE\x84\x84a=KV[a7\xF9\x90`\x01a=\xC4V[\x90P`\x03\x85\x11\x15\x80\x15a8\x0BWP\x84\x81\x11[\x15a8\"Wa8\x1A\x85\x85a=\xC4V[\x91PPa5\xDBV[a8.`\x03_\x19a=KV[\x85\x10\x15\x80\x15a8FWPa8C\x85_\x19a=KV[\x81\x11[\x15a8`Wa8V\x85_\x19a=KV[a8\x1A\x90\x84a=KV[\x82\x85\x11\x15a8\xB3W_a8s\x84\x87a=KV[\x90P_a8\x80\x83\x83a?*V[\x90P\x80_\x03a8\x94W\x84\x93PPPPa5\xDBV[`\x01a8\xA0\x82\x88a=\xC4V[a8\xAA\x91\x90a=KV[\x93PPPa9\x01V[\x83\x85\x10\x15a9\x01W_a8\xC6\x86\x86a=KV[\x90P_a8\xD3\x83\x83a?*V[\x90P\x80_\x03a8\xE7W\x85\x93PPPPa5\xDBV[a8\xF1\x81\x86a=KV[a8\xFC\x90`\x01a=\xC4V[\x93PPP[P\x93\x92PPPV[a\n}\x82\x82`@Q`$\x01a9\x1F\x92\x91\x90a?IV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\x1B\\\x81\x80Qjconsole.log` \x83\x01_\x80\x84\x83\x85Z\xFAPPPPPV[a\x0E\xD4\x80a?k\x839\x01\x90V[a\x10G\x80aN?\x839\x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a9\xC7W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a9\xA0V[P\x90\x95\x94PPPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a:\xBDW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90```\x05\x82\x90\x1B\x88\x01\x81\x01\x91\x90\x88\x01\x90_[\x81\x81\x10\x15a:\xA3W`_\x19\x8A\x85\x03\x01\x83Ra:\x8D\x84\x86Qa9\xD2V[` \x95\x86\x01\x95\x90\x94P\x92\x90\x92\x01\x91`\x01\x01a:qV[P\x91\x97PPP` \x94\x85\x01\x94\x92\x90\x92\x01\x91P`\x01\x01a:&V[P\x92\x96\x95PPPPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a;\x03W\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a:\xDBV[P\x93\x94\x93PPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a:\xBDW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`@\x87Ra;Y`@\x88\x01\x82a9\xD2V[\x90P` \x82\x01Q\x91P\x86\x81\x03` \x88\x01Ra;t\x81\x83a:\xC9V[\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a;3V[__`@\x83\x85\x03\x12\x15a;\x9CW__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a:\xBDW`?\x19\x87\x86\x03\x01\x84Ra;\xED\x85\x83Qa9\xD2V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a;\xD1V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a:\xBDW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90a<c\x90\x87\x01\x82a:\xC9V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a<(V[___``\x84\x86\x03\x12\x15a<\x8BW__\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1B\\W__\xFD[___``\x84\x86\x03\x12\x15a<\xC8W__\xFD[\x835a<\xD3\x81a<\xA2V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[_` \x82\x84\x03\x12\x15a=\x11W__\xFD[\x81Q\x80\x15\x15\x81\x14a5\xDBW__\xFD[_` \x82\x84\x03\x12\x15a=0W__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a=^Wa=^a=7V[\x92\x91PPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a=xW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a=\x96WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a=^Wa=^a=7V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a=^Wa=^a=7V[_` \x82\x84\x03\x12\x15a=\xFEW__\xFD[\x81Qa5\xDB\x81a<\xA2V[_` \x82\x84\x03\x12\x15a>\x19W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>/W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a>?W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>YWa>Ya=\x9CV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a>\x88Wa>\x88a=\x9CV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a>\x9FW__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`@\x81R_a>\xCE`@\x83\x01\x85a9\xD2V[\x82\x81\x03` \x84\x01Ra>\xE0\x81\x85a9\xD2V[\x95\x94PPPPPV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a?\"\x90\x83\x01\x84a9\xD2V[\x94\x93PPPPV[_\x82a?DWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x06\x90V[`@\x81R_a?[`@\x83\x01\x85a9\xD2V[\x90P\x82` \x83\x01R\x93\x92PPPV\xFE`\xA0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x0E\xD48\x03\x80a\x0E\xD4\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01\tV[_a\09\x84\x82a\x02\nV[P`\x01a\0F\x83\x82a\x02\nV[P`\x02\x80T`\xFF\x19\x16`\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UP\x80Q` \x90\x91\x01 `\x80Ra\x02\xC4V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a\0\x8FW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\0\xA8Wa\0\xA8a\0lV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\0\xD6Wa\0\xD6a\0lV[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\0\xEDW__\xFD[\x81` \x85\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[___``\x84\x86\x03\x12\x15a\x01\x1BW__\xFD[\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x010W__\xFD[a\x01<\x86\x82\x87\x01a\0\x80V[` \x86\x01Q\x90\x94P\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01YW__\xFD[a\x01e\x86\x82\x87\x01a\0\x80V[\x92PP`@\x84\x01Q`\xFF\x81\x16\x81\x14a\x01{W__\xFD[\x80\x91PP\x92P\x92P\x92V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01\x9AW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x01\xB8WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x02\x05W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x01\xE3WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x02\x02W_\x81U`\x01\x01a\x01\xEFV[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02#Wa\x02#a\0lV[a\x027\x81a\x021\x84Ta\x01\x86V[\x84a\x01\xBEV[` `\x1F\x82\x11`\x01\x81\x14a\x02iW_\x83\x15a\x02RWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x02\x02V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x02\x98W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x02xV[P\x84\x82\x10\x15a\x02\xB5W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80Qa\x0B\xF1a\x02\xE3_9_\x81\x81a\x03\x99\x01Ra\x04\xDB\x01Ra\x0B\xF1_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xFBW_5`\xE0\x1C\x80c~\xCE\xBE\0\x11a\0\x93W\x80c\xD3\x0E\xD3\xB3\x11a\0cW\x80c\xD3\x0E\xD3\xB3\x14a\x02\x17W\x80c\xD5\x05\xAC\xCF\x14a\x02*W\x80c\xDDb\xED>\x14a\x02=W\x80c\xF8=\x17\x91\x14a\x02PW__\xFD[\x80c~\xCE\xBE\0\x14a\x01\xC4W\x80c\x95\xD8\x9BA\x14a\x01\xE9W\x80c\x9D\xC2\x9F\xAC\x14a\x01\xF1W\x80c\xA9\x05\x9C\xBB\x14a\x02\x04W__\xFD[\x80c1<\xE5g\x11a\0\xCEW\x80c1<\xE5g\x14a\x01mW\x80c6D\xE5\x15\x14a\x01\x82W\x80c@\xC1\x0F\x19\x14a\x01\x8AW\x80cp\xA0\x821\x14a\x01\x9FW__\xFD[\x80c\x06\xFD\xDE\x03\x14a\0\xFFW\x80c\t^\xA7\xB3\x14a\x01\x1DW\x80c\x18\x16\r\xDD\x14a\x01@W\x80c#\xB8r\xDD\x14a\x01ZW[__\xFD[a\x01\x07a\x02cV[`@Qa\x01\x14\x91\x90a\t\xFFV[`@Q\x80\x91\x03\x90\xF3[a\x010a\x01+6`\x04a\nJV[a\x02\xF2V[`@Q\x90\x15\x15\x81R` \x01a\x01\x14V[h\x054\\\xDFw\xEBh\xF4LT[`@Q\x90\x81R` \x01a\x01\x14V[a\x010a\x01h6`\x04a\nrV[a\x03rV[`\x02T`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\x14V[a\x01La\x03\x96V[a\x01\x9Da\x01\x986`\x04a\nJV[a\x048V[\0[a\x01La\x01\xAD6`\x04a\n\xACV[c\x87\xA2\x11\xA2`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[a\x01La\x01\xD26`\x04a\n\xACV[c87u\x08`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[a\x01\x07a\x04NV[a\x01\x9Da\x01\xFF6`\x04a\nJV[a\x04]V[a\x010a\x02\x126`\x04a\nJV[a\x04oV[a\x01\x9Da\x02%6`\x04a\nrV[a\x04\x89V[a\x01\x9Da\x0286`\x04a\n\xC5V[a\x04\xA9V[a\x01La\x02K6`\x04a\x0B2V[a\x06\x83V[a\x01\x9Da\x02^6`\x04a\nrV[a\x06\xC7V[``_\x80Ta\x02q\x90a\x0BcV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\x9D\x90a\x0BcV[\x80\x15a\x02\xE8W\x80`\x1F\x10a\x02\xBFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xE8V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xCBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[_`\x01`\x01`\xA0\x1B\x03\x83\x16n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x18\x82\x19\x15\x17a\x03#Wc?hS\x9A_R`\x04`\x1C\xFD[\x82` Rc\x7F^\x9F `\x0CR3_R\x81`4`\x0C U\x81_R`,Q``\x1C3\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` _\xA3P`\x01[\x92\x91PPV[_a\x03\x8Ea\x03\x7F\x85a\x06\xE2V[a\x03\x88\x85a\x06\xE2V[\x84a\x07\x03V[\x94\x93PPPPV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80a\x03\xCFWa\x03\xC5a\x02cV[\x80Q\x90` \x01 \x90P[`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R` \x81\x01\x92\x90\x92R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6\x90\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 \x91\x90PV[a\x04Ja\x04D\x83a\x06\xE2V[\x82a\x07\xBFV[PPV[```\x01\x80Ta\x02q\x90a\x0BcV[a\x04Ja\x04i\x83a\x06\xE2V[\x82a\x08(V[_a\x04\x82a\x04|\x84a\x06\xE2V[\x83a\x08\x89V[\x93\x92PPPV[a\x04\xA4a\x04\x95\x84a\x06\xE2V[a\x04\x9E\x84a\x06\xE2V[\x83a\x08\xEDV[PPPV[`\x01`\x01`\xA0\x1B\x03\x86\x16n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x18\x85\x19\x15\x17a\x04\xD9Wc?hS\x9A_R`\x04`\x1C\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80a\x05\x11Wa\x05\x07a\x02cV[\x80Q\x90` \x01 \x90P[\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6B\x86\x10\x15a\x05GWc\x1A\x15\xA3\xCC_R`\x04`\x1C\xFD[`@Q\x89``\x1B``\x1C\x99P\x88``\x1B``\x1C\x98Pe87u\x08\x19\x01`\x0ER\x89_R` `\x0C \x80T\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x83R\x84` \x84\x01R\x83`@\x84\x01RF``\x84\x01R0`\x80\x84\x01R`\xA0\x83 `.R\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x83R\x8B` \x84\x01R\x8A`@\x84\x01R\x89``\x84\x01R\x80`\x80\x84\x01R\x88`\xA0\x84\x01R`\xC0\x83 `NR`B`, _R\x87`\xFF\x16` R\x86`@R\x85``R` \x80`\x80_`\x01Z\xFA\x8C=Q\x14a\x06/Wc\xDD\xAF\xBA\xEF_R`\x04`\x1C\xFD[\x01\x90Uc\x03\xFA\xF4\xF9`\xA5\x1B\x89\x17`@R`4`, \x88\x90U\x88\x8A\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` ``\x85\x01\xA3`@RPP_``RPPPPPPPV[_n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA2\x19`\x01`\x01`\xA0\x1B\x03\x83\x16\x01a\x06\xACWP_\x19a\x03lV[P` Rc\x7F^\x9F `\x0C\x90\x81R_\x91\x90\x91R`4\x90 T\x90V[a\x04\xA4a\x06\xD3\x84a\x06\xE2V[a\x06\xDC\x84a\x06\xE2V[\x83a\tQV[_`\x01`\x01`\xA0\x1B\x03\x82\x16\x80`\xA0a\x06\xF9\x82a\t\xB7V[\x90\x1B\x18\x93\x92PPPV[_\x83``\x1Bn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA33\x14a\x07XW3` Rc\x7F^\x9F \x81\x17`\x0CR`4`\x0C \x80T\x80\x19\x15a\x07UW\x80\x85\x11\x15a\x07OWc\x13\xBE%+_R`\x04`\x1C\xFD[\x84\x81\x03\x82U[PP[c\x87\xA2\x11\xA2\x81\x17`\x0CR` `\x0C \x80T\x80\x85\x11\x15a\x07~Wc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x84\x81\x03\x82UPP\x83_R` `\x0C \x83\x81T\x01\x81UP\x82` R`\x0CQ``\x1C\x81``\x1C_Q` a\x0B\x9C_9_Q\x90_R` \x80\xA3PP`\x01\x93\x92PPPV[h\x054\\\xDFw\xEBh\xF4LT\x81\x81\x01\x81\x81\x10\x15a\x07\xE2Wc\xE5\xCF\xE9W_R`\x04`\x1C\xFD[\x80h\x054\\\xDFw\xEBh\xF4LUPPc\x87\xA2\x11\xA2`\x0CR\x81_R` `\x0C \x81\x81T\x01\x81UP\x80` R`\x0CQ``\x1C__Q` a\x0B\x9C_9_Q\x90_R` \x80\xA3PPV[c\x87\xA2\x11\xA2`\x0CR\x81_R` `\x0C \x80T\x80\x83\x11\x15a\x08OWc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x82\x90\x03\x90Uh\x054\\\xDFw\xEBh\xF4L\x80T\x82\x90\x03\x90U_\x81\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16_Q` a\x0B\x9C_9_Q\x90_R` \x83\xA3PPV[_c\x87\xA2\x11\xA2`\x0CR3_R` `\x0C \x80T\x80\x84\x11\x15a\x08\xB1Wc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x83\x81\x03\x82UPP\x82_R` `\x0C \x82\x81T\x01\x81UP\x81` R`\x0CQ``\x1C3_Q` a\x0B\x9C_9_Q\x90_R` \x80\xA3P`\x01\x92\x91PPV[n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA2\x19`\x01`\x01`\xA0\x1B\x03\x83\x16\x01a\t\x12WPPPV[\x81` Rc\x7F^\x9F `\x0CR\x82_R`4`\x0C \x80T\x80\x19\x15a\tJW\x80\x83\x11\x15a\tDWc\x13\xBE%+_R`\x04`\x1C\xFD[\x82\x81\x03\x82U[PPPPPV[\x82``\x1Bc\x87\xA2\x11\xA2\x81\x17`\x0CR` `\x0C \x80T\x80\x84\x11\x15a\t{Wc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x83\x81\x03\x82UPP\x82_R` `\x0C \x82\x81T\x01\x81UP\x81` R`\x0CQ``\x1C\x81``\x1C_Q` a\x0B\x9C_9_Q\x90_R` \x80\xA3PPPPV[`@Q6_\x8276\x81 ` R`\x10Q\x82\x18`\x10R`\x88_ \x90P\x80`\x10R`\xBC\x19p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Q\x82\t`\x80\x1C`\x07\x16a\t\xFAWP_[\x91\x90PV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\xFAW__\xFD[__`@\x83\x85\x03\x12\x15a\n[W__\xFD[a\nd\x83a\n4V[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a\n\x84W__\xFD[a\n\x8D\x84a\n4V[\x92Pa\n\x9B` \x85\x01a\n4V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_` \x82\x84\x03\x12\x15a\n\xBCW__\xFD[a\x04\x82\x82a\n4V[_______`\xE0\x88\x8A\x03\x12\x15a\n\xDBW__\xFD[a\n\xE4\x88a\n4V[\x96Pa\n\xF2` \x89\x01a\n4V[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015`\xFF\x81\x16\x81\x14a\x0B\x15W__\xFD[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x0BCW__\xFD[a\x0BL\x83a\n4V[\x91Pa\x0BZ` \x84\x01a\n4V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0BwW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0B\x95WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \x0CE\xAC\xC3\xC6\xA9\xD1X\x90\xEB\xBA\x87 \xCB c\xE2|m\xAA\xB30y8k\xB3\xDC\xAE\xCF\xA6L\xFBdsolcC\0\x08\x1E\x003`\xC0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x10G8\x03\x80a\x10G\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01CV[_a\09\x85\x82a\x02RV[P`\x01a\0F\x84\x82a\x02RV[P`\x01`\x01`\xA0\x1B\x03\x82\x16`\x80R`\xA0\x81\x90Ra\0b3a\0kV[PPPPa\x03\x0CV[`\x01`\x01`\xA0\x1B\x03\x16c\x8Bx\xC6\xD8\x19\x81\x90U\x80_\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x81\x80\xA3PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a\0\xC9W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\0\xE2Wa\0\xE2a\0\xA6V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x01\x10Wa\x01\x10a\0\xA6V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\x01'W__\xFD[\x81` \x85\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[____`\x80\x85\x87\x03\x12\x15a\x01VW__\xFD[\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01kW__\xFD[a\x01w\x87\x82\x88\x01a\0\xBAV[` \x87\x01Q\x90\x95P\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\x94W__\xFD[a\x01\xA0\x87\x82\x88\x01a\0\xBAV[`@\x87\x01Q\x90\x94P\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xBEW__\xFD[``\x95\x90\x95\x01Q\x93\x96\x92\x95PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01\xE2W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x02\0WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x02MW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x02+WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x02JW_\x81U`\x01\x01a\x027V[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02kWa\x02ka\0\xA6V[a\x02\x7F\x81a\x02y\x84Ta\x01\xCEV[\x84a\x02\x06V[` `\x1F\x82\x11`\x01\x81\x14a\x02\xB1W_\x83\x15a\x02\x9AWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x02JV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x02\xE0W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x02\xC0V[P\x84\x82\x10\x15a\x02\xFDW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\x80Q`\xA0Qa\r\x1Aa\x03-_9_a\x03n\x01R_a\x02G\x01Ra\r\x1A_\xF3\xFE`\x80`@R`\x046\x10a\x01<W_5`\xE0\x1C\x80cqP\x18\xA6\x11a\0\xB3W\x80c\xC0GM\x0B\x11a\0mW\x80c\xC0GM\x0B\x14a\x03]W\x80c\xD5\x05\xAC\xCF\x14a\x03\x90W\x80c\xDDb\xED>\x14a\x03\xAFW\x80c\xF0N(>\x14a\x03\xCEW\x80c\xF2\xFD\xE3\x8B\x14a\x03\xE1W\x80c\xFE\xE8\x1C\xF4\x14a\x03\xF4W__\xFD[\x80cqP\x18\xA6\x14a\x02\xBAW\x80c~\xCE\xBE\0\x14a\x02\xC2W\x80c\x8D\xA5\xCB[\x14a\x02\xF3W\x80c\x95\xD8\x9BA\x14a\x03\x0BW\x80c\x9D\xC2\x9F\xAC\x14a\x03\x1FW\x80c\xA9\x05\x9C\xBB\x14a\x03>W__\xFD[\x80c1<\xE5g\x11a\x01\x04W\x80c1<\xE5g\x14a\x01\xE8W\x80c6D\xE5\x15\x14a\x02\x03W\x80c@\xC1\x0F\x19\x14a\x02\x17W\x80cH\0\xD9\x7F\x14a\x026W\x80cT\xD1\xF1=\x14a\x02\x81W\x80cp\xA0\x821\x14a\x02\x89W__\xFD[\x80c\x06\xFD\xDE\x03\x14a\x01@W\x80c\t^\xA7\xB3\x14a\x01jW\x80c\x18\x16\r\xDD\x14a\x01\x99W\x80c#\xB8r\xDD\x14a\x01\xBFW\x80c%i)b\x14a\x01\xDEW[__\xFD[4\x80\x15a\x01KW__\xFD[Pa\x01Ta\x04%V[`@Qa\x01a\x91\x90a\x0B\x1CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01uW__\xFD[Pa\x01\x89a\x01\x846`\x04a\x0BlV[a\x04\xB4V[`@Q\x90\x15\x15\x81R` \x01a\x01aV[4\x80\x15a\x01\xA4W__\xFD[Ph\x054\\\xDFw\xEBh\xF4LT[`@Q\x90\x81R` \x01a\x01aV[4\x80\x15a\x01\xCAW__\xFD[Pa\x01\x89a\x01\xD96`\x04a\x0B\x94V[a\x054V[a\x01\xE6a\x05\xF0V[\0[4\x80\x15a\x01\xF3W__\xFD[P`@Q`\x12\x81R` \x01a\x01aV[4\x80\x15a\x02\x0EW__\xFD[Pa\x01\xB1a\x06=V[4\x80\x15a\x02\"W__\xFD[Pa\x01\xE6a\x0216`\x04a\x0BlV[a\x06\xB9V[4\x80\x15a\x02AW__\xFD[Pa\x02i\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01aV[a\x01\xE6a\x06\xCFV[4\x80\x15a\x02\x94W__\xFD[Pa\x01\xB1a\x02\xA36`\x04a\x0B\xCEV[c\x87\xA2\x11\xA2`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[a\x01\xE6a\x07\x08V[4\x80\x15a\x02\xCDW__\xFD[Pa\x01\xB1a\x02\xDC6`\x04a\x0B\xCEV[c87u\x08`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[4\x80\x15a\x02\xFEW__\xFD[Pc\x8Bx\xC6\xD8\x19Ta\x02iV[4\x80\x15a\x03\x16W__\xFD[Pa\x01Ta\x07\x1BV[4\x80\x15a\x03*W__\xFD[Pa\x01\xE6a\x0396`\x04a\x0BlV[a\x07*V[4\x80\x15a\x03IW__\xFD[Pa\x01\x89a\x03X6`\x04a\x0BlV[a\x07<V[4\x80\x15a\x03hW__\xFD[Pa\x01\xB1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\x9BW__\xFD[Pa\x01\xE6a\x03\xAA6`\x04a\x0B\xEEV[a\x07\xA0V[4\x80\x15a\x03\xBAW__\xFD[Pa\x01\xB1a\x03\xC96`\x04a\x0C[V[a\tTV[a\x01\xE6a\x03\xDC6`\x04a\x0B\xCEV[a\t\x98V[a\x01\xE6a\x03\xEF6`\x04a\x0B\xCEV[a\t\xD5V[4\x80\x15a\x03\xFFW__\xFD[Pa\x01\xB1a\x04\x0E6`\x04a\x0B\xCEV[c8\x9Au\xE1`\x0C\x90\x81R_\x91\x90\x91R` \x90 T\x90V[``_\x80Ta\x043\x90a\x0C\x8CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04_\x90a\x0C\x8CV[\x80\x15a\x04\xAAW\x80`\x1F\x10a\x04\x81Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xAAV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x8DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[_`\x01`\x01`\xA0\x1B\x03\x83\x16n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x18\x82\x19\x15\x17a\x04\xE5Wc?hS\x9A_R`\x04`\x1C\xFD[\x82` Rc\x7F^\x9F `\x0CR3_R\x81`4`\x0C U\x81_R`,Q``\x1C3\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` _\xA3P`\x01[\x92\x91PPV[_\x83``\x1Bn\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA33\x14a\x05\x89W3` Rc\x7F^\x9F \x81\x17`\x0CR`4`\x0C \x80T\x80\x19\x15a\x05\x86W\x80\x85\x11\x15a\x05\x80Wc\x13\xBE%+_R`\x04`\x1C\xFD[\x84\x81\x03\x82U[PP[c\x87\xA2\x11\xA2\x81\x17`\x0CR` `\x0C \x80T\x80\x85\x11\x15a\x05\xAFWc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x84\x81\x03\x82UPP\x83_R` `\x0C \x83\x81T\x01\x81UP\x82` R`\x0CQ``\x1C\x81``\x1C_Q` a\x0C\xC5_9_Q\x90_R` \x80\xA3PP`\x01\x93\x92PPPV[_b\x02\xA3\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16B\x01\x90Pc8\x9Au\xE1`\x0CR3_R\x80` `\x0C U3\x7F\xDB\xF3j\x10}\xA1\x9EIRzqv\xA1\xBA\xBF\x96;K\x0F\xF8\xCD\xE3^\xE3]l\xD8\xF1\xF9\xAC~\x1D__\xA2PV[_\x80a\x06Ga\x04%V[\x80Q\x90` \x01 \x90P`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R` \x81\x01\x92\x90\x92R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6\x90\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 \x91\x90PV[a\x06\xC1a\t\xFBV[a\x06\xCB\x82\x82a\n\x15V[PPV[c8\x9Au\xE1`\x0CR3_R_` `\x0C U3\x7F\xFA{\x8E\xAB}\xA6\x7FA,\xC9W^\xD44dF\x8F\x9B\xFB\xAE\x89\xD1gY\x174l\xA6\xD8\xFE<\x92__\xA2V[a\x07\x10a\t\xFBV[a\x07\x19_a\n~V[V[```\x01\x80Ta\x043\x90a\x0C\x8CV[a\x072a\t\xFBV[a\x06\xCB\x82\x82a\n\xBBV[_c\x87\xA2\x11\xA2`\x0CR3_R` `\x0C \x80T\x80\x84\x11\x15a\x07dWc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x83\x81\x03\x82UPP\x82_R` `\x0C \x82\x81T\x01\x81UP\x81` R`\x0CQ``\x1C3_Q` a\x0C\xC5_9_Q\x90_R` \x80\xA3P`\x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x86\x16n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA3\x18\x85\x19\x15\x17a\x07\xD0Wc?hS\x9A_R`\x04`\x1C\xFD[_a\x07\xD9a\x04%V[\x80Q\x90` \x01 \x90P\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6B\x86\x10\x15a\x08\x18Wc\x1A\x15\xA3\xCC_R`\x04`\x1C\xFD[`@Q\x89``\x1B``\x1C\x99P\x88``\x1B``\x1C\x98Pe87u\x08\x19\x01`\x0ER\x89_R` `\x0C \x80T\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x83R\x84` \x84\x01R\x83`@\x84\x01RF``\x84\x01R0`\x80\x84\x01R`\xA0\x83 `.R\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x83R\x8B` \x84\x01R\x8A`@\x84\x01R\x89``\x84\x01R\x80`\x80\x84\x01R\x88`\xA0\x84\x01R`\xC0\x83 `NR`B`, _R\x87`\xFF\x16` R\x86`@R\x85``R` \x80`\x80_`\x01Z\xFA\x8C=Q\x14a\t\0Wc\xDD\xAF\xBA\xEF_R`\x04`\x1C\xFD[\x01\x90Uc\x03\xFA\xF4\xF9`\xA5\x1B\x89\x17`@R`4`, \x88\x90U\x88\x8A\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%` ``\x85\x01\xA3`@RPP_``RPPPPPPPV[_n\"\xD4s\x03\x0F\x11m\xDE\xE9\xF6\xB4:\xC7\x8B\xA2\x19`\x01`\x01`\xA0\x1B\x03\x83\x16\x01a\t}WP_\x19a\x05.V[P` Rc\x7F^\x9F `\x0C\x90\x81R_\x91\x90\x91R`4\x90 T\x90V[a\t\xA0a\t\xFBV[c8\x9Au\xE1`\x0CR\x80_R` `\x0C \x80TB\x11\x15a\t\xC6Wco^\x88\x18_R`\x04`\x1C\xFD[_\x90Ua\t\xD2\x81a\n~V[PV[a\t\xDDa\t\xFBV[\x80``\x1Ba\t\xF2WctH\xFB\xAE_R`\x04`\x1C\xFD[a\t\xD2\x81a\n~V[c\x8Bx\xC6\xD8\x19T3\x14a\x07\x19Wc\x82\xB4)\0_R`\x04`\x1C\xFD[h\x054\\\xDFw\xEBh\xF4LT\x81\x81\x01\x81\x81\x10\x15a\n8Wc\xE5\xCF\xE9W_R`\x04`\x1C\xFD[\x80h\x054\\\xDFw\xEBh\xF4LUPPc\x87\xA2\x11\xA2`\x0CR\x81_R` `\x0C \x81\x81T\x01\x81UP\x80` R`\x0CQ``\x1C__Q` a\x0C\xC5_9_Q\x90_R` \x80\xA3PPV[c\x8Bx\xC6\xD8\x19\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3UV[c\x87\xA2\x11\xA2`\x0CR\x81_R` `\x0C \x80T\x80\x83\x11\x15a\n\xE2Wc\xF4\xD6x\xB8_R`\x04`\x1C\xFD[\x82\x90\x03\x90Uh\x054\\\xDFw\xEBh\xF4L\x80T\x82\x90\x03\x90U_\x81\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16_Q` a\x0C\xC5_9_Q\x90_R` \x83\xA3PPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0BgW__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x0B}W__\xFD[a\x0B\x86\x83a\x0BQV[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a\x0B\xA6W__\xFD[a\x0B\xAF\x84a\x0BQV[\x92Pa\x0B\xBD` \x85\x01a\x0BQV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_` \x82\x84\x03\x12\x15a\x0B\xDEW__\xFD[a\x0B\xE7\x82a\x0BQV[\x93\x92PPPV[_______`\xE0\x88\x8A\x03\x12\x15a\x0C\x04W__\xFD[a\x0C\r\x88a\x0BQV[\x96Pa\x0C\x1B` \x89\x01a\x0BQV[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015`\xFF\x81\x16\x81\x14a\x0C>W__\xFD[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[__`@\x83\x85\x03\x12\x15a\x0ClW__\xFD[a\x0Cu\x83a\x0BQV[\x91Pa\x0C\x83` \x84\x01a\x0BQV[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0C\xA0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0C\xBEWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \xF2U\x8A\xD4F\x0C\xA0\x0E\xBE\xBA\xB8\xD0#r\xDDk\x02\x7F\x19\xDE{\xD9\xD72\xADt\xC5\xCF$c_EdsolcC\0\x08\x1E\x003\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\xED\"[\xD3\x85\x90.r\x8B]\xFA/K\xCD\x10+\xD0\xF4\xBB|\xA0\xBE$\xB5\x87\xB9n\xBF\xD5\xFC*d\xA2dipfsX\"\x12 \xA9\xB7\xB2\x06h\xFB\xDA}\xF9\x82\xAEN\xB8[\x18\x05s\x91\xAB\x06\xE4P\x7F\xF2\x02\xD1\xB1\xAF>;j/dsolcC\0\x08\x1E\x003",
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
    /**Function with signature `MARKET_HASH()` and selector `0xc0474d0b`.
```solidity
function MARKET_HASH() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MARKET_HASHCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`MARKET_HASH()`](MARKET_HASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MARKET_HASHReturn {
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
            impl ::core::convert::From<MARKET_HASHCall> for UnderlyingRustTuple<'_> {
                fn from(value: MARKET_HASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MARKET_HASHCall {
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
            impl ::core::convert::From<MARKET_HASHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: MARKET_HASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MARKET_HASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MARKET_HASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MARKET_HASH()";
            const SELECTOR: [u8; 4] = [192u8, 71u8, 77u8, 11u8];
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
                        let r: MARKET_HASHReturn = r.into();
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
                        let r: MARKET_HASHReturn = r.into();
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
    /**Function with signature `asset()` and selector `0x38d52e0f`.
```solidity
function asset() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct assetCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`asset()`](assetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct assetReturn {
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
            impl ::core::convert::From<assetCall> for UnderlyingRustTuple<'_> {
                fn from(value: assetCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for assetCall {
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
            impl ::core::convert::From<assetReturn> for UnderlyingRustTuple<'_> {
                fn from(value: assetReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for assetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for assetCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "asset()";
            const SELECTOR: [u8; 4] = [56u8, 213u8, 46u8, 15u8];
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
                        let r: assetReturn = r.into();
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
                        let r: assetReturn = r.into();
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
    /**Function with signature `noVerse()` and selector `0x74742c08`.
```solidity
function noVerse() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct noVerseCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`noVerse()`](noVerseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct noVerseReturn {
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
            impl ::core::convert::From<noVerseCall> for UnderlyingRustTuple<'_> {
                fn from(value: noVerseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for noVerseCall {
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
            impl ::core::convert::From<noVerseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: noVerseReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for noVerseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for noVerseCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "noVerse()";
            const SELECTOR: [u8; 4] = [116u8, 116u8, 44u8, 8u8];
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
                        let r: noVerseReturn = r.into();
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
                        let r: noVerseReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
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
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
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
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
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
                        let r: ownerReturn = r.into();
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
                        let r: ownerReturn = r.into();
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
    /**Function with signature `testBurnAsNonOwner()` and selector `0x3c08bd5f`.
```solidity
function testBurnAsNonOwner() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testBurnAsNonOwnerCall;
    ///Container type for the return parameters of the [`testBurnAsNonOwner()`](testBurnAsNonOwnerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testBurnAsNonOwnerReturn {}
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
            impl ::core::convert::From<testBurnAsNonOwnerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testBurnAsNonOwnerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testBurnAsNonOwnerCall {
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
            impl ::core::convert::From<testBurnAsNonOwnerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testBurnAsNonOwnerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testBurnAsNonOwnerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testBurnAsNonOwnerReturn {
            fn _tokenize(
                &self,
            ) -> <testBurnAsNonOwnerCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testBurnAsNonOwnerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testBurnAsNonOwnerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testBurnAsNonOwner()";
            const SELECTOR: [u8; 4] = [60u8, 8u8, 189u8, 95u8];
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
                testBurnAsNonOwnerReturn::_tokenize(ret)
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
    /**Function with signature `testBurnAsOwner()` and selector `0x3b8ddf77`.
```solidity
function testBurnAsOwner() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testBurnAsOwnerCall;
    ///Container type for the return parameters of the [`testBurnAsOwner()`](testBurnAsOwnerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testBurnAsOwnerReturn {}
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
            impl ::core::convert::From<testBurnAsOwnerCall> for UnderlyingRustTuple<'_> {
                fn from(value: testBurnAsOwnerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for testBurnAsOwnerCall {
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
            impl ::core::convert::From<testBurnAsOwnerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testBurnAsOwnerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testBurnAsOwnerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testBurnAsOwnerReturn {
            fn _tokenize(
                &self,
            ) -> <testBurnAsOwnerCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testBurnAsOwnerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testBurnAsOwnerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testBurnAsOwner()";
            const SELECTOR: [u8; 4] = [59u8, 141u8, 223u8, 119u8];
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
                testBurnAsOwnerReturn::_tokenize(ret)
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
    /**Function with signature `testConstructor()` and selector `0xc2e9f2e4`.
```solidity
function testConstructor() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testConstructorCall;
    ///Container type for the return parameters of the [`testConstructor()`](testConstructorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testConstructorReturn {}
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
            impl ::core::convert::From<testConstructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: testConstructorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for testConstructorCall {
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
            impl ::core::convert::From<testConstructorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testConstructorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testConstructorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testConstructorReturn {
            fn _tokenize(
                &self,
            ) -> <testConstructorCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testConstructorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testConstructorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testConstructor()";
            const SELECTOR: [u8; 4] = [194u8, 233u8, 242u8, 228u8];
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
                testConstructorReturn::_tokenize(ret)
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
    /**Function with signature `testERC20Approve()` and selector `0x9a22b054`.
```solidity
function testERC20Approve() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testERC20ApproveCall;
    ///Container type for the return parameters of the [`testERC20Approve()`](testERC20ApproveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testERC20ApproveReturn {}
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
            impl ::core::convert::From<testERC20ApproveCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testERC20ApproveCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testERC20ApproveCall {
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
            impl ::core::convert::From<testERC20ApproveReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testERC20ApproveReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testERC20ApproveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testERC20ApproveReturn {
            fn _tokenize(
                &self,
            ) -> <testERC20ApproveCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testERC20ApproveCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testERC20ApproveReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testERC20Approve()";
            const SELECTOR: [u8; 4] = [154u8, 34u8, 176u8, 84u8];
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
                testERC20ApproveReturn::_tokenize(ret)
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
    /**Function with signature `testERC20BasicFunctionality()` and selector `0x105cda16`.
```solidity
function testERC20BasicFunctionality() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testERC20BasicFunctionalityCall;
    ///Container type for the return parameters of the [`testERC20BasicFunctionality()`](testERC20BasicFunctionalityCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testERC20BasicFunctionalityReturn {}
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
            impl ::core::convert::From<testERC20BasicFunctionalityCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testERC20BasicFunctionalityCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testERC20BasicFunctionalityCall {
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
            impl ::core::convert::From<testERC20BasicFunctionalityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testERC20BasicFunctionalityReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testERC20BasicFunctionalityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testERC20BasicFunctionalityReturn {
            fn _tokenize(
                &self,
            ) -> <testERC20BasicFunctionalityCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testERC20BasicFunctionalityCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testERC20BasicFunctionalityReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testERC20BasicFunctionality()";
            const SELECTOR: [u8; 4] = [16u8, 92u8, 218u8, 22u8];
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
                testERC20BasicFunctionalityReturn::_tokenize(ret)
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
    /**Function with signature `testFuzzApproveTransferFrom(uint256,uint256,uint256)` and selector `0xbf8452f5`.
```solidity
function testFuzzApproveTransferFrom(uint256 mintAmount, uint256 approveAmount, uint256 transferAmount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzzApproveTransferFromCall {
        #[allow(missing_docs)]
        pub mintAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub approveAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub transferAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`testFuzzApproveTransferFrom(uint256,uint256,uint256)`](testFuzzApproveTransferFromCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzzApproveTransferFromReturn {}
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<testFuzzApproveTransferFromCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzzApproveTransferFromCall) -> Self {
                    (value.mintAmount, value.approveAmount, value.transferAmount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzzApproveTransferFromCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        mintAmount: tuple.0,
                        approveAmount: tuple.1,
                        transferAmount: tuple.2,
                    }
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
            impl ::core::convert::From<testFuzzApproveTransferFromReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzzApproveTransferFromReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzzApproveTransferFromReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testFuzzApproveTransferFromReturn {
            fn _tokenize(
                &self,
            ) -> <testFuzzApproveTransferFromCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testFuzzApproveTransferFromCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testFuzzApproveTransferFromReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testFuzzApproveTransferFrom(uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [191u8, 132u8, 82u8, 245u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.mintAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.approveAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.transferAmount),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testFuzzApproveTransferFromReturn::_tokenize(ret)
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
    /**Function with signature `testFuzzMintBurn(address,uint256,uint256)` and selector `0xe115c642`.
```solidity
function testFuzzMintBurn(address recipient, uint256 mintAmount, uint256 burnAmount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzzMintBurnCall {
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub mintAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub burnAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`testFuzzMintBurn(address,uint256,uint256)`](testFuzzMintBurnCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzzMintBurnReturn {}
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<testFuzzMintBurnCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzzMintBurnCall) -> Self {
                    (value.recipient, value.mintAmount, value.burnAmount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzzMintBurnCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        recipient: tuple.0,
                        mintAmount: tuple.1,
                        burnAmount: tuple.2,
                    }
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
            impl ::core::convert::From<testFuzzMintBurnReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzzMintBurnReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzzMintBurnReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testFuzzMintBurnReturn {
            fn _tokenize(
                &self,
            ) -> <testFuzzMintBurnCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testFuzzMintBurnCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testFuzzMintBurnReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testFuzzMintBurn(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [225u8, 21u8, 198u8, 66u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.mintAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.burnAmount),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testFuzzMintBurnReturn::_tokenize(ret)
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
    /**Function with signature `testFuzzTransfer(uint256,uint256)` and selector `0x6e40056f`.
```solidity
function testFuzzTransfer(uint256 mintAmount, uint256 transferAmount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzzTransferCall {
        #[allow(missing_docs)]
        pub mintAmount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub transferAmount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`testFuzzTransfer(uint256,uint256)`](testFuzzTransferCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFuzzTransferReturn {}
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<testFuzzTransferCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzzTransferCall) -> Self {
                    (value.mintAmount, value.transferAmount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzzTransferCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        mintAmount: tuple.0,
                        transferAmount: tuple.1,
                    }
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
            impl ::core::convert::From<testFuzzTransferReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFuzzTransferReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFuzzTransferReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testFuzzTransferReturn {
            fn _tokenize(
                &self,
            ) -> <testFuzzTransferCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testFuzzTransferCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testFuzzTransferReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testFuzzTransfer(uint256,uint256)";
            const SELECTOR: [u8; 4] = [110u8, 64u8, 5u8, 111u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.mintAmount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.transferAmount),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                testFuzzTransferReturn::_tokenize(ret)
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
    /**Function with signature `testMintAsNonOwner()` and selector `0x081c69e4`.
```solidity
function testMintAsNonOwner() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testMintAsNonOwnerCall;
    ///Container type for the return parameters of the [`testMintAsNonOwner()`](testMintAsNonOwnerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testMintAsNonOwnerReturn {}
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
            impl ::core::convert::From<testMintAsNonOwnerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testMintAsNonOwnerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testMintAsNonOwnerCall {
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
            impl ::core::convert::From<testMintAsNonOwnerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testMintAsNonOwnerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testMintAsNonOwnerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testMintAsNonOwnerReturn {
            fn _tokenize(
                &self,
            ) -> <testMintAsNonOwnerCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testMintAsNonOwnerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testMintAsNonOwnerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testMintAsNonOwner()";
            const SELECTOR: [u8; 4] = [8u8, 28u8, 105u8, 228u8];
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
                testMintAsNonOwnerReturn::_tokenize(ret)
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
    /**Function with signature `testMintAsOwner()` and selector `0xd883ba03`.
```solidity
function testMintAsOwner() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testMintAsOwnerCall;
    ///Container type for the return parameters of the [`testMintAsOwner()`](testMintAsOwnerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testMintAsOwnerReturn {}
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
            impl ::core::convert::From<testMintAsOwnerCall> for UnderlyingRustTuple<'_> {
                fn from(value: testMintAsOwnerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for testMintAsOwnerCall {
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
            impl ::core::convert::From<testMintAsOwnerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testMintAsOwnerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testMintAsOwnerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testMintAsOwnerReturn {
            fn _tokenize(
                &self,
            ) -> <testMintAsOwnerCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testMintAsOwnerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testMintAsOwnerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testMintAsOwner()";
            const SELECTOR: [u8; 4] = [216u8, 131u8, 186u8, 3u8];
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
                testMintAsOwnerReturn::_tokenize(ret)
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
    /**Function with signature `testMultipleMintsBurns()` and selector `0x2b903d9b`.
```solidity
function testMultipleMintsBurns() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testMultipleMintsBurnsCall;
    ///Container type for the return parameters of the [`testMultipleMintsBurns()`](testMultipleMintsBurnsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testMultipleMintsBurnsReturn {}
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
            impl ::core::convert::From<testMultipleMintsBurnsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testMultipleMintsBurnsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testMultipleMintsBurnsCall {
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
            impl ::core::convert::From<testMultipleMintsBurnsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testMultipleMintsBurnsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testMultipleMintsBurnsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testMultipleMintsBurnsReturn {
            fn _tokenize(
                &self,
            ) -> <testMultipleMintsBurnsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testMultipleMintsBurnsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testMultipleMintsBurnsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testMultipleMintsBurns()";
            const SELECTOR: [u8; 4] = [43u8, 144u8, 61u8, 155u8];
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
                testMultipleMintsBurnsReturn::_tokenize(ret)
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
    /**Function with signature `testOwnershipTransfer()` and selector `0x575fc5d5`.
```solidity
function testOwnershipTransfer() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testOwnershipTransferCall;
    ///Container type for the return parameters of the [`testOwnershipTransfer()`](testOwnershipTransferCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testOwnershipTransferReturn {}
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
            impl ::core::convert::From<testOwnershipTransferCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testOwnershipTransferCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testOwnershipTransferCall {
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
            impl ::core::convert::From<testOwnershipTransferReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testOwnershipTransferReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testOwnershipTransferReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testOwnershipTransferReturn {
            fn _tokenize(
                &self,
            ) -> <testOwnershipTransferCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testOwnershipTransferCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testOwnershipTransferReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testOwnershipTransfer()";
            const SELECTOR: [u8; 4] = [87u8, 95u8, 197u8, 213u8];
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
                testOwnershipTransferReturn::_tokenize(ret)
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
    /**Function with signature `testRenounceOwnership()` and selector `0xf09d1d44`.
```solidity
function testRenounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testRenounceOwnershipCall;
    ///Container type for the return parameters of the [`testRenounceOwnership()`](testRenounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testRenounceOwnershipReturn {}
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
            impl ::core::convert::From<testRenounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testRenounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testRenounceOwnershipCall {
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
            impl ::core::convert::From<testRenounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testRenounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testRenounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl testRenounceOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <testRenounceOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testRenounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testRenounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testRenounceOwnership()";
            const SELECTOR: [u8; 4] = [240u8, 157u8, 29u8, 68u8];
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
                testRenounceOwnershipReturn::_tokenize(ret)
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
    /**Function with signature `yesVerse()` and selector `0x4f19b941`.
```solidity
function yesVerse() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct yesVerseCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`yesVerse()`](yesVerseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct yesVerseReturn {
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
            impl ::core::convert::From<yesVerseCall> for UnderlyingRustTuple<'_> {
                fn from(value: yesVerseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for yesVerseCall {
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
            impl ::core::convert::From<yesVerseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: yesVerseReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for yesVerseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for yesVerseCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "yesVerse()";
            const SELECTOR: [u8; 4] = [79u8, 25u8, 185u8, 65u8];
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
                        let r: yesVerseReturn = r.into();
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
                        let r: yesVerseReturn = r.into();
                        r._0
                    })
            }
        }
    };
    ///Container for all the [`VerseTest`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum VerseTestCalls {
        #[allow(missing_docs)]
        IS_TEST(IS_TESTCall),
        #[allow(missing_docs)]
        MARKET_HASH(MARKET_HASHCall),
        #[allow(missing_docs)]
        alice(aliceCall),
        #[allow(missing_docs)]
        asset(assetCall),
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
        noVerse(noVerseCall),
        #[allow(missing_docs)]
        owner(ownerCall),
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
        testBurnAsNonOwner(testBurnAsNonOwnerCall),
        #[allow(missing_docs)]
        testBurnAsOwner(testBurnAsOwnerCall),
        #[allow(missing_docs)]
        testConstructor(testConstructorCall),
        #[allow(missing_docs)]
        testERC20Approve(testERC20ApproveCall),
        #[allow(missing_docs)]
        testERC20BasicFunctionality(testERC20BasicFunctionalityCall),
        #[allow(missing_docs)]
        testFuzzApproveTransferFrom(testFuzzApproveTransferFromCall),
        #[allow(missing_docs)]
        testFuzzMintBurn(testFuzzMintBurnCall),
        #[allow(missing_docs)]
        testFuzzTransfer(testFuzzTransferCall),
        #[allow(missing_docs)]
        testMintAsNonOwner(testMintAsNonOwnerCall),
        #[allow(missing_docs)]
        testMintAsOwner(testMintAsOwnerCall),
        #[allow(missing_docs)]
        testMultipleMintsBurns(testMultipleMintsBurnsCall),
        #[allow(missing_docs)]
        testOwnershipTransfer(testOwnershipTransferCall),
        #[allow(missing_docs)]
        testRenounceOwnership(testRenounceOwnershipCall),
        #[allow(missing_docs)]
        yesVerse(yesVerseCall),
    }
    impl VerseTestCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [8u8, 28u8, 105u8, 228u8],
            [10u8, 146u8, 84u8, 228u8],
            [16u8, 92u8, 218u8, 22u8],
            [30u8, 215u8, 131u8, 28u8],
            [42u8, 222u8, 56u8, 128u8],
            [43u8, 144u8, 61u8, 155u8],
            [56u8, 213u8, 46u8, 15u8],
            [59u8, 141u8, 223u8, 119u8],
            [60u8, 8u8, 189u8, 95u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [79u8, 25u8, 185u8, 65u8],
            [87u8, 95u8, 197u8, 213u8],
            [102u8, 217u8, 169u8, 160u8],
            [110u8, 64u8, 5u8, 111u8],
            [116u8, 116u8, 44u8, 8u8],
            [133u8, 34u8, 108u8, 129u8],
            [141u8, 165u8, 203u8, 91u8],
            [145u8, 106u8, 23u8, 198u8],
            [154u8, 34u8, 176u8, 84u8],
            [176u8, 70u8, 79u8, 220u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [191u8, 132u8, 82u8, 245u8],
            [192u8, 71u8, 77u8, 11u8],
            [192u8, 156u8, 236u8, 119u8],
            [194u8, 233u8, 242u8, 228u8],
            [216u8, 131u8, 186u8, 3u8],
            [225u8, 21u8, 198u8, 66u8],
            [226u8, 12u8, 159u8, 113u8],
            [240u8, 157u8, 29u8, 68u8],
            [250u8, 118u8, 38u8, 212u8],
            [251u8, 71u8, 227u8, 162u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(testMintAsNonOwner),
            ::core::stringify!(setUp),
            ::core::stringify!(testERC20BasicFunctionality),
            ::core::stringify!(excludeSenders),
            ::core::stringify!(targetInterfaces),
            ::core::stringify!(testMultipleMintsBurns),
            ::core::stringify!(asset),
            ::core::stringify!(testBurnAsOwner),
            ::core::stringify!(testBurnAsNonOwner),
            ::core::stringify!(targetSenders),
            ::core::stringify!(targetContracts),
            ::core::stringify!(yesVerse),
            ::core::stringify!(testOwnershipTransfer),
            ::core::stringify!(targetArtifactSelectors),
            ::core::stringify!(testFuzzTransfer),
            ::core::stringify!(noVerse),
            ::core::stringify!(targetArtifacts),
            ::core::stringify!(owner),
            ::core::stringify!(targetSelectors),
            ::core::stringify!(testERC20Approve),
            ::core::stringify!(excludeSelectors),
            ::core::stringify!(excludeArtifacts),
            ::core::stringify!(failed),
            ::core::stringify!(testFuzzApproveTransferFrom),
            ::core::stringify!(MARKET_HASH),
            ::core::stringify!(bob),
            ::core::stringify!(testConstructor),
            ::core::stringify!(testMintAsOwner),
            ::core::stringify!(testFuzzMintBurn),
            ::core::stringify!(excludeContracts),
            ::core::stringify!(testRenounceOwnership),
            ::core::stringify!(IS_TEST),
            ::core::stringify!(alice),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <testMintAsNonOwnerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setUpCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testERC20BasicFunctionalityCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetInterfacesCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testMultipleMintsBurnsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <assetCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testBurnAsOwnerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testBurnAsNonOwnerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetContractsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <yesVerseCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testOwnershipTransferCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testFuzzTransferCall as alloy_sol_types::SolCall>::SIGNATURE,
            <noVerseCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetArtifactsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <ownerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <targetSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testERC20ApproveCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeSelectorsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeArtifactsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <failedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testFuzzApproveTransferFromCall as alloy_sol_types::SolCall>::SIGNATURE,
            <MARKET_HASHCall as alloy_sol_types::SolCall>::SIGNATURE,
            <bobCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testConstructorCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testMintAsOwnerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testFuzzMintBurnCall as alloy_sol_types::SolCall>::SIGNATURE,
            <excludeContractsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <testRenounceOwnershipCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for VerseTestCalls {
        const NAME: &'static str = "VerseTestCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 33usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::MARKET_HASH(_) => {
                    <MARKET_HASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::alice(_) => <aliceCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::asset(_) => <assetCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::noVerse(_) => <noVerseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::testBurnAsNonOwner(_) => {
                    <testBurnAsNonOwnerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testBurnAsOwner(_) => {
                    <testBurnAsOwnerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testConstructor(_) => {
                    <testConstructorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testERC20Approve(_) => {
                    <testERC20ApproveCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testERC20BasicFunctionality(_) => {
                    <testERC20BasicFunctionalityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testFuzzApproveTransferFrom(_) => {
                    <testFuzzApproveTransferFromCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testFuzzMintBurn(_) => {
                    <testFuzzMintBurnCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testFuzzTransfer(_) => {
                    <testFuzzTransferCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testMintAsNonOwner(_) => {
                    <testMintAsNonOwnerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testMintAsOwner(_) => {
                    <testMintAsOwnerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testMultipleMintsBurns(_) => {
                    <testMultipleMintsBurnsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testOwnershipTransfer(_) => {
                    <testOwnershipTransferCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testRenounceOwnership(_) => {
                    <testRenounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::yesVerse(_) => <yesVerseCall as alloy_sol_types::SolCall>::SELECTOR,
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
            ) -> alloy_sol_types::Result<VerseTestCalls>] = &[
                {
                    fn testMintAsNonOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testMintAsNonOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::testMintAsNonOwner)
                    }
                    testMintAsNonOwner
                },
                {
                    fn setUp(data: &[u8]) -> alloy_sol_types::Result<VerseTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(VerseTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn testERC20BasicFunctionality(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testERC20BasicFunctionalityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::testERC20BasicFunctionality)
                    }
                    testERC20BasicFunctionality
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn testMultipleMintsBurns(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testMultipleMintsBurnsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::testMultipleMintsBurns)
                    }
                    testMultipleMintsBurns
                },
                {
                    fn asset(data: &[u8]) -> alloy_sol_types::Result<VerseTestCalls> {
                        <assetCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(VerseTestCalls::asset)
                    }
                    asset
                },
                {
                    fn testBurnAsOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testBurnAsOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::testBurnAsOwner)
                    }
                    testBurnAsOwner
                },
                {
                    fn testBurnAsNonOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testBurnAsNonOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::testBurnAsNonOwner)
                    }
                    testBurnAsNonOwner
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn yesVerse(data: &[u8]) -> alloy_sol_types::Result<VerseTestCalls> {
                        <yesVerseCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(VerseTestCalls::yesVerse)
                    }
                    yesVerse
                },
                {
                    fn testOwnershipTransfer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testOwnershipTransferCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::testOwnershipTransfer)
                    }
                    testOwnershipTransfer
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn testFuzzTransfer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testFuzzTransferCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::testFuzzTransfer)
                    }
                    testFuzzTransfer
                },
                {
                    fn noVerse(data: &[u8]) -> alloy_sol_types::Result<VerseTestCalls> {
                        <noVerseCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(VerseTestCalls::noVerse)
                    }
                    noVerse
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn owner(data: &[u8]) -> alloy_sol_types::Result<VerseTestCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(VerseTestCalls::owner)
                    }
                    owner
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn testERC20Approve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testERC20ApproveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::testERC20Approve)
                    }
                    testERC20Approve
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(data: &[u8]) -> alloy_sol_types::Result<VerseTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(VerseTestCalls::failed)
                    }
                    failed
                },
                {
                    fn testFuzzApproveTransferFrom(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testFuzzApproveTransferFromCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::testFuzzApproveTransferFrom)
                    }
                    testFuzzApproveTransferFrom
                },
                {
                    fn MARKET_HASH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <MARKET_HASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::MARKET_HASH)
                    }
                    MARKET_HASH
                },
                {
                    fn bob(data: &[u8]) -> alloy_sol_types::Result<VerseTestCalls> {
                        <bobCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(VerseTestCalls::bob)
                    }
                    bob
                },
                {
                    fn testConstructor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testConstructorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::testConstructor)
                    }
                    testConstructor
                },
                {
                    fn testMintAsOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testMintAsOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::testMintAsOwner)
                    }
                    testMintAsOwner
                },
                {
                    fn testFuzzMintBurn(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testFuzzMintBurnCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::testFuzzMintBurn)
                    }
                    testFuzzMintBurn
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn testRenounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testRenounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(VerseTestCalls::testRenounceOwnership)
                    }
                    testRenounceOwnership
                },
                {
                    fn IS_TEST(data: &[u8]) -> alloy_sol_types::Result<VerseTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(VerseTestCalls::IS_TEST)
                    }
                    IS_TEST
                },
                {
                    fn alice(data: &[u8]) -> alloy_sol_types::Result<VerseTestCalls> {
                        <aliceCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(VerseTestCalls::alice)
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
            ) -> alloy_sol_types::Result<VerseTestCalls>] = &[
                {
                    fn testMintAsNonOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testMintAsNonOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::testMintAsNonOwner)
                    }
                    testMintAsNonOwner
                },
                {
                    fn setUp(data: &[u8]) -> alloy_sol_types::Result<VerseTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn testERC20BasicFunctionality(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testERC20BasicFunctionalityCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::testERC20BasicFunctionality)
                    }
                    testERC20BasicFunctionality
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn testMultipleMintsBurns(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testMultipleMintsBurnsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::testMultipleMintsBurns)
                    }
                    testMultipleMintsBurns
                },
                {
                    fn asset(data: &[u8]) -> alloy_sol_types::Result<VerseTestCalls> {
                        <assetCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::asset)
                    }
                    asset
                },
                {
                    fn testBurnAsOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testBurnAsOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::testBurnAsOwner)
                    }
                    testBurnAsOwner
                },
                {
                    fn testBurnAsNonOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testBurnAsNonOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::testBurnAsNonOwner)
                    }
                    testBurnAsNonOwner
                },
                {
                    fn targetSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn yesVerse(data: &[u8]) -> alloy_sol_types::Result<VerseTestCalls> {
                        <yesVerseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::yesVerse)
                    }
                    yesVerse
                },
                {
                    fn testOwnershipTransfer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testOwnershipTransferCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::testOwnershipTransfer)
                    }
                    testOwnershipTransfer
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn testFuzzTransfer(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testFuzzTransferCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::testFuzzTransfer)
                    }
                    testFuzzTransfer
                },
                {
                    fn noVerse(data: &[u8]) -> alloy_sol_types::Result<VerseTestCalls> {
                        <noVerseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::noVerse)
                    }
                    noVerse
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn owner(data: &[u8]) -> alloy_sol_types::Result<VerseTestCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::owner)
                    }
                    owner
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn testERC20Approve(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testERC20ApproveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::testERC20Approve)
                    }
                    testERC20Approve
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(data: &[u8]) -> alloy_sol_types::Result<VerseTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::failed)
                    }
                    failed
                },
                {
                    fn testFuzzApproveTransferFrom(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testFuzzApproveTransferFromCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::testFuzzApproveTransferFrom)
                    }
                    testFuzzApproveTransferFrom
                },
                {
                    fn MARKET_HASH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <MARKET_HASHCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::MARKET_HASH)
                    }
                    MARKET_HASH
                },
                {
                    fn bob(data: &[u8]) -> alloy_sol_types::Result<VerseTestCalls> {
                        <bobCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::bob)
                    }
                    bob
                },
                {
                    fn testConstructor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testConstructorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::testConstructor)
                    }
                    testConstructor
                },
                {
                    fn testMintAsOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testMintAsOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::testMintAsOwner)
                    }
                    testMintAsOwner
                },
                {
                    fn testFuzzMintBurn(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testFuzzMintBurnCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::testFuzzMintBurn)
                    }
                    testFuzzMintBurn
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn testRenounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<VerseTestCalls> {
                        <testRenounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::testRenounceOwnership)
                    }
                    testRenounceOwnership
                },
                {
                    fn IS_TEST(data: &[u8]) -> alloy_sol_types::Result<VerseTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::IS_TEST)
                    }
                    IS_TEST
                },
                {
                    fn alice(data: &[u8]) -> alloy_sol_types::Result<VerseTestCalls> {
                        <aliceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(VerseTestCalls::alice)
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
                Self::MARKET_HASH(inner) => {
                    <MARKET_HASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::alice(inner) => {
                    <aliceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::asset(inner) => {
                    <assetCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::noVerse(inner) => {
                    <noVerseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::testBurnAsNonOwner(inner) => {
                    <testBurnAsNonOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testBurnAsOwner(inner) => {
                    <testBurnAsOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testConstructor(inner) => {
                    <testConstructorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testERC20Approve(inner) => {
                    <testERC20ApproveCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testERC20BasicFunctionality(inner) => {
                    <testERC20BasicFunctionalityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testFuzzApproveTransferFrom(inner) => {
                    <testFuzzApproveTransferFromCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testFuzzMintBurn(inner) => {
                    <testFuzzMintBurnCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testFuzzTransfer(inner) => {
                    <testFuzzTransferCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testMintAsNonOwner(inner) => {
                    <testMintAsNonOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testMintAsOwner(inner) => {
                    <testMintAsOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testMultipleMintsBurns(inner) => {
                    <testMultipleMintsBurnsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testOwnershipTransfer(inner) => {
                    <testOwnershipTransferCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testRenounceOwnership(inner) => {
                    <testRenounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::yesVerse(inner) => {
                    <yesVerseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::MARKET_HASH(inner) => {
                    <MARKET_HASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::alice(inner) => {
                    <aliceCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::asset(inner) => {
                    <assetCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::noVerse(inner) => {
                    <noVerseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::testBurnAsNonOwner(inner) => {
                    <testBurnAsNonOwnerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testBurnAsOwner(inner) => {
                    <testBurnAsOwnerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testConstructor(inner) => {
                    <testConstructorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testERC20Approve(inner) => {
                    <testERC20ApproveCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testERC20BasicFunctionality(inner) => {
                    <testERC20BasicFunctionalityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testFuzzApproveTransferFrom(inner) => {
                    <testFuzzApproveTransferFromCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testFuzzMintBurn(inner) => {
                    <testFuzzMintBurnCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testFuzzTransfer(inner) => {
                    <testFuzzTransferCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testMintAsNonOwner(inner) => {
                    <testMintAsNonOwnerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testMintAsOwner(inner) => {
                    <testMintAsOwnerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testMultipleMintsBurns(inner) => {
                    <testMultipleMintsBurnsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testOwnershipTransfer(inner) => {
                    <testOwnershipTransferCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testRenounceOwnership(inner) => {
                    <testRenounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::yesVerse(inner) => {
                    <yesVerseCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`VerseTest`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum VerseTestEvents {
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
    impl VerseTestEvents {
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
    impl alloy_sol_types::SolEventInterface for VerseTestEvents {
        const NAME: &'static str = "VerseTestEvents";
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
    impl alloy_sol_types::private::IntoLogData for VerseTestEvents {
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
    /**Creates a new wrapper around an on-chain [`VerseTest`](self) contract instance.

See the [wrapper's documentation](`VerseTestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> VerseTestInstance<P, N> {
        VerseTestInstance::<P, N>::new(address, __provider)
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
        Output = alloy_contract::Result<VerseTestInstance<P, N>>,
    > {
        VerseTestInstance::<P, N>::deploy(__provider)
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
        VerseTestInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`VerseTest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`VerseTest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct VerseTestInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for VerseTestInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("VerseTestInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > VerseTestInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`VerseTest`](self) contract instance.

See the [wrapper's documentation](`VerseTestInstance`) for more details.*/
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
        ) -> alloy_contract::Result<VerseTestInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> VerseTestInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> VerseTestInstance<P, N> {
            VerseTestInstance {
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
    > VerseTestInstance<P, N> {
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
        ///Creates a new call builder for the [`MARKET_HASH`] function.
        pub fn MARKET_HASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, MARKET_HASHCall, N> {
            self.call_builder(&MARKET_HASHCall)
        }
        ///Creates a new call builder for the [`alice`] function.
        pub fn alice(&self) -> alloy_contract::SolCallBuilder<&P, aliceCall, N> {
            self.call_builder(&aliceCall)
        }
        ///Creates a new call builder for the [`asset`] function.
        pub fn asset(&self) -> alloy_contract::SolCallBuilder<&P, assetCall, N> {
            self.call_builder(&assetCall)
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
        ///Creates a new call builder for the [`noVerse`] function.
        pub fn noVerse(&self) -> alloy_contract::SolCallBuilder<&P, noVerseCall, N> {
            self.call_builder(&noVerseCall)
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<&P, ownerCall, N> {
            self.call_builder(&ownerCall)
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
        ///Creates a new call builder for the [`testBurnAsNonOwner`] function.
        pub fn testBurnAsNonOwner(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testBurnAsNonOwnerCall, N> {
            self.call_builder(&testBurnAsNonOwnerCall)
        }
        ///Creates a new call builder for the [`testBurnAsOwner`] function.
        pub fn testBurnAsOwner(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testBurnAsOwnerCall, N> {
            self.call_builder(&testBurnAsOwnerCall)
        }
        ///Creates a new call builder for the [`testConstructor`] function.
        pub fn testConstructor(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testConstructorCall, N> {
            self.call_builder(&testConstructorCall)
        }
        ///Creates a new call builder for the [`testERC20Approve`] function.
        pub fn testERC20Approve(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testERC20ApproveCall, N> {
            self.call_builder(&testERC20ApproveCall)
        }
        ///Creates a new call builder for the [`testERC20BasicFunctionality`] function.
        pub fn testERC20BasicFunctionality(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testERC20BasicFunctionalityCall, N> {
            self.call_builder(&testERC20BasicFunctionalityCall)
        }
        ///Creates a new call builder for the [`testFuzzApproveTransferFrom`] function.
        pub fn testFuzzApproveTransferFrom(
            &self,
            mintAmount: alloy::sol_types::private::primitives::aliases::U256,
            approveAmount: alloy::sol_types::private::primitives::aliases::U256,
            transferAmount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, testFuzzApproveTransferFromCall, N> {
            self.call_builder(
                &testFuzzApproveTransferFromCall {
                    mintAmount,
                    approveAmount,
                    transferAmount,
                },
            )
        }
        ///Creates a new call builder for the [`testFuzzMintBurn`] function.
        pub fn testFuzzMintBurn(
            &self,
            recipient: alloy::sol_types::private::Address,
            mintAmount: alloy::sol_types::private::primitives::aliases::U256,
            burnAmount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, testFuzzMintBurnCall, N> {
            self.call_builder(
                &testFuzzMintBurnCall {
                    recipient,
                    mintAmount,
                    burnAmount,
                },
            )
        }
        ///Creates a new call builder for the [`testFuzzTransfer`] function.
        pub fn testFuzzTransfer(
            &self,
            mintAmount: alloy::sol_types::private::primitives::aliases::U256,
            transferAmount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, testFuzzTransferCall, N> {
            self.call_builder(
                &testFuzzTransferCall {
                    mintAmount,
                    transferAmount,
                },
            )
        }
        ///Creates a new call builder for the [`testMintAsNonOwner`] function.
        pub fn testMintAsNonOwner(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testMintAsNonOwnerCall, N> {
            self.call_builder(&testMintAsNonOwnerCall)
        }
        ///Creates a new call builder for the [`testMintAsOwner`] function.
        pub fn testMintAsOwner(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testMintAsOwnerCall, N> {
            self.call_builder(&testMintAsOwnerCall)
        }
        ///Creates a new call builder for the [`testMultipleMintsBurns`] function.
        pub fn testMultipleMintsBurns(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testMultipleMintsBurnsCall, N> {
            self.call_builder(&testMultipleMintsBurnsCall)
        }
        ///Creates a new call builder for the [`testOwnershipTransfer`] function.
        pub fn testOwnershipTransfer(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testOwnershipTransferCall, N> {
            self.call_builder(&testOwnershipTransferCall)
        }
        ///Creates a new call builder for the [`testRenounceOwnership`] function.
        pub fn testRenounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, testRenounceOwnershipCall, N> {
            self.call_builder(&testRenounceOwnershipCall)
        }
        ///Creates a new call builder for the [`yesVerse`] function.
        pub fn yesVerse(&self) -> alloy_contract::SolCallBuilder<&P, yesVerseCall, N> {
            self.call_builder(&yesVerseCall)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > VerseTestInstance<P, N> {
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
