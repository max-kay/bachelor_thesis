#set math.equation(numbering: "1")

// #let vector(a, b) = {
//   math.accent([a space.quarter b], arrow)
// }
// $ vector(X, Y) $

#align(center, text(17pt)[
  *Symmetry Implementation for Pair Distribution Function*
])
#align(center)[Max Krummenacher]
#v(20pt)
#align(center)[Bachelor thesis #datetime.today().display("iso") \ BSc material science and engineering ETH Zürich \ supervised by Arkadiy Simonov]
// #pagebreak()


#align(center)[ 
  #set par(justify: false)
  *Abstract* \
  #lorem(40)
]

= Introduction
== Mathematical Description of Crystals
=== Lattice
A lattice is a collection of points in three dimensional space which fulfill three (linearly independent) translation symmetries.
If $bold(a), bold(b), bold(c) in RR^3$ are the translation vectors of the lattice, a basis for i can be given in terms of these vectors.
Thus with the origin $O$ set at a lattice point any lattice point can be described as a linear combination of the translation vectors.

$ accent(O space.quarter X, arrow) = bold(a) x + bold(b) y + bold(c) z = bold(mat(a, b, c)) mat(x; y; z); x, y, z in ZZ $

To describe any point in space this defintion can be extended by allowing $x, y, z in RR$ giving us $P = {mat(x;y;z); x, y, z in RR}$ the set of all points.

The Metric on this space is given by:
$
d(dot, dot): P times P &arrow.long RR \
(X, Y) &arrow.long.bar abs(accent(X Y, arrow)) = sqrt(accent(X Y, arrow)^T G accent(X Y, arrow))
$ <metric>

Where $G$ is the metric tensor given by:

$
G = mat(
      bold(a) dot bold(a), bold(a) dot bold(b), bold(a) dot bold(c);
      bold(b) dot bold(a), bold(b) dot bold(b), bold(b) dot bold(c);
      bold(c) dot bold(a), bold(c) dot bold(b), bold(c) dot bold(c);
    ) = mat(
      a^2, a b cos(gamma), a c cos(beta);
      b a cos(gamma), b^2, b c cos(alpha);
      c a cos(beta), c b cos(alpha), c^2;
    )
$

The coefficients of the metric tensor on the left are given in $a, b, c$ and $alpha, beta, gamma$ the length of the basis vectors and the angles between them.
This is the most common format used for the lattice parameters.
// should I give explanation what angle is meant?

=== Lattice Isomerties
Lattice isometries are distance and angle preserving transformations on the lattice.
Isometries can be represented as affine transformation, which consist of a matrix and a translation vector.
Following the convention in the International Table for Crystallography, the transformation will be notated like this:
$ cal(Q) = (bold(Q), bold(q)) $
$ accent(O tilde(X), arrow) = accent(O cal(Q) X, arrow) = bold(Q) accent(O X, arrow) + bold(q) $

By simple symbol manipulation the following formulas for composition and inversion can be proven.
$ cal(Q) cal(P) = (bold(Q), bold(q)) (bold(P), bold(p)) = (bold(Q P), bold(Q p + q)) $
$ cal(Q)^(-1) = (bold(Q)^(-1), -bold(Q)^(-1) bold(q)) $

For this transformation to be an isometry the following statement must hold for all $a, b in P$:
$
d(X, Y) &= d(cal(Q)X, cal(Q)Y)\
abs(accent(X Y, arrow)) &= abs(accent(cal(Q)X cal(Q)Y, arrow))\
&= abs(bold(Q) accent(O Y, arrow) + bold(q) - bold(Q) accent(O X, arrow) - bold(q))\
&= abs(bold(Q)(accent(O Y, arrow) - accent(O X, arrow)))\
&= abs(bold(Q) accent(X Y, arrow))
$

As expected the translation vector $q$ is free as a translation doesn't affect the distance between two points.

Since X and Y are arbitrary points the vector $v$ between the is too.
Using the definition of the metric in @metric:
$
abs(bold(v)) &= abs(bold(Q v))\
abs(bold(v))^2 &= abs(bold(Q v))^2\
bold(v)^T bold(G) bold(v) &= (bold(Q v))^T bold(G) (bold(Q v))\
  &= bold(v)^T bold(Q)^T bold(G) bold(Q) bold(v)
$

Since v is arbitrary this equation leads to the following condition on $bold(Q)$:
$ bold(G) = bold(Q)^T bold(G) bold(Q) $
Which might me more familiar to the reader in the standard basis where $bold(G) = bold(I)$
$ bold(I) = bold(Q)^T bold(Q) $
Which is the condition for $bold(Q)$ to be orthagonal.
Orthagonal matrices have determinant $plus.minus 1$ the same is the case for the transformation matrix by:
$
det(bold(G)) = det(bold(Q)^T bold(G) bold(Q)) = det(bold(Q)^T) det(bold(G)) det(bold(Q))\
1 = det(bold(Q)^T) det(bold(Q)) = det(bold(Q))^2\
=> det(bold(Q)) = plus.minus 1
$

==== Additional Restictions


== Pairs
== Yell

= Programm
== Spacegroup implementation
*Definition* Quotient group

group extension

== Pair implementation

= Results

= Discussion

= Appendix

*Definition*
$H$ is a normal subgroup of $G$ if $g h g^(-1) in H$ for all $g in G$ and $h in H$.

*Claim*
Let $frak(H)$ be a spacegroup and $frak(T) = { (bold(I), bold(v)) ; bold(v) in ZZ^3 }$ the group of interger translation.
Then $frak(T)$ is a normal subgroup of $frak(H)$.

*Proof*
Let $cal(H) in frak(H)$ be an arbitrary symmetry element and $cal(T) in frak(T)$ be an arbitrary interger translation.
$
cal(H)cal(T)cal(H)^(-1) &= (bold(H), bold(h))(bold(I), bold(t))(bold(H)^(-1), -bold(H)^(-1)bold(q))\
  &= (bold(H), bold(h))(bold(H)^(-1), -bold(H)^(-1)bold(q) + bold(t))\
  &= (bold(H) bold(H)^(-1), bold(H)(-bold(H)^(-1)bold(q) + bold(t)) + bold(q))\
  &= (bold(I), bold(H)bold(T))
$
