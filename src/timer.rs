use metrics::histogram;
use std::time::Instant;

pub struct Timer {
    function: &'static str,
    start: Instant,
}

impl Timer {
    pub(crate) fn new(function: &'static str) -> Self {
        let start = std::time::Instant::now();
        Self { function, start }
    }
}

const FUNCTION_LABEL: &'static str = "function";

impl Drop for Timer {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        let function = self.function;
        histogram!("elapsed", duration.as_micros() as f64 / 1000f64, FUNCTION_LABEL => function);
    }
}
