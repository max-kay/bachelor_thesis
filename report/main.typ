#set math.equation(numbering: "(1)")
#set heading(numbering: "1.")

#import "@preview/whalogen:0.1.0": ce

// Display inline code in a small box
// that retains the correct baseline.
#show raw.where(block: false): box.with(
  fill: luma(240),
  inset: (x: 3pt, y: 0pt),
  outset: (y: 3pt),
  radius: 2pt,
)

// Display block code in a larger block
// with more padding.
#show raw.where(block: true): block.with(
  fill: luma(240),
  width: 100%,
  inset: 10pt,
  radius: 4pt,
)

#let innerproduct(x, y) = $lr(angle.l #x, #y angle.r)$

#align(
  center,
  text(
    17pt,
  )[
      *Symmetry Implementation for Pair Distribution Function*
    ],
)
#align(center)[Max Krummenacher]
#v(20pt)
#align(
  center,
)[
    Bachelor thesis #datetime.today().display("[day].[month].[year]")\
    Draft\
    B.Sc. material science and engineering, ETH Zürich\
    supervised by Arkadiy Simonov
  ]
#pagebreak()

#align(
  center,
)[
  #set par(justify: false)
  *Abstract* \
  Pairs of symmetry related sites in crystals are important for many applications, for example the 3D#sym.Delta\ PDF,
  which is used to determine the structure of disordered crystals.
  In this work, a program to determine the multiplicity of such pairs was developed.
  This work explains the mathematics behind crystals, special sites in crystals and pairs of symmetry equivalent positions.
  Examples in 1, 2 and 3 dimensions are shown to explain the concepts and algorithms used to determine pair multiplicity
  and the structure of the program is described.
]

= Introduction <intoduction>

== Disordered Crystals <disorder>
At finite temperature, materials inherently posses some type of disorder. But while gases and liquids posses no long-range order, in crystals the 
equilibrium positions of the atoms are arranged in periodic pattern.
But crystals still show a degree of disorder. Most disorder in the crystal is in the thermal motion of the atoms.
But also the formation of vacancies, dislocations and other faults in crystals or magnetic orientations might be a source of disorder.
If a material is degenerate, i.e. the material has many states with similar energies, more disorder is promoted.

