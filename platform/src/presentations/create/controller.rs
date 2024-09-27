#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    authorize_type: Signal<u64>, //0: 개인, 1: 법인
    step: Signal<u64>,
    company: Signal<String>,
    business_number: Signal<String>,

    extend_first: Signal<bool>,
    extend_second: Signal<bool>,
    extend_third: Signal<bool>,
    click_first: Signal<bool>,
    click_second: Signal<bool>,
    click_third: Signal<bool>,

    email_address: Signal<String>,
    authentication_number: Signal<String>,
    name: Signal<String>,
    cellphone_number: Signal<String>,
    simple_address: Signal<String>,
    detail_address: Signal<String>,
    // click_send_authentication: Signal<bool>,
    // click_search_address: Signal<bool>,
    // click_complete_join_membership: Signal<bool>,
}

impl Controller {
    pub fn init() -> Self {
        let ctrl = Self {
            authorize_type: use_signal(|| 0),
            step: use_signal(|| 0),
            company: use_signal(|| "".to_string()),
            business_number: use_signal(|| "".to_string()),

            extend_first: use_signal(|| false),
            extend_second: use_signal(|| false),
            extend_third: use_signal(|| false),
            click_first: use_signal(|| false),
            click_second: use_signal(|| false),
            click_third: use_signal(|| false),

            email_address: use_signal(|| "".to_string()),
            authentication_number: use_signal(|| "".to_string()),
            name: use_signal(|| "".to_string()),
            cellphone_number: use_signal(|| "".to_string()),
            simple_address: use_signal(|| "".to_string()),
            detail_address: use_signal(|| "".to_string()),
            // click_send_authentication: use_signal(|| false),
            // click_search_address: use_signal(|| false),
            // click_complete_join_membership: use_signal(|| false),
        };

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn get_authorize_type(&self) -> u64 {
        (self.authorize_type)()
    }

    pub fn get_step(&self) -> u64 {
        (self.step)()
    }

    pub fn get_company(&self) -> String {
        (self.company)()
    }

    pub fn get_business_number(&self) -> String {
        (self.business_number)()
    }

    pub fn get_extend_first(&self) -> bool {
        (self.extend_first)()
    }

    pub fn get_extend_second(&self) -> bool {
        (self.extend_second)()
    }

    pub fn get_extend_third(&self) -> bool {
        (self.extend_third)()
    }

    pub fn get_click_first(&self) -> bool {
        (self.click_first)()
    }

    pub fn get_click_second(&self) -> bool {
        (self.click_second)()
    }

    pub fn get_click_third(&self) -> bool {
        (self.click_third)()
    }

    pub fn get_email_address(&self) -> String {
        (self.email_address)()
    }

    pub fn get_authentication_number(&self) -> String {
        (self.authentication_number)()
    }

    pub fn get_name(&self) -> String {
        (self.name)()
    }

    pub fn get_cellphone_number(&self) -> String {
        (self.cellphone_number)()
    }

    pub fn get_simple_address(&self) -> String {
        (self.simple_address)()
    }

    pub fn get_detail_address(&self) -> String {
        (self.detail_address)()
    }

    // pub fn get_click_send_authentication(&self) -> bool {
    //     (self.click_send_authentication)()
    // }

    // pub fn get_click_search_address(&self) -> bool {
    //     (self.click_search_address)()
    // }

    // pub fn get_click_complete_join_membership(&self) -> bool {
    //     (self.click_complete_join_membership)()
    // }

    pub fn set_authorize_type(&mut self, authorize_type: u64) {
        self.authorize_type.set(authorize_type);
    }

    pub fn set_step(&mut self, step: u64) {
        self.step.set(step);
    }

    pub fn set_company(&mut self, company: String) {
        self.company.set(company);
    }

    pub fn set_business_number(&mut self, business_number: String) {
        self.business_number.set(business_number);
    }

    pub fn set_extend_first_terms(&mut self, extend: bool) {
        self.extend_first.set(extend);
    }

    pub fn set_extend_second_terms(&mut self, extend: bool) {
        self.extend_second.set(extend);
    }

    pub fn set_extend_third_terms(&mut self, extend: bool) {
        self.extend_third.set(extend);
    }

    pub fn set_click_first_terms(&mut self, clicked: bool) {
        self.click_first.set(clicked);
        self.check_and_update_terms_agreement();
    }

    pub fn set_click_second_terms(&mut self, clicked: bool) {
        self.click_second.set(clicked);
        self.check_and_update_terms_agreement();
    }

    pub fn set_click_third_terms(&mut self, clicked: bool) {
        self.click_third.set(clicked);
        self.check_and_update_terms_agreement();
    }

    pub fn set_email_address(&mut self, email: String) {
        self.email_address.set(email);
    }

    pub fn set_authentication_number(&mut self, authentication_number: String) {
        self.authentication_number.set(authentication_number);
    }

    pub fn set_name(&mut self, name: String) {
        self.name.set(name);
    }

    pub fn set_cellphone_number(&mut self, cellphone_number: String) {
        self.cellphone_number.set(cellphone_number);
    }

    pub fn set_simple_address(&mut self, simple_address: String) {
        self.simple_address.set(simple_address);
    }

    pub fn set_detail_address(&mut self, detail_address: String) {
        self.detail_address.set(detail_address);
    }

    pub fn set_click_send_authentication(&mut self) {
        tracing::info!("send authentication button clicked");
    }

    pub fn set_click_search_address(&mut self) {
        tracing::info!("search address button clicked");
    }

    pub fn set_click_complete_join_membership(&mut self) {
        tracing::info!("complete join membership button clicked");
    }

    fn check_and_update_terms_agreement(&mut self) {
        if self.get_click_first() && self.get_click_second() && self.get_click_third() {
            self.set_step(2);
            self.click_first.set(false);
            self.click_second.set(false);
            self.click_third.set(false);
        }
    }
}
