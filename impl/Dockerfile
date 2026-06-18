# =============================================================================
# VyroLang sandbox image — Compiler API + VyroIDE + VyroVM in one container.
# Build a static-ish musl binary, then run it as a non-root user in a minimal
# read-only Alpine. This is the "Docker Sandbox" layer of the architecture.
# =============================================================================

FROM rust:1-alpine AS build
RUN apk add --no-cache musl-dev
WORKDIR /build
COPY . .
RUN cargo build --release

FROM alpine:3.20
RUN adduser -D -u 10001 vyro
COPY --from=build /build/target/release/vyro /usr/local/bin/vyro
USER vyro
ENV VYRO_HOST=0.0.0.0
EXPOSE 8787
# In-process limits (instruction budget + wall-clock) are enforced by the VM;
# container limits (below) bound CPU/RAM/PIDs from the outside.
CMD ["vyro", "serve", "8787"]
