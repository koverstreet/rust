// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_camel_case_types, non_upper_case_globals)]
#![allow(unused_must_use)] // everything is just a MemWriter, can't fail

pub use self::astencode_tag::*;

// RBML enum definitions and utils shared by the encoder and decoder
//
// 0x00..0x1f: reserved for RBML generic type tags
// 0x20..0xef: free for use, preferred for frequent tags
// 0xf0..0xff: internally used by RBML to encode 0x100..0xfff in two bytes
// 0x100..0xfff: free for use, preferred for infrequent tags

pub const tag_items: usize = 0x100; // top-level only

pub const tag_paths_data_name: usize = 0x20;

pub const tag_def_id: usize = 0x21;

pub const tag_items_data: usize = 0x22;

pub const tag_items_data_item: usize = 0x23;

pub const tag_items_data_item_family: usize = 0x24;

pub const tag_items_data_item_type: usize = 0x25;

pub const tag_items_data_item_symbol: usize = 0x26;

pub const tag_items_data_item_variant: usize = 0x27;

pub const tag_items_data_parent_item: usize = 0x28;

pub const tag_items_data_item_is_tuple_struct_ctor: usize = 0x29;

pub const tag_items_closure_kind: usize = 0x2a;
pub const tag_items_closure_ty: usize = 0x2b;
pub const tag_def_key: usize = 0x2c;

// GAP 0x2d 0x2e

pub const tag_index: usize = 0x110; // top-level only
pub const tag_xref_index: usize = 0x111; // top-level only
pub const tag_xref_data: usize = 0x112; // top-level only

pub const tag_meta_item_name_value: usize = 0x2f;

pub const tag_meta_item_name: usize = 0x30;

pub const tag_meta_item_value: usize = 0x31;

pub const tag_attributes: usize = 0x101; // top-level only

pub const tag_attribute: usize = 0x32;

pub const tag_meta_item_word: usize = 0x33;

pub const tag_meta_item_list: usize = 0x34;

// The list of crates that this crate depends on
pub const tag_crate_deps: usize = 0x102; // top-level only

// A single crate dependency
pub const tag_crate_dep: usize = 0x35;

pub const tag_crate_hash: usize = 0x103; // top-level only
pub const tag_crate_crate_name: usize = 0x104; // top-level only

pub const tag_crate_dep_crate_name: usize = 0x36;
pub const tag_crate_dep_hash: usize = 0x37;
pub const tag_crate_dep_explicitly_linked: usize = 0x38; // top-level only

pub const tag_item_trait_item: usize = 0x3a;

pub const tag_item_trait_ref: usize = 0x3b;

// discriminator value for variants
pub const tag_disr_val: usize = 0x3c;

// used to encode ast_map::PathElem
pub const tag_path: usize = 0x3d;
pub const tag_path_len: usize = 0x3e;
pub const tag_path_elem_mod: usize = 0x3f;
pub const tag_path_elem_name: usize = 0x40;
pub const tag_item_field: usize = 0x41;

pub const tag_item_variances: usize = 0x43;
/*
  trait items contain tag_item_trait_item elements,
  impl items contain tag_item_impl_item elements, and classes
  have both. That's because some code treats classes like traits,
  and other code treats them like impls. Because classes can contain
  both, tag_item_trait_item and tag_item_impl_item have to be two
  different tags.
 */
pub const tag_item_impl_item: usize = 0x44;
pub const tag_item_trait_method_explicit_self: usize = 0x45;


// Reexports are found within module tags. Each reexport contains def_ids
// and names.
pub const tag_items_data_item_reexport: usize = 0x46;
pub const tag_items_data_item_reexport_def_id: usize = 0x47;
pub const tag_items_data_item_reexport_name: usize = 0x48;

// used to encode crate_ctxt side tables
enum_from_u32! {
    #[derive(Copy, Clone, PartialEq)]
    #[repr(usize)]
    pub enum astencode_tag { // Reserves 0x50 -- 0x6f
        tag_ast = 0x50,

        tag_tree = 0x51,

        tag_mir = 0x52,

        tag_table = 0x53,
        // GAP 0x54, 0x55
        tag_table_def = 0x56,
        tag_table_node_type = 0x57,
        tag_table_item_subst = 0x58,
        tag_table_freevars = 0x59,
        // GAP 0x5a, 0x5b, 0x5c, 0x5d, 0x5e
        tag_table_method_map = 0x5f,
        // GAP 0x60
        tag_table_adjustments = 0x61,
        // GAP 0x62, 0x63, 0x64, 0x65
        tag_table_upvar_capture_map = 0x66,
        // GAP 0x67, 0x68
        tag_table_const_qualif = 0x69,
        tag_table_cast_kinds = 0x6a,
    }
}

