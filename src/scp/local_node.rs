use sodiumoxide::crypto::sign;
use std::collections::HashSet;

pub struct LocalNode {
    pk: sign::PublicKey,
    sk: sign::SecretKey,
    qset: QSet,
}

//#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub enum QSet {
    Validator(sign::PublicKey),
    SubSet(usize, Vec<QSet>)
}

impl QSet {
    pub fn normalize(&self) -> QSet {
        match self {
            &QSet::SubSet(0, _) => QSet::SubSet(0, vec![]),
            &QSet::SubSet(1, ref v) if v.len() == 1 => v[0].normalize(),
            _ => self.clone()
        }
    }

    pub fn is_slice(&self, s: &Vec<sign::PublicKey>) -> bool {
        match self {
            &QSet::Validator(pk) => s.contains(&pk),
            &QSet::SubSet(0, _) => true,
            &QSet::SubSet(n, ref v) => {
                let mut i = 0;
                for q in v {
                    if q.is_slice(s) {
                        i = i + 1;
                        if i == n {return true}
                    }
                }
                false
            }
        }
    }

    pub fn is_blocking(&self, s: &Vec<sign::PublicKey>) -> bool {
        match self {
            &QSet::Validator(pk) => s.contains(&pk),
            &QSet::SubSet(0, _) => false,
            &QSet::SubSet(n, ref v) => {
                let mut i = 0;
                for q in v {
                    if q.is_blocking(s) {
                        i = i + 1;
                        if i == v.len() - n + 1 {return true}
                    }
                }
                false
            }
        }
    }
}

