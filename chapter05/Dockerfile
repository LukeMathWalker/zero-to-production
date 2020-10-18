# We are going to use a multi-stage Docker build.
# Once we have a compiled binary there is no need to keep around source codes or
# any of the intermediate artifacts required to generate it.
# We'll use a "builder stage" to get to the binary and then copy it over to our
# "runner stage", the one we'll use as runtime when the Docker image is launched!

# Builder stage
# We use the latest stable release as base image
FROM rust:1.47 as builder

# Copy all files from our working environment to our Docker build context
# We'll put them inside the `app` folder (which will be created if it does not exist)
COPY . app
# Let's switch our working directory to `app` (equivalent to `cd app`)
WORKDIR app
# Let's build our binary!
# We'll use the release profile to make it faaaast
RUN cargo build --release

# Runtime stage
# We use again the latest stable release as base image
FROM rust:1.47 as runtime

# Set our working directory to the `/bin` folder
# Where runnable binaries are usually stored in Linux
WORKDIR /bin
# Let's copy our compiled binary from the builder environment to our runtime environment
COPY --from=builder /app/target/release/chapter05 ./zero2prod
# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./zero2prod"]
