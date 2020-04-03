# From Path to NAnd

## What is a Path
A Path is a triple that consists of a Parent/Context,
a Self, and a Succesor.

Just as in s-expressions, a Path's Succesor can also be a Path,
leading to a recursively constructed Path of.

## What is a Binary Tree
A binary tree (as used in this document) is a directed,
singly rooted acyclic graph.

Aa [binary tree](https://en.wikipedia.org/wiki/Binary_tree) is a tree data structure 
in which each node has at most two children, which are referred 
to as the left child and the right child. 

## From Path to Binary Tree
If we consider the Self of a path to be the left child of the 
binary tree, and further consider the Path's successor to be 
the right child.
 

## From Path to NAnd
A NaND gate has two inputs and one output, and can be seen as an inverted
binary tree.

so a Binary Tree has 
(Parent,Left Child, Right Child) 
and a Catori Path has 
(Context, Self, Successor)
and a NaND gate has (Output Q, Input A, Input B),
these can all be considered equivalent data structures.

## Constructing a NaND circuit from Path constructs
If a NAnd gate's Q is equivalent to a path's context
and the A is equivalent to Self while B is equivalent to