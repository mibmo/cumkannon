ARG PKG_NAME=cumkannon
ARG RUST_VERSION=1.56.0


FROM docker.io/rust:${RUST_VERSION}-alpine AS build
ARG PKG_NAME
WORKDIR /build/

# fetch package dependencies
RUN apk add build-base cmake musl-dev openssl-dev

# unprivileged user
RUN adduser -Du 1000 -g ${PKG_NAME} ${PKG_NAME}
RUN chown -R ${PKG_NAME}:${PKG_NAME} /build
USER ${PKG_NAME}

# build dependencies
RUN cargo init --name $PKG_NAME
ADD --chown=${PKG_NAME}:${PKG_NAME} Cargo.toml Cargo.lock ./
RUN CXX=g++ cargo build --release

# build binary
ADD --chown=${PKG_NAME}:${PKG_NAME} src ./src
RUN touch src/*.rs # make cargo think files have been modified
RUN cargo build --release

FROM scratch
ARG PKG_NAME
WORKDIR /app

# run as unprivileged user
COPY --from=build /etc/passwd /etc/passwd
USER ${PKG_NAME}

# run app
COPY --from=build /build/target/release/${PKG_NAME} /app/bin

ENTRYPOINT ["./bin"]
