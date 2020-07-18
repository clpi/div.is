const { ssr } = require('@sveltech/ssr');
const express = require('express');
const app = express()

const ENTRYPOINT = 'dist/__app.html';
const APP = 'dist/build/bundle.js';
const PORT = 5050;

app.use(express.static('dist'));

app.get('*', async (req, res) => {
  const html = ssr(ENTRYPOINT, APP, req.url);
  res.send(html);
})

app.listen(PORT);
console.log("Serving app on port: " + PORT);
