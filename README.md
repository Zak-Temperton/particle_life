# ParticleLife

This is a simulation of particle "life" based on randomised attractive and repulsive forces between particle types.
This is a clone of [Particle Life by HackerPoet](https://github.com/HackerPoet/Particle-Life), but rewritten in the Rust programming languages with only minor differences.

This was a personal project to practice Rust, and understanding the differences in writing between c++ and Rust. There where some things that had to be written differently due to the borrow checker of Rust, but could be quicky worked around, another difference was not enforced but encouraged by Rust conventions e.g. using `Option<usize>` instead of checking for a negative int for indeces.

There are changes I want to add in the future like:
- Multiple particle sizes
- More consistent wrapping (forces also wrap)
- Individual friction values