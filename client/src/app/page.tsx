'use client'
import { useEffect, useState } from 'react';

enum RequestSignature {
  PING = 0
}

interface Pong {
  ok: true
}

const useSocket = (url: string) => {
  const [socket, setSocket] = useState<WebSocket | null>(null);
  const [isConnecting, setIsConnecting] = useState(true);

  function toBinary(signature: RequestSignature, data: any) {
    const json_data = JSON.stringify(data);
    const json_bytes = Buffer.from(json_data, 'utf8') as unknown as Uint8Array<ArrayBufferLike>
    const reqSignature = Buffer.from([signature]) as unknown as Uint8Array<ArrayBufferLike>
    return Buffer.concat([reqSignature, json_bytes]);
  }

  async function fromBinary<T>(event: MessageEvent<Blob>): Promise<T> {
    const data = event.data
    const signature = await data.slice(0, 1).text()
    const body = await data.slice(1).text()
    return JSON.parse(body)
  }

  useEffect(() => {
    const ws = new WebSocket(url);
    setSocket(ws);
    setIsConnecting(ws.readyState == ws.OPEN)
    return () => ws.close()
  }, [url]);

  return { socket, isConnecting, toBinary, fromBinary };
};

export default function Home() {
  const { socket, isConnecting, toBinary, fromBinary } = useSocket('ws://localhost:80')
  const [messages, setMessages] = useState<string[]>([])

  useEffect(() => {
    if (!socket || isConnecting) return
    socket.onopen = () => {
      console.log('[CONNECTED]');
      const msg = toBinary(RequestSignature.PING, { ping: true })
      socket.send(msg);
    };
    socket.onmessage = async (event: MessageEvent<Blob>) => {
      const data = await fromBinary(event)
      setMessages(prevMessages => [...prevMessages, JSON.stringify(data)]);
      console.log({ data });
    }
    socket.onerror = (error) => console.error('WebSocket Error:', error)
    socket.onclose = () => console.log('WebSocket Disconnected')
  }, [socket])

  return (
    <div>
      <button
        onClick={() => {
          const msg = toBinary(RequestSignature.PING, { ping: true })
          socket?.send(msg);
        }}>
        join
      </button>
      <button
        onClick={() => setMessages([])}>
        clear
      </button>
      <div>
        ---
      </div>
      <div>
        server log
        <div>---</div>
        {messages.map((msg, key) => (
          <div key={key}>entry: {msg}</div>
        ))}
      </div>
    </div>
  );
}
