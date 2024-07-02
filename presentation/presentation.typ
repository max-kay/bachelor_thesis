#import "@preview/polylux:0.3.1": *
#import "theme.typ": *

#show: theme

#show raw: set text(size: 0.6em)

#title-slide[
  = Symmetry Implementation for Pair Distribution Function
  == Max Krummenacher
  July 2024
]


#slide[
  #align(center)[
    = Disordered Crystals
    #image("figs/disorder.png", height: 80%)
  ]
]

#slide[
  = Diffraction
  #grid(columns: (1fr, 1fr),
    align(horizon + center, [#image("figs/structure.png", height: 65%)#v(10pt) Structure]),
    align(horizon + center, [#image("figs/diffraction.png", height: 65%)#v(10pt) Diffraction Pattern]),
  )
]

#slide[
  = Pair Distribution Function
  #set align(center)
  #only("1,2")[
    $ "PDF"(arrow(x)) = integral_(RR^3) rho(arrow(xi)) rho(arrow(xi) - arrow(x)) d arrow(xi) $
  ]
  #only(2)[
    #grid(columns: (1fr, 1fr))[
      #image("figs/struc.png", height: 60%)
    ][
      #image("figs/pdf.png", height: 60%)
    ]
  ]
  #only(3)[
  #set align(center + horizon)
    #grid(columns: (1fr, 1fr))[
      #image("figs/struc_marked.png", height: 80%)
    ][
      #image("figs/pdf_marked.png", height: 80%)
    ]
  ]
]


#slide[
  #align(center + horizon)[
    = Wyckoff Positions
    #image("figs/wyckoff.svg", height: 70%)
    Wallpaper Group p2mg
  ]
]

#slide[
  = Examples
  #align(horizon)[
    - Line Group p1
    - Line Group p1m
    - Wallpaper Group p2mg
    - Space Group Pm$overline(3)$m
  ]
]

#slide[
  #align(center + horizon)[
    == Line Group p1
    #image("figs/p1.svg")
  ]
]


#slide[
  #align(center + horizon)[
    == Line Group p1m General Position
    #image("figs/p1m_g.svg")
  ]
]

#slide[
  #align(center + horizon)[
    == Line Group p1m Special Position
    #image("figs/p1m_s.svg")
  ]
]

#slide[
  #grid(columns: (4fr, 1fr))[
    #image("figs/p2mg.svg")
  ][
    #align(horizon + center)[
      == Wallpaper Group p2mg
      #image("figs/legend.svg")
    ]
  ]
]

#centered-slide[
  = Space Group Pm$overline(3)$m
  #grid(columns: (1fr, 1fr, 1fr), rows: (8fr, 1fr), gutter: 0pt, stroke: none,
    align(horizon, [#image("figs/100.png", height: 70%)\ $angle.l 100 angle.r: 6$]),
    align(horizon, [#image("figs/110.png", height: 70%)\ $angle.l 110 angle.r: 12$]),
    align(horizon, [#image("figs/111.png", height: 70%)\ $angle.l 111 angle.r: 8$]),
  )
]

#slide[
  = Example Yell Input
  #set text(size: 0.29em)
  #columns(6)[
    #let input = read("input.txt")
    #raw(input)
  ]
]

#slide[
  = The Algorithms and their Implementation

  #align(horizon)[
    - Isometries
    - Space Groups
    - Bounds
    - Wyckoff Positions
    - Pair Expansions
    - Main Algorithm
  ]
]

#slide[
  == Isometry Implementation

  #line-by-line(start: 2)[
    - Rational Numbers $p/q$
    - Vectors $bold(a)$
    - Matrices $bold(A)$
    - Positions $X$
    - Affine Transformations $cal(A) = (bold(A), bold(a))$
    $ tilde(X) = cal(A)X = bold(A) X + bold(a) $
    - Isometries
    #align(center)[special affine transformation which preserve distances and angles]
  ]
]

#slide[
  #columns(2)[
    == Space Groups
    Generators
    #grid(columns: (1fr, 1fr))[
      #only(2)[#image("figs/generator.svg")]
      #only("3-")[#image("figs/generator2.svg")]
    ][
    #only(4)[#image("figs/arrow0.svg")]
    #only("5-")[#image("figs/arrow2.svg")]
    ]
    #uncover("6-")[
      Space groups are infinite!\
      What should we do?
    ]
    #colbreak()
    #uncover("7-")[
      == Bounds
      - act on positions, vectors, and affine transformations
      - allow finite representation of space group
      - for unitcell and space group bound to $[1, 1, 1]$
    ]
  ]
]


#slide[
  == Wyckoff Positions
```rust
fn site_new(position, group) -> Site {
  bounds = Bounds3(1, 1, 1);
  position = position % bounds;
  orbit = [position];
  stabilizer = [];

  for sym in group.get_symmetries() {
    new_pos = sym * position;
    if new_pos % bounds == position {
      stabilizer.push(symmetry_from_translation(position - new_pos) * sym);
    } else if new_pos % bounds not in orbit {
      orbit.push(new_pos % bounds);
    }
  }

  return Site { position, orbit };
}
```
]

#slide[
  #columns(2)[
    #line-by-line[
      == Pair Expansions
      - represents all equivalent pairs starting at a certain position
      - generated in a similar fashion to the orbit wyckoff positions
      - can be compared against other pairs
    ]
    #colbreak()
    #line-by-line(start: 5)[
      == Main Algorithm
      - generates the space group
      - determines the Wyckoff position of the sites
      - determines all possble pairs between these position
      - calculates the pair multiplicity
    ]
  ]
]
#slide[
  #set align(center)
  #only(1)[#link("http://localhost:8080/")[Website]]
  #only(2)[Thank you for your Attention!]
  #image("figs/p2mg.svg", height: 80%)
]

