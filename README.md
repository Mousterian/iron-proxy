# iron-proxy
A simple but performant HTTP proxy implemented in Rust using Iron and Hyper

inspired by:

https://gist.github.com/campaul/feeed29ee90e5e725630

this version uses Iron for asynch multithreaded goodness.

TO DO: 

- needs to look for HTML or other texty content type responses and rewrite URLs to keep the client on the proxy site and not walk off to another.
- configuration options for target site, port, etc.
