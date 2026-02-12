#import "@preview/touying:0.6.1": *
#import themes.stargazer: *

#import "@preview/numbly:0.1.0": numbly
#import "@preview/fletcher:0.5.8": *
#import "@preview/algorithmic:1.0.7": *
#show: style-algorithm

#show: stargazer-theme.with(
  aspect-ratio: "16-9",
  config-info(
    title: [Algorithmic Performance Optimisation],
    subtitle: [Binary Search Beyond Big-O],
    author: [BuzzY\_],
    date: datetime.today(),
    bibliography-as-footnote: bibliography(title: [Bibliography], "bib.yaml"),
  ),
  config-colors(
    primary: rgb("#333333"),
    primary-dark: rgb("#222222"),
    secondary: rgb("#ffffff"),
    tertiary: rgb("#777777"),
    neutral-lightest: rgb("#ffffff"),
    neutral-darkest: rgb("#000000"),
  ),
)

#set heading(numbering: numbly("{1}.", default: "1.1"))

#title-slide(author: [BuzzY\_])

== Introduction

#slide[
  - Optimising software:
    - Why theory is important?
    - Why we need to go beyond theory to trully optimise software?

  The study of Binary Search.
]

#outline-slide()

#speaker-note[
  With this presentation, I want us to go on a journey about algorithmic optimisations.

  We are going to talk about how algorithmic theory is important to write efficient software and how we need to go deeper than just actuall theory to make trully optimised sofware.

  Do to this, we are going to go through different level of optimisation of a well know algorithm, binary search.
]

= Binary search basics

== Purpose of the algorithms

#slide[
  - We want to find in a sorted array $a$:
    - $min(i)$ such as $x lt.eq.slant a[i]$ for lower bound
    - $max(i)$ such as $a[i] lt.eq.slant x$ for upper bound
    Where $x$ is our target.
]

== An implementation

#slide[
  To find our $i$, we use a 2 pointer technique. \
  We define $l$ beginning as the leftmost element and $r$ as the rightmost one.
]


#slide[
  To get around edgecases _(see @a_ds)_, we "add" $2$ more elements in the array:
  - $- infinity$ for the leftmost position and starting point of $l$.
  - $+ infinity$ for the rightmost position and starting point of $r$.
]

#slide[
  After initialisation we have:
  #v(5%)
  #align(top + center, diagram(
    node((1, 0), $- infinity$),
    node((1.5, 0), $5$),
    node((2, 0), $8$),
    node((2.5, 0), $dots.h.c$),
    node((3, 0), $15$),
    node((3.5, 0), $18$),
    node((4, 0), $+ infinity$),

    node((1, 0.75), text(red, $l$)),
    node((4, 0.75), text(red, $r$)),

    edge((1, 0.75), (1, 0), "->", stroke: red),
    edge((4, 0.75), (4, 0), "->", stroke: red),
  ))
]

#slide[
  Execution:
  - We loop while $r > l + 1$ _(again see @a_ds)_.
  - We compute $m colon.eq l + floor((r-l) / 2)$
  - We update $l$ or $r$ as $m$ following the following invariants:
    - $(a[l] lt x lt.eq.slant a[r])$  for lower bound
    - $(a[l] lt.eq.slant x lt a[r])$ for upper bound
]

#slide[
  === Lower bound
  #figure(
    [
      #set text(size: 18pt)

      #algorithm-figure("Binary Search for lower bound", vstroke: .2pt + luma(200), {
        Procedure("Binary-Search", ("a", "x"), {
          Assign[$l$][$-1$]
          Assign[$r$][$n$]
          While($r gt.eq.slant l + 1$, {
            Assign([m], [$floor(l + (r - l) / 2)$])
            IfElseChain(
              $a ["m"] gt.eq.slant x$,
              { Assign[$r$][$"m"$] },
              { Assign[$l$][$"m"$] },
            )
          })
          Return[*r*]
        })
      })],
    caption: [Pseudo-code implementation for lower bound],
  )
]

