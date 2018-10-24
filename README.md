1. at the moment there is an http parser written as state machine which reads
the stream once without extra memory alocations.

2. there is a router for the requests action which is implemented using a trie (need to see if using an array will be faster because there will be less cache missmatch)

3. the request ad response are passed all the way, from the top to bottom, by passing ownership from one to another (not borrow mutable), because the owner of the request is the one which process it at the moment and once it pass it a way there is nothing in need to do after 