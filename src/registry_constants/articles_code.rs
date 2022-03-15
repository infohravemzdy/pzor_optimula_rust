#[derive(Debug)]
pub(crate) enum OptimulaArticleConst {
    ARTICLE_TIMESHEETS_PLAN = 1001,     // Full Timesheets Hours
    ARTICLE_TIMESHEETS_WORK = 1002,     // Work Timesheets Hours
    ARTICLE_TIMEACTUAL_WORK = 1003,     // Work Timeactual Hours
    ARTICLE_MSALARY_TARGETS = 2001,     // Base Salary
    ARTICLE_HSALARY_TARGETS = 2002,     // Base Salary
    ARTICLE_MAWARDS_TARGETS = 2003,     // Personal  Salary - Targets
    ARTICLE_HAWARDS_TARGETS = 2004,     // Personal  Salary - Targets
    ARTICLE_PREMIUM_TARGETS = 2005,     // Premium Bonus    - Targets
    ARTICLE_PREMADV_TARGETS = 2006,     // Premium Boss     - Targets
    ARTICLE_PREMEXT_TARGETS = 2007,     // Premium Personal - Targets
    ARTICLE_AGRWORK_TARGETS = 2010,     // Agreement Work Tariff - Targets
    ARTICLE_AGRTASK_TARGETS = 2011,     // Agreement Task Tariff - Targets
    ARTICLE_OFFWORK_TARGETS = 2012,     // Agreement Work Tariff - Targets Plus
    ARTICLE_OFFTASK_TARGETS = 2013,     // Agreement Task Tariff - Targets Plus
    ARTICLE_SETTLEM_TARGETS = 3001,     // Setlement - Targets
    ARTICLE_SETTLEM_TARNETT = 3002,     // Setlement - Targets
    ARTICLE_SETTLEM_AGRWORK = 3003,     // Setlement - Agreement Work
    ARTICLE_SETTLEM_AGRTASK = 3004,     // Setlement - Agreement Task
    ARTICLE_SETTLEM_ALLOWCE = 3005,     // Setlement - Allowance
    ARTICLE_SETTLEM_ALLNETT = 3006,     // Setlement - Allowance Netto
    ARTICLE_SETTLEM_OFFWORK = 3007,     // Setlement - Agreement Work Plus
    ARTICLE_SETTLEM_OFFTASK = 3008,     // Setlement - Agreement Task Plus
    ARTICLE_SETTLEM_OFFSETS = 3009,     // Setlement - Allowance Plus
    ARTICLE_PREMEXT_RESULTS = 4001,     // Premium Personal - Results
    ARTICLE_PREMADV_RESULTS = 4002,     // Premium Boss     - Results
    ARTICLE_PREMIUM_RESULTS = 4003,     // Premium Bonus    - Results
    ARTICLE_MAWARDS_RESULTS = 4004,     // Personal Award   - Results
    ARTICLE_HAWARDS_RESULTS = 4005,     // Personal Award   - Results
    ARTICLE_SETTLEM_RESULTS = 4011,     // Setlement - Results
    ARTICLE_SETTLEM_RESNETT = 4012,     // Setlement - Results
    ARTICLE_ALLOWCE_HOFFICE = 4021,     // HomeOffice Tariff
    ARTICLE_ALLOWCE_CLOTDAY = 4022,     // Clothing Daily Tarrif
    ARTICLE_ALLOWCE_CLOTHRS = 4023,     // Clothing Hours Tarrif
    ARTICLE_ALLOWCE_MEALDAY = 4024,     // Meal Contribution Tariff
    ARTICLE_OFFSETS_HOFFICE = 4031,     // HomeOffice Tariff
    ARTICLE_OFFSETS_CLOTDAY = 4032,     // Clothing Daily Tarrif
    ARTICLE_OFFSETS_CLOTHRS = 4033,     // Clothing Hours Tarrif
    ARTICLE_OFFSETS_MEALDAY = 4034,     // Meal Contribution Tariff
    ARTICLE_INCOMES_TAXFREE = 5001,     // Incomes Tax Free
    ARTICLE_INCOMES_TAXBASE = 5002,     // Incomes Tax
    ARTICLE_INCOMES_TAXWINS = 5003,     // Incomes Tax and Insurance
    ARTICLE_INCOMES_SUMMARY = 5004,     // Incomes Summary
}

#[allow(dead_code)]
impl OptimulaArticleConst {
    pub(crate) fn name_of_article(item: i32) -> String {
        let article: OptimulaArticleConst = unsafe { std::mem::transmute(item as i32) };
        format!("{:?}", article)
    }
}

