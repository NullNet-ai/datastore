# Step 1: Use Node.js 21 Alpine base image (lightweight)
FROM node:21-alpine

# Step 2: Install dependencies for node-gyp and better-sqlite3
RUN apk add python3 make g++
RUN apk add --no-cache python3 make g++ postgresql-client
# Step 3: Set the working directory inside the container
WORKDIR /usr/src/app

# Step 4: Copy package.json and package-lock.json from the project root
COPY ../package*.json ./

# Step 5: Install dependencies
RUN npm install --force

# Step 6: Copy the entire project directory (excluding files via .dockerignore)
COPY ../ ./

# Step 7: Build the NestJS application
RUN npm run build

# Step 8: Copy the entrypoint script and set executable permissions
COPY docker/entrypoint.sh /usr/src/app/entrypoint.sh
RUN chmod +x /usr/src/app/entrypoint.sh

# Step 9: Expose the port dynamically (default to 5001 if not defined)
EXPOSE ${PORT:-5001}
EXPOSE 6000

# Step 10: Use the entrypoint script to run migrations and start the app
ENTRYPOINT ["/usr/src/app/entrypoint.sh"]
