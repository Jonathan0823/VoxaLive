/**
 * WebSocket client for VoxaLive unified endpoint.
 * Handles audio input for STT and text input for conversation.
 */

import type { ServerMessage } from './types';

const PROTOCOL_VERSION = 1;

/** Get WebSocket URL based on current location */
function getWebSocketUrl(): string {
  const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
  const host = window.location.host;
  return `${protocol}//${host}/ws/unified?frontend=vts`;
}

/** Message types for client to server */
export type ClientMessageType = 'input.text' | 'input.audio.start' | 'input.audio.end';

export interface InputTextMessage {
  v: number;
  type: 'input.text';
  request_id: string;
  text: string;
}

export interface InputAudioStartMessage {
  v: number;
  type: 'input.audio.start';
  request_id: string;
  format: string;
  sample_rate: number;
  channels: number;
  language?: string | null;
}

export interface InputAudioEndMessage {
  v: number;
  type: 'input.audio.end';
  request_id: string;
}

export type OutgoingMessage = InputTextMessage | InputAudioStartMessage | InputAudioEndMessage;

/** Callback types for incoming messages */
export type MessageHandler = (message: ServerMessage) => void;
export type ConnectionHandler = (sessionId: string) => void;
export type ErrorHandler = (error: Error) => void;

export interface WebSocketClientOptions {
  onMessage?: MessageHandler;
  onConnected?: ConnectionHandler;
  onDisconnected?: ConnectionHandler;
  onError?: ErrorHandler;
}

export class WebSocketClient {
  private socket: WebSocket | null = null;
  private options: Required<WebSocketClientOptions>;
  private sessionId: string | null = null;
  private requestIdCounter = 0;

  constructor(options: WebSocketClientOptions = {}) {
    this.options = {
      onMessage: options.onMessage ?? (() => {}),
      onConnected: options.onConnected ?? (() => {}),
      onDisconnected: options.onDisconnected ?? (() => {}),
      onError: options.onError ?? (() => {}),
    };
  }

  /** Generate a unique request ID */
  private generateRequestId(): string {
    return `req_${Date.now()}_${++this.requestIdCounter}`;
  }

  /** Connect to the WebSocket server */
  connect(): Promise<string> {
    return new Promise((resolve, reject) => {
      if (this.socket?.readyState === WebSocket.OPEN) {
        resolve(this.sessionId ?? '');
        return;
      }

      const url = getWebSocketUrl();
      this.socket = new WebSocket(url);

      this.socket.onopen = () => {
        console.log('[WS] Connected to', url);
      };

      this.socket.onmessage = (event) => {
        try {
          if (typeof event.data === 'string') {
            const message = JSON.parse(event.data) as ServerMessage;
            
            // Handle connection.ready
            if (message.type === 'connection.ready') {
              this.sessionId = message.session_id;
              this.options.onConnected(message.session_id);
              resolve(message.session_id);
              return;
            }

            this.options.onMessage(message);
          } else if (event.data instanceof Blob) {
            // Handle binary audio data - for now just log it
            console.log('[WS] Received binary data:', event.data.size, 'bytes');
          }
        } catch (err) {
          console.error('[WS] Failed to parse message:', err);
        }
      };

      this.socket.onerror = (event) => {
        console.error('[WS] Error:', event);
        this.options.onError(new Error('WebSocket connection error'));
        reject(new Error('WebSocket connection error'));
      };

      this.socket.onclose = (event) => {
        console.log('[WS] Disconnected:', event.code, event.reason);
        const oldSessionId = this.sessionId;
        this.sessionId = null;
        this.options.onDisconnected(oldSessionId ?? '');
      };
    });
  }

  /** Disconnect from the WebSocket server */
  disconnect(): void {
    if (this.socket) {
      this.socket.close();
      this.socket = null;
    }
  }

  /** Check if connected */
  isConnected(): boolean {
    return this.socket?.readyState === WebSocket.OPEN;
  }

  /** Send a JSON message */
  private sendJson(message: OutgoingMessage): void {
    if (this.socket?.readyState === WebSocket.OPEN) {
      this.socket.send(JSON.stringify(message));
    } else {
      console.warn('[WS] Cannot send message, not connected');
    }
  }

  /** Send binary data */
  private sendBinary(data: ArrayBuffer): void {
    if (this.socket?.readyState === WebSocket.OPEN) {
      this.socket.send(data);
    } else {
      console.warn('[WS] Cannot send binary, not connected');
    }
  }

  /** Send text input */
  sendText(text: string): string {
    const requestId = this.generateRequestId();
    const message: InputTextMessage = {
      v: PROTOCOL_VERSION,
      type: 'input.text',
      request_id: requestId,
      text,
    };
    this.sendJson(message);
    return requestId;
  }

  /** Signal audio input start */
  sendAudioStart(language?: string | null): string {
    const requestId = this.generateRequestId();
    const message: InputAudioStartMessage = {
      v: PROTOCOL_VERSION,
      type: 'input.audio.start',
      request_id: requestId,
      format: 'pcm16',
      sample_rate: 16000,
      channels: 1,
      language: language ?? undefined,
    };
    this.sendJson(message);
    return requestId;
  }

  /** Send binary audio chunk */
  sendAudioChunk(data: ArrayBuffer): void {
    this.sendBinary(data);
  }

  /** Signal audio input end */
  sendAudioEnd(requestId: string): void {
    const message: InputAudioEndMessage = {
      v: PROTOCOL_VERSION,
      type: 'input.audio.end',
      request_id: requestId,
    };
    this.sendJson(message);
  }
}

/** Singleton instance for convenience */
let clientInstance: WebSocketClient | null = null;

export function getWebSocketClient(options?: WebSocketClientOptions): WebSocketClient {
  if (!clientInstance) {
    clientInstance = new WebSocketClient(options);
  }
  return clientInstance;
}
