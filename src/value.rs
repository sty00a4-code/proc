use std::{collections::{HashSet, HashMap}, hash::Hash};
use crate::*;

#[derive(Clone)]
pub enum V {
    Wildcard, Null,
    Int(i64), Float(f64), Bool(bool), String(String),
    Tuple(Vec<V>),
    Vector(Vec<V>, Type), Object(HashMap<String, V>),
    Proc(Vec<(String, Option<Type>)>, Node),
    ForeignProc(Vec<(String, Option<Type>)>, fn(&mut Context) -> Result<V, E>),
    Type(Type)
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
            Self::Tuple(v) => write!(f, "({})", v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")),
            Self::Vector(v, _) => write!(f, "{v:?}"),
            Self::Object(v) => write!(f, "{{ {} }}", v.iter().map(|(k, v)| format!("{k} = {v}")).collect::<Vec<String>>().join(", ")),
            Self::Proc(_, body) => write!(f, "proc:{:?}", body as *const Node),
            Self::ForeignProc(_, func) => write!(f, "foreign-proc:{:?}", func as *const fn(&mut Context) -> Result<V, E>),
            Self::Type(v) => write!(f, "{v}"),
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
            Self::Tuple(v) => write!(f, "({})", v.iter().map(|x| format!("{x:?}")).collect::<Vec<String>>().join(", ")),
            Self::Vector(v, _) => write!(f, "{v:?}"),
            Self::Object(v) => write!(f, "{{ {} }}", v.iter().map(|(k, v)| format!("{k} = {v:?}")).collect::<Vec<String>>().join(", ")),
            Self::Proc(_, body) => write!(f, "proc:{:?}", body as *const Node),
            Self::ForeignProc(_, func) => write!(f, "foreign-proc:{:?}", func as *const fn(&mut Context) -> Result<V, E>),
            Self::Type(v) => write!(f, "{v:?}"),
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
            Self::Tuple(v1) => match other {
                Self::Tuple(v2) => v1 == v2,
                _ => false
            }
            Self::Vector(v1, t1) => match other {
                Self::Vector(v2, t2) => v1 == v2 && t1 == t2,
                _ => false
            }
            Self::Object(v1) => match other {
                Self::Object(v2) => v1 == v2,
                _ => false
            }
            Self::Proc(params1, body1) => match other {
                Self::Proc(params2, body2) => (body1 as *const Node) == (body2 as *const Node),
                _ => false
            }
            Self::ForeignProc(params1, func1) => match other {
                Self::ForeignProc(params2, func2) => (func1 as *const fn(&mut Context) -> Result<V, E>) == (func2 as *const fn(&mut Context) -> Result<V, E>),
                _ => false
            }
            Self::Type(v1) => match other {
                Self::Type(v2) => v1 == v2,
                _ => false
            }
        }
    }
}
impl V {
    pub fn typ(&self) -> Type {
        match self {
            Self::Wildcard => Type::Any,
            Self::Null => Type::Undefiend,
            Self::Int(_) => Type::Int,
            Self::Float(_) => Type::Float,
            Self::Bool(_) => Type::Bool,
            Self::String(_) => Type::String,
            Self::Tuple(v) => Type::Tuple(v.iter().map(|x| x.typ()).collect()),
            Self::Vector(_, t) => t.clone(),
            Self::Object(_) => Type::Object,
            Self::Proc(_, _) => Type::Proc,
            Self::ForeignProc(_, _) => Type::ForeignProc,
            Self::Type(_) => Type::Type,
        }
    }
}

#[derive(Clone)]
pub enum Type {
    Any, Undefiend,
    Int, Float, Bool, String,
    Tuple(Vec<Type>), Vector(Box<Type>), Object,
    Proc, ForeignProc,
    Type,
    Union(Vec<Type>), Scission(Vec<Type>)
}
impl Type {
    pub fn create_union(types: Vec<Self>) -> Self {
        let mut collected: Vec<Self> = vec![];
        for t in types { if !collected.contains(&t) { collected.push(t); } }
        Self::Union(collected)
    }
}
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
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
            Self::Tuple(types) => write!(f, "({})", types.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")),
            Self::Vector(t) => write!(f, "vec"),
            Self::Object => write!(f, "obj"),
            Self::Proc => write!(f, "proc"),
            Self::ForeignProc => write!(f, "foreign-proc"),
            Self::Type => write!(f, "type"),
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
            Self::Tuple(t1) => match other {
                Self::Tuple(t2) => t1 == t2,
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
            Self::Object => match other {
                Self::Object => true,
                Self::Any => true,
                Self::Union(_) => other == self,
                Self::Scission(_) => other == self,
                _ => false
            }
            Self::Proc => match other {
                Self::Proc => true,
                Self::Any => true,
                Self::Union(_) => other == self,
                Self::Scission(_) => other == self,
                _ => false
            }
            Self::ForeignProc => match other {
                Self::ForeignProc => true,
                Self::Any => true,
                Self::Union(_) => other == self,
                Self::Scission(_) => other == self,
                _ => false
            }
            Self::Type => match other {
                Self::Type => true,
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
        }
    }
}