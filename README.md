# catori_core-rs
Catori attempts to be a world view, a programming language, 
and a general approach to logical construction.

Catori is structurally typed, and any object is uniquely identified by its relationship to other objects

Catori structures are paths between other structures and exist in an 
[ordered substructural type system](https://en.wikipedia.org/wiki/Substructural_type_system#Ordered_type_system)
and therefore obey the laws of [noncommutative logic](https://en.wikipedia.org/wiki/Noncommutative_logic)

This means that interacting with any structures destroys it (from linear logic)
and that it is only possible to process the items in a path linearly
and without random access.

Catori uses lisp-like s-expressions but has a fundamentally different 
evaluation model

## Catori Structures
The simplest catori structure is a single Point. This can be represented as 
```
(true)
```

All Catori structures consist of nothing but truth values. Falseness/Nil
only exists at the beginning and end of a Path and is not reachable/occupyable

### Paths
All Catori structures consist of paths. A Point is a path, as are multiple points
```
(true true)
```
Paths are ordered representations of structural truth and can contain nested paths
```
(true (true true) true)
```
The above is a 3 element path where the second element is another path two elements long

### Orientation
Catori encourages thinking spatially. Paths can be considered to either
be parallel to one another, or perpendicular. By embedding one path into 
another with parentheses, as above, we are saying that the second item
projects perpendicularly to the main path until or unless the path is flattened

## Operations

### Sum Type
Any two paths can be viewed as parallel with each other and concatenated together.
Concatenation is performed with a '+' operator.

Because a path of length 2 is simply two paths of length 1 concatenated together,
the full s-expression form of a path (with or without nested elements),
will always start with a '+'
```
(+ true (+ true true) true) #Fully formed version of the example above
```
Concatenation is exactly the same as addition in a Catori universe

The + operator can also be seen as constructing an enumeration, sum type,
disjoint union, etc. This is because it sets up a Path structure where only one of its
elements can be selected/occupied at any given time.

### Product Type
Paths can also be constructed using a product/multiplication operator '*'.
When paths are multiplied together, they can be considered to be perpedincular to each
other and creating a field of all of their possible combination.
```
#A 1x1 field only contains one possible value
(* true true) => (+ true) 

#A 2x1 field contains two possible values)
(* (+ true true) (+ true)) => (+ true true)
 
#A 3x2 field contains six possible values)
(* (+ true true true) (+ true true) => (+ true true true true true true) 


```
### Flattening
As we saw in the above examples, a nested/multidimensional structure can be viewed
as a flattened representation of its values.

Any Catori structure with nested paths can be flattened to non-nested form
e.g.
```
(+ true (+ true true) true) 
```
is the same as
 ```
(+ true true true true)
```

If we view Catori structures as dimensions (see Orientation above), then flattening is simply
viewing the structure from a particular perspective, and projecting the
source structure into that new perspective.

Flattening is merely the recursive iteration through all paths and subpaths,
outputting the truth values and discarding all structure. It can be seen as 
maintaining the same object, but discarding all history of how it *became*
this object. 

Flattening a nested structure into a single dimension can be represented using a '~' operator
```
(~ (true (true true) true) => (+ true true true true)
```

### Syntactic Plus Elision
From here on out, any sexpr that doesn't start with an operator or aliased operator
will be considered to start with a plus

### Splitting
A path can be split into pieces. Splitting a path adds structure to the source path
Because a path can be split at many different locations, an unconstrained split
operation can and will result in an iterator (new Path) through all the possible results of splitting the original
Splitting is performed with the '?' operator. The first path after 
the split operator is the pattern to split something into. The second path
after the split operator is the path being split.

```
(? ( _ ) (true true) 
=> (true true)
```
The underscore is an unnamed placeholder indicating that the source path is
to be split into one piece.

```
(? ( _ ) (true true))
=> (true true)
```

Multiple underscores can be used to split the source path
into multiple parts
```
(? (_ _) (true true true))
=> (
    (true (true true)) //first solution
    (true true (true)) //second solution
   )
```
*Note* that a path cannot be empty, so it is only valid to split a path of length
3 into (1,2) and (2,1), not (0,3) and (3,0).

Patterns used for splitting can also be named. Any token with a leading
underscore can be used in place to alias the output.
TODO work out better method of inline aliasing
```
(? ( _first _second) (true true true))
=> (
   _first => (true (true true)) 
   _second => (true true (true))
   )
```



### Equality

There is no traditional notion of equality, in a Catori universe. Instead, partly inspired by Vladimir Vovodsky's 
Univalence Axiom, there is only a notion of Observational Equality. Observational Equality can be defined as:

Two objects are observationally equivalent if all possible observations result in the two returning the same result


When performing an observation, the way to think and speak about it is:

In the context of A, B and C are (or are not) observationally equivalent

```
((true true true true true)),((true true true true) (true))) ? (A,A) => true
```

both path of length five , and paths of lengths four plus one evaluate to the same value in one dimension
```
(alias = (? _A _A)) //Forces both the first and second values to be evaluatable to the same thing
```

### Inequality
Inequality operators can be defined in terms of splits

```
(alias > (a b)) (
     //plus function as a match operator in that each branch is evaluated
     //   + implicit plus functions as match operator 
     //an observation that the a path has a longer length than the
     // b path, and therefore can be split into b + remainder
        (? (b,_) a) (true)
        () 
    )
)

//inverted
(alias < (a b)) (
        (? (a,_) b) (true)
        () 
    )
)
```

### Exponentiation
Concatentation allows for the evaluation of multiple disjoint branches/structures.

```
//stolen verbatim from recursive lisp implementation with defun replaced with
alias and cond replaced with (implicit) +
We also replace the standard observation operator with > since 
we want to only return the higher of the  two branches even if we evaluate both of them
(alias ^ (a b)
  (>
    (
        (= b ()) true)
    //catori supports recursively building nested structures
        (_ (* a (^ a (- b true))))))
```
Unlike most languages, all matching branches will be evaluated and 
executed as a lazy iterator



## Other forms of truth
So far, paths have consisted only of sequences of truth values. Given that no position
in a structure can contain anything other than a single value, we have been operating
in a base one representation.
By usind a traditional Church/Peano encoding of the base 10 digits, we can replace
length paths of truth with the numeric equivalents

See [Church Numerals/Encoding](https://en.wikipedia.org/wiki/Church_encoding) and Peano Encoding

```
let Zero   = ()
let One    = (Zero true)
let Two    = (One true)
let Three  = (Two true)
let Four   = (Three true)
let Five   = (Four true)
let Six    = (Five true)
let Seven  = (Six true)
let Eight  = (Seven true)
let Nine   = (Eight true)
```



It's also possible to construct an equivalent representation of the base 10 numerals
by defining the first 10 digits and then associating position in a numeric
token with its dimensionality

We can view the base 10 digits as an N dimensional field where each dimension
is a path of length 10^pos where pos is the offset from the left of the integer in question 

So 
```

(alias 1 true)
(alias 2 (1 true)
(alias 3 (1 true)
(alias 4 (2 true)
(alias 5 (4 true)
(alias 6 (5 true)
(alias 7 (6 true)
(alias 8 (7 true)
(alias 9 (8 true)

23 => (
        ( * 2 (^ 10 1)) 
        ( * 3 (^ 10 0))
       )
475 => ( 
        ( * 4 (^ 10 2 )) 
        ( * 7 (^ 10 1 )) 
        ( *5 (^ 10 0 ))
    )
```




```
(? (_+_) two_plus_two ) => (Two+Two) 
//While four could be derived from 3+1, there is no way to get there from Two+Two that doesn't require dimensional expansion

(? (Two+_) two_plus_two ) => (Two+Two) 
//The underscore gets matched and the rest passes through

(? two_plus_two (3 _)) => false
```

### Subtraction

Since subtraction is the reciprocal of addition, we can use a destructuring observation to perform subtraction

```
(? (Two (_))) Three) => One
```

```
(alias (- A) (? (+ (_)  (A)))
)
```

### Division

Similarly, since division is the reciprocal of multiplication, integer division can be performed as a 
destructuring observation, as well

```
(? Six  (* (_) (Two))) => Three
```

```
(alias (? /A (* (_) (A)))
```

However, 
```
(*(? Five _ (Two) => False //Five is not evenly divisible by two, and all paths/remnants must be consumed
```

But
```
(? Five (* (_)  (Two ) ))+  _) => (2,1)
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

# TODO rebuild the rest of this

### Constructing bits
A bit is a path of length two. At any given time, it can occupy one of its positions or the other.

(alias Bit (+ true true))

Note that while there is no global representation of falsehood, something that means "false" to a subdomain can
be constructed. 
```
(alias (false,true) (Bit _ _));
```
### Constructing bounded integers

#### Bytes
A byte exists as a useful concept because it matches machine architecture of 8 bits. 
Following the same rules established for base 10 integers above, but with dimensional width of two instead of 
ten, and a maximum of 8 dimensions. These can be constructed as 

```
(alias Byte(? (Bit,Bit,Bit,Bit,Bit,Bit,Bit,Bit)) (
( Byte true false true true true true false true)
```

where each of those true and false values actually actually bit paths from above


or with compact notation where true is 1, false is 0 and the comman delimiter is implied:

```
Byte(10111101);
```

Since an 8 dimensional byte can be represented by it's one dimensional flattening,
there is an isomorphism between a byte and the integers from 0-127. Therefore an alternate representation of a byte is
```
(? 189 ? byte => byte(10111101)
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


## The Language
### Symbolic Expressions
Catori uses Lisp-like S-expressions to construct its N-dimensional paths. However, instead of eval,
Catori has the observation(?) operator, and all execution is performed by observing one structure as another structure

An atomic symbol is a string of numerals, letters, and underscores. 
The first character must be a letter or underscore.

#### Examples
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

## Simplifying Syntax

### Destructuring
Destructuring (?) is a binary operator that tries to fit the LHS into the pattern expressed by the RHS.
If will always return either false or the exact number of dimensions asked for on the RHS
(The => is not part of the syntax and just refers to the value being returned)
 ```
(2 + 2 ) ? (_) => 4
(2 + 2) ? (_,_) => (2 , 2)
```

If a LHS can be destructured multiple ways into the pattern expressed by the RHS, then 
an observatin will result in new path containing all the possible ways that the LHS can be validly destructured
```
(12) ? (_ ,_) => (1,11) | (2,10) | (3,9) | etc
```
Destructuring is always lazy




As a unary postfix operator, a ? always tries to destructure into a single dimensional value
```
2+2 ? => 4
```

```
(true,true,true)*(true,true,true) ?= 9
3*3 ?= 9
```

### Observation
Observation is eagerly evaluated destructuring or pattern matching. We will use ?= as the this eager observation operator.
Nothing other than using ?= will actually result in any evaluation. Otherwise, Observing using ?= is semnatically and
syntactically identical to destructuring using ?


Any operator that is an alias for a destructuring operation can be made eager insted of lazy by adding '='.

#### Lazy division:

```
6/3 => 2 (but only lazily)
6/=3 => 2 (eagerly)
```

#### Lazy multiplication

```
6 * 3 => 18 (lazily)
```

#### Lazy addition

```
6 + 3 => 9 (lazily)
```

#### Lazy subtraction

```
6 - 2 => 4 (lazily)
```

In general use of eager variations should be minimized and only used to force output/processing


### Observing into complex structure

Observation and destructuring can also have nested structure
```
((3+3),(4+5)) ?= (_,(_,_)) => (6,4,5)
```

```
((3+3),(4+5)) ?= (Byte(_),(_,_)) => (00000110,(4,5))
```



```

(? (_) (+ 2 4) )
=> 6

(? (A) (+ 2 4) )
=> A=6

(? (A) (+ 2 4 8) )
=> A=14

(? (A,B) (+ 2 4 8) )
=> (A=14 B=0) (A=13 B=1)

(? (_) (* 2 4) )
=> 8

(? (B) (* 2 4) )
=> B=8

(? (_,_) (+ 2 4))
=> (+ 2 4)

(? (*,A,B) (* 2 4))
=> (* A=2 B=4)

let A =(_)
let B = (_,_)

(? A (* 2 4))
=> A=8

(? B (* 2 4))
=> B=(* 2 4)

(? (+ A B) (+ 5))
//what is the path that includes all the possible solutions to splitting 5 into two parts?
=> (+ 1 4) (+ 2 3) (+ 3 2) (+ 2 3) (+ 1 4) (+ 0 5)

? Observation Structure
=> Solution

(? Byte(A) 8)
=> Byte<bit,bit,bit,bit,bit,bit,bit,bit,bit>[*0*2^8 0,0,0,1,1,1,0]
Where bit is defined as (+ Zero One)


let TwoBit = (* 1*2^1 0)
=> TwoBit(10)
? (_) TwoBit(10)
=> 2


(+ 2 3 4)
(* 2 3)

and Byte defined as 

? Byte(A B C D E F G H) 8
=> A=0 B=0 C=0 D=0 E=1 F=1 G=1 H=1

(* A*2^8 B*2^7 C*2^6 D*2^5 E*2^4 F*2^3 (G*2^2 (H*2^1)))






```

