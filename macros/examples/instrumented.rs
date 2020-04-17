use lightstep_tracer_macros as lightstep_macros;

struct LightstepTracerContext {
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
