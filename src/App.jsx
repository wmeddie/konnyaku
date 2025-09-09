import React, { useState, useEffect } from 'react';
import { LanguageSelector } from './components/LanguageSelector';
import { TranslationPanel } from './components/TranslationPanel';
import { useTranslation } from './hooks/useTranslation';
import './App.css';

/**
 * Main App component for Konnyaku translation application
 * Manages the overall state and coordinates between components
 */
function App() {
  const [sourceText, setSourceText] = useState('');
  const [translatedText, setTranslatedText] = useState('');
  const [direction, setDirection] = useState('en_to_ja');
  
  const {
    translate,
    isLoading,
    error,
    modelStatus,
    checkModelStatus,
    clearError
  } = useTranslation();

  // Check model status on mount
  useEffect(() => {
    checkModelStatus();
  }, [checkModelStatus]);

  // Handle translation
  const handleTranslate = async () => {
    clearError();
    const result = await translate(sourceText, direction);
    setTranslatedText(result);
  };

  // Handle direction change
  const handleDirectionChange = (newDirection) => {
    setDirection(newDirection);
    // Clear texts when switching direction
    setSourceText('');
    setTranslatedText('');
    clearError();
  };

  // Handle source text change
  const handleSourceTextChange = (text) => {
    setSourceText(text);
    // Clear translated text when source changes
    if (!text) {
      setTranslatedText('');
    }
  };

  return (
    <div className="app">
      <header className="app-header">
        <div className="header-content">
          <div className="logo-section">
            <h1 className="app-title">
              <span className="logo-icon">🌐</span>
              Konnyaku
            </h1>
            <p className="app-subtitle">
              Local AI-Powered Japanese-English Translation
            </p>
          </div>
          <div className="header-info">
            <span className="status-badge">
              {modelStatus.isLoaded ? (
                <>
                  <span className="status-dot active"></span>
                  Model Ready
                </>
              ) : modelStatus.isInitializing ? (
                <>
                  <span className="status-dot loading"></span>
                  Initializing...
                </>
              ) : (
                <>
                  <span className="status-dot"></span>
                  Model Not Loaded
                </>
              )}
            </span>
          </div>
        </div>
      </header>

      <main className="app-main">
        <div className="translation-container">
          <LanguageSelector 
            direction={direction}
            onDirectionChange={handleDirectionChange}
          />
          
          <TranslationPanel
            sourceText={sourceText}
            translatedText={translatedText}
            onSourceTextChange={handleSourceTextChange}
            onTranslate={handleTranslate}
            isLoading={isLoading}
            error={error}
            direction={direction}
            modelStatus={modelStatus}
          />
        </div>

        <footer className="app-footer">
          <p className="footer-text">
            Powered by LiquidAI LFM2-350M-ENJP-MT • Running 100% locally on your device
          </p>
          <p className="footer-hints">
            Tip: Press <kbd>Cmd</kbd>+<kbd>Enter</kbd> (Mac) or <kbd>Ctrl</kbd>+<kbd>Enter</kbd> (Windows/Linux) to translate
          </p>
        </footer>
      </main>
    </div>
  );
}

export default App;
