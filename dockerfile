FROM node:20
ENV NODE_TLS_REJECT_UNAUTHORIZED=0
RUN apt update
RUN apt-get install nano python3 make g++ postgresql-client -y
RUN apt autoremove -y
RUN npm install -g pm2
RUN mkdir upload

WORKDIR /var/app
COPY package.json /var/app/package.json
COPY .npmrc /var/app/.npmrc
RUN npm i --force
COPY . /var/app
RUN npm run build
ENV DATABASE_URL=${DATABASE_URL}
RUN npm run drizzle:generate

EXPOSE 5001
EXPOSE 6000
CMD [ "npm","run", "start:prod" ]

