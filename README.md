# BCSAT
## Description
A parser, printer and utilities for BCSAT. BCSAT is a language for boolean circuits. It supports the standard propositional gates (and, or, not), n-ary gates and a number of specialized gates: if-then-else, even/odd and threshold gates.

## Planned features:

The following features were considered but not implemented. If requested I'll implement them or if I need them for my own project.

* More simplifications:
  * Reducing implication chains to a conjunction and a single implication or a single disjunction.

    `a => (b => (c => d)) === (a & b & c) => d === ~a | ~b | ~c | d`
* Equivalent / equisatifiable mutations: due to the small number of benchmarks, SAT solvers a criticized to over-fit heuristics to the yearly SAT-race. This could be combated by artificially augmenting the benchmark set. A transformation to randomize the benchmark to defeat the preprocessing these solvers do could help with this. This will probably be a different crate.

* Reduction to cnf: there is an existing tool, a rewrite is not immediately needed except maybe for integration.

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0 http://www.apache.org/licenses/LICENSE-2.0 or the MIT license http://opensource.org/licenses/MIT, at your option. This project may not be copied, modified, or distributed except according to those terms.
