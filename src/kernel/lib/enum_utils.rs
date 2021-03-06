#[macro_export]
macro_rules! enum_str {
    (enum $name:ident { $($variant:ident = $val:expr),*, }) => {
        pub enum $name {
            $($variant = $val),*
        }

        impl $name {
            pub const fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}