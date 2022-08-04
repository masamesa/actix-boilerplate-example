# actix-boilerplate-example

Example boilerplate showcasing how to use Rust, Actix REST library and Docker.

# Building and running
Build the image by doing:

```docker build -t rustexample .```

Run the container by doing:

```docker run --rm -p 8080:8080 --name server rustexample ```

# Project stucture

```
src
    │   main.rs # Main which contains the factory to start up the API
    │
    └───services
        │   mod.rs # Mod for the child services
        │
        └───user
                mod.rs     # Mod for the dependent files
                models.rs  # Models for structs, enums, impl and traits.
                           # Feel free to break this up into multiple models in a model folder.
                router.rs  # Router for the endpoints of this service
                service.rs # Buisness logic 
```

# Extra comments

When posting to the endpoint make sure to use `http` unless you're using a reverse proxy with SSL.

Ensure the ports under the argument `-p` in the `docker run` command are the same as the `ENV SERER_PORT` in the `dockerfile`

I didn't bother making this more complex than this needed to be. However, I would strongly suggest looking into [dependency injection](https://medium.com/geekculture/dependency-injection-in-rust-3822bf689888) to implement a database service. 

Read the comments in the project for more information.
