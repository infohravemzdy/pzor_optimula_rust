use legalios::service::period::IPeriod;
use procezor::registry_factories::concept_factory::{ConceptSpecFactory, IConceptSpecFactory};
use procezor::registry_providers::concept_provider::{BoxConceptSpec, BoxConceptSpecProvider};
use procezor::service_types::concept_code::ConceptCode;
use procezor::service_types::version_code::VersionCode;

pub(crate) struct OptimulaConceptFactory {
    factory: ConceptSpecFactory,
}

#[allow(dead_code)]
impl OptimulaConceptFactory {
    pub(crate) fn new() -> OptimulaConceptFactory {
        OptimulaConceptFactory {
            factory: ConceptSpecFactory::new(Self::factory_builder),
        }
    }
    fn factory_builder() -> Vec<BoxConceptSpecProvider> {
        vec![
            Box::new(TimeshtWorkingConProv::new()),
        ]
    }
}

impl IConceptSpecFactory for OptimulaConceptFactory {
    fn get_spec(&self, code: &ConceptCode, period: &dyn IPeriod, version: &VersionCode) -> BoxConceptSpec {
        self.factory.get_spec(code, period, version)
    }

    fn get_spec_list(&self, period: &dyn IPeriod, version: &VersionCode) -> Vec<BoxConceptSpec> {
        self.factory.get_spec_list(period, version)
    }
}