pub const tag_item_trait_item_sort: usize = 0x70;

pub const tag_crate_triple: usize = 0x105; // top-level only

pub const tag_dylib_dependency_formats: usize = 0x106; // top-level only

// Language items are a top-level directory (for speed). Hierarchy:
//
// tag_lang_items
// - tag_lang_items_item
//   - tag_lang_items_item_id: u32
//   - tag_lang_items_item_index: u32

pub const tag_lang_items: usize = 0x107; // top-level only
pub const tag_lang_items_item: usize = 0x73;
pub const tag_lang_items_item_id: usize = 0x74;
pub const tag_lang_items_item_index: usize = 0x75;
pub const tag_lang_items_missing: usize = 0x76;

pub const tag_item_unnamed_field: usize = 0x77;
pub const tag_items_data_item_visibility: usize = 0x78;
pub const tag_items_data_item_inherent_impl: usize = 0x79;
// GAP 0x7a
pub const tag_mod_child: usize = 0x7b;
pub const tag_misc_info: usize = 0x108; // top-level only
pub const tag_misc_info_crate_items: usize = 0x7c;

pub const tag_impls: usize = 0x109; // top-level only
pub const tag_impls_trait: usize = 0x7d;
pub const tag_impls_trait_impl: usize = 0x7e;

// GAP 0x7f, 0x80, 0x81

pub const tag_native_libraries: usize = 0x10a; // top-level only
pub const tag_native_libraries_lib: usize = 0x82;
pub const tag_native_libraries_name: usize = 0x83;
pub const tag_native_libraries_kind: usize = 0x84;

pub const tag_plugin_registrar_fn: usize = 0x10b; // top-level only

pub const tag_method_argument_names: usize = 0x85;
pub const tag_method_argument_name: usize = 0x86;

pub const tag_reachable_ids: usize = 0x10c; // top-level only
pub const tag_reachable_id: usize = 0x87;

pub const tag_items_data_item_stability: usize = 0x88;

pub const tag_items_data_item_repr: usize = 0x89;

pub const tag_struct_fields: usize = 0x10d; // top-level only
pub const tag_struct_field: usize = 0x8a;

pub const tag_items_data_item_struct_ctor: usize = 0x8b;
pub const tag_attribute_is_sugared_doc: usize = 0x8c;
// GAP 0x8d
pub const tag_items_data_region: usize = 0x8e;

pub const tag_region_param_def: usize = 0x8f;
pub const tag_region_param_def_ident: usize = 0x90;
pub const tag_region_param_def_def_id: usize = 0x91;
pub const tag_region_param_def_space: usize = 0x92;
pub const tag_region_param_def_index: usize = 0x93;

pub const tag_type_param_def: usize = 0x94;

pub const tag_item_generics: usize = 0x95;
pub const tag_method_ty_generics: usize = 0x96;

pub const tag_type_predicate: usize = 0x97;
pub const tag_self_predicate: usize = 0x98;
pub const tag_fn_predicate: usize = 0x99;

pub const tag_unsafety: usize = 0x9a;

pub const tag_associated_type_names: usize = 0x9b;
pub const tag_associated_type_name: usize = 0x9c;

pub const tag_polarity: usize = 0x9d;

pub const tag_macro_defs: usize = 0x10e; // top-level only
pub const tag_macro_def: usize = 0x9e;
pub const tag_macro_def_body: usize = 0x9f;

pub const tag_paren_sugar: usize = 0xa0;

pub const tag_codemap: usize = 0xa1;
pub const tag_codemap_filemap: usize = 0xa2;

pub const tag_item_super_predicates: usize = 0xa3;

pub const tag_defaulted_trait: usize = 0xa4;

pub const tag_impl_coerce_unsized_kind: usize = 0xa5;

pub const tag_items_data_item_constness: usize = 0xa6;

pub const tag_rustc_version: usize = 0x10f;
pub fn rustc_version() -> String {
    format!(
        "rustc {}",
        option_env!("CFG_VERSION").unwrap_or("unknown version")
    )
}

