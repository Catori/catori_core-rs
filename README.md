# catori_core-rs
Catori attempts to be a world view, a programming language, 
and a general approach to logical construction.

Catori is structurally typed, and any object is uniquely identified by its relationship to other objects

Catori structures are paths between other structures and exist in an 
[ordered substructural type system](https://en.wikipedia.org/wiki/Substructural_type_system#Ordered_type_system)
and therefore obey the laws of [noncommutative logic](https://en.wikipedia.org/wiki/Noncommutative_logic)

## Catori Types

### Nil Type
The Nil type is implicitly the beginning and end of all Paths. 
It represents non-existence, and cannot actually be inhabited except by a
failed observation?

### Here/True Type
Any position within Catori space that is occupyable is, by definition, not nil.
HERE can always be used synonymously with True. 
Note that a bare HERE type is actually just shorthand for 
```
Path<Nil,True,Nil>(true)
```

### Path Type
The Path Type represents a traversal between Nils
It is representable in loose rust syntax as:

```
trait Path<FROM, HERE, THERE>{}
```
FROM is any Catori type, but it is not traversable to.
HERE is a non-Nil Catori type that is currently occupied. It is always true.
However HERE can be a path itself, and as such, represents all the entanglements
that this particular HERE has.
There is 

The generic version of a Path is

```
struct Path<FROM, HERE, THERE(PhantonData<FROM>, HERE, THERE);
```

The simplest possible concrete representation of a Path is a struct that 
goes from Nil to Nil and is entangled with nothing.

```
struct Path<Nil, True, Nil>(true:);
```

for representational convenience, Paths can be named
```
let One = Path<Nil, True, Nil>(true);
```
(Note that the equals sign is only used for naming things)

and built on
```
let Two = Path<Nile, One, Nil(One);
```
which, of course, expands to:
```
Path<Nil, Path<Nil, True, Nil>(true), Nil>(Path<Nil, True, Nil>(true))
```
Which, once all the types collapse, amounts to (true, true), or more precisely (true(true))

and the progression continues
```
let Three = Path<Nil, Two, Nil(Two);
```

See [Church Numerals/Encoding](https://en.wikipedia.org/wiki/Church_encoding) and Peano Encoding



##Algebraic Types
See [Algebraic Data Types](https://en.wikipedia.org/wiki/Algebraic_data_type)

(Note that I'm using => instead of = or ==. This is to strongly imply the notion that 
left hand side (LHS) produces(=>) the right hand side(RHS).

LHS=>RHS

### Sum/Concatenation Type
The Sum type is the addition or concatenation of two paths. We will use the standard addition 
"operator"(+) to define a Sum Type 
relationship between two existing types:

Summation can be viewed as linearly laying out two paths along the same dimension.

```
Path<Nil, True, Nil>(true) + Path<Nil, True, Nil>(true) => Path<Nil, Path<Nil, True, Nil>(true), Nil>(Path<Nil, True, Nil>(true))
```
or in other words
```
One + One => Two
```
### Product/Field/Entanglement Type
We will use the common multiplication operator (*) to combine exists Paths into their Product Path type.
In contrast to Summation, the Product type involves combining two Paths along different dimensions, and 
creating a field of all possible permutations of the two paths. Without showing the full type expansion, the runtime
of a summation type can be characterized as:
```
Three * Three => Nine
(true, true, true) * (true, true, true) => (true, true, true, true, true, true, true, true, true)
```

### Observation Type
Any Catori Path can be an Observation Type. When one Path is observed by another Path, 
the Observee collapses (evaluates) itself, in an attempt to conform to the expectations of the observer.
All operations except observations are lazy, or in other words, there is only structure until
observation puts things into motion.

When an Observation Type is fully specified, The result is either true/Here or false/Nil.

The question mark is used to indicate an observation relationship.

So if we define a Universe/Path as 
```
let two_plus_two = (2 + 2);
```
The
```
two_plus_two ? Three => false
```
whereas
```
two_plus_two ? Four => true
```

because
```
two_plus_two=>(true+true)+(true+true)=>(true+true+true+true)=>Four

```

 Because observations that are only true/false are somewhat limiting, 
 wildcards can be used in any observation. To constrain the output to a single value(dimension), we
 use the underscore(_). 
 ```
two_plus_two ? (_) => Four
```
Wildcards can be used multiple times to represent different dimensions. In order to evaluate to
a value, the entire structure must be observable *structurally matching* the wildcards, so:
```
two_plus_two ? (_+_) => (Two+Two) 
//While four could be derived from 3+1, there is no way to get there from Two+Two that doesn't require dimensional expansion

two_plus_two ? (Two+_) => (Two+Two) 
//The underscore gets matched and the rest passes through

two_plus_two ? (3+_) => false
```

Finally, wildcards can be named and referenced in the output:
```
one_plus_two ? (VAR1+VAR2) => (VAR1*VAR2) ? (_) => Nine
```


###Subtraction
Since subtraction is the reciprocal of addition, we can use a destructuring observation to perform subtraction

```
Three ? (_) + (Two) => One
```


###Division
Similarly, since division is the reciprocal of multiplication, integer division can be performed as a 
destructuring observation, as well

```
Six ? (_) * (Two) => Three
```

However, 
```
Five ? (_) * (Two) => False //Five is not evenly divisible by two, and all paths/remnants must be consumed
```

But
```
Five ? (_) * (Two +  _) => (2,1)
