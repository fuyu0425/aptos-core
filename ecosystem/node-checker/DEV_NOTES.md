Random thoughts while developing, I'll remove these before landing the stack.

there should be a bunch of checker impls.
you then instantiate them all, some have requirements for insantiating them.
then they all have `check` methods.
i suspect we don't need something as complex as a DAG to structure the checkers.
the thing that fetches the metrics from the node should also be a trait.

do we need websockets here? perhaps a pull with rate limiting is sufficient.

should the request from the client be detached from the server? for example, a client
makes a "register" call with the server and then the server will keep periodically
checking the metrics until the client stops calling. how will this work with there
being multiple servers. how does this work in the local check mode?

ideally we can send back the first set of metrics instantly? or is that worth the dev
overhead to design a UI that handles that? could we leverage long polling for this?

Would we prefer to collect args for the server from the environment? Not as obvious
to the user but perhaps better for our setup.