# Ion - A scripting language built on Rust.

I love Rust, it's no secret. Comparing traditional OOP languages (e.g. Java) to Rust leaves a lot to be disired in some ways. 
The power of functional solutions is incredible, however I do feel there is a missing middle ground that Ion aims to fix in 2 ways.

## 1. Traditional Object Oriented Interfaces
Traits in Rust is a very powerful tool to control behavior. However, having a traditional Interface to control data members for a struct would be an excellent addition. This doesn't, however, mean tradtitional OOP inheritance by any means! Ion will limit this behavior to a single layer (meaning implementing an interface will never inherit anything beyond said interface), but a struct *can* implement multiple interfaces to provide very strong control for both data and behavior.

## 2. Dynamic Typing
This flies in the face of Rust, and yet I still feel that a dynamically typed scripting language with a focused functional core is exactly what I want.

## A Functional Core
Ion's ideology is rooted in functional languages like Rust and Haskell, and strives to naturally lend itself to functional solutions, rather than layers of inheritance and class methods. Data in, data out.


# Current State

Ion is under daily development, with the lexical analysis stage nearing completion.