A good example for such structures is the material class of Prussian blue analogues 
(materials with composition $#ce("M[M'(CN)_6]")_text(x)$ where M and M' are metal ions).
Prussian blue analogues generally have an fcc structure.
However, not all hexacyano metallates are occupied, as charge neutrality dictates the ratio x between metal ions and hexacyano metallates.
In some Prussian blue analogues such as #ce("Mn^II [Co^III (CN)_6]_⅔") this fraction of vacancies is impossible to arrange in a way that follows
the symmetries of the system. The symmetry of the crystal is broken.

The concept of an average structure simplifies this.
Instead of providing precise information about the positions in the whole crystal, the average structure provides probabilistic information about the
occupation of a position. In the example above, the probability of occupation of a hexacyano metallate is given by x = ⅔.
But by considering only average structure, a lot of information about a material is lost.

In fact, these vacancies are not uniformly distributed throughout the crystal. For example, we might expect the vacancies to be more stable if they
are arranged along certain direction or at certain distances. These correlations are essential as they influence the properties of the material.
Such as porosity, diffusion coefficients, absorption coefficients and more, which find application ranging from medicine to the development of new
batteries.

== Diffuse Scattering <scattering>
X-ray diffraction is a common technique to determine the crystal structure as well as the lattice parameters of crystals.
The technique uses the diffraction pattern generated by shining an X-ray beam through a single crystal.
The pattern captured using this method reflects the symmetry of a crystal and is the Fourier transform of the electron density.
The complex structure factor $F(arrow(h))$ can be calculated by:
$ F(arrow(h)) = integral_(RR^3) rho(arrow(x)) exp(2 pi i arrow(h) dot arrow(x)) d x $
However, only the magnitude $I(arrow(h)) = abs(F(arrow(h)))^2 = F(arrow(h)) F^*(arrow(h))$
of the structure factor can be measured in a diffraction experiment.
For perfectly ordered crystals, the intensity measured is zero everywhere except at certain points. 
From these diffraction peaks, the Laue group of the crystal can be determined.
Using iterative approaches such as charge flipping, the phase of the signal can be reconstructed and the average structure can be determined.@superfilp



=== 3D Pair Distribution Function <PDF>
The 3D pair distribution function (3D-PDF) is a function used to find correlations in structures.
In the application for crystallography, it specifically refers to the autocorrelation of the electron density $rho$.

$ text(P D F)(arrow(x)) = integral_(RR^3)rho(arrow(xi))rho(arrow(xi)-arrow(x)) d arrow(xi) $

The PDF essentially shifts the electron density with respect to itself and then compares the shifted function to itself.
A high value at point $arrow(x)$ means that the structure is highly correlated with a shift by $arrow(x)$.

In diffraction experiments, the PDF is convenient as it is easily obtained as the inverse Fourier transform of the intensities measured.
As such, no calculation of the phase factors is necessary.

$
cal(F) ^(-1)(I(arrow(h))) &= cal(F)^(-1) [cal(F)(rho(arrow(x))) cal(F)^*(rho(arrow(x)))] \
  &= cal(F)^(-1) [cal(F)(rho(arrow(x))) cal(F)(rho(-arrow(x)))]\
  &= rho(arrow(x)) * rho(-arrow(x))
$

The diffraction pattern from an experiment using a disordered crystal not only contains the Bragg peaks but a diffuse pattern in between the peaks.
This diffuse part of the diffraction pattern contains information about the correlations described in the section on disordered crystals.
If the average structure is known, the difference between the average PDF and PDF calculated from the measured pattern can be formed.
This function is called the difference pair distribution function #sym.Delta PDF.

The PDF and #sym.Delta PDF can be calculated from measurements from powder diffraction. This, however, leads to a loss of information because 
powder diffraction inherently averages the measurement radially.

The 3D -#sym.Delta PDF need to be measured from more complicated single crystal experiments using scaning techniques, but allow more 
information to be gained from the data analysis.

The #sym.Delta PDF can be used by programs like Yell to find the local correlations of structure.
This is where the main work of the program comes in. The program Yell needs to be given the multiplicity of pairs in a crystal to properly expand
the PDF.


= The Project <project>
The program developed as part of this thesis covers the calculation of pair multiplicities.
The next sections cover the mathematics behind space groups and how the program was implemented.
To this end, affine and Euclidean spaces and transformations of them will be explained. 
Furthermore, this report contains the explanation of Wyckoff positions and pairs of sites.
Additionally, examples of pairs in line groups, wallpaper groups and space groups are given and explained.

= Mathematical Description of Crystals <math>
In the following sections, the described spaces are assumed to be three-dimensional, but all concepts can trivially be extended to one and two spaces.
Higher dimensional spaces are possible, but exceed the scope of this work.

A Euclidean space is an affine space with a scalar product.
In simple terms, an affine space is a vector space whose origin is forgotten, allowing translations to be part of a linear transformation. @Berger_2004
The scalar product then allows for the calculation of distances and angles, producing a Euclidean space from an affine space.

An affine space consists of an associated point space $A$ and an associated vectors space $arrow(A)$.
These can be thought of as a collection of points and the group of translations acting on them.
For vectors the notation $bold(v) in arrow(A)$, for a translation from point $X in A$ to $Y in A$ the notation $arrow(A B)$ used,
this can also be thought of as map $arrow(dot dot): A times A -> arrow(A)$.

In the point space there is one special position $O$ which is the point relative to which all coordinates are given.
The coordinates of a point are then given in terms of a translation $arrow(v) in arrow(A)$ away from the origin,
for which we know how to work with coordinates from linear algebra.
This vector is called the position vector.
Thus, we can assign a coordinate triple to a point $P$ based on the position vector $arrow(O P)$. 
Note that to specify a basis for an affine space, a basis for the associated vector space and an origin need to be specified.

In the context of crystallography points are usually refered to as positions. This is reflected in the following section and the naming of types
in the program.

A general linear transformation of a point $X$ can then be described by a linear transformation of the position vector plus a translation.

Following the convention used by the International Table for Crystallography, the transformation will be notated in the following way:
$ cal(Q) = (bold(Q), bold(q)) $
$ arrow(O tilde(X)) = arrow(O cal(Q) X) = bold(Q) arrow(O X) + bold(q) $

Note that the inversion of $cal(Q)$ is given by $cal(Q)^(-1) = (bold(Q)^(-1), -bold(Q)^(-1) bold(q))$, the composition of two translations is given by 
$cal(Q) cal(P) = (bold(Q P), bold(Q p + q))$ and the identity is given by $cal(I) = (bold(I), bold(o))$
Proofs for these equations, as well as associativity, are given in the appendix.

The scalar product is defined on the vector space with the usual axioms for the scalar product.

The coordinates of the affine space are called Cartesian, if the coefficients of the associated vector space are given in an orthonormal basis.
In this case, the scalar product is the dot product.

If the coordinates are not Cartesian, the scalar product is given by:
$ innerproduct(bold(v), bold(w)) = bold(v)^T bold(G) bold(w) $
Where G is a symmetric 3 by 3 matrix known as the metric tensor.
The norm is naturally induced by the scalar product $norm(bold(v)) = sqrt(innerproduct(bold(v), bold(v)))$
and distances between points $P, Q$ can be measured by taking the norm of the translation vector between them $d(P, Q) = norm(arrow(P Q))$.

== Crystals Structures <crystals>
In crystallography, a crystal structure is an ordered structure which fulfills three linearly independent translation symmetries
$bold(t)_1, bold(t)_2, bold(t)_3 in RR^3$.

The basis for the vector space can then be given in terms of these vectors. However, in crystallography the basis is sometimes given in terms of
combinations of these vectors for an easier description of symmetries in the crystal structure.
Since we aren't concerned about the orientation of the space we can give the basis in terms of the length of the basis vectors $a, b, c$ and
the angles between them $alpha, beta, gamma$ known as fractional coordinates.

The metric tensor $G$ can be calculated from these values in the following way:
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


=== Isometries <isometries>
Isometries are distance and angle preserving transformations on a collection of points and can be represented by an affine transformation.
For an affine transformation to be an isometry, the following statement must hold for all $X, Y in P$:
$
d(X, Y) &= d(cal(Q)X, cal(Q)Y)\
norm(accent(X Y, arrow)) &= norm(accent(cal(Q)X cal(Q)Y, arrow))\
&= norm(bold(Q) accent(O Y, arrow) + bold(q) - bold(Q) accent(O X, arrow) - bold(q))\
&= norm(bold(Q)(accent(O Y, arrow) - accent(O X, arrow)))\
&= norm(bold(Q) accent(X Y, arrow))
$

As expected, the translation vector $bold(q)$ is free, as a translation doesn't affect the distances between points.

Since X and Y are arbitrary points, so is the vector $arrow(v)$ between them.
Using the definition of the metric:
$
norm(bold(v)) &= norm(bold(Q v))\
norm(bold(v))^2 &= norm(bold(Q v))^2\
bold(v)^T bold(G) bold(v) &= (bold(Q v))^T bold(G) (bold(Q v))\
&= bold(v)^T bold(Q)^T bold(G) bold(Q) bold(v)
$

Since $bold(v)$ is arbitrary, this equation leads to the following condition on $bold(Q)$:
$ bold(G) = bold(Q)^T bold(G) bold(Q) $
Which might be more familiar to the reader in Cartesian coordinates where $bold(G) = bold(I)$
$ bold(I) = bold(Q)^T bold(Q) $
Which is the condition for $bold(Q)$ to be orthogonal.
Orthogonal matrices have determinant $plus.minus 1$ the same is the case for the transformation matrix by:
$
det(bold(G)) = det(bold(Q)^T bold(G) bold(Q)) = det(bold(Q)^T) det(bold(G)) det(bold(Q))\
1 = det(bold(Q)^T) det(bold(Q)) = det(bold(Q))^2\
=> det(bold(Q)) = plus.minus 1
$

In addition to the orthogonality condition, we also know that the matrix $bold(Q)$ must map integer vectors to integer vectors.
In fact, for all space groups a representation using only matrices from ${-1, 0, 1}^(3 times 3)$ can be found.

For applications in computation, it is necessary to find a finite representation of space groups.
One such representation can be found in what will be referred to as the normalized space group of $frak(G)$ denoted $tilde(frak(G))$.
It is defined as the quotient group of $frak(tilde(G))=frak(G) slash frak(T)$ where $frak(T) = {(bold(I), bold(v))| v in ZZ^3}$ is 
the group generated by translations along the basis vectors.
In the same fashion, the quotient vector space $tilde(arrow(A)) = arrow(A) slash {vec(x, y, z) | x, y, z in ZZ}$
and the quotient point space $tilde(A) = A slash {vec(x, y, z) | x, y, z in ZZ}$ is defined.
The order of a space group is defined in such terms.


== Wyckoff Positions <positions>
In simple terms, Wyckoff positions describe how many times a site occurs in a unit cell and what symmetries it must follow.

Mathematically, a Wyckoff position can be described by an orbit and a stabilizer.
An orbit $O_frak(G)(P) = {cal(G) P | cal(G) in frak(G)}$  is the set of points which the point $P$ is mapped to by elements of the group $frak(G)$.
The stabilizer $S_frak(G)(P) = {cal(G) | cal(G) P = P, cal(G) in frak(G)}$ is the subgroup of $frak(G)$ for which the point $P$.
Wyckoff sites are commonly given in terms of their position in the unit cell.
Thus, the normalized space group must be used to determine their multiplicity.

The multiplicity $n$ of a site is given by $abs(O_tilde(frak(G))(P))$ and the site symmetry at point $P$ is given by $S_tilde(frak(G))(P)$.

A position $P$ is called general if the $S_tilde(frak(G))(P)$ only contains the identity. By the orbit stabilizer theorem its multiplicity is $abs(tilde(frak(G)))$ the order of the space group $frak(G)$.

A special position $P$ is a position with a nontrivial stabilizer $S_tilde(frak(G))(P)$. Any object at the point $P$ in crystal must at least have an internal symmetry of its site symmetry, otherwise the symmetry of the crystal is broken.

== Pairs in Crystals <pairs>
In this work, only pairs between symmetry equivalent positions are considered.
Let $P$ be a point in a lattice and $bold(p)$ be a vector from $P$ to a symmetry related position $tilde(P) = cal(G) P$ for some $cal(G) in frak(G)$
where $frak(G)$ is the space group of the crystal.
Then we can describe this pair $P$ in terms of $(P_1, P_2)$. The action of $cal(G)$ on a pair is then defined by:
$
tilde(P) = cal(G) P = (cal(G)P_1, cal(G)P_2)
$
Note that this describes ordered pairs. This can be remedied by defining the following equivalence relation:
$ p ~ q <=> (P_1 = Q_1 and P_2 = Q_2) or (P_1 = Q_2 and P_1 = Q_2) $

Let the expansion $E_frak(G)(p))$ be the set of all symmetry equivalent pairs to the pair $(P, bold(p))$. The expansion of a point contains
two types of pairs. The first type is generated by the stabilizer of $P$.
By definition, any element of the stabilizer of $P$ does not change the point $P$. These pairs are of the form 

