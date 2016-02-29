var express = require('express');
var parser = require('body-parser');
var app = express();
const port = process.env.PORT || 80;

app.post('/', parser.text(), function (req, res) {
  console.log('Plain data received from ' + req.connection.remoteAddress);
  res.send("Gotcha! Here's your data back: " + req.body);
});

app.post('/json', parser.json(), function (req, res) {
  console.log('JSON data received from ' + req.connection.remoteAddress);
  res.send("Gotcha! Here's your data back: " + req.body);
});

app.listen(port, function () {
  console.log('Server listening on port ' + port);
});
