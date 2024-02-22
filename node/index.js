const net = require('net');

const client = new net.Socket();

client.connect(8080, '127.0.0.1', () => {
  console.log('Conectado ao servidor');
  client.write('Testando 123');
});

client.on('data', (data) => {
  console.log(data.toString());
  //if (workerMessage.path === '/') console.log("Você acessou a rota inicial");
});

client.on('close', () => {
  console.log("Cliente fechado");
})