#slide(config: config-page(margin: (bottom: 1.4em)))[
  #figure(
    image("assets/b_lwb.svg", width: 63%),
    caption: [Rust implementation for lower bound],
    gap: 0.2em,
    placement: bottom,
  )
]

#slide[
  === Upper bound
  #figure(
    [
      #set text(size: 18pt)

      #algorithm-figure("Binary Search for upper bound", vstroke: .2pt + luma(200), {
        Procedure("Binary-Search", ("a", "x"), {
          Assign[$l$][$-1$]
          Assign[$r$][$n$]
          While($r gt.eq.slant l + 1$, {
            Assign([m], [$floor(l + (r - l) / 2)$])
            IfElseChain(
              $a ["m"] gt x$,
              { Assign[$r$][$"m"$] },
              { Assign[$l$][$"m"$] },
            )
          })
          Return[*r*]
        })
      })],
    caption: [Pseudo-code implementation for lower bound],
  )
]

#slide(config: config-page(margin: (bottom: 1.4em)))[
  #figure(
    image("assets/b_upb.svg", width: 63%),
    caption: [Rust implementation for upper bound],
    gap: 0.2em,
    placement: bottom,
  )
]

#slide[
  #align(center)[Those implementation are the best we can do from a theorical strandpoint.]

  #align(center)[*Run in $Omicron (log n)$*]
]

= Not so optimized

#slide[
  This is the best in theory, but we can do better. \
  #pause
  Two main areas of improvment:
  #pause
  - Branch prediction (not much to gain here in reality).
  #pause
  - Caching optimisation (the real improvment).
]

== Branch prediction

#slide[
  CPU does not work sequentialy instruction per instruction. \
  They are fetching ahead. \
]

#slide[
  Taking guesses when encountering conditional branches.
]

#slide[
  Branch prediction:
  - *Optimisation* technique of modern CPU.
  - Roughtly $5 "ns"$ per mispredict.
]

#slide[
  Not much to gain but this is good to know.
]

== Caching

#slide[
  Breaching between the "slow" memory (RAM) and the very fast (CPU registers). \
  $approx 10times$ faster the caching layer. Orders are $0.5 "ns"$ to $5 "ns"$ for cache, and $50 "ns"$ for RAM access.

  Speaking in cycle it takes:
  - $1 - 4$ cycles for L1.
  - $10 - 20$ cycles for L2.
  - $40 - 75$ cycles for L3.
  - $100+$ cycles for RAM.
]

#slide[
  But memory that fast is not cheap and is not available in quantity. \
  Every day computer have $approx 10 "MB"$ of cache all layers included.
]

#slide[
  The main difference between the layer (L1, L2 and L3) is if they are shared between CPU cores or not.
]

#slide[
  It works like that:

  #align(center, diagram(
    node-shape: rect,
    node-stroke: 1pt,
    node-corner-radius: 2pt,
    node((0, 0), "CPU", name: <cpu>),
    node((3, 0), "Cache", name: <cache>),
    node((6, 0), "RAM", name: <ram>),

    edge(<cpu>, <cache>, "->", text(15pt)[Check for data in cache]),
    edge(<cache>, <ram>, "->", text(15pt, red)[Miss: check in RAM]),
    edge(<cpu>, <cache>, "<-", text(15pt, green)[Hit: return data], bend: -50deg, loop-angle: 90deg),
    edge(
      <cache>,
      <ram>,
      "<-",
      align(center, text(15pt)[Return data, \ must chose what to keep]),
      bend: -50deg,
      loop-angle: 90deg,
    ),
  ))
]

#slide(align: center + horizon)[
  *Why is this a problem in our case?*
]