= Examples of Pair Multiplicities <examples>

== One-dimensional Examples <examples1>
In the one-dimensional case, there are only two space groups. 

=== Line Group p1 <examples1.1>
#figure(
  image("figs/p1.svg"),
  caption: [Pairs in line group p1],
)<p1>
In p1 all symmetry operations are translations by multiples of the lattice vector $arrow(l)$, it is isomorphic to $(ZZ, +)$.
Thus, all pairs of symmetry related position can be uniquely described by a position within the unit cell and the translation $n arrow(l)$ between them.
Figure @p1 shows how, starting from one position, there are two possibilities to construct each type of pair.
Thus, the multiplicity of such any pair is 2. Per unit cell, there is one starting position $bold(p)$ 
with the paired position at $bold(p') = bold(p) + n arrow(l)$.

=== Line Group p1m <examples1.2>
In p1m additional to the translations by multiples of the lattice $arrow(l)$ there are mirror operations which mirror the line at positions $bold(m) + n arrow(l)$. This group is isomorphic to the direct product $(ZZ, +) times ({-1, 1}, dot)$.
In p1m there are two Wyckoff positions.
The general positions with Wyckoff multiplicity 2, in other words it exists twice per unit cell,
and the special positions at $bold(m)$ and $bold(m) + 1/2 arrow(l)$, which have Wyckoff multiplicity 1 and the site symmetry m.
Here the calculation of the pair multiplicity must be treated differently for origins at general and special positions.

#figure(
  image("figs/p1m_g.svg"),
  caption: [Pairs of general positions in line group p1m],
)
The general position has site symmetry 1. For general positions, the pairs can be categorized further into two categories.
For pairs of type $n arrow(l)$, there exist two possibilities for each pair, one in the positive direction on in the negative.
Since there are two starting positions per unit cell, the pair multiplicity is 4.
For pairs which do not occur over a basis vector length, there is only one possibility for the construction of such a pair.
Considering the two possible starting positions, the pair multiplicity is 2.

