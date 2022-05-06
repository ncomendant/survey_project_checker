use js_wasm::dom::greed;
use web_sys::{HtmlInputElement, HtmlFormElement, HtmlTextAreaElement, HtmlElement};
use crate::Result;
pub struct Html {
    pub form: HtmlFormElement,
    pub responses_inp: HtmlTextAreaElement,
    pub avg_inp: HtmlInputElement,
    pub median_inp: HtmlInputElement,
    pub mode_inp: HtmlInputElement,
    pub range_inp: HtmlInputElement,
    pub min_inp: HtmlInputElement,
    pub max_inp: HtmlInputElement,
    pub q1_inp: HtmlInputElement,
    pub q3_inp: HtmlInputElement,
    pub score_lab: HtmlElement,
}

impl Html {
    pub fn new() -> Result<Self> {
        Ok(Html {
            form: greed("form")?,
            score_lab: greed("#scoreLab")?,
            responses_inp: greed("form .responsesInp")?,
            avg_inp: greed("form .avgInp")?,
            median_inp: greed("form .medianInp")?,
            mode_inp: greed("form .modeInp")?,
            range_inp: greed("form .rangeInp")?,
            min_inp: greed("form .minInp")?,
            max_inp: greed("form .maxInp")?,
            q1_inp: greed("form .q1Inp")?,
            q3_inp: greed("form .q3Inp")?,
        })
    }
}