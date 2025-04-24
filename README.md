[# pq_messenger](https://github.com/rneeraaj/pq_messenger/pull/new/master# 🚀 Post-Quantum Secure Messenger

<div align="center">
  <img src="https://img.shields.io/badge/Rust-1.75.0-orange" alt="Rust">
  <img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License">
</div>

## 📋 Overview

A secure messaging application that implements post-quantum cryptography for end-to-end encrypted communication. This application allows users to create accounts and exchange encrypted messages securely.

## ✨ Features

- **User Management**: Create and manage multiple users
- **End-to-End Encryption**: Messages are encrypted using post-quantum cryptography
- **Secure Communication**: Messages are encrypted before transmission
- **User-Friendly CLI**: Simple command-line interface for easy interaction

## 🛠️ Technical Stack

- **Language**: Rust
- **Cryptography**: Post-quantum cryptography algorithms
- **Dependencies**: 
  - pqcrypto-kyber
  - aes-gcm

## 🚀 Getting Started

### Prerequisites

- Rust 1.75.0 or later
- Git

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/pq_messenger.git
cd pq_messenger
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run
```

## 📁 Project Structure

```
pq_messenger/
├── src/
│   ├── main.rs      # Main application logic
│   ├── user.rs      # User management and encryption
│   └── chatroom.rs  # Chat room functionality
├── Cargo.toml       # Project dependencies
└── Cargo.lock       # Dependency lock file
```

## 🔧 Usage

### Creating a New User

1. Start the application:
```bash
cargo run
```

2. When prompted for the sender, type `new` to create a new user
3. Enter your desired username
4. The system will create a new user with encryption keys

### Sending Messages

1. Enter the sender's username when prompted
2. Enter the recipient's username
3. Type your message
4. The message will be encrypted and sent securely

### Example Session

```
🚀 Post-Quantum Secure Messenger Started!

🔁 New Message (type 'exit' to quit)
From: new
Enter new username: Alice
User 'Alice' created successfully!

From: Alice
To: Bob
Message: Hello, this is a secure message!

🔐 Message sent securely!
📦 Ciphertext: [...]
🕵️‍♀️ Nonce: [...]
📬 Bob received: "Hello, this is a secure message!"
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👥 Authors

- Your Name - Initial work

## 🙏 Acknowledgments

- Rust community
- Post-quantum cryptography researchers
- All contributors and supporters

## 📞 Contact

Your Name - [@yourtwitter](https://twitter.com/yourtwitter)

Project Link: [https://github.com/yourusername/pq_messenger](https://github.com/yourusername/pq_messenger)

---

<div align="center">
  <sub>Built with ❤️ using Rust</sub>
</div> )
