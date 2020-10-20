use criterion::{black_box, criterion_group, criterion_main, Criterion};
use intl_messageformat_parser_rs::Parser;

// TODO: add back HTML tags
const COMPLEX_MSG: &str = "
{gender_of_host, select,
    female {
        {num_guests, plural, offset:1
            =0 {{host} does not give a party.}
            =1 {{host} invites {guest} to her party.}
            =2 {{host} invites {guest} and one other person to her party.}
    other {{host} invites {guest} and # other people to her party.}}}
    male {
        {num_guests, plural, offset:1
            =0 {{host} does not give a party.}
            =1 {{host} invites {guest} to his party.}
            =2 {{host} invites {guest} and one other person to his party.}
            other {{host} invites {guest} and # other people to his party.}}}
    other {
        {num_guests, plural, offset:1
        =0 {{host} does not give a party.}
        =1 {{host} invites {guest} to their party.}
        =2 {{host} invites {guest} and one other person to their party.}
        other {{host} invites {guest} and # other people to their party.}}}}
";

const NORMAL_MSG: &str = "
Yo, {firstName} {lastName} has
{numBooks, number, integer}
{numBooks, plural,
    one {book}
    other {books}}.
";

#[allow(unused_must_use)]
fn parse_complex_msg() {
    Parser::new(black_box(COMPLEX_MSG)).parse().unwrap();
}

#[allow(unused_must_use)]
fn parse_normal_msg() {
    Parser::new(black_box(NORMAL_MSG)).parse().unwrap();
}

fn benchmark_normal(c: &mut Criterion) {
    c.bench_function("normal msg", |b| b.iter(parse_normal_msg));
}

fn benchmark_complex(c: &mut Criterion) {
    c.bench_function("complex msg", |b| b.iter(parse_complex_msg));
}

criterion_group!(benches, benchmark_normal, benchmark_complex);
criterion_main!(benches);
