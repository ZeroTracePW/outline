# Outline
Rust CLI for interactive security testing and fuzzing of WebSocket servers: handshake, fuzzing (random/injection), and message-based vulnerability checks. Async with Tokio, modular structure.

# ToDo's
```
[ X ] 1. Connect and Handshake
[ X ] 2. Send and Receive Messages from Websocket
[   ] 3. Random garbage (see if server crashes or errors)
[   ] 4. Common injection payloads (like "' OR '1'='1", </script>, {{7*7}})
[   ] 5. Injection Fuzzing: Send SQLi, XSS, SSRF payloads inside WebSocket messages.
[   ] 6. DoS Fuzzing: Send massive payloads (1 MB+) and see if server hangs.
[   ] 7. Authentication Bypass: Try sending privileged commands without authenticating.
[   ] 8. If JSON, Parse and look for stuff like e\.g Content, Input and do Injection
```

# Showcase
![image](https://github.com/user-attachments/assets/5fc0b538-7880-4dd5-a04d-641f10c8e301)

### Contribution
Contributions are welcome! Please open issues for feature requests or bug reports, and submit pull requests for improvements.
