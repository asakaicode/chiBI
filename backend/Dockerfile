#################
## development ##
#################
FROM rust:1.83.0 AS development
WORKDIR /code

RUN USER=root cargo init
COPY Cargo.toml src ./
RUN cargo fetch

###########
## build ##
###########
FROM development AS build
COPY . .
RUN cargo build --release

################
## production ##
################
FROM ubuntu:22.04 AS production
RUN apt-get update && apt-get install -y bash libmysqlclient-dev

WORKDIR /app

COPY --from=build /code/target/release/app /app/app
RUN chmod +x /app/app

USER 1001

EXPOSE 8080

CMD ["/app/app"]