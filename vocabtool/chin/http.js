const http = require("http");

const PORT = process.env.PORT || 9000; 

const tokenize = require('chinese-tokenizer').loadFile('./cedict_ts.u8')

const server = http.createServer(async (req, res) => {
    //set the request route
    if (req.url === "/api" && req.method === "GET") {
        //response headers
        res.writeHead(200, { "Content-Type": "application/json" });
        //set the response
	res.write(req.getRawHeaderNames());
        res.write("Hi there, This is a Vanilla Node.js API");
        //end the response
        res.end();
    }

    // If no route present
    else {
        res.writeHead(404, { "Content-Type": "application/json" });
        res.end(JSON.stringify({ message: "Route not found" }));
    }
});

server.listen(PORT, () => {
    console.log(`server started on port: ${PORT}`);
});

    //conn.write(JSON.stringify(tokenize(d), null, '  '));
