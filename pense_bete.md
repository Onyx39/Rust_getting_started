# Executer des fichiers/projets Rust
## I) Lancer un fichier Rust simple
### 1- Compliation
    ```{shell}
    rustc file.rs
    ```

### 2 - Execution
    ```{shell}
    ./file
    ```

## II) Lancer un projet avec cargo 
### 1 - Créer un projet cargo 
    ```{shell}
    cargo new name
    ```

### 2 - Vérification du code
    ```{shell}
    cd name 
    cargo check
    ```
    Cette commande de créer pas d'exécutable, il vérifie simplement le code.

### 3 - Compilation
    ```{shell}
    cargo build
    ```

### 4 - Execution
    ```{shell}
    cargo run
    ```