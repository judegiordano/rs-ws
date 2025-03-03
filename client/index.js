const WebSocket = require('ws');

// Connect to the WebSocket server
const ws = new WebSocket('ws://0.0.0.0:80');

// Create a binary message with the type identifier (0 for coordinates)
// const msg = Buffer.concat([Buffer.from([0]), json_bytes]);

// Send the binary message when the connection is open
ws.on('open', function open() {
    for (let index = 0; index < 20; index++) {
        if (index % 2 === 0) {
            const ping = { ping: true };
            // Serialize the coordinates to JSON and convert to a binary format
            const json_data = JSON.stringify(ping);
            const json_bytes = Buffer.from(json_data, 'utf8');
            // coordinates identifier
            const msg = Buffer.concat([Buffer.from([1]), json_bytes]);
            console.log('Sent Ping:', msg);
            ws.send(msg);
            continue
        }
        const coordinates = { x: index + 1, y: index };
        // Serialize the coordinates to JSON and convert to a binary format
        const json_data = JSON.stringify(coordinates);
        const json_bytes = Buffer.from(json_data, 'utf8');

        // coordinates identifier
        const msg = Buffer.concat([Buffer.from([0]), json_bytes]);
        console.log('Sent Coord:', msg);
        ws.send(msg);
    }
    ws.close()
    // console.log('Sent:', msg);
});

// Log any messages received from the server
ws.on('message', function incoming(data) {
    console.log('Received:', data);
});

// Handle errors
ws.on('error', function error(err) {
    console.error('WebSocket error:', err);
});

// Handle connection close
ws.on('close', function close() {
    console.log('WebSocket connection closed');
});