use lightstep_tracer_macros as lightstep_macros;

#[lightstep_macros::traced]
fn traced() {
    println!("work");
}

fn main() {
    traced();
}
