# CubeString

Calcule l'ensemble des solution d'un casse-tête irl : une cordelette enfilée à travers plusieurs petits cubes articulés qu’il faut replier pour former un cube complet (snake cube).

- ![Cordelette et cubes](pic1.jpg)
- ![Cube assemblé](pic2.jpg)

Le casse-tête des "snake cubes" consiste en une suite de cubes reliés par un élastique interne. Chaque articulation permet de faire pivoter les cubes pour former différentes formes. Il faut trouver la séquence exacte de pliages menant au cube 3×3×3 final.

Ce dépôt contient un programme rust qui explore et valide les configurations possibles afin de reconstruire automatiquement le cube complet.

```bash
cargo build --release
./target/release/cubestring
```

