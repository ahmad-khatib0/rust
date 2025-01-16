pub fn iterators_perf() {
    // As another example, the following code is taken from an audio decoder. The decoding algorithm
    // uses the linear prediction mathematical operation to estimate future values based on a linear
    // function of the previous samples. This code uses an iterator chain to do some math on three
    // variables in scope: a buffer slice of data, an array of 12 coefficients, and an amount by which
    // to shift data in qlp_shift. We’ve declared the variables within this example but not given them
    // any values; although this code doesn’t have much meaning outside of its context, it’s still a
    // concise, real-world example of how Rust translates high-level ideas to low-level code.
    //
    // let buffer: &mut [i32];
    // let coefficients: [i64; 12];
    // let qlp_shift: i16;

    // for i in 12..buffer.len() {
    //     let prediction = coefficients
    //         .iter()
    //         .zip(&buffer[i - 12..i])
    //         .map(|(&c, &s)| c * s as i64)
    //         .sum::<i64>()
    //         >> qlp_shift;

    //     let delta = buffer[i];
    //     buffer[i] = prediction as i32 + delta;
    // }

    // To calculate the value of prediction, this code iterates through each of the 12 values in
    // coefficients and uses the zip method to pair the coefficient values with the previous 12 values
    // in buffer. Then, for each pair, it multiplies the values together, sums all the results, and
    // shifts the bits in the sum qlp_shift bits to the right.
    // Calculations in applications like audio decoders often prioritize performance most highly. Here,
    // we’re creating an iterator, using two adapters, and then consuming the value. What assembly code
    // would this Rust code compile to? Well, as of this writing, it compiles down to the same assembly
    // you’d write by hand. There’s no loop at all corresponding to the iteration over the values in
    // coefficients: Rust knows that there are 12 iterations, so it “unrolls” the loop. Unrolling is
    // an optimization that removes the overhead of the loop controlling code and instead generates
    // repetitive code for each iteration of the loop.
    // All of the coefficients get stored in registers, which means accessing the values is very fast.
    // There are no bounds checks on the array access at runtime. All of these optimizations that Rust
    // is able to apply make the resultant code extremely efficient. Now that you know this, you can
    // use iterators and closures without fear! They make code seem like it’s higher level but don’t
    // impose a runtime performance penalty for doing so.
}
