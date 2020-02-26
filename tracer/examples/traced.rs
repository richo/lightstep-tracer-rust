use lightstep_tracer::Tracer as Tracer;

fn traced() {
    let span = Tracer::span();
    span.begin();
    println!("work");
    span.end();
}

fn main() {
    traced();
}
