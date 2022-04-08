use anyhow::Result;
use essentials::app::Context;

use crate::io::VarInt;

pub mod incoming;

pub trait Packet {
    fn handle(context: &Context) -> Result<()>;
}

macro_rules! user_type {
    (VarInt) => {
        i32
    };
    ($typ:ty) => {
        $typ
    };
}

macro_rules! packet_enum {
    (
        $ident:ident {
            $($id:literal = $packet:ident),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub enum $ident {
            $(
                $packet($packet),
            )*
        }

        impl $ident {
            /// Returns the packet ID of this packet.
            pub fn id(&self) -> u32 {
                match self {
                    $(
                        $ident::$packet(_) => $id,
                    )*
                }
            }
        }

        impl crate::io::Readable for $ident {
            fn read(buffer: &mut ::std::io::Cursor<&[u8]>) -> anyhow::Result<Self>
            where
                Self: Sized
            {
                let packet_id = VarInt::read(buffer)?.0;
                match packet_id {
                    $(
                        id if id == $id => Ok($ident::$packet($packet::read(buffer)?)),
                    )*
                    _ => Err(anyhow::anyhow!("unknown packet ID {}", packet_id)),
                }
            }
        }
    };
}

macro_rules! packet {
    (
        $packet:ident {
            $(
                $field:ident $typ:ident $(<$generics:ident>)?
            );* $(;)?
        }, $context:ident -> $($body:tt)*
    ) => {
        #[derive(Debug, Clone)]
        pub struct $packet {
            $(
                pub $field: user_type!($typ $(<$generics>)?),
            )*
        }

        #[allow(unused_imports, unused_variables)]
        impl crate::io::Readable for $packet {
            fn read(buffer: &mut ::std::io::Cursor<&[u8]>) -> anyhow::Result<Self>
            where
                Self: Sized
            {
                use anyhow::Context as _;
                $(
                    let $field = <$typ $(<$generics>)?>::read(buffer)
                        .context(concat!("failed to read field `", stringify!($field), "` of packet `", stringify!($packet), "`"))?
                        .into();
                )*

                Ok(Self {
                    $(
                        $field,
                    )*
                })
            }
        }

        #[allow(unused_imports, unused_variables)]
        impl Packet for $packet {
            fn handle(context: &Context) -> anyhow::Result<()> {
                let $context = context;

                $($body)*
            }
        }
    };
}

pub(crate) use packet;
pub(crate) use packet_enum;
pub(crate) use user_type;
