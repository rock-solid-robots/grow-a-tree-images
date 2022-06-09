FROM rustlang/rust:nightly-slim as builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=app
ENV UID=10001

RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "${UID}" \
  "${USER}"

WORKDIR /app

COPY dummy.rs .

# If this changed likely the Cargo.toml changed so lets trigger the
# recopying of it anyways
COPY Cargo.lock Cargo.toml ./

# We'll get to what this substitution is for but replace main.rs with
# lib.rs if this is a library
RUN sed -i 's/src\/main.rs/dummy.rs/' Cargo.toml

# Drop release if you want debug builds. This step cache's our deps!
RUN cargo build --target x86_64-unknown-linux-musl --release

# Now return the file back to normal
RUN sed -i 's/dummy.rs/src\/main.rs/' Cargo.toml

# Copy the rest of the files into the container
COPY . .

# Now this only builds our changes to things like src
RUN cargo build --target x86_64-unknown-linux-musl --release

## Final Image

FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

# Copy our build
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/gat-image-server ./

# Use an unprivileged user.
USER app:app

# Run the binary
CMD ["/app/gat-image-server"]