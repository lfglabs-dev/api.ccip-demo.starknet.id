use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use starknet::core::{crypto::pedersen_hash, types::FieldElement};

#[macro_export]
macro_rules! pub_struct {
    ($($derive:path),*; $name:ident {$($field:ident: $t:ty),* $(,)?}) => {
        #[derive($($derive),*)]
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub fn get_error(error: String) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, error).into_response()
}

pub fn hash_domain(domain: Vec<FieldElement>) -> FieldElement {
    if domain.is_empty() {
        return FieldElement::ZERO;
    }
    let new_len = domain.len() - 1;
    let x = domain[new_len];
    let y = hash_domain(domain[0..new_len].to_vec());
    pedersen_hash(&x, &y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use starknet::core::types::FieldElement;

    #[test]
    fn test_hash_domain_empty() {
        let domain: Vec<FieldElement> = vec![];
        let hash: FieldElement = hash_domain(domain);
        assert_eq!(hash, FieldElement::ZERO);
    }

    #[test]
    fn test_hash_domain_single_element() {
        // hash 'iris'
        let domain: Vec<FieldElement> = vec![FieldElement::from_dec_str("999902").unwrap()];
        let hash = hash_domain(domain);
        let expected_hash = FieldElement::from_dec_str(
            "2819968515778978195378012518635693386896866121180586187462905840795338238772",
        )
        .unwrap();
        assert_eq!(hash, expected_hash);
    }

    #[test]
    fn test_hash_domain_multiple_elements() {
        // hash 'iris.notion'
        let domain: Vec<FieldElement> = vec![
            FieldElement::from_dec_str("999902").unwrap(),
            FieldElement::from_dec_str("1059716045").unwrap(),
        ];
        let hash = hash_domain(domain);
        let expected_hash = FieldElement::from_dec_str(
            "743232737575968623292492568916248379498607022315110255883250117418490029830",
        )
        .unwrap();
        assert_eq!(hash, expected_hash);
    }
}