#slide[
  Our way to to get infomation is the worst possible in term of optimising the cache.

  #pause
  This is because of the bounce: \
  #pause

  #align(center)[
    $mat(1, 1, 2, dots.h.c, 10, 13, 15, dots.h.c, 20, 22, 34, dots.h.c, 50, 54, 73, dots.h.c, 85, 89, 94;)$
    #pause
    $mat(1, 1, 2, dots.h.c, 10, 13, 15, dots.h.c, 20, #text(red)[22], 34, dots.h.c, 50, 54, 73, dots.h.c, 85, 89, 94;)$
    #pause
    $mat(
      1, 1, 2, dots.h.c, 10, #text(red)[13], 15, dots.h.c, 20, 22, 34, dots.h.c, 50, #text(red)[54], 73, dots.h.c, 85, 89, 94;
    )$
    #pause \
    $dots.v$ \
    $mat(
      #text(red)[1], 1, #text(red)[2], dots.h.c, #text(red)[10], 13, #text(red)[15], dots.h.c, #text(red)[20], 22, #text(red)[34], dots.h.c, #text(red)[50], 54, #text(red)[73], dots.h.c, #text(red)[85], 89, #text(red)[94];
    )$
  ]
]

#slide[
  Or to be formal, we have an iteration depth $n in NN^*$ and we define candidate numbers for $m$:
  $
    S_n eq.def { (1 + K dot (2^1, 2^2, dots, 2^(n-1))) / 2^n mid(|) K in FF_2^(n-1) }
  $
]

= A first solution

#slide[
  Now that we know the problem lets make a solution for it!
]

#slide[
  Problem is with the way we order our sorted array. \
  Lets find an other way, optimised for the way our cache works.
]

#slide(align: center + horizon)[
  *But first history point. \
  When the solution we gonna see was theorised?*
  #pause \
  That is it ! *1598*
]

== Optimized layout

#slide[
  Aside of that trivia, what is this layout?
  #pause \
  Very common flatten tree structure, used in many places, heaps...
]

#slide[
  Idea is the following, we order the data as would print it. \
  Meaning that we have for the node of index $k$:
  - Left child at index $2k$
  - Right child at index $2k + 1$
]

#slide[
  Which means that instead of bouncing around with the cache, \
  it goes from start to finish, taking the previous exemple we have:
  #align(center)[
    $mat(22, 13, 54, dots.h.c, dots.h.c, dots.h.c, dots.h.c;)$
  ]

  This means much less cache misses.
]

== Cache prefetching technique

#slide[
  We can tune the prefetching. \
  Since we chose one of the child (go left or right), we can forget about the part of the tree we did not chose.
]

#slide[
  Lets see an example with target $= 2$: \
  #let normal-stroke = 1pt + black
  #let faded-color = black.transparentize(0%)
  #let faded-stroke = 1pt + faded-color

  #align(center, diagram(
    node-stroke: normal-stroke, // Default to normal
    edge-stroke: normal-stroke, // Default to normal
    node-fill: white,
    spacing: 1.5em, // Adjust spacing for clarity

    // --- ROOT (Normal) ---
    node((0, 0), text(red)[5], radius: 1em),

    // --- LEFT SUBTREE (Normal opacity) ---
    edge((0, 0), (-2, 1), "-|>"),
    node((-2, 1), "3", radius: 1em),

    edge((-2, 1), (-3, 2), "-|>"),
    node((-3, 2), "2", radius: 1em),

    edge((-2, 1), (-1, 2), "-|>"),
    node((-1, 2), "4", radius: 1em),

    edge((-3, 2), (-3.5, 3), "-|>"),
    node((-3.5, 3), "1", radius: 1em),


    // --- RIGHT SUBTREE (50% Opacity) ---
    // We override the stroke and text color for these specific nodes and edges

    // Edge to 7
    edge((0, 0), (2, 1), "-|>", stroke: faded-stroke),
    // Node 7 (Fade border and text)
    node((2, 1), text(fill: faded-color, "7"), radius: 1em, stroke: faded-stroke),

    // Children of 7
    edge((2, 1), (1, 2), "-|>", stroke: faded-stroke),
    node((1, 2), text(fill: faded-color, "6"), radius: 1em, stroke: faded-stroke),

    edge((2, 1), (3, 2), "-|>", stroke: faded-stroke),
    node((3, 2), text(fill: faded-color, "8"), radius: 1em, stroke: faded-stroke),
  ))
]

