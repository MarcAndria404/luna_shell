# Mini-shell

Un shell interactif basique écrit en Rust : lecture de commandes, exécution de programmes externes, et quelques built-ins essentiels.

## Fonctionnalités

- Boucle de lecture-exécution (REPL) avec prompt
- Parsing d'une ligne de commande en commande + arguments
- Exécution de commandes externes via `std::process::Command`
- Built-ins : `cd` (changement de répertoire), `exit`
- Gestion d'erreurs sans crash : commande invalide, ligne vide, `cd` sans argument ou vers un chemin invalide

## Ce que ce projet m'a appris

**Ownership et lifetimes en pratique**
La première version de la fonction de lecture retournait des `&str` empruntés à une `String` locale à la fonction — code qui ne compile pas, puisque ces références auraient survécu à la destruction de la `String` dont elles dépendaient (*dangling reference*). Solution : convertir chaque mot en `String` possédée (`.map(|x| x.to_string())`) avant de le faire sortir de la fonction, pour que le `Vec<String>` retourné soit totalement indépendant.

**Gestion d'erreurs idiomatique**
- `?` pour propager les erreurs de lecture (`io::Result`)
- `let ... else` pour gérer un pattern réfutable (`split_first()` sur une ligne potentiellement vide) sans faire planter le programme
- `match` sur `Ok`/`Err` pour distinguer succès et échec de lancement d'une commande, sans jamais paniquer sur une entrée invalide
- `if let Err(e) = ...` comme forme idiomatique quand seul le cas d'erreur nécessite une action

**Pourquoi `cd` ne peut pas être une commande externe**
`cd` doit modifier le répertoire de travail du shell lui-même, pas celui d'un sous-processus qui se termine immédiatement après son exécution. D'où l'usage de `std::env::set_current_dir` directement dans le processus du shell, plutôt que `std::process::Command`.

**`Command::new()` et `AsRef`**
Fonctionne directement avec `&String` et `&[String]` grâce au trait `AsRef<OsStr>`, sans conversion manuelle nécessaire.

## Lancer le projet

```bash
cargo run
```

## Pistes d'amélioration (à venir)

- `cd` sans argument → aller au répertoire `$HOME`
- Gestion des guillemets dans le parsing (`echo "hello world"`)
- Built-in `pwd`
- Distinguer l'échec de lancement d'une commande de son code de sortie non-nul