#figure(
  image("figs/p1m_s.svg"),
  caption: [Pairs of special positions in line group p1m],
)
Special positions in contrast have site symmetry m. Thus, each pair can be built once in each direction.
Since the Wyckoff multiplicity is 1, pairs built from special positions have pair multiplicity 2.

== Two-dimensional Example <examples2>
#figure(
  image("figs/p2mg.svg"),
  caption: [Pairs in the wallpaper group p2mg],
)
As a next example, consider pairs of positions on the mirror axis of the wallpaper group p2mg.
The coordinates of such a point are ($x, 1/4$) with its symmetry equivalent point at ($1-x, 3/4$).

== Three-dimensional Example <examples3>
The three-dimensional examples shown in figure @fig3d are pair expansions from the site $[0,0,0]$ in a primitive cubic structure with the corresponding
space group Pm3\u{0305}m. As three-dimensional structures are harder to visualize, no more complicated examples are shown here.
#figure(
  image("figs/p2mg.svg"),
  caption: [still TODO],
)<fig3d>

Since there is only one $[0, 0, 0]$ position in a primitive centered cell, the $[0, 0, 0]$ pair has multiplicity one.
The $[1, 0, 0]$ pairs have a pair multiplicity of 6, the $[1, 1, 0]$ pairs have a multiplicity of 12 and the $[1, 1, 1]$ pairs have a multiplicity of 8.
Note the correspondence between these pair of multiplicities and the number of faces, edges, and vertices of a cube.