#slide[
  #let normal-stroke = 1pt + black
  #let faded-color = black.transparentize(90%)
  #let faded-stroke = 1pt + faded-color

  #align(center, diagram(
    node-stroke: normal-stroke, // Default to normal
    edge-stroke: normal-stroke, // Default to normal
    node-fill: white,
    spacing: 1.5em, // Adjust spacing for clarity

    // --- ROOT (Normal) ---
    node((0, 0), "5", radius: 1em),

    // --- LEFT SUBTREE (Normal opacity) ---
    edge((0, 0), (-2, 1), "-|>"),
    node((-2, 1), text(red)[3], radius: 1em),

    edge((-2, 1), (-3, 2), "-|>"),
    node((-3, 2), "2", radius: 1em),

    edge((-2, 1), (-1, 2), "-|>"),
    node((-1, 2), "4", radius: 1em),

    edge((-3, 2), (-3.5, 3), "-|>"),
    node((-3.5, 3), "1", radius: 1em),


    // --- RIGHT SUBTREE (50% Opacity) ---
    // We override the stroke and text color for these specific nodes and edges

    // Edge to 7
    edge((0, 0), (2, 1), "-|>", stroke: faded-stroke),
    // Node 7 (Fade border and text)
    node((2, 1), text(fill: faded-color, "7"), radius: 1em, stroke: faded-stroke),

    // Children of 7
    edge((2, 1), (1, 2), "-|>", stroke: faded-stroke),
    node((1, 2), text(fill: faded-color, "6"), radius: 1em, stroke: faded-stroke),

    edge((2, 1), (3, 2), "-|>", stroke: faded-stroke),
    node((3, 2), text(fill: faded-color, "8"), radius: 1em, stroke: faded-stroke),
  ))
]

#slide[
  #let normal-stroke = 1pt + black
  #let faded-color = black.transparentize(90%)
  #let faded-stroke = 1pt + faded-color

  #align(center, diagram(
    node-stroke: normal-stroke, // Default to normal
    edge-stroke: normal-stroke, // Default to normal
    node-fill: white,
    spacing: 1.5em, // Adjust spacing for clarity

    // --- ROOT (Normal) ---
    node((0, 0), "5", radius: 1em),

    // --- LEFT SUBTREE (Normal opacity) ---
    edge((0, 0), (-2, 1), "-|>"),
    node((-2, 1), "3", radius: 1em),

    edge((-2, 1), (-3, 2), "-|>"),
    node((-3, 2), text(green)[2], radius: 1em),

    edge((-2, 1), (-1, 2), "-|>", stroke: faded-stroke),
    node((-1, 2), text(fill: faded-color, "4"), radius: 1em, stroke: faded-stroke),

    edge((-3, 2), (-3.5, 3), "-|>"),
    node((-3.5, 3), "1", radius: 1em),


    // --- RIGHT SUBTREE (50% Opacity) ---
    // We override the stroke and text color for these specific nodes and edges

    // Edge to 7
    edge((0, 0), (2, 1), "-|>", stroke: faded-stroke),
    // Node 7 (Fade border and text)
    node((2, 1), text(fill: faded-color, "7"), radius: 1em, stroke: faded-stroke),

    // Children of 7
    edge((2, 1), (1, 2), "-|>", stroke: faded-stroke),
    node((1, 2), text(fill: faded-color, "6"), radius: 1em, stroke: faded-stroke),

    edge((2, 1), (3, 2), "-|>", stroke: faded-stroke),
    node((3, 2), text(fill: faded-color, "8"), radius: 1em, stroke: faded-stroke),
  ))
]

#slide[This gives us way better results.]

== Going branchless

#slide[
  Last thing this layout allows us to do is:
  - Get rid of if statements.
  Since we know the index of the childs, we can chose the child based on that.
]

