mod html;
mod error;

use std::rc::Rc;
use error::Error;
use html::Html;
use js_wasm::dom::body;
use math_util::rational_number::RationalNumber;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use js_wasm::*;
use web_sys::HtmlInputElement;
use web_sys::HtmlTextAreaElement;

pub type Result<T> = std::result::Result<T, Error>;

#[wasm_bindgen(start)]
pub async fn main() -> std::result::Result<(), JsValue> {
    console_error_panic_hook::set_once();
    if let Err(e) = init().await {
        console_error!("{}", e);
    }
    Ok(())
}

async fn init() -> Result<()> {
    let html = Rc::new(Html::new()?);

    let h = html.clone();
    html.form.set_on_submit(move |event| {
        event.prevent_default();
        check(&h).unwrap();
    }).forget();

    Ok(())
}

fn check(html: &Html) -> Result<()> {
    html.score_lab.add_class("hidden")?;

    for el in &body()?.query_all::<HtmlTextAreaElement>("form textarea")? {
        el.remove_class("correct")?;
        el.remove_class("incorrect")?;
    }

    for el in &body()?.query_all::<HtmlInputElement>("form input")? {
        el.remove_class("correct")?;
        el.remove_class("incorrect")?;
    }

    let mut correct_count = 0;

    let mut responses = if let Ok(responses) = parse_responses(&html.responses_inp) {
        if responses.len() < 20 {
            html.responses_inp.add_class("incorrect")?;
            return Ok(());
        }
        responses
    } else {
        html.responses_inp.add_class("incorrect")?;
        return Ok(());
    };

    let ordered = responses.iter().enumerate().all(|(i, num)| i == 0 || *num >= responses[i-1]);
    if ordered {
        correct_count += 1;
        html.responses_inp.add_class("correct")?;
    } else {
        html.responses_inp.add_class("warning")?;
    }

    responses.sort();

    let avg = avg(&responses);
    let min = responses[0];
    let max = responses[responses.len() - 1];
    let range = max - min;
    let q1 = median(&responses[..responses.len()/2]);
    let q2 = median(&responses);
    let q3 = if responses.len() % 2 == 0 {
        median(&responses[responses.len()/2..])
    } else {
        median(&responses[responses.len()/2 + 1..])
    };
    let modes = mode(&responses);

    if check_value(&html.avg_inp, avg) {
        correct_count += 1;
    }
    if check_value(&html.range_inp, range) {
        correct_count += 1;
    }
    if check_value(&html.min_inp, min) {
        correct_count += 1;
    }
    if check_value(&html.q1_inp, q1) {
        correct_count += 1;
    }
    if check_value(&html.median_inp, q2) {
        correct_count += 1;
    }
    if check_value(&html.q3_inp, q3) {
        correct_count += 1;
    }
    if check_value(&html.max_inp, max) {
        correct_count += 1;
    }
    if check_modes(&html.mode_inp, &modes) {
        correct_count += 1;
    }

    let percent = (((correct_count as f32)/(9f32))*100f32).round() as u32;
    let score_text = format!("Score: {}/9 ({}%)", correct_count, percent);
    html.score_lab.set_text_content(Some(&score_text));
    html.score_lab.remove_class("hidden")?;

    Ok(())
}

fn parse_modes(modes_inp: &HtmlInputElement) -> Result<Vec<RationalNumber>> {
    modes_inp.value()
        .split(|c: char| c.is_whitespace() || c == ',' || c == ';')
        .try_fold(Vec::new(), |mut acc, s| {
            if !s.is_empty() {
                let n = RationalNumber::parse(s).map_err(|_e| Error::InvalidResponses)?;
                acc.push(n);
            }
            Ok(acc)
        })
}

fn check_modes(el: &HtmlInputElement, modes: &[RationalNumber]) -> bool {
    let correct = if let Ok(responses) = parse_modes(el) {
        responses.len() == modes.len() &&
        responses.iter().all(|r| modes.iter().any(|m| m == r)) &&
        modes.iter().all(|m| responses.iter().any(|r| r == m))
    } else {
        false
    };

    if correct {
        el.add_class("correct").expect("failed to add correct class");
    } else {
        el.add_class("incorrect").expect("failed to add incorrect class");
    }

    correct
}

fn check_value(el: &HtmlInputElement, num: RationalNumber) -> bool {
    let val = RationalNumber::parse(&el.value().trim());
    let correct = val.is_ok() && val.unwrap() == num;
    if correct {
        el.add_class("correct").expect("failed to add correct class");
    } else {
        el.add_class("incorrect").expect("failed to add incorrect class");
    }
    correct
}

fn parse_responses(responses_inp: &HtmlTextAreaElement) -> Result<Vec<RationalNumber>> {
    responses_inp.value()
        .split(|c: char| c.is_whitespace() || c == ',' || c == ';')
        .try_fold(Vec::new(), |mut acc, s| {
            if !s.is_empty() {
                let n = RationalNumber::parse(s).map_err(|_e| Error::InvalidResponses)?;
                acc.push(n);
            }
            Ok(acc)
        })
}

fn avg(nums: &[RationalNumber]) -> RationalNumber {
    let sum = nums.iter().fold(RationalNumber::from(0u32), |acc, n| {
        acc + *n
    });
    sum / RationalNumber::from(nums.len() as u32)
}

fn median(nums: &[RationalNumber]) -> RationalNumber {
    if nums.len() % 2 == 0 {
        avg(&nums[nums.len()/2 - 1..=nums.len()/2])
    } else {
        nums[nums.len()/2]
    }
}

fn mode(nums: &[RationalNumber]) -> Vec<RationalNumber> {
    let mut v = nums.iter().fold(Vec::new(), |mut acc, n| {
        let entry = if let Some(entry) = acc.iter_mut().find(|(num, _count)| num == n) {
            entry
        } else {
            acc.push((*n, 0));
            acc.last_mut().unwrap()
        };
        entry.1 += 1;
        acc
    });
    v.sort_by(|a, b| b.1.cmp(&a.1));

    if v[0].1 == v[v.len()-1].1 { // all numbers are equally frequent
        return Vec::new();
    }

    let max_count = v[0].1;
    

    v.into_iter().filter_map(|(num, count)| {
        if count == max_count {
            Some(num)
        } else {
            None
        }
    }).collect()
}