# YMath
A coherent collection of math-related crates

## Tasks to implement
All in exact form of course
[] Normalizing vectors
[] Simplifying surds
[] Derivatives
[] Integrals
[] Rational simplification
[] Variable substitution

## Structure
Bunch of super generic structs like Frac, Addition<const N>
These store syntactical information (e.g. to use vinculum or / or .-. notation for fractions) for (mostly) lossless text conversion when using proc_macro (i.e. ignoring spaces)
- Structs store all necessary information, but display impls (e.g. using latex or custom bevy renderer) need only to display 'necessary' information, e.g. integration bounds showing x = 1 instead of just '1'
Bunch of super specific functions take really specific types like Frac<Int, NonZeroInt> and do simple things like simplify, de-negative, etc.
- These functions should be the only way to manipulate the expressions, that way all mutations can be tracked
These functions can then be traced using a type like MathContext (as well as debugging `tracing`)
More complex, but still specific typed, functions can then be built out to solve real world problems like derivatives, integrals e.t.c.
A focus on explainability by linking, i.e. no hard-coded text but hard-coded IDs into a database of nodes (infi-map) that can have editable, linkable and expandable descriptions
