'use client'
import { useEffect, useState } from 'react';

import { useSocket } from '@/hooks/useWebsocket';
import { RequestSignature, ResponseSignature } from '@/types/request';

export default function Home() {
  const { socket, isConnecting, toBinary, fromBinary } = useSocket('ws://localhost:80')
  const [messages, setMessages] = useState<string[]>([])
  const [roomId, setRoomId] = useState('')
  const [playerId, setPLayerId] = useState('')

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

  async function leaveRoom() {
    const msg = toBinary(RequestSignature.LEAVE_ROOM, {
      room_id: roomId,
      player_id: playerId
    })
    socket?.send(msg);
  }

  useEffect(() => {
    if (!socket || isConnecting) return
    socket.onmessage = async (event: MessageEvent<Blob>) => {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const [signature, data] = await fromBinary<any>(event)
      if (signature == ResponseSignature.JOIN_ROOM_SUCCESS) {
        setPLayerId(data.player_id)
        console.log({ playerId });
      }
      setMessages(prevMessages => [...prevMessages, JSON.stringify(data)]);
    }
    socket.onclose = () => leaveRoom()

  }, [socket, isConnecting])

  return (
    <div>
      <div>
        <button
          onClick={() => {
            const msg = toBinary(RequestSignature.PING, { ping: true })
            socket?.send(msg);
          }}>
          ping
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
        <button
          onClick={leaveRoom}>
          leave room
        </button>
      </div>
      {/*  */}
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
