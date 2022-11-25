use std::{collections::HashSet, hash::Hash};

#[derive(Clone)]
pub enum V {
    Wildcard, Null,
    Int(i64), Float(f64), Bool(bool), String(String),
    Vector(Vec<V>, Type)
}
impl std::fmt::Display for V {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wildcard => write!(f, "_"),
            Self::Null => write!(f, "null"),
            Self::Int(v) => write!(f, "{v}"),
            Self::Float(v) => write!(f, "{v}"),
            Self::Bool(v) => write!(f, "{v}"),
            Self::String(v) => write!(f, "{v}"),
            Self::Vector(v, _) => write!(f, "{v:?}"),
        }
    }
}
impl std::fmt::Debug for V {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wildcard => write!(f, "_"),
            Self::Null => write!(f, "null"),
            Self::Int(v) => write!(f, "{v:?}"),
            Self::Float(v) => write!(f, "{v:?}"),
            Self::Bool(v) => write!(f, "{v:?}"),
            Self::String(v) => write!(f, "{v:?}"),
            Self::Vector(v, _) => write!(f, "{v:?}"),
        }
    }
}
impl PartialEq for V {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Wildcard => true,
            Self::Null => match other {
                Self::Null => true,
                Self::Wildcard => true,
                _ => false
            }
            Self::Int(v1) => match other {
                Self::Int(v2) => *v1 == *v2,
                Self::Float(v2) => *v1 as f64 == *v2,
                Self::Wildcard => true,
                _ => false
            }
            Self::Float(v1) => match other {
                Self::Int(v2) => *v1 == *v2 as f64,
                Self::Float(v2) => *v1 == *v2,
                Self::Wildcard => true,
                _ => false
            }
            Self::Bool(v1) => match other {
                Self::Bool(v2) => *v1 == *v2,
                Self::Wildcard => true,
                _ => false
            }
            Self::String(v1) => match other {
                Self::String(v2) => v1 == v2,
                Self::Wildcard => true,
                _ => false
            }
            Self::Vector(v1, t1) => match other {
                Self::Vector(v2, t2) => v1 == v2 && t1 == t2,
                _ => false
            }
            _ => false
        }
    }
}

#[derive(Clone)]
pub enum Type {
    Any, Undefiend,
    Int, Float, Bool, String,
    Vector(Box<Type>),
    Union(Vec<Type>), Scission(Vec<Type>)
}
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any => write!(f, "any"),
            Self::Undefiend => write!(f, "undefined"),
            Self::Int => write!(f, "int"),
            Self::Float => write!(f, "float"),
            Self::Bool => write!(f, "bool"),
            Self::String => write!(f, "str"),
            Self::Vector(t) => write!(f, "vec"),
            Self::Union(types) => write!(f, "union[{}]", types.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("|")),
            Self::Scission(types) => write!(f, "scission[{}]", types.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("|")),
        }
    }
}
impl std::fmt::Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any => write!(f, "any"),
            Self::Undefiend => write!(f, "undefined"),
            Self::Int => write!(f, "int"),
            Self::Float => write!(f, "float"),
            Self::Bool => write!(f, "bool"),
            Self::String => write!(f, "str"),
            Self::Vector(t) => write!(f, "vec"),
            Self::Union(types) => write!(f, "union[{}]", types.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("|")),
            Self::Scission(types) => write!(f, "scission[{}]", types.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("|")),
        }
    }
}
impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Any => true,
            Self::Undefiend => match other {
                Self::Undefiend => true,
                Self::Any => true,
                Self::Union(_) => other == self,
                Self::Scission(_) => other == self,
                _ => false
            }
            Self::Int => match other {
                Self::Int => true,
                Self::Any => true,
                Self::Union(_) => other == self,
                Self::Scission(_) => other == self,
                _ => false
            }
            Self::Float => match other {
                Self::Float => true,
                Self::Any => true,
                Self::Union(_) => other == self,
                Self::Scission(_) => other == self,
                _ => false
            }
            Self::Bool => match other {
                Self::Bool => true,
                Self::Any => true,
                Self::Union(_) => other == self,
                Self::Scission(_) => other == self,
                _ => false
            }
            Self::String => match other {
                Self::String => true,
                Self::Any => true,
                Self::Union(_) => other == self,
                Self::Scission(_) => other == self,
                _ => false
            }
            Self::Vector(t1) => match other {
                Self::Vector(t2) => t1.as_ref() == t2.as_ref(),
                Self::Any => true,
                Self::Union(_) => other == self,
                Self::Scission(_) => other == self,
                _ => false
            }
            Self::Union(t1) => match other {
                Self::Union(t2) => {
                    for type1 in t1.iter() {
                        let mut matches = false;
                        for type2 in t2.iter() {
                            if type1 == type2 { matches = true; break }
                        }
                        if !matches { return false }
                    }
                    true
                }
                Self::Scission(t2) => {
                    for type1 in t1.iter() {
                        let mut matches = false;
                        for type2 in t2.iter() {
                            if type1 == type2 { matches = true; break }
                        }
                        if matches { return false }
                    }
                    true
                }
                Self::Any => true,
                _ => {
                    for t in t1.iter() {
                        if t == other { return true }
                    }
                    false
                }
            }
            Self::Scission(t1) => match other {
                Self::Union(t2) => {
                    for type1 in t1.iter() {
                        let mut matches = false;
                        for type2 in t2.iter() {
                            if type1 == type2 { matches = true; break }
                        }
                        if matches { return false }
                    }
                    true
                }
                Self::Scission(t2) => {
                    for type1 in t1.iter() {
                        let mut matches = false;
                        for type2 in t2.iter() {
                            if type1 == type2 { matches = true; break }
                        }
                        if !matches { return false }
                    }
                    true
                }
                Self::Any => true,
                _ => {
                    for t in t1.iter() {
                        if t == other { return false }
                    }
                    true
                }
            }
            _ => false
        }
    }
}