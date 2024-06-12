use std::io::{self, BufReader, Write, Read};
use std::net::TcpStream;

pub fn fuzzing() -> io::Result<()> {
    // Adresse IP et port du serveur
    let server_address = "10.10.18.138:1337";
    
    // Se connecter au serveur
    let stream = match TcpStream::connect(server_address) {
        Ok(stream) => {
            println!("Connecté au serveur à {}", server_address);
            stream
        },
        Err(e) => {
            eprintln!("Erreur de connexion: {}", e);
            return Err(e);
        }
    };

    // Cloner le stream pour pouvoir lire et écrire simultanément
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream;

    // Lire et afficher les messages de bienvenue du serveur
    read_and_display(&mut reader)?;

    // Boucle pour envoyer des messages et lire les réponses du serveur
    loop {
        // Lire un message de l'utilisateur
        let mut input = String::new();
        println!("Entrez un message à envoyer au serveur (ou 'exit' pour quitter) :");
        io::stdin().read_line(&mut input)?;

        // Quitter la boucle si l'utilisateur entre "exit"
        if input.trim().eq_ignore_ascii_case("exit") {
            println!("Fermeture de la connexion.");
            break;
        }

        // Envoyer le message au serveur
        if let Err(e) = writer.write_all(input.as_bytes()) {
            eprintln!("Erreur d'envoi du message: {}", e);
            break;
        }

        if let Err(e) = writer.flush() {
            eprintln!("Erreur lors du flush du message: {}", e);
            break;
        }

        // Lire et afficher les réponses du serveur
        read_and_display(&mut reader)?;
    }

    Ok(())
}

fn read_and_display(reader: &mut BufReader<TcpStream>) -> io::Result<()> {
    // Lire et afficher tous les messages du serveur
    let mut buffer = vec![0; 512];
    let bytes_read = reader.read(&mut buffer)?;
    if bytes_read > 0 {
        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Message reçu: {}", response);
    }
    Ok(())
}