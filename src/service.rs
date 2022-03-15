use std::collections::HashMap;
use legalios::service::bundle_props::IBundleProps;
use legalios::service::period::IPeriod;
use procezor::registry_factories::article_factory::BoxArticleSpecFactory;
use procezor::registry_factories::concept_factory::BoxConceptSpecFactory;
use procezor::registry_providers::article_provider::ArcArticleSpec;
use procezor::registry_providers::concept_provider::BoxConceptSpec;
use procezor::service::service_procezor::{IServiceProcezor, ServiceProcezor};
use procezor::service_types::article_code::ArticleCode;
use procezor::service_types::article_define::ArticleDefine;
use procezor::service_types::article_term::ArticleTerm;
use procezor::service_types::concept_code::ConceptCode;
use procezor::service_types::contract_term::ArcContractTermList;
use procezor::service_types::position_term::ArcPositionTermList;
use procezor::service_types::term_result::ResultArcTermResultList;
use procezor::service_types::term_target::ArcTermTargetList;
use procezor::service_types::version_code::VersionCode;
use crate::registry_constants::articles_code::OptimulaArticleConst;
use crate::registry_factories::articles_factory::OptimulaArticleFactory;
use crate::registry_factories::concepts_factory::OptimulaConceptFactory;

pub(crate) struct OptimulaService {
    service: ServiceProcezor,
}

impl IServiceProcezor for OptimulaService {
    fn get_article_spec(&self, _article: &ArticleCode, _period: &dyn IPeriod, _version: &VersionCode) -> ArcArticleSpec {
        self.service.get_article_spec(_article, _period, _version)
    }

    fn get_concept_spec(&self, _concept: &ConceptCode, _period: &dyn IPeriod, _version: &VersionCode) -> BoxConceptSpec {
        self.service.get_concept_spec(_concept, _period, _version)
    }

    fn init_with_period(&mut self, _period: &dyn IPeriod) -> bool {
        self.service.init_with_period(_period)
    }

    fn get_results(&mut self, _period: &dyn IPeriod, _ruleset: &dyn IBundleProps,
                   targets: &ArcTermTargetList) -> ResultArcTermResultList {
        self.service.get_results(_period, _ruleset, targets)
    }

    fn builder_order(&self) -> &Vec<ArticleTerm> {
        self.service.builder_order()
    }

    fn builder_paths(&self) -> &HashMap<ArticleTerm, Vec<ArticleDefine>> {
        self.service.builder_paths()
    }

    fn get_contract_terms(&self, _period: &dyn IPeriod, _targets: &ArcTermTargetList) -> ArcContractTermList {
        vec![]
    }

    fn get_position_terms(&self, _period: &dyn IPeriod, _contracts: &ArcContractTermList, _targets: &ArcTermTargetList) -> ArcPositionTermList {
        vec![]
    }
}

#[allow(dead_code)]
impl OptimulaService {
    pub(crate) fn new(_version: i32) -> OptimulaService {
        let calc_arts: Vec<ArticleCode> = vec![
            ArticleCode::get(OptimulaArticleConst::ARTICLE_SETTLEM_TARGETS as i32),
            ArticleCode::get(OptimulaArticleConst::ARTICLE_SETTLEM_RESULTS as i32),
            ArticleCode::get(OptimulaArticleConst::ARTICLE_SETTLEM_ALLOWCE as i32),
            ArticleCode::get(OptimulaArticleConst::ARTICLE_SETTLEM_AGRWORK as i32),
            ArticleCode::get(OptimulaArticleConst::ARTICLE_INCOMES_TAXFREE as i32),
            ArticleCode::get(OptimulaArticleConst::ARTICLE_INCOMES_TAXBASE as i32),
            ArticleCode::get(OptimulaArticleConst::ARTICLE_INCOMES_TAXWINS as i32),
            ArticleCode::get(OptimulaArticleConst::ARTICLE_INCOMES_SUMMARY as i32),
        ];
        let service = OptimulaService {
            service: ServiceProcezor::new(
                _version, &calc_arts,
                Self::article_factory_builder,
                Self::concept_factory_builder,
            ),
        };
        service
    }
    fn article_factory_builder() -> BoxArticleSpecFactory {
        Box::new(OptimulaArticleFactory::new())
    }
    fn concept_factory_builder() -> BoxConceptSpecFactory {
        Box::new(OptimulaConceptFactory::new())
    }
}