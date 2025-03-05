const WebSocket = require('ws');

// Connect to the WebSocket server
const ws = new WebSocket('ws://0.0.0.0:80');

// Create a binary message with the type identifier (0 for coordinates)
// const msg = Buffer.concat([Buffer.from([0]), json_bytes]);

const SIGNATURES = {
    COORDINATES: 0,
    PING: 1
}

// Send the binary message when the connection is open
// ws.on('open', function open() {
//     for (let index = 0; index < 100; index++) {
//         if (index === 10) {
//             const ping = { ping: 1 };
//             // Serialize the coordinates to JSON and convert to a binary format
//             const json_data = JSON.stringify(ping);
//             const json_bytes = Buffer.from(json_data, 'utf8');
//             // coordinates identifier
//             const msg = Buffer.concat([Buffer.from([1]), json_bytes]);
//             console.log('Sent bad data:', msg);
//             ws.send(msg);
//             continue
//         }
//         if (index === 19) {
//             const ping = { blah: 1 };
//             // Serialize the coordinates to JSON and convert to a binary format
//             const json_data = JSON.stringify(ping);
//             const json_bytes = Buffer.from(json_data, 'utf8');
//             // coordinates identifier
//             const msg = Buffer.concat([Buffer.from([10]), json_bytes]);

//             console.log('Sent bad:', msg);
//             ws.send(msg);
//             continue
//         }

//         if (index % 2 === 0) {
//             const ping = { ping: true };
//             // Serialize the coordinates to JSON and convert to a binary format
//             const json_data = JSON.stringify(ping);
//             const json_bytes = Buffer.from(json_data, 'utf8');
//             // coordinates identifier
//             const msg = Buffer.concat([Buffer.from([1]), json_bytes]);
//             console.log('Sent Ping:', msg);
//             ws.send(msg);
//             continue
//         }
//         const coordinates = { x: index + 1, y: index };
//         // Serialize the coordinates to JSON and convert to a binary format
//         const json_data = JSON.stringify(coordinates);
//         const json_bytes = Buffer.from(json_data, 'utf8');
//         // coordinates identifier
//         const msg = Buffer.concat([Buffer.from([0]), json_bytes]);
//         console.log('len', Buffer.byteLength(Buffer.from([0, 1, 2_0000])));
//         console.log('Sent Coord:', msg);
//         ws.send(msg);
//     }
//     ws.close()
//     // console.log('Sent:', msg);
// });

ws.on('open', function open() {
    function createRoom() {
        const now = Date.now()
        const data = { name: `my test room ${now}` };
        const json_data = JSON.stringify(data);
        const json_bytes = Buffer.from(json_data, 'utf8');
        const msg = Buffer.concat([Buffer.from([3]), json_bytes]);
        console.log('Sent:', msg);
        ws.send(msg);
    }

    function joinRoom() {
        const now = Date.now()
        const data = { room_id: '2d8ec11f-6cf8-44ef-a2d4-c6c6f3543621', display_name: `my username ${now}` };
        const json_data = JSON.stringify(data);
        const json_bytes = Buffer.from(json_data, 'utf8');
        const msg = Buffer.concat([Buffer.from([2]), json_bytes]);
        console.log('SENT', msg.toString('hex').match(/../g).join(' '));
        ws.send(msg);
    }
    // createRoom()
    joinRoom()
    ws.close()
});

// Log any messages received from the server
ws.on('message', function incoming(data) {
    const json_data = data.slice(1).toString('utf8');
    const response = JSON.parse(json_data);
    console.log({ signature: data[0], response });
});

// Handle errors
ws.on('error', function error(err) {
    console.error('WebSocket error:', err);
});

// Handle connection close
ws.on('close', function close() {
    console.log('WebSocket connection closed');
});