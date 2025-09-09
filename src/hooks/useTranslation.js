import { useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';

/**
 * Custom hook for managing translation state and logic
 * Handles Tauri command invocation, loading states, and error management
 */
export function useTranslation() {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);
  const [modelStatus, setModelStatus] = useState({
    isLoaded: false,
    isDownloaded: false,
    isInitializing: false
  });

  /**
   * Check if the translation model is loaded
   */
  const checkModelStatus = useCallback(async () => {
    try {
      const status = await invoke('get_model_status');
      // Backend returns { loaded: bool }, convert to isLoaded for frontend
      const isLoaded = status?.loaded || false;
      setModelStatus(prev => ({ ...prev, isLoaded }));
      return isLoaded;
    } catch (err) {
      console.error('Failed to check model status:', err);
      return false;
    }
  }, []);

  /**
   * Ensure the model is downloaded (downloads if not present)
   */
  const ensureModelDownloaded = useCallback(async () => {
    try {
      setModelStatus(prev => ({ ...prev, isInitializing: true }));
      setError(null);
      await invoke('ensure_model_downloaded');
      setModelStatus(prev => ({ ...prev, isDownloaded: true, isInitializing: false }));
      return true;
    } catch (err) {
      setError(`Failed to download model: ${err}`);
      setModelStatus(prev => ({ ...prev, isInitializing: false }));
      return false;
    }
  }, []);

  /**
   * Initialize the translation model
   */
  const initializeModel = useCallback(async () => {
    try {
      setModelStatus(prev => ({ ...prev, isInitializing: true }));
      setError(null);
      await invoke('initialize_model');
      setModelStatus(prev => ({ 
        ...prev, 
        isLoaded: true, 
        isInitializing: false 
      }));
      return true;
    } catch (err) {
      setError(`Failed to initialize model: ${err}`);
      setModelStatus(prev => ({ ...prev, isInitializing: false }));
      return false;
    }
  }, []);

  /**
   * Perform translation
   * @param {string} text - Text to translate
   * @param {string} direction - Translation direction ("en_to_ja" or "ja_to_en")
   * @returns {Promise<string>} Translated text
   */
  const translate = useCallback(async (text, direction) => {
    if (!text?.trim()) {
      return '';
    }

    setIsLoading(true);
    setError(null);

    try {
      // Ensure model is ready
      if (!modelStatus.isLoaded) {
        const downloaded = modelStatus.isDownloaded || await ensureModelDownloaded();
        if (downloaded) {
          await initializeModel();
        } else {
          throw new Error('Model could not be downloaded');
        }
      }

      // Perform translation
      // Convert direction format from en_to_ja to en-ja for backend compatibility
      const backendDirection = direction.replace('_to_', '-');
      const result = await invoke('translate', {
        request: {
          text: text.trim(),
          direction: backendDirection
        }
      });
      
      // Handle the response - check if successful
      if (result?.success && result?.translation) {
        return result.translation;
      } else if (result?.error) {
        throw new Error(result.error);
      } else {
        return '';
      }
    } catch (err) {
      const errorMessage = err?.message || err?.toString() || 'Translation failed';
      setError(errorMessage);
      console.error('Translation error:', err);
      return '';
    } finally {
      setIsLoading(false);
    }
  }, [modelStatus.isLoaded, modelStatus.isDownloaded, ensureModelDownloaded, initializeModel]);

  /**
   * Get supported language pairs
   */
  const getSupportedLanguages = useCallback(async () => {
    try {
      const languages = await invoke('get_supported_languages');
      return languages;
    } catch (err) {
      console.error('Failed to get supported languages:', err);
      return [];
    }
  }, []);

  return {
    translate,
    isLoading,
    error,
    modelStatus,
    checkModelStatus,
    ensureModelDownloaded,
    initializeModel,
    getSupportedLanguages,
    clearError: () => setError(null)
  };
}