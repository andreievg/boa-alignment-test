FROM --platform=linux/arm64 ubuntu:22.04

# Install basic Android runtime dependencies
RUN apt-get update && apt-get install -y \
    libc6 \
    libstdc++6 \
    libgcc-s1 \
    && rm -rf /var/lib/apt/lists/*

# Create Android-like directory structure
RUN mkdir -p /system/lib64 /data/local/tmp

# Set up environment similar to Android
ENV LD_LIBRARY_PATH=/system/lib64:/lib:/usr/lib
ENV PATH=/system/bin:/bin:/usr/bin

WORKDIR /data/local/tmp