= The Program <program>
The following section presents the implementation of the objects described in section @math.
The code examples are written in Rust-like pseudo language, which ignores Rust borrow checker to simplify the code.
Additionally, the code of the project includes some additional structs and steps in the algorithms to allow for more performant code.
The code can be found on GitHub #link("https://github.com/max-kay/bachelor_thesis")[github.com/max-kay/bachelor_thesis].

== Affine Space Implementation <affine_impl>
All coordinates used in the program are rational numbers implemented as a pair of `i32`, which are always represented in reduced form and positive denominator. Rational numbers were chosen instead of floating-point numbers to allow for exact calculations and comparisons of coordinates.
From triplets of rational numbers, vectors `Vec3` and positions `Pos3` and operations needed for them to form an affine space were defined.

Similarly, matrices `Mat3` and affine transformations `Affine3` were implemented, where a matrix is represented as a list of nine rational numbers and an affine transformation is represented as a pair of a matrix and a vector, as described above.

Additionally, a struct `Bounds3` was defined. It is used for the implementation of the remainder. Here 3 integers were chosen as it only makes sense to 
produce integer bounds in a crystal lattice.
The remainder operation was defined on positions `Pos3` such that each the result of the operation `Pos3(x', y', z') = Pos3(x, y, z) % Bounds3(a, b, c)`
produces coordinates `x'`, `y'`and `z'` in the ranges $[0, a)$, $[0, b)$ and $[0, c)$ respectively.
Note that if $a, b, c = 1$ the remainder produces positions in the unit cell.
For vectors `Vec3` the operation `Vec3(x', y', z') = Vec3(x, y, z) % Bounds3(a, b, c)` such that it produces coefficients
`x'`, `y'`and `z'` in the ranges $(-a/2, a/2]$, $(-b/2, b/2]$ and $(-c/2, c/2]$ respectively.
The different implementation of the remainder for `Vec3` allows vectors to be represented by the shortest vector in the equivalence class. 
The remainder operation was defined on affine transformations `Affine3` too. Here the operation leaves the matrix untouched but takes the remainder of the associated vector.

