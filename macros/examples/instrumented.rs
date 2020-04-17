use lightstep_tracer_macros as lightstep_macros;

struct LightstepSpan {}
impl Drop for LightstepSpan {
    fn drop(&mut self) {
        println!("Span going away");
    }
}

trait LightstepTracing {
    type Span: Drop;
    fn start_span(&self) -> Self::Span;
}

struct LightstepTracerContext {
}

impl LightstepTracing for LightstepTracerContext {
    type Span = LightstepSpan;
    fn start_span(&self) -> LightstepSpan {
        println!("Creating a span");
        LightstepSpan {}
    }
}


#[lightstep_macros::traced]
fn traced() {
    println!("work");
}

#[lightstep_macros::traced]
fn other(arg: Vec<String>) {
    println!("work");
}

fn main() {
    let ctx = LightstepTracerContext{};
    traced(ctx);
}