// Given a field (field: tag => type), returns the type - where the type might
// be a type literal, an array, or an anonymous struct
macro_rules! crate_metadata_field_type {
    ( $field:ident: S $block:tt) => {
        $field
    };

    ( $field:ident: E $block:tt) => {
        $field
    };

    ( $field:ident: A [ $subtag:expr => $subtyp:tt $subtyv:tt ] ) => {
        Vec<crate_metadata_field_type!($field: $subtyp $subtyv)>
    };

    ( $field:ident: V [ $subtag:expr => $subtyp:tt $subtyv:tt ] ) => {
        Vec<crate_metadata_field_type!($field: $subtyp $subtyv)>
    };

    ( $field:ident: O [ $subtyp:tt $subtyv:tt ] ) => {
        Option<crate_metadata_field_type!($field: $subtyp $subtyv)>
    };

    ( $field:ident: P $typ:ty ) => { $typ };
}

// Declare types for the crate metadata:
// Argument is of the form name: tagnr => type - where type might be an array
// or an anonymous (nested) struct
//
// Tricky bit: Since rust doesn't have anonymous structs, when the type
// argument is a nested struct we have to first recurse, declaring types for
// all the children, then declare the outer struct.
macro_rules! crate_metadata_decltypes {
    ( $field:ident: $tag:expr => S {
        $($subfield:ident: $subtag:expr => $subtyp:tt $subtyv:tt,)*
    } ) => {
        $(
            crate_metadata_decltypes!($subfield: $subtag => $subtyp $subtyv);
        )*

        pub struct $field {
            $(
                pub $subfield: crate_metadata_field_type!($subfield: $subtyp $subtyv),
            )*
        }
    };

    ( $field:ident: $tag:expr => E {
        $($subfield:ident: $subtag:expr => $subtyp:tt $subtyv:tt,)*
    } ) => {
        $(
            crate_metadata_decltypes!($subfield: $subtag => $subtyp $subtyv);
        )*

        pub enum $field {
            $(
                $subfield(crate_metadata_field_type!($subfield: $subtyp $subtyv)),
            )*
        }
    };

    ( $field:ident: $tag:expr => A [
      $subtag:expr => $subtyp:tt $subtyv:tt
    ] ) => {
        crate_metadata_decltypes!($field: $tag => $subtyp $subtyv);
    };

    ( $field:ident: $tag:expr => V [
      $subtag:expr => $subtyp:tt $subtyv:tt
    ] ) => {
        crate_metadata_decltypes!($field: $tag => $subtyp $subtyv);
    };

    ( $field:ident: $tag:expr => O [ $subtyp:tt $subtyv:tt
    ] ) => {
        crate_metadata_decltypes!($field: $tag => $subtyp $subtyv);
    };

    ( $field:ident: $tag:expr => P $typ:ty ) => {};
}

macro_rules! crate_metadata_type {
    ($($field:ident: $tag:expr => $typ:tt $tyv:tt,)*) => {
        crate_metadata_decltypes!(CrateMetadata: 0 => S {
            $($field: $tag => $typ $tyv,)*
        });
    };
}

use std::usize;
const NOTAG: usize = usize::MAX;

use rbml;
use rbml::EbmlEncoderTag::*;
use serialize::Encoder;         // emit_u32 etc.

trait MyFancyEncode {
    fn enc(&self, rbml_w: &mut rbml::writer::Encoder);
}

