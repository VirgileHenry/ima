## Machine Virtuelle IMA

[présentation]

#### Changements:

- L'opération OPP peut maintenant overflow

#### Questions:

- Est-ce que les opérations sont quand même exécutées si le flag ov est levé ? (ex: div par 0) ça peut avoir du sens dans certains cas (float * float -> inf est un résultat valide) mais parfois peut faire paniquer la machine (division entière par zéro). Pour l'instant, la machine éxecute l'opération sauf si elle va faire paniquer le programme.
- Est-ce qu'il y a une différence etiquette / adresse code ? le datatype peut être addresse code, quel interet que dval puisse aussi l'etre ? redondance entre Dval qui est une étiquette, ou Dval qui est une valeur de type adresse code ?
- wutf8 interprète R1 comme un u32 ?

#### à faire:

- compter les cycles d'horloge (pas trop compliqué ?)
- changer le mode d'arrondi des calculs flottant (besoin d'une lib C et de la link)
- détection des boucles infinies ? (Hash de l'état d'ima, et trace des Hash.) Peut être couteux, mais utile pour les tests qui doivent boucler à l'infini quand tout va bien. 

#### En développement:

je travail actuellement sur Vima (visual-ima), une version débug d'ima avec un affichage plus clair et intuitif (et des couleurs !) de l'état de la machine. L'objectif est de rendre plus facile le débuggage de programmes ima.

#### Tests:

La machine actuelle a été testé sur un millier de fichier assembleur générés par différents projets, et 122/978 d'entre eux causent une erreur. Cependant, ce résultat est à prendre avec des pincettes: j'ai trouvé dans eles fichiers incorrect des fichiers qui n'étaient pas des fichiers IMA (des fichiers de résultats) et des fichiers avec des instructions inexistantes. Je pense donc que les fichiers qui ont échoués sont des fichiers incorrects.

De plus, quelques tests unitaires ont été écrits, mais il en faudrait beaucoup plus pour couvrir correctement toutes les instructions.

#### Comparatif avec la machine précédente:

De ce que j'ai pu voir, la nouvelle machine est beaucoup plus légère que l'ancienne. Les sources présentent 90% de code en moins. Il reste à essayer de faire des tests de performances, mais cette première implémentation reste assez direct et aucune optimisation n'a été faite.

#### Contribution:

Le projet à été initialement créé par Virgile Henry, en faisant suite à la machine IMA originelle faite par les responsables du projet GL Ensimag (je n'ai pas trouvé leur noms dans les sources originelles, et je serait heureux de les avoir pour les citer).