'use client'
import { useEffect, useState } from 'react';

import { useSocket } from '@/hooks/useWebsocket';
import { RequestSignature } from '@/types/request';

export default function Home() {
  const { socket, isConnecting, toBinary, fromBinary } = useSocket('ws://localhost:80')
  const [messages, setMessages] = useState<string[]>([])
  const [roomId, setRoomId] = useState('')

  async function createRoom() {
    const msg = toBinary(RequestSignature.CREATE_ROOM, { name: 'test room' })
    socket?.send(msg);
  }

  async function joinRoom() {
    const msg = toBinary(RequestSignature.JOIN_ROOM, {
      room_id: roomId,
      display_name: `new_username_${new Date().getTime()}`
    })
    socket?.send(msg);
  }

  useEffect(() => {
    if (!socket || isConnecting) return
    socket.onmessage = async (event: MessageEvent<Blob>) => {
      const data = await fromBinary(event)
      setMessages(prevMessages => [...prevMessages, JSON.stringify(data)]);
    }
  }, [socket, isConnecting])

  return (
    <div>
      <div>
        <button
          onClick={() => {
            const msg = toBinary(RequestSignature.PING, { ping: true })
            socket?.send(msg);
          }}>
          join
        </button>
      </div>
      {/*  */}
      <div>
        <button
          onClick={() => setMessages([])}>
          clear
        </button>
      </div>
      {/*  */}
      <div>
        <button
          onClick={createRoom}>
          create room
        </button>
      </div>
      {/*  */}
      <div>
        <button
          onClick={joinRoom}>
          join room
        </button>
        <input type="text" placeholder='room id...' onChange={e => setRoomId(e.target.value)} />
      </div>
      {/*  */}
      <div>
        {/*  */}
        ---
      </div>
      <div>
        server log
        <div>---</div>
        {messages.map((msg, key) => (
          <div key={key}>entry {key}: {msg}</div>
        ))}
      </div>
    </div>
  );
}
