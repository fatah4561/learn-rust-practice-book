pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize_with_default(&self) -> String {
        String::from("paw paw paw")
    }
    // func that calls other func
    // this has default impl so the impl only need to implement the summarize() func
    fn summarize_with_other(&self) -> String {
        format!("this and that {}", &self.summarize())
    }
}