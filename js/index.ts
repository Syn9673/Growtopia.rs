import restana = require("restana");
const http = restana();

http.post("*", (req, res) => {
  res.send("server|127.0.0.1\nport|17091\ntype|1\n#maint|Mainetrance message (Not used for now) -- NodeJS-GTPS\n\nbeta_server|127.0.0.1\nbeta_port|17091\n\nbeta_type|1\nmeta|localhost\nRTENDMARKERBS1001");
});

http.start(80);