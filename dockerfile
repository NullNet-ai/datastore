FROM node:22
ENV NODE_TLS_REJECT_UNAUTHORIZED=0
RUN apt update
RUN apt install nano python3 make g++ postgresql-client -y
RUN npm install -g pm2
RUN mkdir upload

WORKDIR /var/app
COPY package.json /var/app/package.json
COPY .npmrc /var/app/.npmrc
RUN npm i --force
COPY . /var/app
RUN npm run build
RUN mkdir sql
RUN mkdir upload

# Set environment variable to avoid interactive prompts
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get install redis -y


EXPOSE 5001
EXPOSE 6000
CMD ["pm2-runtime", "--name=data-store", "dist/main.js"]
