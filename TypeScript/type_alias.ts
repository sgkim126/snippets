type IP = string;
type Port = number;

function connect(ip: IP, port: Port) {
  console.error("%j %j", ip, port);
}

connect("127.0.0.1", 80);
