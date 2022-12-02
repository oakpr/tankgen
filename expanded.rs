#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::collections::BTreeMap;
use gdnative::prelude::*;
#[inherit(Object)]
pub struct Level {
    grid: BTreeMap<(isize, isize), Tile>,
    cell_size: (u32, u32),
    cell_amt: (u32, u32),
}
#[automatically_derived]
#[allow(
    nonstandard_style,
    unused,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic
)]
impl ::gdnative::export::NativeClass for Level {
    type Base = Object;
    type UserData = ::gdnative::export::user_data::DefaultUserData<Level>;
    fn nativeclass_init(owner: ::gdnative::object::TRef<Self::Base>) -> Self {
        Self::new(::gdnative::export::OwnerArg::from_safe_ref(owner))
    }
    fn nativeclass_register_properties(
        builder: &::gdnative::export::ClassBuilder<Self>,
    ) {
        {}
    }
}
#[automatically_derived]
#[allow(
    nonstandard_style,
    unused,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic
)]
impl ::gdnative::export::StaticallyNamed for Level {
    const CLASS_NAME: &'static str = "Level";
}
#[automatically_derived]
impl ::core::default::Default for Level {
    #[inline]
    fn default() -> Level {
        Level {
            grid: ::core::default::Default::default(),
            cell_size: ::core::default::Default::default(),
            cell_amt: ::core::default::Default::default(),
        }
    }
}
pub enum Tile {
    Floor,
    FragileWall,
    Wall,
    SuperWall,
    Enemy(Option<String>),
    Item(Option<String>),
}
#[automatically_derived]
#[allow(
    nonstandard_style,
    unused,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic
)]
impl ::gdnative::core_types::FromVariant for Tile {
    fn from_variant(
        __variant: &::gdnative::core_types::Variant,
    ) -> ::std::result::Result<Self, ::gdnative::core_types::FromVariantError> {
        use ::gdnative::core_types::ToVariant;
        use ::gdnative::core_types::FromVariant;
        use ::gdnative::core_types::FromVariantError as FVE;
        use ::gdnative::core_types::VariantEnumRepr;
        use ::gdnative::core_types::VariantStructRepr;
        {
            let __dict = ::gdnative::core_types::Dictionary::from_variant(__variant)
                .map_err(|__err| FVE::InvalidEnumRepr {
                    expected: VariantEnumRepr::ExternallyTagged,
                    error: std::boxed::Box::new(__err),
                })?;
            let __keys = __dict.keys();
            if __keys.len() != 1 {
                Err(FVE::InvalidEnumRepr {
                    expected: VariantEnumRepr::ExternallyTagged,
                    error: std::boxed::Box::new(FVE::InvalidLength {
                        expected: 1,
                        len: __keys.len() as usize,
                    }),
                })
            } else {
                let __key = String::from_variant(&__keys.get(0))
                    .map_err(|__err| FVE::InvalidEnumRepr {
                        expected: VariantEnumRepr::ExternallyTagged,
                        error: std::boxed::Box::new(__err),
                    })?;
                match __key.as_str() {
                    "Floor" => {
                        let __enum_variant = &__dict.get_or_nil(&__keys.get(0));
                        (if __enum_variant.is_nil() {
                            Err(FVE::InvalidStructRepr {
                                expected: VariantStructRepr::Unit,
                                error: Box::new(FVE::InvalidNil),
                            })
                        } else {
                            Ok(Tile::Floor)
                        })
                            .map_err(|err| FVE::InvalidEnumVariant {
                                variant: "Floor",
                                error: std::boxed::Box::new(err),
                            })
                    }
                    "FragileWall" => {
                        let __enum_variant = &__dict.get_or_nil(&__keys.get(0));
                        (if __enum_variant.is_nil() {
                            Err(FVE::InvalidStructRepr {
                                expected: VariantStructRepr::Unit,
                                error: Box::new(FVE::InvalidNil),
                            })
                        } else {
                            Ok(Tile::FragileWall)
                        })
                            .map_err(|err| FVE::InvalidEnumVariant {
                                variant: "FragileWall",
                                error: std::boxed::Box::new(err),
                            })
                    }
                    "Wall" => {
                        let __enum_variant = &__dict.get_or_nil(&__keys.get(0));
                        (if __enum_variant.is_nil() {
                            Err(FVE::InvalidStructRepr {
                                expected: VariantStructRepr::Unit,
                                error: Box::new(FVE::InvalidNil),
                            })
                        } else {
                            Ok(Tile::Wall)
                        })
                            .map_err(|err| FVE::InvalidEnumVariant {
                                variant: "Wall",
                                error: std::boxed::Box::new(err),
                            })
                    }
                    "SuperWall" => {
                        let __enum_variant = &__dict.get_or_nil(&__keys.get(0));
                        (if __enum_variant.is_nil() {
                            Err(FVE::InvalidStructRepr {
                                expected: VariantStructRepr::Unit,
                                error: Box::new(FVE::InvalidNil),
                            })
                        } else {
                            Ok(Tile::SuperWall)
                        })
                            .map_err(|err| FVE::InvalidEnumVariant {
                                variant: "SuperWall",
                                error: std::boxed::Box::new(err),
                            })
                    }
                    "Enemy" => {
                        let __enum_variant = &__dict.get_or_nil(&__keys.get(0));
                        ({
                            ::gdnative::core_types::FromVariant::from_variant(
                                    __enum_variant,
                                )
                                .map(Tile::Enemy)
                        })
                            .map_err(|err| FVE::InvalidEnumVariant {
                                variant: "Enemy",
                                error: std::boxed::Box::new(err),
                            })
                    }
                    "Item" => {
                        let __enum_variant = &__dict.get_or_nil(&__keys.get(0));
                        ({
                            ::gdnative::core_types::FromVariant::from_variant(
                                    __enum_variant,
                                )
                                .map(Tile::Item)
                        })
                            .map_err(|err| FVE::InvalidEnumVariant {
                                variant: "Item",
                                error: std::boxed::Box::new(err),
                            })
                    }
                    variant => {
                        Err(FVE::UnknownEnumVariant {
                            variant: variant.to_string(),
                            expected: &[
                                "Floor",
                                "FragileWall",
                                "Wall",
                                "SuperWall",
                                "Enemy",
                                "Item",
                            ],
                        })
                    }
                }
            }
        }
    }
}
#[automatically_derived]
#[allow(
    nonstandard_style,
    unused,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic
)]
impl ::gdnative::core_types::ToVariant for Tile {
    fn to_variant(&self) -> ::gdnative::core_types::Variant {
        use ::gdnative::core_types::ToVariant;
        use ::gdnative::core_types::FromVariant;
        match &self {
            Tile::Floor => {
                let __dict = ::gdnative::core_types::Dictionary::new();
                let __key = ::gdnative::core_types::ToVariant::to_variant(
                    &::gdnative::core_types::GodotString::from("Floor"),
                );
                let __value = ::gdnative::core_types::Dictionary::new()
                    .into_shared()
                    .to_variant();
                __dict.insert(&__key, &__value);
                ::gdnative::core_types::ToVariant::to_variant(&__dict.into_shared())
            }
            Tile::FragileWall => {
                let __dict = ::gdnative::core_types::Dictionary::new();
                let __key = ::gdnative::core_types::ToVariant::to_variant(
                    &::gdnative::core_types::GodotString::from("FragileWall"),
                );
                let __value = ::gdnative::core_types::Dictionary::new()
                    .into_shared()
                    .to_variant();
                __dict.insert(&__key, &__value);
                ::gdnative::core_types::ToVariant::to_variant(&__dict.into_shared())
            }
            Tile::Wall => {
                let __dict = ::gdnative::core_types::Dictionary::new();
                let __key = ::gdnative::core_types::ToVariant::to_variant(
                    &::gdnative::core_types::GodotString::from("Wall"),
                );
                let __value = ::gdnative::core_types::Dictionary::new()
                    .into_shared()
                    .to_variant();
                __dict.insert(&__key, &__value);
                ::gdnative::core_types::ToVariant::to_variant(&__dict.into_shared())
            }
            Tile::SuperWall => {
                let __dict = ::gdnative::core_types::Dictionary::new();
                let __key = ::gdnative::core_types::ToVariant::to_variant(
                    &::gdnative::core_types::GodotString::from("SuperWall"),
                );
                let __value = ::gdnative::core_types::Dictionary::new()
                    .into_shared()
                    .to_variant();
                __dict.insert(&__key, &__value);
                ::gdnative::core_types::ToVariant::to_variant(&__dict.into_shared())
            }
            Tile::Enemy(__field_0) => {
                let __dict = ::gdnative::core_types::Dictionary::new();
                let __key = ::gdnative::core_types::ToVariant::to_variant(
                    &::gdnative::core_types::GodotString::from("Enemy"),
                );
                let __value = (__field_0).to_variant();
                __dict.insert(&__key, &__value);
                ::gdnative::core_types::ToVariant::to_variant(&__dict.into_shared())
            }
            Tile::Item(__field_0) => {
                let __dict = ::gdnative::core_types::Dictionary::new();
                let __key = ::gdnative::core_types::ToVariant::to_variant(
                    &::gdnative::core_types::GodotString::from("Item"),
                );
                let __value = (__field_0).to_variant();
                __dict.insert(&__key, &__value);
                ::gdnative::core_types::ToVariant::to_variant(&__dict.into_shared())
            }
        }
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Tile {
    #[inline]
    fn clone(&self) -> Tile {
        match self {
            Tile::Floor => Tile::Floor,
            Tile::FragileWall => Tile::FragileWall,
            Tile::Wall => Tile::Wall,
            Tile::SuperWall => Tile::SuperWall,
            Tile::Enemy(__self_0) => Tile::Enemy(::core::clone::Clone::clone(__self_0)),
            Tile::Item(__self_0) => Tile::Item(::core::clone::Clone::clone(__self_0)),
        }
    }
}
impl Level {
    fn new(_base: &Object) -> Self {
        Default::default()
    }
    pub fn set_cell_size(&mut self, size: Vector2) {
        self.cell_size = (size.x.round() as u32, size.y.round() as u32);
    }
    pub fn get_cell_size(&self) -> Vector2 {
        Vector2::new(self.cell_size.0 as f32, self.cell_size.1 as f32)
    }
    pub fn set_cell_amt(&mut self, size: Vector2) {
        self.cell_amt = (size.x.round() as u32, size.y.round() as u32);
    }
    pub fn get_cell_amt(&self) -> Vector2 {
        Vector2::new(self.cell_amt.0 as f32, self.cell_amt.1 as f32)
    }
    pub fn put_tile(&mut self, pos: Vector2, cell: Tile) {
        let p = (pos.x.round() as isize, pos.y.round() as isize);
        self.grid.insert(p, cell);
    }
    pub fn get_tile(&self, pos: Vector2) -> Tile {
        let p = (pos.x.round() as isize, pos.y.round() as isize);
        self.grid.get(&p).cloned().unwrap_or(Tile::Floor)
    }
}
#[automatically_derived]
#[allow(
    nonstandard_style,
    unused,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic
)]
impl gdnative::export::NativeClassMethods for Level {
    fn nativeclass_register(builder: &::gdnative::export::ClassBuilder<Self>) {
        use gdnative::export::*;
    }
}
fn init(handle: InitHandle) {
    handle.add_class::<Level>();
}
fn godot_gdnative_init_empty(_options: &::gdnative_core::init::InitializeInfo) {}
#[no_mangle]
#[doc(hidden)]
#[allow(unused_unsafe)]
pub unsafe extern "C" fn godot_gdnative_init(
    options: *mut ::gdnative_core::sys::godot_gdnative_init_options,
) {
    if !::gdnative_core::private::bind_api(options) {
        return;
    }
    let __result = ::std::panic::catch_unwind(|| {
        let callback_options = ::gdnative_core::init::InitializeInfo::new(options);
        godot_gdnative_init_empty(&callback_options)
    });
    if __result.is_err() {
        {
            ::gdnative_core::log::error(
                {
                    #[allow(unused_unsafe)]
                    let site: ::gdnative_core::log::Site<'static> = unsafe {
                        let file = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                            "src/lib.rs\u{0}".as_bytes(),
                        );
                        let func = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                            b"<unset>\0",
                        );
                        ::gdnative_core::log::Site::new(file, func, 65u32)
                    };
                    site
                },
                ::core::fmt::Arguments::new_v1(
                    &["gdnative-core: gdnative_init callback panicked"],
                    &[],
                ),
            );
        };
    }
}
#[no_mangle]
#[doc(hidden)]
#[allow(unused_unsafe)]
pub unsafe extern "C" fn godot_nativescript_init(
    handle: *mut ::gdnative_core::libc::c_void,
) {
    if !::gdnative_core::private::is_api_bound() {
        return;
    }
    #[cfg(not(feature = "custom-godot"))]
    {
        use ::gdnative_core::core_types::Variant;
        let engine = gdnative::api::Engine::godot_singleton();
        let info = engine.get_version_info();
        if info.get("major").expect("major version") != Variant::new(3)
            || info.get("minor").expect("minor version") != Variant::new(5)
            || info.get("patch").expect("patch version") < Variant::new(1)
        {
            let string = info
                .get("string")
                .expect("version str")
                .to::<String>()
                .expect("version str type");
            {
                ::gdnative_core::log::warn(
                    {
                        #[allow(unused_unsafe)]
                        let site: ::gdnative_core::log::Site<'static> = unsafe {
                            let file = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                                "src/lib.rs\u{0}".as_bytes(),
                            );
                            let func = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                                b"<unset>\0",
                            );
                            ::gdnative_core::log::Site::new(file, func, 65u32)
                        };
                        site
                    },
                    ::core::fmt::Arguments::new_v1(
                        &[
                            "This godot-rust version is only compatible with Godot >= 3.5.1 and < 3.6; detected version ",
                            ".\nGDNative mismatches may lead to subtle bugs, undefined behavior or crashes at runtime.\nApply the \'custom-godot\' feature if you want to use current godot-rust with another Godot engine version.",
                        ],
                        &[::core::fmt::ArgumentV1::new_display(&string)],
                    ),
                );
            };
        }
    }
    let __result = ::std::panic::catch_unwind(|| {
        init(::gdnative_core::init::InitHandle::new(handle));
    });
    if __result.is_err() {
        {
            ::gdnative_core::log::error(
                {
                    #[allow(unused_unsafe)]
                    let site: ::gdnative_core::log::Site<'static> = unsafe {
                        let file = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                            "src/lib.rs\u{0}".as_bytes(),
                        );
                        let func = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                            b"<unset>\0",
                        );
                        ::gdnative_core::log::Site::new(file, func, 65u32)
                    };
                    site
                },
                ::core::fmt::Arguments::new_v1(
                    &["gdnative-core: nativescript_init callback panicked"],
                    &[],
                ),
            );
        };
    }
}
fn godot_gdnative_terminate_empty(_term_info: &::gdnative_core::init::TerminateInfo) {}
#[no_mangle]
#[doc(hidden)]
#[allow(unused_unsafe)]
pub unsafe extern "C" fn godot_gdnative_terminate(
    options: *mut ::gdnative_core::sys::godot_gdnative_terminate_options,
) {
    if !::gdnative_core::private::is_api_bound() {
        return;
    }
    let __result = ::std::panic::catch_unwind(|| {
        let term_info = ::gdnative_core::init::TerminateInfo::new(options);
        godot_gdnative_terminate_empty(&term_info)
    });
    if __result.is_err() {
        {
            ::gdnative_core::log::error(
                {
                    #[allow(unused_unsafe)]
                    let site: ::gdnative_core::log::Site<'static> = unsafe {
                        let file = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                            "src/lib.rs\u{0}".as_bytes(),
                        );
                        let func = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                            b"<unset>\0",
                        );
                        ::gdnative_core::log::Site::new(file, func, 65u32)
                    };
                    site
                },
                ::core::fmt::Arguments::new_v1(
                    &["gdnative-core: nativescript_init callback panicked"],
                    &[],
                ),
            );
        };
    }
    ::gdnative_core::private::cleanup_internal_state();
}
