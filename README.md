# salsa

*A generic framework for on-demand, incrementalized computation.*

## Obligatory warning

Very much a WORK IN PROGRESS at this point. Ready for experimental use
but expect frequent breaking changes.

## Credits

This system is heavily inspired by adapton, glimmer, and rustc's query
system. So credit goes to Eduard-Mihai Burtescu, Matthew Hammer,
Yehuda Katz, and Michael Woerister.

## Key idea

The key idea of `salsa` is that you define your program as a set
of **queries**. Queries come in two basic varieties:

- **Inputs**: the base inputs to your system. You can change these
  whenever you like.
- **Functions**: pure functions (no side effects) that transform your
  inputs into other values. The results of queries is memoized to
  avoid recomputing them a lot. When you make changes to the inputs,
  we'll figure out (fairly intelligently) when we can re-use these
  memoized values and when we have to recompute them.

## How to use Salsa in three easy steps

Using salsa is as easy as 1, 2, 3...

1. Define one or more **query context traits** that contain the inputs
   and queries you will need. We'll start with one such trait, but
   later on you can use more than one to break up your system into
   components (or spread your code across crates).
2. **Implement the queries** using the `query_definition!` macro.
3. **Implement the query context trait** for your query context
   struct, which contains a full listing of all the inputs/queries you
   will be using. The query struct will contain the storage for all of
   the inputs/queries and may also contain anything else that your
   code needs (e.g., configuration data).
  
To see an example of this in action, check out [the `hello_world`
example](examples/hello_world/main.rs), which has a number of comments
explaining how things work. The [`hello_world`
README](examples/hello_world/README.md) has a more detailed writeup.

