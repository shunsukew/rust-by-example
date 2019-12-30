struct A;
struct S(A);
struct SGen<T>(T);

struct Single(A);

struct SingleGen<T>(T);

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

struct Val (f64, );
struct GenVal<T>(T,);

impl Val {
    fn value(&self) -> &f64 {&self.0}
}

impl <T> GenVal<T> {
    fn value(&self) -> &T {&self.0}
}

fn main() {
    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');

    let t = SingleGen(A);
    let i32 = SingleGen(6);
    let char = SingleGen('a');

    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    generic::<char>(SGen('a'));

    generic(SGen('c'));

    let x = Val(3.0);
    let y = GenVal(3i32);

    println!("{}, {}", x.value(), y.value());
}
