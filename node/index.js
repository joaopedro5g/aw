const net = require('net');

const client = new net.Socket();

client.connect(8080, '127.0.0.1', () => {
  console.log('Conectado ao servidor');
});

client.on('data', (data) => {
  const workerMessage = JSON.parse(data.toString());
  console.log(workerMessage);
  if (workerMessage.path === '/') console.log("Você acessou a rota inicial");
});

client.on('close', () => {
  console.log("Cliente fechado");
})