'use client'
import { RequestSignature, ResponseSignature } from "@/types/request";
import { useEffect, useState } from "react";

export const useSocket = (url: string) => {
    const [socket, setSocket] = useState<WebSocket | null>(null);
    const [isConnecting, setIsConnecting] = useState(true);

    function toBinary(signature: RequestSignature, data: unknown) {
        const json_data = JSON.stringify(data);
        const json_bytes = Buffer.from(json_data, 'utf8') as unknown as Uint8Array<ArrayBufferLike>
        const reqSignature = Buffer.from([signature]) as unknown as Uint8Array<ArrayBufferLike>
        return Buffer.concat([reqSignature, json_bytes]);
    }

    async function fromBinary<T>(event: MessageEvent<Blob>): Promise<[ResponseSignature, T]> {
        const data = event.data
        const asciSignature = await data.slice(0, 1).text()
        const signature = asciSignature.charCodeAt(0);
        console.log({ signature });
        const body = await data.slice(1).text()
        return [signature, JSON.parse(body)]
    }

    useEffect(() => {
        const ws = new WebSocket(url);
        setSocket(ws);
        setIsConnecting(ws.readyState == ws.OPEN)
        return () => ws.close()
    }, [url]);

    useEffect(() => {
        if (!socket || isConnecting) return
        socket.onopen = () => {
            console.log('[CONNECTED]');
            const msg = toBinary(RequestSignature.PING, { ping: true })
            socket.send(msg);
        };
        socket.onerror = (error) => console.error('WebSocket Error:', error)
        socket.onclose = () => console.log('WebSocket Disconnected')
    }, [socket, isConnecting])

    return { socket, isConnecting, toBinary, fromBinary };
};