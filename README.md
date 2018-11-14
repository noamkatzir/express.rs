1. at the moment there is an http parser written as state machine which reads
the stream once without extra memory alocations.

2. there is a router for the requests action which is implemented using a trie (need to see if using an array will be faster because there will be less cache missmatch)

3. the request ad response are passed all the way, from the top to bottom, by passing ownership from one to another (not borrow mutable), because the owner of the request is the one which process it at the moment and once it pass it a way there is nothing in need to do after 

4. at the moment the routing using simple thread pool which run the parsing of the request and then trigger the routing method. there are some notes:
    1. if running to match connection concurrent I get to the "Too manyopen files" error, I guess a better solution will be 
    2. the pool of the parsing create and destroy each time the buffer for parsing, and maybe fester to reuse them
    3. maybe it's better to use chnannels and other thread pool for handling the action from the routing.
    4. maybe usefull to add async api for network and filesystem using futures.rs
    5. I want to make an integration with rocksDB
    6. I want to implement translation service with my HTTP server
