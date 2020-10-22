#![allow(dead_code)]
#![warn(unused_variables)]
#![warn(unused_imports)]

mod basic_type;
mod closure_base;
mod enum_base;
mod error_base;
mod define_error_type;
mod borrow_base;
mod borrow_mut_base;
mod fn_base;
mod generic_base;
mod lifetime_base;
mod macros_base;


mod struct_base;
mod trait_base;
mod trait_override_add_operator;
mod generic_bound;
mod generic_bound_where;
mod generic_impl;


mod std_box_base;
mod std_vector_base;
mod std_string_base;
mod std_option_base;
mod std_result_base;
mod std_hashmap_base;
mod std_hashset_base;

mod std_misc_threads_base;
mod std_misc_channels_base;
mod std_misc_path_file_base;
mod std_misc_process_base;
mod std_misc_io_fs_base;
mod std_misc_env_arg_base;


mod std_misc_ffi_base;

fn main() {
    println!("Hello World!")
}