import React, { useState, useEffect } from 'react';
import { getConfig, patchConfig } from '../api/admin';
import type { LlmConfig, ConfigData } from '../api/admin';

const LLM_PROVIDERS = ['gemini', 'openrouter', 'ollama'] as const;

const Providers: React.FC = () => {
  // Core state
  const [config, setConfig] = useState<ConfigData | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [saveSuccess, setSaveSuccess] = useState(false);

  // Form state (local editable copy of LLM config)
  const [formData, setFormData] = useState<LlmConfig>({
    provider: 'gemini',
    model: '',
    temperature: 0.7,
    max_tokens: 1024,
  });

  useEffect(() => {
    let cancelled = false;

    const loadConfig = async () => {
      try {
        setLoading(true);
        setError(null);
        const data = await getConfig();
        if (cancelled) {
          return;
        }

        setConfig(data);
        setFormData(data.llm);
      } catch (err) {
        if (!cancelled) {
          setError(err instanceof Error ? err.message : 'Failed to load configuration');
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    };

    void loadConfig();

    return () => {
      cancelled = true;
    };
  }, []);

  // Handle form input changes
  const handleInputChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>
  ) => {
    const { name, value } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: name === 'temperature' || name === 'max_tokens' 
        ? parseFloat(value) 
        : value,
    }));
  };

  // Handle temperature slider/number input specifically
  const handleTemperatureChange = (
    e: React.ChangeEvent<HTMLInputElement>
  ) => {
    const value = parseFloat(e.target.value);
    if (!isNaN(value) && value >= 0 && value <= 2) {
      setFormData(prev => ({ ...prev, temperature: value }));
    }
  };

  // Handle form submission
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setSaving(true);
    setError(null);
    setSaveSuccess(false);

    try {
      // Save changes via PATCH
      await patchConfig({ llm: formData });
      
      // Refresh config state as required
      const updatedConfig = await getConfig();
      setConfig(updatedConfig);
      
      setSaveSuccess(true);
      setTimeout(() => setSaveSuccess(false), 3000);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to save configuration');
    } finally {
      setSaving(false);
    }
  };

  // Loading state
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
      <h1 className="text-2xl font-bold text-gray-900 mb-2">LLM Provider Settings</h1>
      <p className="text-gray-600 mb-6">
        Configure the active Large Language Model provider and inference settings.
      </p>

      {/* Error state */}
      {error && (
        <div className="mb-4 p-4 bg-red-50 border border-red-200 rounded-md">
          <p className="text-red-700 font-medium">Error</p>
          <p className="text-red-600 text-sm mt-1">{error}</p>
        </div>
      )}

      {/* Success state */}
      {saveSuccess && (
        <div className="mb-4 p-4 bg-green-50 border border-green-200 rounded-md">
          <p className="text-green-700 font-medium">Success</p>
          <p className="text-green-600 text-sm mt-1">Settings saved successfully!</p>
        </div>
      )}

      {/* Main form */}
      <form 
        onSubmit={handleSubmit} 
        className="space-y-6 bg-white p-6 rounded-lg shadow-sm border border-gray-200"
      >
        {/* Provider Selector */}
        <div>
          <label 
            htmlFor="provider" 
            className="block text-sm font-medium text-gray-700 mb-1"
          >
            LLM Provider
          </label>
          <select
            id="provider"
            name="provider"
            value={formData.provider}
            onChange={handleInputChange}
            className="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm rounded-md"
            required
          >
            {LLM_PROVIDERS.map(provider => (
              <option key={provider} value={provider}>
                {provider.charAt(0).toUpperCase() + provider.slice(1)}
              </option>
            ))}
          </select>
          <p className="mt-1 text-sm text-gray-500">
            Select the LLM provider to use for AI responses.
          </p>
        </div>

        {/* Model Input */}
        <div>
          <label 
            htmlFor="model" 
            className="block text-sm font-medium text-gray-700 mb-1"
          >
            Model Identifier
          </label>
          <input
            type="text"
            id="model"
            name="model"
            value={formData.model}
            onChange={handleInputChange}
            className="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
            placeholder="e.g., gemini-2.0-flash-exp"
            required
          />
          <p className="mt-1 text-sm text-gray-500">
            Model name/identifier for the selected provider.
          </p>
        </div>

        {/* Temperature Control */}
        <div>
          <label 
            htmlFor="temperature" 
            className="block text-sm font-medium text-gray-700 mb-1"
          >
            Temperature: {formData.temperature.toFixed(1)}
          </label>
          <div className="flex items-center space-x-4">
            <input
              type="range"
              id="temperature"
              name="temperature"
              min="0"
              max="2"
              step="0.1"
              value={formData.temperature}
              onChange={handleTemperatureChange}
              className="flex-1 h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-blue-500"
            />
            <input
              type="number"
              min="0"
              max="2"
              step="0.1"
              value={formData.temperature}
              onChange={handleTemperatureChange}
              className="w-20 border border-gray-300 rounded-md shadow-sm py-1 px-2 text-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
          <p className="mt-1 text-sm text-gray-500">
            Controls response randomness: 0.0 = deterministic, 2.0 = highly random.
          </p>
        </div>

        {/* Max Tokens Input */}
        <div>
          <label 
            htmlFor="max_tokens" 
            className="block text-sm font-medium text-gray-700 mb-1"
          >
            Max Tokens
          </label>
          <input
            type="number"
            id="max_tokens"
            name="max_tokens"
            min="1"
            value={formData.max_tokens}
            onChange={handleInputChange}
            className="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-blue-500 focus:border-blue-500 sm:text-sm"
            required
          />
          <p className="mt-1 text-sm text-gray-500">
            Maximum number of tokens to generate per response.
          </p>
        </div>

        {/* Submit Button */}
        <div className="flex justify-end">
          <button
            type="submit"
            disabled={saving}
            className="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {saving ? 'Saving...' : 'Save Settings'}
          </button>
        </div>
      </form>

      {/* Current Config Readout */}
      {config && (
        <div className="mt-8 p-6 bg-gray-50 rounded-lg border border-gray-200">
          <h2 className="text-lg font-medium text-gray-900 mb-4">Current Active Configuration</h2>
          <dl className="grid grid-cols-1 gap-x-4 gap-y-6 sm:grid-cols-2">
            <div>
              <dt className="text-sm font-medium text-gray-500">Provider</dt>
              <dd className="mt-1 text-sm text-gray-900">{config.llm.provider}</dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-gray-500">Model</dt>
              <dd className="mt-1 text-sm text-gray-900">{config.llm.model}</dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-gray-500">Temperature</dt>
              <dd className="mt-1 text-sm text-gray-900">{config.llm.temperature}</dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-gray-500">Max Tokens</dt>
              <dd className="mt-1 text-sm text-gray-900">{config.llm.max_tokens}</dd>
            </div>
          </dl>
        </div>
      )}
    </div>
  );
};

export default Providers;
