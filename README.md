# catori_core-rs
Catori attempts to be a world view, a programming language, 
and a general approach to logical construction.

Catori is structurally typed, and any object is uniquely identified by its relationship to other objects

Catori structures are paths between other structures and exist in an 
[ordered substructural type system](https://en.wikipedia.org/wiki/Substructural_type_system#Ordered_type_system)
and therefore obey the laws of [noncommutative logic](https://en.wikipedia.org/wiki/Noncommutative_logic)

## Catori Types - A Theoretical Foundation

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

The type system is established the rules that can be performed by these types.

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

Observation is based on destructuring

Any Catori Path can be an Observation Type. When one Path is observed by another Path, 
the Observee collapses (evaluates) itself, in an attempt to conform to the expectations of the observer.
All operations except observations are lazy, or in other words, there is only structure until
observation puts things into motion.

When an Observation Type is fully specified, The result is either true/Here or false/Nil.

The question mark is used to indicate an observation relationship.

So if we define a Universe/Path as 

(true,true)(true,true)
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

```
alias '-A' ?(_) + (A)
```

###Division

Similarly, since division is the reciprocal of multiplication, integer division can be performed as a 
destructuring observation, as well

```
Six ? (_) * (Two) => Three
```

```
alias '/A' ?(_) * (A)
```

However, 
```
Five ? (_) * (Two) => False //Five is not evenly divisible by two, and all paths/remnants must be consumed
```

But
```
Five ? ((_) * (Two )+  _) => (2,1)
```


###
Deriving additional operations
So far we have only used 


##Deriving Equality

There is no traditional notion of equality, in a Catori universe. Instead, partly inspired by Vladimir Vovodsky's 
Univalence Axiom, there is only a notion of Observational Equality. Observational Equality can be defined as:

Two objects are observationally equivalent if all possible observations result in the two returning the same result


When performing an observation, the way to think and speak about it is:

In the context of A, B and C are (or are not) observationally equivalent

```
(Five,(Four+One)) ? (A,A) => true
```

both Five, and Four+One evaluate to the same value in one dimension
```
alias '==' (?(A,A))
```

### Constructing Base 10 Integers
Let's supposed that we can also use digits as structural aliases, and we can numerals from 1 to 9 as
the following:

```
let 1 = Path<Nil, True, Nil>(true);
let 2 = Path<Nil, 1, Nil(1);
let 3 = Path<Nil, 2, Nil(2);
let 4 = Path<Nil, 3, Nil(3);
let 5 = Path<Nil, 4, Nil(4);
let 6 = Path<Nil, 5, Nil(5);
let 7 = Path<Nil, 6, Nil(6);
let 8 = Path<Nil, 7, Nil(7);
let 9 = Path<Nil, 8, Nil(8);
```

Further allow the syntactic construction such that two digits adjacent to each other create a
two dimensional field plus a one dimensional path
```
78 => (7*10) + (8)
```
or more generally
```
ij => (i*10) + (j)
```
and it extends to N dimensions so that
```
ijk => (i*100) + (j*10) + k
```

TODO figure out how to derive this more formally in ways that don't rely on apriori knowledge of integers

For convenience, a path of length N can have its positions represented by their numerical position, starting with zero

### Constructing bits
A bit is a path of length two. At any given time, it can occupy one of its positions or the other.

let bit = Path<Nil,true,true,Nil>

Note that while there is no global representation of falsehood, something that means "false" to a subdomain can
be constructed. 
let false = bit(0);
let true = bit(1);

### Constructing bounded size integers

#### Bytes
A byte exists as a useful concept because it matches machine architecture of 8 bits. 
Following the same rules established for base 10 integers above, but with dimensional width of two instead of 
ten, and a maximum of 8 dimensions. These can be constructed as 

```
alias Byte = Path<Nil,bit,bit,bit,bit,bit,bit,bit,bit,Nil>
byte(true,false,true,true,true,true,false,true)
```

where each of those true and false values actually actually bit paths from above


or with compact notation where true is 1, false is 0 and the comman delimiter is implied:

```
byte(10111101);
```

Since an 8 dimensional byte can be represented by it's one dimensional flattening,
there is an isomorphism between a byte and the integers from 0-127. Therefore an alternate representation of a byte is
```
189 ? byte => byte(10111101)
```
This returns the byte viewed in its canonical form, and can be read as
Construct the integer that is isomophic to byte 189, and then observe it as a native 8 dimensional byte.

This works because byte is an alias for an 8 dimensional space, and we are asked to upconvert 
the one dimensional integer to an 8 dimensional byte and then print its short format.

However
```
321 ? byte => false
```
because 321 can't be fit into an 8 dimensional 1 bit space without overflowing


##The Language
###Symbolic Expressions
Catori uses Lisp-like S-expressions to construct its N-dimensional paths. However, instead of eval,
Catori has the observation(?) operator, and all execution is performed by observing one structure as another structure

An atomic symbol is a string of numerals, letters, and underscores. 
The first character must be a letter or underscore.

####Examples
A
Apple
PART_2
_EXTRALONGSTRINGOFLETTERS
a4_32adad_3241fgdd

These symbols are called atomic because they are taken as a whole 
and are not capable of being split withn Catorin into individual characters.

An S-expression is either  an atomic  symbol or it is composed of these elements in the following order: 
a left parenthesis, an S-expression, a  dot, an S- expression, and a right parenthesis

For convenience, like modern LISPs, 
```
(x . (y . (z . NIL)))
(x y z)
```
are equivalent

We'll assume from this point on that most basic types, including integers and strings have been formally constructed and
are loosely equivalent to the cooresponding types in LISP. Floating point numbers will not yet be used
as they are complex little buggers

Examples of Catori atoms include:
```
100
hyphenated-name
false
```

A list  is  a  sequence  of  either  atoms  or  other  lists  separated  by  blanks  and enclosed in parentheses. 
 Examples of lists include:
 (1 2 3 4)
 (george kate james joyce)
 (a (b c) (d (e f)))
 ()
 
 Unlike LISP, suffixes are often used to represent the type of a numeric atom when constants are being used
 
 ```
337u32
28u8
-123i16
```

But types can be inferred in most cases