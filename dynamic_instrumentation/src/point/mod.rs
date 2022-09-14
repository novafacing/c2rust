pub mod apply;
pub mod build;
mod cast;
pub mod source;

use c2rust_analysis_rt::mir_loc::{self, EventMetadata};
use rustc_middle::{
    mir::{Body, HasLocalDecls, Local, LocalDecls, Location, Place, Rvalue},
    ty::TyCtxt,
};
use rustc_span::def_id::DefId;
use std::collections::{HashMap, HashSet};

use crate::{arg::InstrumentationArg, hooks::Hooks, util::Convert};

pub use apply::InstrumentationApplier;
pub use cast::cast_ptr_to_usize;

pub struct InstrumentationPoint<'tcx> {
    id: usize,
    pub original_location: Location,
    pub instrumentation_location: Location,
    pub func: DefId,
    pub args: Vec<InstrumentationArg<'tcx>>,
    pub is_cleanup: bool,
    pub after_call: bool,
    pub metadata: EventMetadata,
}

pub struct CheckAddressTakenLocals<'a, 'tcx: 'a> {
    pub address_taken: HashSet<Local>,
    tcx: TyCtxt<'tcx>,
    body: &'a Body<'tcx>,
}

impl<'a, 'tcx: 'a> CheckAddressTakenLocals<'a, 'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, body: &'a Body<'tcx>) -> Self {
        Self {
            address_taken: HashSet::new(),
            tcx,
            body,
        }
    }

    pub fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }
}

pub struct SubAddressTakenLocals<'tcx> {
    pub address_taken: HashSet<Local>,
    pub local_substitute: HashMap<Local, Local>,
    tcx: TyCtxt<'tcx>,
}

impl<'tcx> SubAddressTakenLocals<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, address_taken: HashSet<Local>) -> Self {
        Self {
            address_taken,
            local_substitute: HashMap::new(),
            tcx,
        }
    }

    pub fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }
}

pub struct CollectInstrumentationPoints<'a, 'tcx: 'a> {
    tcx: TyCtxt<'tcx>,
    hooks: Hooks<'tcx>,
    body: &'a Body<'tcx>,
    instrumentation_points: Vec<InstrumentationPoint<'tcx>>,
    assignment: Option<(Place<'tcx>, Rvalue<'tcx>)>,
    addr_taken_local_substitutions: HashMap<Local, Local>,
}

impl<'a, 'tcx: 'a> CollectInstrumentationPoints<'a, 'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, hooks: Hooks<'tcx>, body: &'a Body<'tcx>) -> Self {
        Self {
            tcx,
            hooks,
            body,
            instrumentation_points: Default::default(),
            assignment: Default::default(),
            addr_taken_local_substitutions: Default::default(),
        }
    }

    pub fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }
}

impl<'a, 'tcx: 'a> HasLocalDecls<'tcx> for CollectInstrumentationPoints<'a, 'tcx> {
    fn local_decls(&self) -> &'a LocalDecls<'tcx> {
        self.body.local_decls()
    }
}

impl<'a, 'tcx: 'a> HasLocalDecls<'tcx> for CheckAddressTakenLocals<'a, 'tcx> {
    fn local_decls(&self) -> &'a LocalDecls<'tcx> {
        self.body.local_decls()
    }
}

impl<'a, 'tcx: 'a> CollectInstrumentationPoints<'a, 'tcx> {
    pub fn hooks(&self) -> &Hooks<'tcx> {
        &self.hooks
    }

    pub fn assignment(&self) -> Option<&(Place<'tcx>, Rvalue<'tcx>)> {
        self.assignment.as_ref()
    }

    pub fn with_assignment(
        &mut self,
        assignment: (Place<'tcx>, Rvalue<'tcx>),
        f: impl Fn(&mut Self),
    ) {
        let old_assignment = self.assignment.replace(assignment);
        f(self);
        self.assignment = old_assignment;
    }

    pub fn func_hash(&self) -> mir_loc::DefPathHash {
        self.tcx()
            .def_path_hash(self.body.source.def_id())
            .convert()
    }
}
