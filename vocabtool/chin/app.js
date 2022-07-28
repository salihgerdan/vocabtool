var net = require('net');

const tokenize = require('chinese-tokenizer').loadFile('./cedict_ts.u8')
// needs the cedict dictionary at this location

var server = net.createServer();
server.on('connection', handleConnection);

server.listen(9001, function() {
  console.log('server listening to %j', server.address());
});

function handleConnection(conn) {
  var remoteAddress = conn.remoteAddress + ':' + conn.remotePort;
  console.log('new client connection from %s', remoteAddress);
  conn.setEncoding('utf8');
  conn.on('data', onConnData);  
  conn.once('close', onConnClose);  
  conn.on('error', onConnError);
  function onConnData(d) {
    console.log('connection data from %s: %j', remoteAddress, d);
    conn.end(JSON.stringify(tokenize(d)));
  }
  function onConnClose() {  
    console.log('connection from %s closed', remoteAddress);  
  }
  function onConnError(err) {  
    console.log('Connection %s error: %s', remoteAddress, err.message);  
  }  
}
