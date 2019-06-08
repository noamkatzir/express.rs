#Goals

##Motivation

because Rust has it's amazing type system you can express some of your goals easly.

but it is also very eawsy to forget them. this document reveal some of those goals

##the list
1. a builder which build an entity has method which pass the ownership of the builder because this is the end game, for example you can see RequestBuilder

2. the router has immutable action to encorage you to use more share by communication pattern, to insure thread safety, you can see example of usage in router tests
