use std::io::{self, Write};

mod user;
mod chatroom;

use user::User;
use chatroom::ChatRoom;

fn main() {
    let mut chatroom = ChatRoom::new();

    // Create users
    let alice = User::new("Alice");
    let bob = User::new("Bob");
    let charlie = User::new("Charlie");

    // Add users to chatroom
    chatroom.add_user(alice);
    chatroom.add_user(bob);
    chatroom.add_user(charlie);

    println!("🚀 Post-Quantum Secure Messenger Started!");
    println!("👥 Available Users: Alice, Bob, Charlie");

    loop {
        println!("\n🔁 New Message (type 'exit' to quit)");

        // Get sender
        print!("From: ");
        io::stdout().flush().unwrap();
        let mut from = String::new();
        io::stdin().read_line(&mut from).unwrap();
        let from = from.trim();
        if from.eq_ignore_ascii_case("exit") { break; }

        // Check if sender exists
        if !chatroom.has_user(from) {
            println!("❌ Unknown user '{}'. Please enter a valid username.", from);
            continue;
        }

        // Get recipient
        print!("To: ");
        io::stdout().flush().unwrap();
        let mut to = String::new();
        io::stdin().read_line(&mut to).unwrap();
        let to = to.trim();
        if to.eq_ignore_ascii_case("exit") { break; }

        // Check if recipient exists
        if !chatroom.has_user(to) {
            println!("❌ Unknown user '{}'. Please enter a valid username.", to);
            continue;
        }

        // Get message
        print!("Message: ");
        io::stdout().flush().unwrap();
        let mut msg = String::new();
        io::stdin().read_line(&mut msg).unwrap();
        let msg = msg.trim();
        if msg.eq_ignore_ascii_case("exit") { break; }

        // Send encrypted message
        match chatroom.send_encrypted(from, to, msg) {
            Some((ciphertext, nonce)) => {
                println!("\n🔐 Message sent securely!");
                println!("📦 Ciphertext: {:?}", ciphertext);
                println!("🕵️‍♀️ Nonce: {:?}", nonce);

                match chatroom.receive_encrypted(to, from, &ciphertext, &nonce) {
                    Some(decrypted) => println!("📬 {} received: \"{}\"", to, decrypted),
                    None => println!("❌ {} failed to decrypt the message.", to),
                }
            }
            None => println!("❌ Failed to send encrypted message. Check usernames."),
        }
    }

    println!("\n👋 Exiting Secure Messenger. Stay quantum safe!");
}
