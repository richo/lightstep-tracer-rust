use lightstep_tracer as lightstep;

fn traced() {
    println!("work");
}

fn main() {
    traced();
}
