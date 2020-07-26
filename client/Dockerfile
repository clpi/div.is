FROM node:lts-alpine

WORKDIR /usr/src/div/client

COPY package*.json ./

RUN npm install

ENV API_URL http://io.div.is/api

COPY . .

RUN npm run build

EXPOSE 5005

CMD [ "npm", "run", "serve" ]
