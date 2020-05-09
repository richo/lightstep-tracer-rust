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
    let haha = "haha";
    traced_inner(haha);
    untraced_inner("ahahaa");
}

#[lightstep_macros::traced]
fn traced_inner(asdf: &'static str) {
    println!("inner workings: {}", asdf);
}

fn untraced_inner(other: &'static str) {
    println!("untraced inner workings: {}", other);
}

#[lightstep_macros::traced]
fn other(arg: Vec<String>) {
    println!("work");
}

fn main() {
    let ctx = LightstepTracerContext{};
    traced(ctx);
}
