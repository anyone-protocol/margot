# Use a lightweight base image like alpine
FROM alpine:3.12

# Install required tools like netcat
RUN apk add --no-cache bash netcat

# Set the working directory
WORKDIR /usr/src/app

# Copy the script into the container
COPY script.sh /usr/src/app/script.sh

# Make the shell script executable
RUN chmod +x /usr/src/app/script.sh

# Define the default command to run the script
CMD ["/usr/src/app/script.sh"]
