const { ssr } = require('@sveltech/ssr');
const express = require('express');
const proxy = require('http-proxy-middleware')
const app = express()

const ENTRYPOINT = 'dist/__app.html';
const APP = 'dist/build/bundle.js';
const PORT = 5050;


app.use(express.static('dist'));

app.get('*', async (req, res) => {
  const html = ssr(ENTRYPOINT, APP, req.url);
  res.send(html);
})

let apiProxy = proxy('/api', {target: 'http://localhost:3000/api'});
app.use(apiProxy)

// TODO: make the API proxy link conditional on dev/staging/prod

app.listen(PORT);
console.log("Serving app on port: " + PORT);
