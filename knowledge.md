## Day 1

1. Quelle est la différence entre let x = 5; et let mut x = 5; ?

let x = 5; le x est immutable alors que le let mut x = 5; la variable x peux être modifié aprés. 

2. Pourquoi Rust rend les variables immuables par défaut ?

Pour rendre le code le code sécure. ça permettrai d'avoir moins de bugs sur le code.

3. Quelle est la différence entre mutation et shadowing ?

la mutation permet de changer la valeur d'une variable sans la redéclarer alors que le shadowing permet de redéclarer la variable avec une nouvelle valeur 


## Day 2

1.Est-ce que ça compile ?

ça ne compile pas.

2. Si non, pourquoi ?

la chaine String est stockée dans la heap. La propriété de la variable est transférée vers s2. donc s1 n'a plus de valeur.

