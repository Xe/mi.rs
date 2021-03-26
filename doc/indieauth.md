# IndieAuth Plan

This document outlines my plans to make mi (and xesite) integrated with
[IndieAuth](https://indieweb.org/IndieAuth).

## TODO

- [ ] Implement Authorization Endpoint
  - [ ] Paseto in a cookie for tracking intermediate state
  - [ ] https://indieweb.org/authorization-endpoint
- [ ] Implement Token Endpoint
  - [ ] https://indieweb.org/token-endpoint
  - [ ] https://indieweb.org/obtaining-an-access-token
- [ ] Define API scopes
  - `read` - Read-only calls
  - `write` - Write calls
  - `within` - Scope parent for all of my personal API calls
  - `within:all` - All access to everything (`mi.within.website` or
    `christine.website` only)
  - `within:switch` - Access to the switch tracking APIs
  - `within:package` - Access to the package tracking API
  - `within:token` - Access to the token creation/list/deletion calls
  - `within:posse` - Access to the POSSE calls (results in twitter, etc posts)
  - `within:blog:update` - Access to the blog update call (scoped separately)
  - `within:webmention` - Access to the webmention API
- [ ] Define public API
- [ ] Refresh token support

## Why

Why not? It allows me to do some experimentation and also lets me write a more
complicated webapp in Rust. This would also allow for research in using Rocket's
more advanced features to make it an example on how to do this.

I also want to bring back the contact form, but I don't want to have it be an
open season for people to spam me to hell and back. I may introduce GitHub
oauth2 as well like [shurcooL]()

## Public API Calls

See ideas from [here](https://twitter.com/theprincessxena/status/1373265029739520009)

- Printer facts
- 
