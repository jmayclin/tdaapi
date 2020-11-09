get serde working
environmental variables
Then token refresh
Then positions
Then price history
Then SQL lit
Then current price
Then place order
Then sell order
Then stop limit

Then scrape orders

The Rate Limiting could probably best be implemented as a struct that can be cloned (arc mutex)
and has a queue and ticker internally.

Limited Client needs to have a lock to prevent access to client when token is being refreshed