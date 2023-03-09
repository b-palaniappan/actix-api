####################################################################################################
## Builder
####################################################################################################
FROM rust:slim AS builder

# install dependencies
RUN rustup target add x86_64-unknown-linux-musl \
    && apt-get update && apt-get install -y musl-tools musl-dev libssl-dev pkg-config \
    && update-ca-certificates

# Create appuser
ENV USER=webapp
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /actix-api

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /actix-api

# Copy our build
COPY --from=builder /actix-api/target/x86_64-unknown-linux-musl/release/actix-api ./

# Use an unprivileged user.
USER webapp:webapp

CMD ["/actix-api/actix-api"]
