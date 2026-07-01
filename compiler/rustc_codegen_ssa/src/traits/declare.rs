use rustc_hir::attrs::Linkage;
use rustc_hir::def_id::DefId;
use rustc_middle::mono::Visibility;
use rustc_middle::ty::Instance;

pub trait PreDefineCodegenMethods<'tcx> {
    fn predefine_static(
        &mut self,
        def_id: DefId,
        linkage: Linkage,
        visibility: Visibility,
        symbol_name: &str,
    );
    fn predefine_fn(
        &mut self,
        instance: Instance<'tcx>,
        linkage: Linkage,
        visibility: Visibility,
        symbol_name: &str,
    );
    /// Predefine the global for a reified `#[link_section]` const monomorphization.
    ///
    /// See the `monomorphized_link_section` feature.
    fn predefine_reified_const(
        &mut self,
        instance: Instance<'tcx>,
        linkage: Linkage,
        visibility: Visibility,
        symbol_name: &str,
    );
    /// Codegen the initializer for a reified `#[link_section]` const monomorphization.
    ///
    /// See the `monomorphized_link_section` feature.
    fn codegen_reified_const(&mut self, instance: Instance<'tcx>);
}
