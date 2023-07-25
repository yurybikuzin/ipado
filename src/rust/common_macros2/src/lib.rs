#[macro_export]
macro_rules! impl_from_ref {
    ($from:ty => $for:ty, $ident:ident, $($body:tt)*) => {
        impl From<&$from> for $for {
            fn from($ident: &$from) -> Self {
                $($body)*
            }
        }
        impl From<&mut $from> for $for {
            fn from($ident: &mut $from) -> Self {
                $($body)*
            }
        }
    };
    ($from:ty => $for:ty, $ident:ident : $type:ident, $($body:tt)*) => {
        impl From<&$from> for $for {
            fn from($ident: &$from) -> Self {
                $($body)*
            }
        }
        impl From<&mut $from> for $for {
            fn from($ident: &mut $from) -> Self {
                $($body)*
            }
        }
    };
    // for compatibility with impl_try_from
    ($from:ty, $for:ty, $ident:ident, $($body:tt)*) => {
        impl From<&$from> for $for {
            fn from($ident: &$from) -> Self {
                $($body)*
            }
        }
        impl From<&mut $from> for $for {
            fn from($ident: &mut $from) -> Self {
                $($body)*
            }
        }
    };
}

#[macro_export]
macro_rules! impl_from {
    ($from:ty => $for:ty, $ident:ident : $type:ident, $($body:tt)* ) => {
        impl From<$from> for $for {
            fn from($ident: $from) -> Self {
                type $type = $from;
                $($body)*
            }
        }
    };
    ($from:ty => $for:ty, $ident:ident, $($body:tt)* ) => {
        impl From<$from> for $for {
            fn from($ident: $from) -> Self {
                $($body)*
            }
        }
    };
    // for compatibility with impl_try_from
    ($from:ty => $for:ty, $error:ty, $ident:ident, $($body:tt)* ) => {
        impl From<$from> for $for {
            fn from($ident: $from) -> Self {
                let ret: Result<Self> = { $($body)* };
                ret.ok().unwrap()
            }
        }
    };
    // for compatibility with old code
    ($from:ty, $for:ty, $ident:ident, $($body:tt)* ) => {
        impl From<$from> for $for {
            fn from($ident: $from) -> Self {
                $($body)*
            }
        }
    };
}

#[macro_export]
macro_rules! impl_try_from {
    ($from:ty => $for:ty, $error:ty, $ident:ident, $($body:tt)* ) => {
        impl TryFrom<$from> for $for {
            type Error = $error;
            fn try_from($ident: $from) -> Result<Self, Self::Error> {
                $($body)*
            }
        }
    };
}

#[macro_export]
macro_rules! impl_display {
    ($for:ty, $self:ident, $f:ident, $($body:tt)+) => {
        impl std::fmt::Display for $for {
            fn fmt(&$self, $f: &mut std::fmt::Formatter) -> std::fmt::Result {
                $($body)+
            }
        }
    };
    ($for:ty, $self:ident, $fmt:literal, $($args:expr),+) => {
        impl std::fmt::Display for $for {
            fn fmt(&$self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    $fmt,
                    $($args),+
                )
            }
        }
    }
}

pub mod declare_env_settings;
pub mod declare_settings;
pub mod pasitos;
pub mod will_did;

#[macro_export]
macro_rules! get_rwlock_opt {
    ($CACHE:expr, $do_what:expr, $($body:tt)*) => {{
        let val = get_rwlock_opt!(get => $CACHE);
        let val = if let Some(val) = val {
            val
        } else {
            will_did!(trace => $do_what, {
                let start = std::time::Instant::now();
                let val = {
                    $($body)*
                };
                get_rwlock_opt!(set => $CACHE, val);
                val
            })
        };
        val
    }};
    (get => $CACHE:expr) => {
        if let Some(val) = (*$CACHE.read().unwrap()).as_ref() {
            Some(val.clone())
        } else {
            None
        }
    };
    (set => $CACHE:expr, $val:expr) => {
        *$CACHE.write().unwrap() = Some($val.clone());
    };
}

#[macro_export]
macro_rules! pg {
    (pool => $POOLS:expr, $url:expr, $max_connections:expr, $for:expr) => {{
        let url: &str = $url.as_ref();
        let pool = if let Some(pool) = (*$POOLS.read().unwrap()).get(url) {
            Some(pool.clone())
        } else {
            None
        };
        let pool = if let Some(pool) = pool {
            pool
        } else {
            will_did!(trace => format!("get pool of {} for {}", url, $for), {
                let pool = sqlx::postgres::PgPoolOptions::new()
                    .max_connections($max_connections)
                    .connect(url)
                    .await?;
                (*$POOLS.write().unwrap()).insert(url.to_owned(), pool.clone());
                pool
            })
        };
        pool
    }};
}

