const CORE: &str = "col-12 col-sm-12 col-md-10 col-lg-8 col-xl-6 col-xxl-6";
const HALF: &str = "col-12 col-sm-6";
const HOME: &str = "col-12 col-sm-6 col-lg-4";
const TEXT: &str = "col-12 col-sm-12 col-md-12 col-lg-9 col-xl-8 col-xxl-7";

pub fn class_core(class: &str) -> String {
    class.to_string() + " " + CORE
}

pub fn class_half(class: &str) -> String {
    class.to_string() + " " + HALF
}

pub fn class_home(class: &str) -> String {
    class.to_string() + " " + HOME
}

pub fn class_text(class: &str) -> String {
    class.to_string() + " " + TEXT
}