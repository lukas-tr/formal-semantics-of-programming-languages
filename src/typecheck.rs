use std::collections::{HashMap, HashSet};

use crate::types::*;

static INT_SORT: Sort<'static> = Sort(Identifier("Int"));
static BOOL_SORT: Sort<'static> = Sort(Identifier("Bool"));

static VALID_SORTS: HashSet<Sort<'static>> = HashSet::from([INT_SORT.clone(), BOOL_SORT.clone()]);

pub trait Typecheck {
    fn typecheck(&self, variables: &HashMap<Identifier, Sort>) -> Result<HashMap<Identifier, Sort>, &'static str>;
}

impl<'a> Typecheck for Program<'a> {
    fn typecheck(&self, variables: &HashMap<Identifier, Sort>) -> Result<HashMap<Identifier, Sort>, &'static str> {
        todo!()
    }
}
