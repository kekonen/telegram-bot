use crate::requests::*;
use crate::types::*;

#[derive(Serialize, Debug)]
pub struct AnswerInlineQuery {
    inline_query_id: InlineQueryId,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    results: Vec<InlineQueryResult>,

    is_personal: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    switch_pm_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    switch_pm_parameter: Option<String>,
    
    // TODO: Rest of the fields
}

impl Request for AnswerInlineQuery {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("answerInlineQuery"), self)
    }
}

pub trait CanAnswerInlineQuery {
    fn answer(self, results: Vec<InlineQueryResult>, switch_pm_text: Option<String>, switch_pm_parameter: Option<String>) -> AnswerInlineQuery;
}

impl<T> CanAnswerInlineQuery for T
where
    T: Into<InlineQueryId>,
{
    fn answer(self, results: Vec<InlineQueryResult>, switch_pm_text: Option<String>, switch_pm_parameter: Option<String>) -> AnswerInlineQuery {
        AnswerInlineQuery::new(self.into(), results, switch_pm_text, switch_pm_parameter)
    }
}

impl AnswerInlineQuery {
    pub fn new(
        inline_query_id: InlineQueryId,
        results: Vec<InlineQueryResult>,
        switch_pm_text: Option<String>,
        switch_pm_parameter: Option<String>,
    ) -> AnswerInlineQuery {
        AnswerInlineQuery {
            inline_query_id,
            results,
            is_personal: true,
            switch_pm_text,
            switch_pm_parameter,
        }
    }

    pub fn add_inline_result<T: Into<InlineQueryResult>>(&mut self, result: T) {
        self.results.push(result.into());
    }
}
