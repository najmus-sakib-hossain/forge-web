warning: both package.include and package.exclude are specified; the exclude list will be ignored
   Packaging dx-forge v0.1.1 (F:\Code\forge)
    Updating crates.io index
    Packaged 325 files, 1.8MiB (500.4KiB compressed)
   Verifying dx-forge v0.1.1 (F:\Code\forge)
   Compiling dx-forge v0.1.1 (F:\Code\forge\target\package\dx-forge-0.1.1)
warning: creating a shared reference to mutable static
   --> src\api\lifecycle.rs:111:33
    |
111 |         if let Some(registry) = &TOOL_REGISTRY {
    |                                 ^^^^^^^^^^^^^^ shared reference to mutable static
    |
    = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
    = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference is created for it while the shared reference lives
    = note: `#[warn(static_mut_refs)]` (part of `#[warn(rust_2024_compatibility)]`) on by default
help: use `&raw const` instead to create a raw pointer
    |
111 |         if let Some(registry) = &raw const TOOL_REGISTRY {
    |                                  +++++++++

warning: creating a shared reference to mutable static
   --> src\api\lifecycle.rs:142:32
    |
142 |         if let Some(context) = &CURRENT_CONTEXT {
    |                                ^^^^^^^^^^^^^^^^ shared reference to mutable static
    |
    = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
    = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference is created for it while the shared reference lives
help: use `&raw const` instead to create a raw pointer
    |
142 |         if let Some(context) = &raw const CURRENT_CONTEXT {
    |                                 +++++++++

warning: creating a mutable reference to mutable static
   --> src\api\lifecycle.rs:171:33
    |
171 |         if let Some(registry) = TOOL_REGISTRY.take() {
    |                                 ^^^^^^^^^^^^^^^^^^^^ mutable reference to mutable static
    |
    = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
    = note: mutable references to mutable statics are dangerous; it's undefined behavior if any other pointer to the static is used or if 
any other reference is created for the static while the mutable reference lives

warning: creating a mutable reference to mutable static
   --> src\api\lifecycle.rs:178:30
    |
178 |         if let Some(forge) = FORGE_INSTANCE.take() {
    |                              ^^^^^^^^^^^^^^^^^^^^^ mutable reference to mutable static
    |
    = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
    = note: mutable references to mutable statics are dangerous; it's undefined behavior if any other pointer to the static is used or if 
any other reference is created for the static while the mutable reference lives

warning: creating a shared reference to mutable static
   --> src\api\lifecycle.rs:195:12
    |
195 |         if FORGE_INSTANCE.is_none() {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^ shared reference to mutable static
    |
    = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
    = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference is created for it while the shared reference lives

warning: creating a shared reference to mutable static
  --> src\api\pipeline.rs:30:12
   |
30 |         if PIPELINE_STATE.is_none() {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^ shared reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference 
is created for it while the shared reference lives

warning: creating a shared reference to mutable static
  --> src\api\pipeline.rs:33:9
   |
33 |         PIPELINE_STATE.as_ref().unwrap().clone()
   |         ^^^^^^^^^^^^^^^^^^^^^^^ shared reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference 
is created for it while the shared reference lives

warning: creating a shared reference to mutable static
  --> src\api\reactivity.rs:28:12
   |
28 |         if REACTIVITY_STATE.is_none() {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^ shared reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference 
is created for it while the shared reference lives

warning: creating a shared reference to mutable static
  --> src\api\reactivity.rs:31:9
   |
31 |         REACTIVITY_STATE.as_ref().unwrap().clone()
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^ shared reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference 
is created for it while the shared reference lives

warning: creating a shared reference to mutable static
  --> src\api\branching.rs:59:12
   |
59 |         if BRANCHING_STATE.is_none() {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^ shared reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference 
is created for it while the shared reference lives

warning: creating a shared reference to mutable static
  --> src\api\branching.rs:62:9
   |
62 |         BRANCHING_STATE.as_ref().unwrap().clone()
   |         ^^^^^^^^^^^^^^^^^^^^^^^^ shared reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference 
is created for it while the shared reference lives

warning: creating a shared reference to mutable static
  --> src\api\events.rs:38:12
   |
38 |         if EVENT_BUS.is_none() {
   |            ^^^^^^^^^^^^^^^^^^^ shared reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference 
is created for it while the shared reference lives

warning: creating a shared reference to mutable static
  --> src\api\events.rs:41:9
   |
41 |         EVENT_BUS.as_ref().unwrap().clone()
   |         ^^^^^^^^^^^^^^^^^^ shared reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference 
is created for it while the shared reference lives

warning: creating a shared reference to mutable static
  --> src\api\cart.rs:22:12
   |
22 |         if CART.is_none() {
   |            ^^^^^^^^^^^^^^ shared reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference 
is created for it while the shared reference lives

warning: creating a shared reference to mutable static
  --> src\api\cart.rs:25:9
   |
25 |         CART.as_ref().unwrap().clone()
   |         ^^^^^^^^^^^^^ shared reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference 
is created for it while the shared reference lives

warning: creating a shared reference to mutable static
  --> src\api\codegen.rs:22:12
   |
22 |         if CODE_REGIONS.is_none() {
   |            ^^^^^^^^^^^^^^^^^^^^^^ shared reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference 
is created for it while the shared reference lives

warning: creating a shared reference to mutable static
  --> src\api\codegen.rs:25:9
   |
25 |         CODE_REGIONS.as_ref().unwrap().clone()
   |         ^^^^^^^^^^^^^^^^^^^^^ shared reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference 
is created for it while the shared reference lives

warning: creating a shared reference to mutable static
  --> src\api\codegen.rs:31:12
   |
31 |         if FILE_OWNERSHIP.is_none() {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^ shared reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference 
is created for it while the shared reference lives

warning: creating a shared reference to mutable static
  --> src\api\codegen.rs:34:9
   |
34 |         FILE_OWNERSHIP.as_ref().unwrap().clone()
   |         ^^^^^^^^^^^^^^^^^^^^^^^ shared reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference 
is created for it while the shared reference lives

warning: `dx-forge` (lib) generated 19 warnings