use legalios::service::period::IPeriod;
use procezor::registry_factories::article_factory::{ArticleSpecFactory, IArticleSpecFactory};
use procezor::registry_providers::article_provider::{ArcArticleSpec, BoxArticleSpecProvider};
use procezor::service_types::article_code::ArticleCode;
use procezor::service_types::version_code::VersionCode;

pub(crate) struct OptimulaArticleFactory {
    factory: ArticleSpecFactory,
}

#[allow(dead_code)]
impl OptimulaArticleFactory {
    pub(crate) fn new() -> OptimulaArticleFactory {
        OptimulaArticleFactory {
            factory: ArticleSpecFactory::new(Self::factory_builder),
        }
    }
    fn factory_builder() -> Vec<BoxArticleSpecProvider> {
        let article_default_sequens: i16 = 0;
        let providers_config = vec![
        ];
        ArticleSpecFactory::build_providers_from_records(providers_config)
    }
}

impl IArticleSpecFactory for OptimulaArticleFactory {
    fn get_spec(&self, code: &ArticleCode, period: &dyn IPeriod, version: &VersionCode) -> ArcArticleSpec {
        self.factory.get_spec(code, period, version)
    }

    fn get_spec_list(&self, period: &dyn IPeriod, version: &VersionCode) -> Vec<ArcArticleSpec> {
        self.factory.get_spec_list(period, version)
    }
}

