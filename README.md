# ringo
Rust bingo application.

## API

If a player has joined a room, the server switches the connection to a socket.
This socket communicates global information and tells the client what information has changed 
and should be updated via a request to the API.