== Implementation

#slide(config: config-page(margin: (bottom: 1.4em)))[
  #figure(
    image("assets/e_lwpb.svg", width: 57%),
    caption: [Rust implementation for upper bound],
    gap: 0.2em,
    placement: bottom,
  )
]

= Playing with SIMD

#slide[
  This is the difficult partso lets get the general idea. \
  We do the exact same thing as before (using tree like structure).

  We take advantage of SIMD: \
  meaning that instead of 1 node = 1 number, we have 1 node = 16 numbers.
]

#slide[
  Why this work? \
  #pause
  A cache line is 64 bytes, we work with 4 bytes intergers, making it 16 numbers in a line.
]

== Presentation: Our tree layout

#slide[
  We use S-tree, a flatten immutable version of B-tree. \
  This is the exact same as before but it have 16 numbers per nodes and each one of them have 17 children.
]

== What is SIMD

#slide[
  === General idea
  We do an operation, but instead of doing it on 1 element, we doing it on 16 at a time (in our case of 32 bits integer).

  #pause
  Because yes this is a problem with SIMD, since it is loading multiple element at the same time the number of elements we use at the same times vary with its size.

  #pause
  Other problem is that SIMD is dependend of the architecture, you can see in the source code the big difference in paradigm between AVX (intel x86_64 SIMD) and NEON (aarch64 SIMD).
]

#slide[
  #figure(
    grid(
      columns: (70%, 30%),
      image("assets/SIMD_x86.svg", width: 80%), align(left)[We use intel paradigm, using cheap masking operations.],
    ),
    caption: [SIMD implementation for inter x86_64 architecture.],
  )
]

#slide[
  #figure(
    grid(
      columns: (70%, 30%),
      image("assets/SIMD_aarch64.svg", width: 70%), align(left)[We use NEON paradigm, using cheap sum operations.],
    ),
    caption: [SIMD implementation for inter ARM64 architecture.],
  )
]

= Conclusion

#slide[
  Yeah cool, you yap a lot but is this even worth the headache?

  #pause
  Depend on how much you think 600%+ improvment is something big.
]

#slide[
  #let best(content) = text(weight: "bold", fill: rgb("#1b5e20"), content)
  #let speedup(content) = text(weight: "bold", fill: rgb("#0d47a1"), content)

  #figure(
    table(
      columns: (auto, 1fr, 1fr, 1fr, auto),
      inset: 12pt,
      align: (col, row) => if col == 0 { left + horizon } else { center + horizon },
      stroke: none,

      // Zebra striping for readability
      fill: (col, row) => if row == 0 {
        luma(230) // Header background
      } else if calc.even(row) {
        luma(250) // Alternating rows
      } else {
        white
      },

      // --- Header ---
      table.header(
        [*Nb of elements*],
        [*Basic Lower Bound*\ (ns/op)],
        [*Eytzinger*\ (ns/op)],
        [*S-Tree SIMD*\ (ns/op)],
        [*Speedup Factor*\ (vs Basic)],
      ),

      // --- Data Row 1 (100M) ---
      [$1 dot 10^8$], [411.99], [153.33], best[63.05], speedup[6.53x],

      // --- Data Row 2 (1B) ---
      [$1 dot 10^9$], [629.15], [253.83], best[97.78], speedup[6.43x],

      // --- Data Row 3 (1.5B) ---
      [$1.5 dot 10^9$], [799.25], [295.80], best[128.26], speedup[6.23x],

      // --- Data Row 4 (2B) ---
      [$2 dot 10^9$], [812.34], [371.96], best[123.87], speedup[6.56x],
    ),
    caption: [ *Performance Benchmark* (Apple Silicon M4pro | L1:192kB, L2:16MB, L3:#sym.approx 32MB) ],
  )
]

#slide[
  #align(center + horizon, text(32pt)[*Thanks for listenning!*])
]

#bibliography("bib.yaml")
