pub trait RecursiveTuple {
    type Pop;
    type Remainder;
}

// (((), A), B)

impl<A: RecursiveTuple, B> RecursiveTuple for (A, B) {
    type Pop = A::Pop;
    type Remainder = (A::Remainder, B);
}

impl<A> RecursiveTuple for ((), A) {
    type Pop = A;
    type Remainder = ();
}