macro_rules! crate_metadata_encode_field {
    ($fieldtype:ident, $field:expr, $rbml_w:ident, $tag:expr => S {
        $($subfield:ident: $subtag:expr => $subtyp:tt $subtyv:tt,)*
    }) => { {
        impl MyFancyEncode for $fieldtype {
            fn enc(&self, rbml_w: &mut rbml::writer::Encoder) {
                if $tag as usize != NOTAG {
                    rbml_w.start_tag($tag as usize);
                }

                $(
                    crate_metadata_encode_field!($subfield,
                                                 &self.$subfield,
                                                 rbml_w,
                                                 $subtag => $subtyp $subtyv);
                )*

                if $tag as usize != NOTAG {
                    rbml_w.end_tag();
                }
            }
        }

        $field.enc($rbml_w);
    } };

    ($fieldtype:ident, $field:expr, $rbml_w:ident, $tag:expr => E {
        $($subfield:ident: $subtag:expr => $subtyp:tt $subtyv:tt,)*
    }) => { {
        impl MyFancyEncode for $fieldtype {
            fn enc(&self, rbml_w: &mut rbml::writer::Encoder) {
                if $tag as usize != NOTAG {
                    rbml_w.start_tag($tag);
                }

                match self { $(
                    &$fieldtype::$subfield(ref v) =>
                        crate_metadata_encode_field!($subfield,
                                            v, rbml_w,
                                            $subtag => $subtyp $subtyv),
                )* }

                if $tag as usize != NOTAG {
                    rbml_w.end_tag();
                }
            }
        }

        $field.enc($rbml_w);
    } };

    ($fieldtype:ident, $field:expr, $rbml_w:ident, $tag:expr => A [
        $subtag:expr => $subtyp:tt $subtyv:tt
    ]) => { {
        if $tag as usize != NOTAG {
            $rbml_w.start_tag($tag as usize);
        }

        for i in $field.iter() {
            crate_metadata_encode_field!($fieldtype, i, $rbml_w,
                                         $subtag => $subtyp $subtyv);
        }

        if $tag as usize != NOTAG {
            $rbml_w.end_tag();
        }
    } };

    ($fieldtype:ident, $field:expr, $rbml_w:ident, $tag:expr => V [
        $subtag:expr => $subtyp:tt $subtyv:tt
    ]) => { {
        // we can't emulate .emit_seq() due to relevant methods being private:
        assert!($tag as usize == EsVec as usize);

        $rbml_w.emit_seq($field.len(), |rbml_w| {
            for i in $field.iter() {
                crate_metadata_encode_field!($fieldtype, i, rbml_w,
                                             $subtag => $subtyp $subtyv);
            }
            Ok(())
        })
    } };

    ($fieldtype:ident, $field:expr, $rbml_w:ident, $tag:expr => O [
        $subtyp:tt $subtyv:tt
    ]) => { {
        match *$field {
            Some(ref v) => {
                crate_metadata_encode_field!($fieldtype, v,
                                             $rbml_w,
                                             $tag => $subtyp $subtyv);
            }
            None    => (),
        }
    } };

    ($fieldtype:ident, $field:expr, $rbml_w:ident, $tag:expr => P str) => {
        $rbml_w.wr_tagged_str($tag as usize, $field);
    };

    ($fieldtype:ident, $field:expr, $rbml_w:ident, $tag:expr => P String) => {
        $rbml_w.wr_tagged_str($tag as usize, &$field[..]);
    };

    ($fieldtype:ident, $field:expr, $rbml_w:ident, $tag:expr => P u64) => {
        if $tag as usize != EsU64 as usize {
            $rbml_w.wr_tagged_u64($tag as usize, *$field);
        } else {
            $rbml_w.emit_u64(*$field);
        }
    };

    ($fieldtype:ident, $field:expr, $rbml_w:ident, $tag:expr => P u32) => {
        if $tag as usize != EsU32 as usize {
            $rbml_w.wr_tagged_u32($tag as usize, *$field);
        } else {
            $rbml_w.emit_u32(*$field);
        }
    };

    ($fieldtype:ident, $field:expr, $rbml_w:ident, $tag:expr => P u16) => {
        if $tag as usize != EsU16 as usize {
            $rbml_w.wr_tagged_u16($tag as usize, *$field);
        } else {
            $rbml_w.emit_u16(*$field);
        }
    };

    ($fieldtype:ident, $field:expr, $rbml_w:ident, $tag:expr => P u8) => {
        if $tag as usize != EsU8 as usize {
            $rbml_w.wr_tagged_u8($tag as usize, *$field);
        } else {
            $rbml_w.emit_u8(*$field);
        }
    };

    ($fieldtype:ident, $field:expr, $rbml_w:ident, $tag:expr => P $typ:ty) => {
        $field.enc($rbml_w);
    };
}

macro_rules! crate_metadata_encoder {
    ($($field:ident: $tag:expr => $typ:tt $tyv:tt,)*) => {
        pub fn encode_crate_metadata(krate: &CrateMetadata,
                                     rbml_w: &mut rbml::writer::Encoder) {
            $(
                crate_metadata_encode_field!($field, &krate.$field, rbml_w,
                                             $tag => $typ $tyv);
            )*
        }
    };
}

