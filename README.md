# Rust GraphQL example app

based on [graphql-app-example](https://github.com/davidpdrsn/graphql-app-example)

## Dependencies

```sh
# app dependencies
sudo dnf install sqlite-devel sqlite
# Additional dependencies to build diesel_cli
sudo dnf install mysql-devel postgresql-devel
cargo install diesel_cli
```

## Running the app

Bootstrap the databases (only needs to be done once, or if you want to reset the dbs)

```sh
./bin/setup
```

Start the app

```sh
cargo run
```

Then go to <http://localhost:8000/graphiql>.

Or run the tests with

```bash
$ cargo test
```

## Note

This is by no means meant to demonstrate the best practices for making a web app with Rocket. Several important topics such as authentication and error handling is not addressed. It is meant to be used as a template for starting new apps.
