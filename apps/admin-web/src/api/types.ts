/**
 * Type definitions for WebSocket protocol messages.
 * See docs/08_WS_PROTOCOL.md
 */

// ========== Server to Client ==========

export type ServerMessageType =
  | 'connection.ready'
  | 'response.text'
  | 'response.transcript'
  | 'response.audio.start'
  | 'response.audio.end'
  | 'response.visemes'
  | 'error';

export interface ConnectionReady {
  type: 'connection.ready';
  v: number;
  frontend: string;
  session_id: string;
}

export interface ResponseText {
  type: 'response.text';
  v: number;
  request_id: string;
  text: string;
}

export interface ResponseTranscript {
  type: 'response.transcript';
  v: number;
  request_id: string;
  transcript: string;
  language?: string | null;
}

export interface InputAudioStart {
  type: 'input.audio.start';
  v: number;
  request_id: string;
  format: string;
  sample_rate: number;
  channels: number;
  language?: string | null;
}

export interface ResponseAudioStart {
  type: 'response.audio.start';
  v: number;
  request_id: string;
  format: string;
  sample_rate: number;
  channels: number;
}

export interface ResponseAudioEnd {
  type: 'response.audio.end';
  v: number;
  request_id: string;
}

export interface Viseme {
  time_ms: number;
  value: string;
  weight: number;
}

export interface ResponseVisemes {
  type: 'response.visemes';
  v: number;
  request_id: string;
  visemes: Viseme[];
}

export interface WebSocketError {
  type: 'error';
  v: number;
  request_id: string | null;
  code: string;
  message: string;
}

export type ServerMessage =
  | ConnectionReady
  | ResponseText
  | ResponseTranscript
  | ResponseAudioStart
  | ResponseAudioEnd
  | ResponseVisemes
  | WebSocketError;
