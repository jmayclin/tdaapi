environmental variables
Then positions
Then place order
Then sell order
Then stop limit

// the token thing is going to make me scream, so lets call that good for now
// better logging
// try making something that requires a json request
// properly parse in the price information
// fix the access token writing to file thing

Then scrape orders

The Rate Limiting could probably best be implemented as a struct that can be cloned (arc mutex)
and has a queue and ticker internally.

Limited Client needs to have a lock to prevent access to client when token is being refreshed