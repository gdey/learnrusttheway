# Overview.

  This started out with me going through the book Learn Python the Hard Way;
  but I quickly learned that there are things you can do very easily in Python
  that are not trivial in Rust. One example of this is printing a line using
  a dynamic format:
  ```Python
     format = "I said: %r."
     said_what = "Hello world"
     print format % said_what
  ```
  At current I don't know how to do this. I would have to parse the format line
  figure out where the token place holders are; verify that the number of 
  number of tokens match the number of arguments, and then check to see if the 
  types of arguments match the types of the tokens. This turns out to be harder
  then I would think because Rust trys to be a memory safe language. Where as in
  C you could use sprintf/printf; but those methods are memory unsafe, as they 
  don't do those checks. 

  So, I'm going to change over the Learn C the Hard Way. 


