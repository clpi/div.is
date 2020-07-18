FROM node:lts-alpine

WORKDIR /usr/src/app/client

COPY package*.json ./

RUN npm install

COPY . .

EXPOSE 5005

ENV HOST=0.0.0.0

CMD [ "npm", "run", "serve" ]
