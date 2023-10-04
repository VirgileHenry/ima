# Vima 

Vima (Visual-IMA) est une version débug de la machine virtuelle IMA, qui permet d'inspecter l'état de la machine pendant son éxecution de façon simple et claire. Elle à été conçue par dessus ma propre implémentation d'IMA. 

![VIMA terminal interface](https://github.com/VirgileHenry/ima/blob/master/vima/rm_images/vima.png?raw=true)

## Installation

Le tout est un projet en rust, donc un simple `cargo build --release` devrait suffire. L'executable se trouvera dans `target/release/vima`. On peut également directement lancer le projet avec `cargo run --release`.

## Utilisation

Le programme doit être appelé en passant un programme IMA en argument.

Une fois dans l'interface, toutes les entrées sont redirigés vers la zone commandes. Les commandes disponibles sont :
- `x`: éxecute une seule instruction
- `a <int>`: ajoute un point d'arrêt à la ligne donnée
- `e <int>`: enlève un point d'arrêt à la ligne donnée
- `c`: éxecute le programme jusqu'au prochain point d'arrêt, ou l'arrêt du programme. 

Tout les entrées / sorties spécifiques à IMA sont affichés dans la zone de sortie. Si IMA à besoin d'une entrée, les entrées utilisateurs sont redirigés dans la zone d'entrée IMA le temps de la demande.

VIMA est décomposé en plusieurs zones :
- La zone de commandes, décrite ci-dessus
- la zone I/O IMA, qui affiche les entrées / sorties de IMA
- la zone program, qui affiche le program IMA autours du pointeur d'instruction. On peut y voir le pointeur d'instruction, et les points d'arrêts.
- la zone pile, qui affiche la pile IMA, ainsi que les pointeurs SP, LB, GB.
- la zone tas, qui affiche toute les allocations effectuées par la machine.
- la zone registres, qui affiche les registres IMA.
- la zone flags, qui affichent les flags de comparaisons et d'overflow d'IMA.
- la zone énergie, qui affiche le nombre de cycles dépensés par IMA, ainsi que le nombre de cycle requis pour la prochaine instruction.

## Todo

Note à moi même, les quelques features qu'ils faudrait encore rajouter
- Bonne gestion des erreurs, cf ima
- Colorations des zones utilisés par l'instruction courante
- explorer le tas (voire même la pile / le programme ?)

## Contribution

Le Projet offre la bienvenue à toute contribution qui permettrait de l'améliorer.