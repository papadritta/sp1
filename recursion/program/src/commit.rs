use p3_commit::{LagrangeSelectors, PolynomialSpace};
use sp1_recursion_compiler::ir::{Array, Builder, Config, Ext, FromConstant, Usize};

use crate::{fri::TwoAdicPcsRoundVariable, types::FriConfigVariable};

pub trait PolynomialSpaceVariable<C: Config>: Sized + FromConstant<C> {
    type Constant: PolynomialSpace<Val = C::F>;

    // fn from_constant(builder: &mut Builder<C>, constant: Self::Constant) -> Self;

    fn next_point(&self, builder: &mut Builder<C>, point: Ext<C::F, C::EF>) -> Ext<C::F, C::EF>;

    fn selectors_at_point(
        &self,
        builder: &mut Builder<C>,
        point: Ext<C::F, C::EF>,
    ) -> LagrangeSelectors<Ext<C::F, C::EF>>;

    fn zp_at_point(&self, builder: &mut Builder<C>, point: Ext<C::F, C::EF>) -> Ext<C::F, C::EF>;

    fn split_domains(&self, builder: &mut Builder<C>, log_num_chunks: usize) -> Vec<Self>;

    fn create_disjoint_domain(
        &self,
        builder: &mut Builder<C>,
        log_degree: Usize<C::N>,
        config: &FriConfigVariable<C>,
    ) -> Self;
}

pub trait PcsVariable<C: Config, Challenger> {
    type Domain: PolynomialSpaceVariable<C>;

    type Commitment;

    type Proof;

    fn natural_domain_for_log_degree(
        &self,
        builder: &mut Builder<C>,
        log_degree: Usize<C::N>,
    ) -> Self::Domain;

    // Todo: change TwoAdicPcsRoundVariable to RoundVariable
    fn verify(
        &self,
        builder: &mut Builder<C>,
        rounds: Array<C, TwoAdicPcsRoundVariable<C>>,
        proof: Self::Proof,
        challenger: &mut Challenger,
    );
}
