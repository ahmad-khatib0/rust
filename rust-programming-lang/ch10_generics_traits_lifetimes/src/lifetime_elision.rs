// Lifetime Elision
// compiled without lifetime annotations, even though the parameter and return type are references!
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// The reason this function compiles without lifetime annotations is historical: in early versions
// (pre-1.0) of Rust, this code wouldn’t have compiled because every reference needed an explicit lifetime.
// At that time, the function signature would have been written like this:
// fn first_word<'a>(s: &'a str) -> &'a str {
// After writing a lot of Rust code, the Rust team found that Rust programmers were entering the same
// lifetime annotations over and over in particular situations. These situations were predictable and
// followed a few deterministic patterns. The developers programmed these patterns into the compiler’s
// code so the borrow checker could infer the lifetimes in these situations and wouldn’t need explicit
// annotations. This piece of Rust history is relevant because it’s possible that more deterministic
// patterns will emerge and be added to the compiler. In the future, even fewer lifetime annotations
// might be required.
// The patterns programmed into Rust’s analysis of references are called the lifetime elision rules.
// These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler
// will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.
//
// The compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit
// annotations. The first rule applies to input lifetimes, and the second and third rules apply to output
// lifetimes. If the compiler gets to the end of the three rules and there are still references for which
// it can’t figure out lifetimes, the compiler will stop with an error. These rules apply to fn definitions
// as well as impl blocks.
//
// The first rule is that the compiler assigns a lifetime parameter to each
// parameter that’s a reference. In other words,
// a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
// a function with two parameters gets two separate lifetime parameters:
//   fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
//
// The second rule is that, if there is exactly one input lifetime parameter, that lifetime is
// assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
//
// The third rule is that, if there are multiple input lifetime parameters, but one of them is &self
// or &mut self because this is a method, the lifetime of self is assigned to all output lifetime
// parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

// Let’s pretend we’re the compiler. We’ll apply these rules to figure out the lifetimes of the references
// in the signature of the first_word function. The signature starts without any lifetimes associated with
// the references:
// fn first_word(s: &str) -> &str {
// Then the compiler applies the first rule, which specifies that each parameter gets its own
// lifetime. We’ll call it 'a as usual, so now the signature is this:
// fn first_word<'a>(s: &'a str) -> &str {
// The second rule applies because there is exactly one input lifetime. The second rule
// specifies that the lifetime of the one input parameter gets assigned to the output
// lifetime, so the signature is now this:
// fn first_word<'a>(s: &'a str) -> &'a str {
// Now all the references in this function signature have lifetimes, and the compiler can continue
// its analysis without needing the programmer to annotate the lifetimes in this function signature.
//
// Let’s look at another example, this time using the longest function
// that had no lifetime parameters when we started working with it
// fn longest(x: &str, y: &str) -> &str {
// Let’s apply the first rule: each parameter gets its own lifetime. This
// time we have two parameters instead of one, so we have two lifetimes:
// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
// You can see that the second rule doesn’t apply because there is more than one input lifetime.
// The third rule doesn’t apply either, because longest is a function rather than a method, so none
// of the parameters are self. After working through all three rules, we still haven’t figured out
// what the return type’s lifetime is. This is why we got an error trying to compile the code in
// Listing 10-20: the compiler worked through the lifetime elision rules but still couldn’t figure
// out all the lifetimes of the references in the signature.
