use ark_ff::Field;
use ark_std::{cell::RefCell, rc::Rc, vec::Vec};

/// A representation of constraint matrices.
pub type Matrix<F> = Vec<Vec<(F, usize)>>;

/// A PLONK Constraint System with optimization including customized gates and
/// higher fan-in
#[derive(Debug, Clone)]
pub struct ConstraintSystem<F: Field> {
    selector: Matrix<F>,
    // TODO: refer to TurboPlonkConstraintSystem implementation
}

/// Computations are expressed in terms of Plonk Constraint System.
/// The `generate_constraints` method is called to generate constraints for
/// both CRS generation and for proving.
pub trait ConstraintSynthesizer<F: Field> {
    /// Drives generation of new constraints inside `cs`.
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> crate::r1cs::Result<()>;
}

/// A shared reference to a constraint system that can be stored in high level
/// variables.
#[derive(Debug, Clone)]
pub enum ConstraintSystemRef<F: Field> {
    /// Represents the case where we *don't* need to allocate variables or
    /// enforce constraints. Encountered when operating over constant
    /// values.
    None,
    /// Represents the case where we *do* allocate variables or enforce
    /// constraints.
    CS(Rc<RefCell<ConstraintSystem<F>>>),
}
