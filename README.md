## Machine Virtuelle IMA

Machine Virtuelle IMA du projet GL Ensimag, en Rust (et une interfcae visuelle)

#### Changements:

- L'opération OPP peut maintenant overflow

#### à faire:

- changer le mode d'arrondi des calculs flottant (besoin d'une lib C et de la link)
- détection des boucles infinies ? (Hash de l'état d'ima, et trace des Hash.) Peut être couteux, mais utile pour les tests qui doivent boucler à l'infini quand tout va bien. 

#### Contribution:

Le projet fait suite à la machine IMA originelle faite par les responsables du projet GL Ensimag (je n'ai pas trouvé leur noms dans les sources originelles, et je serait heureux de les avoir pour les citer). Toute contribution est la bienvenue !

Contributeurs: 
- Virgile HENRY