== Space Group Implementation <space_group_impl>
The space group is represented as the normalized space group, where all operations take place modulo `Bounds3(1, 1, 1)`.
Using this representation, there is a finite list of symmetry operations and
all other symmetries can be easily generated by the multiplication of integer translation with the operations already present.
The space group can be constructed from a list of generators by the following simple algorithm.

```rust
fn isometry_group_new(generators) -> IsometryGroup {
  bounds = Bounds3(1, 1, 1);
  symmetries = [];

  for sym in generators {
    normalized_sym = sym % bounds;
    if normalized_sym not in symmerties {
      symmetries.push(normalized_sym);
    }
  }

  added_new = true;

  while added_new {
    added_new = false;
    for sym_1 in symmetries {
      for sym_2 in symmetries {
        new_op = sym_1 * sym_2 % bounds;
        if new_op not in symmetries {
          symmetries.push(new_op);
          added_new = true;
        }
      }
    }
  }
  return IsometryGroup{ symmetries };
}
```

First, this algorithm brings the generator into the form required and checks for duplicates.
Then it tries to close the group by going through all multiplications of elements. If the new operation is not present, it is appended to the list.
This process is repeated until no new operation was added to the list in one pass through.


This algorithm is far from optimal, but because of the small numbers of elements in normalized space groups this is of no concern.
If the wrong symmetries are supplied, the group might not be finitely closable. In this case, the algorithm described above results in an infinite loop.
As a safety precaution, an upper limit of 10000 iterations was placed on the outer while loop.

The struct `IsometryGroup` contains the method `symmetries_in_bounds` which produces all symmetries within the bounds provided.

== Wyckoff Positions <positions_impl>

For a position in the unit cell, the Wyckoff position can be calculated from the starting position `position` and the space group `group` by the following process.

```rust
fn site_new(position, group) -> Site {
  bounds = Bounds3(1, 1, 1);
  position = position % bounds;
  orbit = [position];

  for sym in symmetries {
    new_pos = sym * position % bounds;
    if new_pos == position {
      stabilizer.push(sym);
    } else if new_pos not in orbit {
      orbit.push(new_pos);
    }
  }

  return Site { position, orbit };
}
```

The Wyckoff multiplicity is the length of the orbit, as they represent all symmetry equivalent positions in the unit cell.
A struct called `Site` is used to collect the starting position, the orbit and the stabilizer.

The struct `Site` contains the method `orbit_in_bounds` which produces the orbit within the bounds provided.

== Pair Expansion Implementation <pairs_impl>
The constructor for `Expansion` takes the starting position, the vector associated with the pair and the isometry group.
After translating the starting position into the unit cell. The `Site` is constructed from the starting position and the space group.
The pair vector is added to the starting position and the resulting position is checked against the orbit of the `Site`.


Then the following algorithm is applied to the pair to determine its expansion:

