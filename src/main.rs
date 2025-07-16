use std::env;
use std::fs;
use std::io::{Read, stdin};  

fn main() {
    //Ouverture du fichier etc
    let args: Vec<String> = env::args().collect();

    
    if args.len() != 2 {
        panic!("Utilisation : bf_inter <fichier.bf>");

    }
    let fd = &args[1];
    //let fichier = File::open(fd).expect("Fichier pas trouvé");
    let contenu = fs::read_to_string(fd).expect("Impossible d'ouvrir le ficheir");
    //Environnement
    let mut memoire = vec![0u8; 30000];
    let mut pointeur = 0;

    let instructions: Vec<char> = contenu.chars().collect();
    let mut actuel = 0;

    while actuel < instructions.len() {
        match instructions[actuel] {
            '>' => pointeur += 1,
            '<' => pointeur -= 1,
            '+' => memoire[pointeur] = memoire[pointeur] + 1,
            '-' => memoire[pointeur] = memoire[pointeur] - 1,
            '.' => print!("{}", memoire[pointeur] as char),
            ',' => {
                 
                let mut stdin_handle = stdin().lock();  
                let mut byte = [0_u8];  
                stdin_handle.read_exact(&mut byte).unwrap();
                memoire[pointeur] = byte[0];

            },
            '[' => {
                if memoire[pointeur] == 0 {
                    let mut prof = 1;
                    while prof != 0 {
                        actuel += 1;
                        match instructions[actuel] {
                            '[' => prof += 1,
                            ']' => prof -= 1,
                            _ => (),
                        }
                    }

                }
            }
            ']' => {
                if memoire[pointeur] != 0 {
                    let mut prof = 1;
                    while prof != 0 {
                        actuel -= 1;
                        match instructions[actuel] {
                            ']' => prof += 1,
                            '[' => prof -= 1,
                            _ => (),
                        }
                    }
                }
            }
            _ => ()

        }
        
        actuel+=1;

    }
}