// #[macro_export]
// macro_rules! mysql {
//     (pool => $POOLS:expr, $url:expr, $max_connections:expr, $for:expr) => {{
//         let url: &str = $url.as_ref();
//         let pool = if let Some(pool) = (*$POOLS.read().unwrap()).get(url) {
//             Some(pool.clone())
//         } else {
//             None
//         };
//         let pool = if let Some(pool) = pool {
//             pool
//         } else {
//             will_did!(trace => format!("get pool of {} for {}", url, $for), {
//                 let pool = sqlx::mysql::MySqlPoolOptions::new()
//                     .max_connections($max_connections)
//                     .connect(url)
//                     .await?;
//                 (*$POOLS.write().unwrap()).insert(url.to_owned(), pool.clone());
//                 pool
//             })
//         };
//         pool
//     }};
// }

#[macro_export]
macro_rules! r#impl {
    (FromStr for $type:ty; strum) => {
        impl std::str::FromStr for $type {
            type Err = anyhow::Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use strum::IntoEnumIterator;
                if let Some(found) = Self::iter().find(|i| {
                    let eta = i.to_string();
                    let mut eta_iter = eta.chars();
                    let mut tst_iter = s.chars();
                    // let mut ret = true;
                    loop {
                        let eta = eta_iter.next();
                        let tst = tst_iter.next();
                        if eta.is_some() & tst.is_some() {
                            let eta = eta.unwrap();
                            let tst = tst.unwrap();
                            if !(eta == tst || match (eta, tst) { 
                                ('С', 'C') => true,
                                (_, _) => false,
                            }) { break false; }
                        } else if eta.is_none() && tst.is_none() {
                            break true;
                        } else {
                            break false;
                        }
                    }
                }) {
                    Ok(found)
                } else {
                    Err(anyhow!(
                        "failed {}::from_str({:?}): valid values: {}",
                        stringify!($type),
                        s,
                        Self::iter()
                            .map(|i| format!("{:?}", i.to_string()))
                            .collect::<Vec<String>>()
                            .join(", ")
                    ))
                }
            }
        }
    };
}

#[macro_export]
macro_rules! entry {
    ($hash_map:expr, $key:expr => 
         and_modify |$e:ident| $occupied:block 
         or_insert $vacant:expr 
    ) => {
        match $hash_map.entry($key) {
            std::collections::hash_map::Entry::Occupied(mut $e) => {
                let $e = $e.get_mut();
                $occupied;
            }
            std::collections::hash_map::Entry::Vacant($e) => {
                #[allow(unreachable_code)]
                $e.insert($vacant);
            }
        }
    };
    ($hash_map:expr, $key:expr => 
         and_modify_entry |$e:ident| $occupied:block 
         or_insert_opt $vacant:expr 
    ) => {
        match $hash_map.entry($key) {
            std::collections::hash_map::Entry::Occupied(mut $e) => {
                $occupied;
            }
            std::collections::hash_map::Entry::Vacant($e) => {
                if let Some(v) = $vacant {
                    $e.insert(v);
                }
            }
        }
    };
    ($hash_map:expr, $key:expr => 
         and_modify_entry |$e:ident| $occupied:block 
         or_insert $vacant:expr 
    ) => {
        match $hash_map.entry($key) {
            std::collections::hash_map::Entry::Occupied(mut $e) => {
                $occupied;
            }
            std::collections::hash_map::Entry::Vacant($e) => {
                #[allow(unreachable_code)]
                $e.insert($vacant);
            }
        }
    };
}

#[macro_export]
macro_rules! plural(
    ($count:expr, 1 $single:literal$(,)? 2 $some:literal$(,)? 5 $many:literal$(,)?) => {
        {
            let count = $count;
            (count % 100 / 10 != 1).then_some(0).and(
                match count % 10 {
                    1 => Some($single),
                    2 | 3 | 4 => Some($some),
                    _ => None,
                }
            ).unwrap_or($many)
        }
    };
    ($count:expr, 1 $single:expr, 2 $some:expr, 5 $many:expr$(,)?) => {
        {
            let count = $count;
            (count % 100 / 10 != 1).then_some(0).and(
                match count % 10 {
                    1 => Some($single),
                    2 | 3 | 4 => Some($some),
                    _ => None,
                }
            ).unwrap_or($many)
        }
    };
);

