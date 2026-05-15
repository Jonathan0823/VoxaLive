import React, { useState, useCallback, useRef, useEffect } from 'react';
import { testLlm, testTts, testVts, type LlmTestResponse, type TtsTestResponse, type VtsTestResponse } from '../api/admin';
import { WebSocketClient, getWebSocketClient } from '../api/websocket';
import type { ServerMessage } from '../api/types';

const LLM_PROVIDERS = ['gemini', 'openrouter', 'ollama'] as const;
const TTS_PROVIDERS = ['piper', 'qwen'] as const;

type LlmProvider = (typeof LLM_PROVIDERS)[number];
type TtsProvider = (typeof TTS_PROVIDERS)[number];

const TestConsole: React.FC = () => {
  // LLM test state
  const [llmMessage, setLlmMessage] = useState('Hello, test response only.');
  const [llmProvider, setLlmProvider] = useState<LlmProvider>('gemini');
  const [llmLoading, setLlmLoading] = useState(false);
  const [llmResult, setLlmResult] = useState<LlmTestResponse | null>(null);
  const [llmError, setLlmError] = useState<string | null>(null);

  // TTS test state
  const [ttsText, setTtsText] = useState('Hello, this is a voice test.');
  const [ttsProvider, setTtsProvider] = useState<TtsProvider>('piper');
  const [ttsLoading, setTtsLoading] = useState(false);
  const [ttsResult, setTtsResult] = useState<TtsTestResponse | null>(null);
  const [ttsError, setTtsError] = useState<string | null>(null);
  const [ttsAudioUrl, setTtsAudioUrl] = useState<string | null>(null);
  const [ttsAudioError, setTtsAudioError] = useState<string | null>(null);
  const audioUrlRef = useRef<string | null>(null);

  // VTS test state
  const [vtsLoading, setVtsLoading] = useState(false);
  const [vtsResult, setVtsResult] = useState<VtsTestResponse | null>(null);
  const [vtsError, setVtsError] = useState<string | null>(null);

  // Audio input state
  const [isRecording, setIsRecording] = useState(false);
  const [audioError, setAudioError] = useState<string | null>(null);
  const [audioTranscript, setAudioTranscript] = useState<string | null>(null);
  const [wsConnected, setWsConnected] = useState(false);
  const [recordedAudioUrl, setRecordedAudioUrl] = useState<string | null>(null);
  const recordedAudioUrlRef = useRef<string | null>(null);
  const wsClientRef = useRef<WebSocketClient | null>(null);
  const mediaRecorderRef = useRef<MediaRecorder | null>(null);
  const audioContextRef = useRef<AudioContext | null>(null);
  const audioChunksRef = useRef<Blob[]>([]);
  const currentRequestIdRef = useRef<string | null>(null);

  const handleLlmTest = useCallback(async (e: React.FormEvent) => {
    e.preventDefault();
    setLlmLoading(true);
    setLlmError(null);
    setLlmResult(null);

    try {
      const result = await testLlm({ message: llmMessage, provider: llmProvider });
      setLlmResult(result);
    } catch (err) {
      setLlmError(err instanceof Error ? err.message : 'LLM test failed');
    } finally {
      setLlmLoading(false);
    }
  }, [llmMessage, llmProvider]);

  const handleTtsTest = useCallback(async (e: React.FormEvent) => {
    e.preventDefault();
    setTtsLoading(true);
    setTtsError(null);
    setTtsResult(null);
    setTtsAudioError(null);

    // Revoke previous audio URL to avoid memory leaks
    if (audioUrlRef.current) {
      URL.revokeObjectURL(audioUrlRef.current);
      audioUrlRef.current = null;
      setTtsAudioUrl(null);
    }

    try {
      const result = await testTts({ text: ttsText, provider: ttsProvider });
      setTtsResult(result);

      // Create audio URL from base64 if present
      if (result.audio_base64) {
        try {
          // Determine MIME type based on audio format
          const mimeType = result.audio_format === 'wav' ? 'audio/wav' : 'audio/wav';
          const byteCharacters = atob(result.audio_base64);
          const byteNumbers = new Array(byteCharacters.length);
          for (let i = 0; i < byteCharacters.length; i++) {
            byteNumbers[i] = byteCharacters.charCodeAt(i);
          }
          const byteArray = new Uint8Array(byteNumbers);
          const blob = new Blob([byteArray], { type: mimeType });
          const audioUrl = URL.createObjectURL(blob);
          audioUrlRef.current = audioUrl;
          setTtsAudioUrl(audioUrl);
        } catch (decodeErr) {
          setTtsAudioError('Failed to decode audio payload');
        }
      }
    } catch (err) {
      setTtsError(err instanceof Error ? err.message : 'TTS test failed');
    } finally {
      setTtsLoading(false);
    }
  }, [ttsText, ttsProvider]);

  const handleVtsTest = useCallback(async () => {
    setVtsLoading(true);
    setVtsError(null);
    setVtsResult(null);

    try {
      const result = await testVts();
      setVtsResult(result);
    } catch (err) {
      setVtsError(err instanceof Error ? err.message : 'VTS test failed');
    } finally {
      setVtsLoading(false);
    }
  }, []);

  // Initialize WebSocket connection
  const initWebSocket = useCallback(async () => {
    if (wsClientRef.current?.isConnected()) {
      return;
    }

    const client = getWebSocketClient({
      onConnected: (sessionId) => {
        console.log('[TestConsole] WS connected:', sessionId);
        setWsConnected(true);
      },
      onDisconnected: () => {
        console.log('[TestConsole] WS disconnected');
        setWsConnected(false);
      },
      onMessage: (message: ServerMessage) => {
        console.log('[TestConsole] WS message:', message);
        if (message.type === 'response.text') {
          setAudioTranscript(message.text);
        } else if (message.type === 'error') {
          setAudioError(message.message);
        }
      },
      onError: (error) => {
        console.error('[TestConsole] WS error:', error);
        setAudioError(error.message);
      },
    });

    try {
      await client.connect();
      wsClientRef.current = client;
    } catch (err) {
      setAudioError(err instanceof Error ? err.message : 'Failed to connect');
    }
  }, []);

  // Convert audio blob to PCM16 format (16kHz, mono, 16-bit little-endian)
  const convertToPcm16 = useCallback(async (audioBlob: Blob): Promise<ArrayBuffer> => {
    const arrayBuffer = await audioBlob.arrayBuffer();
    const audioContext = new AudioContext();
    audioContextRef.current = audioContext;

    const audioBuffer = await audioContext.decodeAudioData(arrayBuffer);
    
    // Convert to mono and resample to 16kHz
    const channelData = audioBuffer.getChannelData(0);
    const targetSampleRate = 16000;
    const targetLength = Math.round(channelData.length * targetSampleRate / audioBuffer.sampleRate);
    
    // Create the PCM16 buffer
    const pcm16Data = new Int16Array(targetLength);
    
    for (let i = 0; i < targetLength; i++) {
      const sourceIndex = i * audioBuffer.sampleRate / targetSampleRate;
      const sample = channelData[Math.floor(sourceIndex)];
      // Clamp to valid int16 range
      pcm16Data[i] = Math.max(-32768, Math.min(32767, Math.round(sample * 32767)));
    }

    // Convert to little-endian bytes
    const pcm16Bytes = new Uint8Array(pcm16Data.buffer);
    return pcm16Bytes.buffer;
  }, []);

  // Start recording audio
  const startRecording = useCallback(async () => {
    setAudioError(null);
    setAudioTranscript(null);
    audioChunksRef.current = [];

    try {
      // Initialize WebSocket if not connected
      await initWebSocket();

      // Request microphone access
      const stream = await navigator.mediaDevices.getUserMedia({ 
        audio: {
          sampleRate: 16000,
          channelCount: 1,
          echoCancellation: true,
          noiseSuppression: true,
        } 
      });

      // Create MediaRecorder
      const mediaRecorder = new MediaRecorder(stream, {
        mimeType: 'audio/webm;codecs=opus',
      });
      mediaRecorderRef.current = mediaRecorder;

      // Handle data available
      mediaRecorder.ondataavailable = async (event) => {
        if (event.data.size > 0) {
          audioChunksRef.current.push(event.data);
        }
      };

      // Start recording
      mediaRecorder.start(100); // Collect data every 100ms
      setIsRecording(true);

      // Send audio start message
      if (wsClientRef.current?.isConnected()) {
        currentRequestIdRef.current = wsClientRef.current.sendAudioStart();
      }
    } catch (err) {
      if (err instanceof Error) {
        if (err.name === 'NotAllowedError' || err.name === 'PermissionDeniedError') {
          setAudioError('Microphone permission denied. Please allow microphone access in your browser settings.');
        } else {
          setAudioError(`Failed to start recording: ${err.message}`);
        }
      } else {
        setAudioError('Failed to start recording');
      }
    }
  }, [initWebSocket]);

  // Stop recording and send audio
  const stopRecording = useCallback(async () => {
    if (!mediaRecorderRef.current || !wsClientRef.current?.isConnected()) {
      return;
    }

    const mediaRecorder = mediaRecorderRef.current;
    const client = wsClientRef.current;
    const requestId = currentRequestIdRef.current;

    // Stop recording
    mediaRecorder.stop();
    mediaRecorder.stream.getTracks().forEach(track => track.stop());
    setIsRecording(false);

    // Wait for final chunk
    await new Promise(resolve => {
      const checkEnd = () => {
        if (mediaRecorder.state === 'inactive') {
          resolve(true);
        } else {
          setTimeout(checkEnd, 50);
        }
      };
      checkEnd();
    });

    // Combine all chunks
    const audioBlob = new Blob(audioChunksRef.current, { type: 'audio/webm' });
    
    // Save raw audio for preview (before PCM conversion)
    // Clean up previous URL first
    if (recordedAudioUrlRef.current) {
      URL.revokeObjectURL(recordedAudioUrlRef.current);
    }
    const audioUrl = URL.createObjectURL(audioBlob);
    recordedAudioUrlRef.current = audioUrl;
    setRecordedAudioUrl(audioUrl);
    
    try {
      // Convert to PCM16 and send
      const pcm16Data = await convertToPcm16(audioBlob);
      client.sendAudioChunk(pcm16Data);
      
      // Send audio end
      if (requestId) {
        client.sendAudioEnd(requestId);
      }
    } catch (err) {
      setAudioError(err instanceof Error ? err.message : 'Failed to process audio');
    }

    mediaRecorderRef.current = null;
    audioChunksRef.current = [];
  }, [convertToPcm16]);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      // Cleanup TTS audio URL
      if (audioUrlRef.current) {
        URL.revokeObjectURL(audioUrlRef.current);
      }
      // Cleanup recorded input audio URL
      if (recordedAudioUrlRef.current) {
        URL.revokeObjectURL(recordedAudioUrlRef.current);
      }
      // Stop any ongoing recording
      if (mediaRecorderRef.current && mediaRecorderRef.current.state === 'recording') {
        mediaRecorderRef.current.stop();
        mediaRecorderRef.current.stream.getTracks().forEach(track => track.stop());
      }
      // Close audio context
      if (audioContextRef.current) {
        audioContextRef.current.close();
      }
      // Disconnect WebSocket
      if (wsClientRef.current) {
        wsClientRef.current.disconnect();
      }
    };
  }, []);

  return (
    <div className="p-6 max-w-4xl mx-auto">
      <h1 className="text-2xl font-bold text-gray-900 mb-2">Test Console</h1>
      <p className="text-gray-600 mb-6">
        Test LLM, TTS, and VTube Studio provider connections.
      </p>

      {/* LLM Test Section */}
      <section className="mb-8 p-6 bg-white rounded-lg shadow-sm border border-gray-200">
        <h2 className="text-lg font-medium text-gray-900 mb-4">LLM Provider Test</h2>
        
        {llmError && (
          <div className="mb-4 p-4 bg-red-50 border border-red-200 rounded-md">
            <p className="text-red-700 font-medium">Error</p>
            <p className="text-red-600 text-sm mt-1">{llmError}</p>
          </div>
        )}

        {llmResult && (
          <div className="mb-4 p-4 bg-green-50 border border-green-200 rounded-md">
            <p className="text-green-700 font-medium">Success</p>
            <dl className="mt-2 grid grid-cols-2 gap-2 text-sm">
              <div>
                <dt className="text-gray-500">Provider</dt>
                <dd className="text-gray-900">{llmResult.provider}</dd>
              </div>
              <div>
                <dt className="text-gray-500">Model</dt>
                <dd className="text-gray-900">{llmResult.model}</dd>
              </div>
              <div>
                <dt className="text-gray-500">Latency</dt>
                <dd className="text-gray-900">{llmResult.latency_ms}ms</dd>
              </div>
            </dl>
            <div className="mt-2">
              <dt className="text-gray-500 text-sm">Response</dt>
              <dd className="mt-1 p-2 bg-white rounded border text-sm text-gray-800">{llmResult.text}</dd>
            </div>
          </div>
        )}

        <form onSubmit={handleLlmTest} className="space-y-4">
          <div>
            <label htmlFor="llm-provider" className="block text-sm font-medium text-gray-700 mb-1">
              Provider
            </label>
            <select
              id="llm-provider"
              value={llmProvider}
              onChange={(e) => setLlmProvider(e.target.value as LlmProvider)}
              className="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm rounded-md"
            >
              {LLM_PROVIDERS.map(provider => (
                <option key={provider} value={provider}>
                  {provider.charAt(0).toUpperCase() + provider.slice(1)}
                </option>
              ))}
            </select>
          </div>

          <div>
            <label htmlFor="llm-message" className="block text-sm font-medium text-gray-700 mb-1">
              Test Message
            </label>
            <textarea
              id="llm-message"
              value={llmMessage}
              onChange={(e) => setLlmMessage(e.target.value)}
              rows={3}
              className="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
              required
            />
          </div>

          <div className="flex justify-end">
            <button
              type="submit"
              disabled={llmLoading}
              className="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
              {llmLoading ? 'Testing...' : 'Test LLM'}
            </button>
          </div>
        </form>
      </section>

      {/* TTS Test Section */}
      <section className="mb-8 p-6 bg-white rounded-lg shadow-sm border border-gray-200">
        <h2 className="text-lg font-medium text-gray-900 mb-4">TTS Provider Test</h2>
        
        {ttsError && (
          <div className="mb-4 p-4 bg-red-50 border border-red-200 rounded-md">
            <p className="text-red-700 font-medium">Error</p>
            <p className="text-red-600 text-sm mt-1">{ttsError}</p>
          </div>
        )}

        {ttsResult && (
          <div className="mb-4 p-4 bg-green-50 border border-green-200 rounded-md">
            <p className="text-green-700 font-medium">Success</p>
            <dl className="mt-2 grid grid-cols-3 gap-2 text-sm">
              <div>
                <dt className="text-gray-500">Provider</dt>
                <dd className="text-gray-900">{ttsResult.provider}</dd>
              </div>
              <div>
                <dt className="text-gray-500">Latency</dt>
                <dd className="text-gray-900">{ttsResult.latency_ms}ms</dd>
              </div>
              <div>
                <dt className="text-gray-500">Format</dt>
                <dd className="text-gray-900">{ttsResult.audio_format}</dd>
              </div>
            </dl>
            {ttsAudioUrl && (
              <div className="mt-4">
                <p className="text-gray-500 text-sm mb-2">Audio Preview</p>
                <audio controls src={ttsAudioUrl} className="w-full">
                  Your browser does not support audio playback.
                </audio>
              </div>
            )}
            {ttsAudioError && (
              <p className="mt-2 text-sm text-red-600">{ttsAudioError}</p>
            )}
          </div>
        )}

        <form onSubmit={handleTtsTest} className="space-y-4">
          <div>
            <label htmlFor="tts-provider" className="block text-sm font-medium text-gray-700 mb-1">
              Provider
            </label>
            <select
              id="tts-provider"
              value={ttsProvider}
              onChange={(e) => setTtsProvider(e.target.value as TtsProvider)}
              className="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm rounded-md"
            >
              {TTS_PROVIDERS.map(provider => (
                <option key={provider} value={provider}>
                  {provider.charAt(0).toUpperCase() + provider.slice(1)}
                </option>
              ))}
            </select>
          </div>

          <div>
            <label htmlFor="tts-text" className="block text-sm font-medium text-gray-700 mb-1">
              Test Text
            </label>
            <textarea
              id="tts-text"
              value={ttsText}
              onChange={(e) => setTtsText(e.target.value)}
              rows={3}
              className="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
              required
            />
          </div>

          <div className="flex justify-end">
            <button
              type="submit"
              disabled={ttsLoading}
              className="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
              {ttsLoading ? 'Testing...' : 'Test TTS'}
            </button>
          </div>
        </form>
      </section>

      {/* VTS Test Section */}
      <section className="p-6 bg-white rounded-lg shadow-sm border border-gray-200">
        <h2 className="text-lg font-medium text-gray-900 mb-4">VTube Studio Connection Test</h2>
        
        {vtsError && (
          <div className="mb-4 p-4 bg-red-50 border border-red-200 rounded-md">
            <p className="text-red-700 font-medium">Error</p>
            <p className="text-red-600 text-sm mt-1">{vtsError}</p>
          </div>
        )}

        {vtsResult && (
          <div className="mb-4 p-4 bg-green-50 border border-green-200 rounded-md">
            <p className="text-green-700 font-medium">Success</p>
            <dl className="mt-2 grid grid-cols-2 gap-2 text-sm">
              <div>
                <dt className="text-gray-500">Connected</dt>
                <dd className={`font-medium ${vtsResult.success ? 'text-green-600' : 'text-red-600'}`}>
                  {vtsResult.success ? 'Yes' : 'No'}
                </dd>
              </div>
              <div>
                <dt className="text-gray-500">Latency</dt>
                <dd className="text-gray-900">{vtsResult.latency_ms}ms</dd>
              </div>
            </dl>
          </div>
        )}

        <div className="flex justify-end">
          <button
            type="button"
            onClick={handleVtsTest}
            disabled={vtsLoading}
            className="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {vtsLoading ? 'Testing...' : 'Test VTube Studio'}
          </button>
        </div>
      </section>

      {/* Audio Input Section */}
      <section className="mt-8 p-6 bg-white rounded-lg shadow-sm border border-gray-200">
        <h2 className="text-lg font-medium text-gray-900 mb-4">Voice Input (STT)</h2>
        
        <div className="flex items-center gap-4 mb-4">
          <div className="flex items-center gap-2">
            <span className={`w-2 h-2 rounded-full ${wsConnected ? 'bg-green-500' : 'bg-gray-300'}`}></span>
            <span className="text-sm text-gray-600">
              {wsConnected ? 'WebSocket Connected' : 'WebSocket Disconnected'}
            </span>
          </div>
        </div>

        {audioError && (
          <div className="mb-4 p-4 bg-red-50 border border-red-200 rounded-md">
            <p className="text-red-700 font-medium">Error</p>
            <p className="text-red-600 text-sm mt-1">{audioError}</p>
          </div>
        )}

        {/* Raw recorded audio preview */}
        {recordedAudioUrl && (
          <div className="mb-4 p-4 bg-blue-50 border border-blue-200 rounded-md">
            <p className="text-blue-700 font-medium mb-2">Recorded Input (Debug)</p>
            <audio controls src={recordedAudioUrl} className="w-full mb-2" />
            <a
              href={recordedAudioUrl}
              download="recorded-input.webm"
              className="text-sm text-blue-600 hover:text-blue-800 underline"
            >
              Download raw recording
            </a>
          </div>
        )}

        {audioTranscript && (
          <div className="mb-4 p-4 bg-green-50 border border-green-200 rounded-md">
            <p className="text-green-700 font-medium">Transcript</p>
            <p className="text-gray-800 mt-1">{audioTranscript}</p>
          </div>
        )}

        <div className="flex items-center gap-4">
          {!isRecording ? (
            <button
              type="button"
              onClick={startRecording}
              className="inline-flex items-center justify-center py-3 px-6 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
            >
              <svg className="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z" />
              </svg>
              Record
            </button>
          ) : (
            <button
              type="button"
              onClick={stopRecording}
              className="inline-flex items-center justify-center py-3 px-6 border border-transparent text-base font-medium rounded-md text-white bg-red-600 hover:bg-red-700 animate-pulse"
            >
              <svg className="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20">
                <rect x="6" y="6" width="8" height="8" rx="1" />
              </svg>
              Stop & Send
            </button>
          )}
          <p className="text-sm text-gray-500">
            {isRecording ? 'Recording... click Stop to send' : 'Click Record to start, Stop to send'}
          </p>
        </div>
      </section>
    </div>
  );
};

export default TestConsole;
