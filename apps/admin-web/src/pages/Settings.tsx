import React, { useState, useEffect } from "react";
import { getConfig, patchConfig } from "../api/admin";
import type { ServerPatchConfig, SttConfig } from "../api/admin";

const Settings: React.FC = () => {
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [saveSuccess, setSaveSuccess] = useState(false);
  const [vtsEndpoint, setVtsEndpoint] = useState("");
  const [sttServiceUrl, setSttServiceUrl] = useState("http://127.0.0.1:8002");

  useEffect(() => {
    let cancelled = false;
    getConfig()
      .then((data) => {
        if (!cancelled) {
          setVtsEndpoint(data.server.vts_endpoint);
          setSttServiceUrl(data.stt.service_url ?? "http://127.0.0.1:8002");
          setLoading(false);
        }
      })
      .catch((err) => {
        if (!cancelled) {
          setError(
            err instanceof Error ? err.message : "Failed to load configuration",
          );
          setLoading(false);
        }
      });
    return () => {
      cancelled = true;
    };
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setSaving(true);
    setError(null);
    setSaveSuccess(false);

    try {
      const patch: ServerPatchConfig = { vts_endpoint: vtsEndpoint };
      const sttPatch: SttConfig = { service_url: sttServiceUrl };
      const data = await patchConfig({ server: patch, stt: sttPatch });
      setVtsEndpoint(data.server.vts_endpoint);
      setSttServiceUrl(data.stt.service_url ?? "http://127.0.0.1:8002");
      setSaveSuccess(true);
      setTimeout(() => setSaveSuccess(false), 3000);
    } catch (err) {
      setError(
        err instanceof Error ? err.message : "Failed to save configuration",
      );
    } finally {
      setSaving(false);
    }
  };

  if (loading) {
    return (
      <div className="p-6 max-w-4xl mx-auto">
        <div className="flex items-center justify-center h-64">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"></div>
          <span className="ml-4 text-gray-600">Loading configuration...</span>
        </div>
      </div>
    );
  }

  return (
    <div className="p-6 max-w-4xl mx-auto">
      <h1 className="text-2xl font-bold text-gray-900 mb-2">Server Settings</h1>
      <p className="text-gray-600 mb-6">
        Configure server connection settings.
      </p>

      {error && (
        <div className="mb-4 p-4 bg-red-50 border border-red-200 rounded-md">
          <p className="text-red-700 font-medium">Error</p>
          <p className="text-red-600 text-sm mt-1">{error}</p>
        </div>
      )}

      {saveSuccess && (
        <div className="mb-4 p-4 bg-green-50 border border-green-200 rounded-md">
          <p className="text-green-700 font-medium">Success</p>
          <p className="text-green-600 text-sm mt-1">
            Settings saved successfully!
          </p>
        </div>
      )}

      <form
        onSubmit={handleSubmit}
        className="space-y-6 bg-white p-6 rounded-lg shadow-sm border border-gray-200"
      >
        <div>
          <h2 className="text-lg font-medium text-gray-900 mb-4">STT Settings</h2>
          <div className="space-y-4">
            <div>
              <label htmlFor="stt_service_url" className="block text-sm font-medium text-gray-700 mb-1">
                Audio Inference Service URL
              </label>
              <input
                type="text"
                id="stt_service_url"
                value={sttServiceUrl}
                onChange={(e) => setSttServiceUrl(e.target.value)}
                className="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
                placeholder="http://127.0.0.1:8002"
              />
              <p className="mt-1 text-sm text-gray-500">
                URL of the audio-inference service that runs Whisper STT.
              </p>
            </div>
          </div>
        </div>

        <div>
          <label
            htmlFor="vts_endpoint"
            className="block text-sm font-medium text-gray-700 mb-1"
          >
            VTS Endpoint
          </label>
          <input
            type="text"
            id="vts_endpoint"
            value={vtsEndpoint}
            onChange={(e) => setVtsEndpoint(e.target.value)}
            className="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
            placeholder="ws://127.0.0.1:8001"
          />
          <p className="mt-1 text-sm text-gray-500">
            WebSocket URL for VTube Studio. Must be a real IP/hostname, not a
            bind address like 0.0.0.0.
          </p>
          <p className="mt-1 text-sm text-amber-600">
            <strong>WSL2 note:</strong> If VTube Studio runs on Windows, use
            your Windows host IP (e.g., <code>ws://172.20.176.1:8001</code>)
            instead of localhost. Find your host IP with:{" "}
            <code>
              {`Get-NetIPAddress -AddressFamily IPv4 | Where-Object {$_.IPAddress -ne "127.0.0.1"} | Select-Object InterfaceAlias, IPAddress`}
            </code>
          </p>
        </div>

        <div className="flex justify-end">
          <button
            type="submit"
            disabled={saving}
            className="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {saving ? "Saving..." : "Save Settings"}
          </button>
        </div>
      </form>
    </div>
  );
};

export default Settings;
