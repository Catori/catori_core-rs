# Catori Foundations: [NAnd Logic](https://en.wikipedia.org/wiki/NAND_logic)

##Introduction

A NAnd gate consists of two inputs, one output, and obeys the following truth table.

| A 	| B 	| Q 	|
|---	|---	|----	|
| 0 	| 0 	| 1 	|
| 0 	| 1 	| 1 	|
| 1 	| 0 	| 1 	|
| 1 	| 1 	| 0 	|

If one input must always be connected to, at most, one output, then
a circuit consisting of nothing but nand gates can be modeled as a simple
binary tree, with the output at the top, and the inputs as the leaves.

In order to build functional circuits out of just NAnd gates, however,
it is necessary to be able to bind two inputs to the same output. 

e.g. to create a NOT gate, inputs A and B have to be bound to the same 
source. This effectively entangles A and B so that the new truth table becomes

| A 	| B 	| Q 	|
|---	|---	|----	|
| 0 	| 0 	| 1 	|
| 1 	| 1 	| 0 	|

The rows (from above) where A and B varied are no longer occupyable/reachable, and 
the resulting truth table clearly demonstrates negation from input to output.

## Using SExpressions to construct NAnd Circuitsi
### NAnd Tree Construction
Because the untangled tree form of a NAnd circuit is stpurictly hierarchical,
they can be represented as simple s-expressions, using only structural elements and no additional symbols
e.g.
```
(())
```
 Represents two NAnds, where the inner NAnd's output is assigned to the 
outer NAnd's A-input. and
```
(()(()))
```
represents separate trees, the first containing one NAnd gate,
and the second containing two gates with a Q-A relationship.
Collectively, the two trees have 4 3 unoccupied inputs and two unoccupied outputs
Because of the lack of entanglement represented at this level, all of the 
B-inputs are empty, and the resultant N-gate circuit will have N open B-inputs
and one open Q output.

### Entangling Inputs
With none of the B-inputs occupied, no circuit expressible in this form
is interesting nor can they perform any meaningful computation. By treating 
all the leaves of the S-Expression as inputs, we can walk the tree and derive an
ordered list of all unoccupied inputs in this circuit.

By using the entanglement operator '?', we can take all the outputs of the 
first argument and use the second argument to entangle individual outputs
```
(? (() ()) ( _1 _2 )
```
Says that each of the two input leaves in the ```(() ())``` expression,
must be bound to the same value during any execution. 