```rust
fn expansion_new(origin_site, end_position, group, bounds) -> Expansion {
  expansion = [];
  start_position = orgin_site.position;

  for sym in group.symmetries_in_bounds(bounds) {
    new_p1 = (op * start_position) % bounds;
    new_p2 = (op * end_position) % bounds;
    if new_p1 == start_position and new_pos2 not in expansion {
      expansion.push(new_p2);
    }
    if new_p2 == start_position and new_pos1 not in expansion {
      expansion.push(new_p1);
    }
  }

  return Expansion {
    is_ab_pair: not site_contains_pos(origin_site, end_position),
    origin_site,
    vec: end_position - origin_position,
    expansion,
  }
}
```
This algorithm applies each operation to the starting and end position of the pair and then checks either one of the positions was mapped to the
original starting position, if so, it appends the other position to the expansion.
Subsequently the end position is tested against the origin_site to see if the pair is a single site or mixed site pair.

The pair multiplicity now can now be calculated by multiplying the length of the expansion with the length of the orbit of the pairs and 
with an additional factor of 2 for mixed pairs, as each pair can be constructed from either of the two sites.

== Pair Calculation <main_algo>
As arguments, the program takes a space group, the positions from which the pairs need to generated, the bounds applied to the problem and optionally
a boolean, which determines if pairs of mixed sites should be calculated.

The program starts by calculating and deduplicating the sites from the given positions.

Then the program produces all single site pairs and continues to do the same for all pairs of sites if mixed pairs should be calculated.
The algorithm is implemented as follows:

```rust
fn calculate_multiplicities(
  group,
  positions,
  bounds,
  construct_mixed_pairs
) -> [Expansion] 
{
  sites = [];
  for pos in positions {
    pos = pos % Bounds3(1, 1, 1);
    if not contains_pos(sites, pos) {
      sites.push(new_site(position, group, bounds));
    }
  }

  expansions = [];
  for site in sites{
    expansion.append(construct_expansions(site, site, group, bounds))
  }

  if construct_mixed_pairs{
    for i in 0..len(sites) {
      for j in (i+1)..len(sites) {
        expansion.append(construct_expansions(sites[i], site[j], group, bounds));
      }
    }
  }

  return expansions;
}
```
The implementation of `contains_pos` iterates through all sites in the list and checks if the orbits of any of the sites contains the position.

The function `construct_expansions` iterates through the orbit of `site_2` expanded to the bounds,
checks if any of the constructed expansions already contains the pair and if not, it pushes the expansion on the list.


```rust
fn construct_expansions(site_1, site_2, group, bounds) -> [PairExpansion] {
  expansions = [];
  origin_position = site_1.position;
  for pos in site_2.orbit_in_bounds(bounds) {
    if not contains_pair(expansions, origin_position, pos) {
      expansions.push(PairExpansion::new(origin_position, pos, group, bounds));
    }
  }
  return expansions;
}
```
`contains_pair` iterates through all expansions in the list and checks if any of the expansions contains the pair by calling `expansion_contains_pair`,
which checks if the list of expansion positions of the pair contains the end position of the pair. Note that for single site pairs this requires the
pair to be always started from the same position and for mixed site pairs that the pair is always started from the same position from the same site.
This is guaranteed by the rest of the program.

=== Note on Bounds <note_bounds>
The implementation of this program requires bounds to be specified to construct the pair expansions.
This has consequences for the pair multiplicities.

As an example, consider the pair with vector `[2, 0, 0]`. In an infinite crystal, the pair with vector
`[-2, 0, 0]` can always be constructed from this pair. But in the bounds `Bounds3(4, 4, 4)` these vectors are considered equivalent.
Thus, the pairs multiplicities constructed in bounds, are different from those in an infinite crystal.
For the main application of this program for the correlation fitting program Yell, this is an advantage. As Yell works with similar bounds and needs
the pair multiplicities calculated in bounds.

== The File Format <file_format>
The file format was implemented using the Rust crate `pest`. 
`pest` allows for the grammar to be defined in an external file, which needs to be included in the code.
The rest of the code needed for the implementation of the code is then automatically generated using Rusts macro system,
allowing for easy changing of the grammar.

An example file is shown below:

#let text = read("../files/input/commented_example")
#raw(text, block: true, lang: "rust")

The input file can be divided into four sections.

In the `Space Group` section, the symmetry elements are specified. The elements are given per index, separated by commas and terminated by semicolons.
Pure translations can be written simply as vectors.
Note that the identity `x, y, z;` and the translations `1, 0, 0;`, `0, 1, 0;` and `0, 0, 1;` are implicitly included in the space group.

