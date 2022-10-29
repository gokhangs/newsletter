#Most programming languages follow the same playbook: you COPY a lock-file of some kind first, 
#build your dependencies, COPY over the rest of your source code and then build your project.

#STEP 1 Compute receipt file
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR app
RUN apt update && apt install lld clang -y

#STEP 2 Caches our dependencies and builds our binary. 
#As long as our dependencies do not change the recipe.json file will stay the same.
#So, the outcome of "cargo chef cook --release --recipe-path recipe.json" will be cached, massively speeding up builds
FROM chef as planner
COPY . .
# Compute a lock-like file for our project 
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .
ENV SQLX_OFFLINE true
# Build our project
RUN cargo build --release --bin zero2prod


#runtime stage
FROM debian:bullseye-slim AS runtime
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod
# We need the configuration file at runtime!
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]