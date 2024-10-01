# Context

Project for the first year of the STL master at Sorbonne Université.
The goal of the project was to create a compiler from a functionnal toy language to WebAssembly, written in Rust.


# PSTL

Lancer l'interprète : 
    
    cargo run (--release) i <fichier.pstl> <fonction>

Compiler du code : 
    
- Etape 1 : Compiler le code en code WAT : 
    
    `cargo run (--release) c <fichier_entree> <fichier_compile.wat>`
    
- Etape 2 : Compiler le code WAT en WASM : 
    
    `wat2wasm <fichier_compile.wat> <fichier_sortie.wasm>`