macro_rules! crate_metadata_decoder {
    ($($field:ident: $tag:expr => $typ:tt,)*) => {
        pub struct CrateMetadataDecoder<'a> {
            data: &'a [u8]
        }

        impl<'a> CrateMetadataDecoder<'a> {
            $(
                pub fn $field(&self) -> Option<$typ> {
                    rbml::reader::maybe_get_doc(rbml::Doc::new(self.data),
                                                $tag).map(|s| s.as_str())
                }
            )*
        }
    };
}

macro_rules! crate_metadata_schema {
    ($($field:ident: $tag:expr => $typ:tt $tyv:tt,)*) => {
        crate_metadata_type!    { $($field: $tag => $typ $tyv ,)* }
        crate_metadata_encoder! { $($field: $tag => $typ $tyv ,)* }
        //crate_metadata_decoder! { $($field: $tag => $typ $tyv ,)* }
    };
}

/* XXX: use &str instead of String */
/* XXX: instead of taking vecs, take vecs or something to work lazily */
/* XXX: need token pasting */
/* XXX: hide everything (types!) behind a mod */

crate_metadata_schema! {
    rustc_version:              0x10f => P String,
    crate_name:                 0x104 => P String,
    crate_triple:               0x105 => P String,
    crate_hash:                 0x103 => P String,
    dylib_dependency_formats:   0x106 => P String,

    cr_attributes:              0x101 => A [
                                0x32  => S {
        is_sugared_doc:         0x8c  => P u8,
        meta:                   NOTAG => E {
            word:               0x33  => S {
                name:           0x30  => P String,
            },

            name_value:         0x2f  => S {
                name:           0x30  => P String,
                value:          0x31  => P String,
            },

            item_list:          0x34  => S {
                name:           0x30  => P String,
                list:           NOTAG => A [
                                NOTAG => P meta
                ],
            },

            /* XXX: need None */
        },
    }],

    crate_deps:                 0x102 => A [
                                0x35  => S {
        name:                   0x36  => P String,
        hash:                   0x37  => P String,
        explicitly_linked:      0x38  => P u8,
    }],

    cr_lang_items:              0x107 => S {
        items:                  NOTAG => A [
                                0x73  => S {
            id:                 0x74  => P u32,
            index:              0x75  => P u32,
        }],


        items_missing:          NOTAG => A [
                                0x76  => P u32
        ],
    },

    native_libraries:           0x10a => A [
                                0x82  => S {
        kind:                   0x84  => P u32,
        name:                   0x84  => P String,
    }],

    plugin_registrar_fn:        0x10b => O [
                                         P u32
    ],

    cr_codemap:                 0xa1  => A [
                                0xa2  => S {
        name:                   EsStr => P String,
        start_pos:              EsU32 => P u32,
        end_pos:                EsU32 => P u32,
        nr_lines:               EsU32 => P u32,
        lines:                  NOTAG => O [
                                         S {
                /*
                 * this field is pointless - rbml always encodes each integer
                 * as small as possible
                 */
                bytes_per_diff: EsU8  => P u8,
                lines:          NOTAG => A [
                                EsU32 => P u32
                ],
            }
        ],
        /* also, why are line offsets in here at all? they're useless without
         * the original source, but if we've got the original source we can
         * reparse that
         */

        multibyte_chars:        EsVec => V [
                                EsVecElt => S {
            pos:                EsU32 => P u32,
            bytes:              EsU32 => P u32,
        }],
    }],

    macro_defs:                 0x10e => A [
                                0x9e  => S {
        name:                   0x20  => P String,
        attrs:                  0x101 => A [
                                0x32  => P cr_attributes
        ],
        body:                   0x9f  => P String,
    }],

    /*
    impls:                      0x109 => A [
                                0x7d  => S {
            def_id:             0x21  => P u64,
            trait_impls:        NOTAG => A [
                                0x7e  => P u64,
            ],
        },
    ],

    misc_info:                  0x108 => S {
        crate_items:            0x7c  => A [
            mod_child:          0x7b  => P u64,
        ],

        reexports:              NOTAG => A [
                                0x46  => S {
            def_id:             0x47  => P u64,
            name:               0x48  => P String,
        }],
    }

    reachable_symbols:          0x10c => A [
                                0x87  => P u32,
    ],

    items:                      0x100 => S {
        data:                   0x22  => S {
            data_item:          0x23  => S {
                def_id:         0x21  => P u64,
                def_key:        0x2c  => S {
                },
            },
        },
    },
    */
}
