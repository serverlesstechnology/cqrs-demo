# cqrs-demo

> A demo application using the [cqrs-es](https://github.com/serverlesstechnology/cqrs) framework
> and postgres persistence.

## Requirements
- rust stable
- docker & [docker-compose](https://docs.docker.com/compose/) for starting an instance of Postgres
- [postman](https://www.postman.com/) (or curl or your favorite Restful client)

Alternatively, if a a standard Postgres instance is running locally it can be utilized instead of the docker instance,
see [the init script](db/init.sql) for the expected table configuration. 
## Installation

Clone this repository

    git clone https://github.com/serverlesstechnology/cqrs-demo

Enter the project folder and start postgress

    cd cqrs-demo
    docker-compose up -d

Start the application

    cargo run

Call the API, the easiest way to do this is to import 
[the provided postman collection](cqrs-demo.postman_collection.json)
into your Postman client. Note that the command calls return a 204 status with no content. 
For feedback on state you should call a query.

### Documentation

[Documentation can be found here](https://doc.rust-cqrs.org/)
 for the CQRS and event sourcing portions of this demo application. 