In the `Positions` section, the positions are defined.
Similarly to the symmetry operations, the positions are given as comma separated values terminated by semicolons.
Note that all coefficients need to be given as rational numbers. The program currently does not support floating-point coefficients.

The `Bounds` section defines the bounds which are applied to the problem, as described in section @note_bounds.
Here, the coefficients need to be positive integers.

The `Mixed Pairs` section is optional and can only contain `true` or `false`. This boolean specifies if mixed pairs should be calculated.
If this section is not specified, it defaults to `false`.


= Conclusion

A program to calculate the pair multiplicities was successfully developed. To this end, positions and vectors from a three-dimensional affine space,
as well as transformations on them were implemented.
Furthermore, algorithms to calculate Wyckoff positions and pair expansions were developed and implemented.
These implementations were used to create a program to construct all possible pairs of positions in any bounded space group.
Additionally, the pair multiplicities were explained using one-, two- and three-dimensional examples.

The code for this project (including the code for this report and the slides for the talk)
can be found in a GitHub repository #link("https://github.com/max-kay/bachelor_thesis")[github.com/max-kay/bachelor_thesis].
The program can be used in two ways.
Either it is used as a command line tool. Please follow the instructions in the `README.md` on GitHub to build the tool from source.
Or in a simple website, hosted through GitHub Pages #link("https://max-kay.github.io/bachelor_thesis/")[max-kay.github.io/bachelor_thesis].

#bibliography("ref.bib")


= Appendix


*Claim*
$cal(Q) cal(P) = (bold(Q P), bold(Q p + q))$ is the formula for the composition of affine transformations.

*Proof*
Let $X$ be any arbitrary point.
$
cal(Q) cal(P) arrow(O X) &= cal(Q) (bold(P) arrow(O X) + bold(p))\
&= bold(Q)(bold(P) arrow(O X) + bold(p)) + bold(q)\
&= (bold(Q)bold(P)) arrow(O X) + (bold(Q) bold(p) + bold(q))\
&=(bold(Q P), bold(Q p + q)) arrow(O X)
$


*Claim*
The inverse of $cal(Q) = (bold(Q), bold(q))$ is given by $cal(Q)^(-1) = (bold(Q)^(-1), -bold(Q)^(-1) bold(q))$.

*Proof*
Consider the following composition using the formula proofed above.
$
cal(Q)cal(Q)^(-1) &= (bold(Q), bold(q)) (bold(Q)^(-1), -bold(Q)^(-1) bold(q))\
&= (bold(Q) cal(Q)^(-1), bold(q) - bold(Q)bold(Q)^(-1) bold(q))\
&= (bold(I), bold(q) -bold(q)),
&= cal(I)
$

*Definition*
$H$ is a normal subgroup of $G$ if $g h g^(-1) in H$ for all $g in G$ and $h in H$.

*Claim*
Let $frak(H)$ be a space group and $frak(T) = { (bold(I), bold(v)) ; bold(v) in ZZ^3 }$ the group of integer translation.
Then $frak(T)$ is a normal subgroup of $frak(H)$.

*Proof*
Let $cal(H) in frak(H)$ be an arbitrary symmetry element and $cal(T) in frak(T)$ be an arbitrary integer translation.
$
cal(H)cal(T)cal(H)^(-1) &= (bold(H), bold(h))(bold(I), bold(t))(bold(H)^(-1), -bold(H)^(-1)bold(q))\
&= (bold(H), bold(h))(bold(H)^(-1), -bold(H)^(-1)bold(q) + bold(t))\
&= (bold(H) bold(H)^(-1), bold(H)(-bold(H)^(-1)bold(q) + bold(t)) + bold(q))\
&= (bold(I), bold(H)bold(t))
$

To be a valid space group, $bold(H)$ must map integer vector to integer vector. Thus, $cal(H)cal(T)cal(H)^(-1) = (bold(I), bold(H)bold(t))$
is an element of $frak(T)$. qed
