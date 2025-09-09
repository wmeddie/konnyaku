import React, { useState, useRef, useEffect } from 'react';
import './TranslationPanel.css';

/**
 * TranslationPanel component for the main translation interface
 * Handles text input, output display, and user interactions
 */
export function TranslationPanel({ 
  sourceText, 
  translatedText, 
  onSourceTextChange, 
  onTranslate,
  isLoading,
  error,
  direction,
  modelStatus
}) {
  const [charCount, setCharCount] = useState(0);
  const [copySuccess, setCopySuccess] = useState(false);
  const textareaRef = useRef(null);
  const maxChars = 5000; // Maximum character limit

  // Update character count when source text changes
  useEffect(() => {
    setCharCount(sourceText.length);
  }, [sourceText]);

  // Auto-resize textarea based on content
  const handleTextAreaChange = (e) => {
    const textarea = e.target;
    const text = textarea.value;
    
    // Enforce character limit
    if (text.length <= maxChars) {
      onSourceTextChange(text);
      
      // Auto-resize
      textarea.style.height = 'auto';
      textarea.style.height = Math.min(textarea.scrollHeight, 400) + 'px';
    }
  };

  // Clear source text
  const handleClear = () => {
    onSourceTextChange('');
    if (textareaRef.current) {
      textareaRef.current.style.height = 'auto';
    }
  };

  // Copy translated text to clipboard
  const handleCopy = async () => {
    if (translatedText) {
      try {
        await navigator.clipboard.writeText(translatedText);
        setCopySuccess(true);
        setTimeout(() => setCopySuccess(false), 2000);
      } catch (err) {
        console.error('Failed to copy text:', err);
      }
    }
  };

  // Handle translate button click or keyboard shortcut
  const handleTranslate = () => {
    if (sourceText.trim() && !isLoading) {
      onTranslate();
    }
  };

  // Handle keyboard shortcuts
  const handleKeyDown = (e) => {
    // Cmd/Ctrl + Enter to translate
    if ((e.metaKey || e.ctrlKey) && e.key === 'Enter') {
      e.preventDefault();
      handleTranslate();
    }
  };

  // Determine source and target language labels
  const sourceLabel = direction === 'en_to_ja' ? 'English' : '日本語';
  const targetLabel = direction === 'en_to_ja' ? '日本語' : 'English';

  return (
    <div className="translation-panel">
      <div className="panel-container">
        {/* Source Text Panel */}
        <div className="source-panel">
          <div className="panel-header">
            <h3 className="panel-title">{sourceLabel}</h3>
            <div className="panel-actions">
              <span className={`char-count ${charCount > maxChars * 0.9 ? 'warning' : ''}`}>
                {charCount} / {maxChars}
              </span>
              {sourceText && (
                <button 
                  className="clear-button"
                  onClick={handleClear}
                  aria-label="Clear text"
                  title="Clear text"
                >
                  Clear
                </button>
              )}
            </div>
          </div>
          <textarea
            ref={textareaRef}
            className="text-input"
            placeholder={`Enter ${sourceLabel} text to translate...`}
            value={sourceText}
            onChange={handleTextAreaChange}
            onKeyDown={handleKeyDown}
            spellCheck="false"
            autoFocus
          />
          {error && (
            <div className="error-message">
              <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                <path d="M8 1a7 7 0 100 14A7 7 0 008 1zm0 13a6 6 0 110-12 6 6 0 010 12zm0-9a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1zm0 5a1 1 0 100 2 1 1 0 000-2z"/>
              </svg>
              {error}
            </div>
          )}
        </div>

        {/* Translation Controls */}
        <div className="translation-controls">
          <button
            className={`translate-button ${isLoading ? 'loading' : ''}`}
            onClick={handleTranslate}
            disabled={!sourceText.trim() || isLoading}
            title={modelStatus.isInitializing ? 'Model is initializing...' : 'Translate (Cmd/Ctrl + Enter)'}
          >
            {isLoading ? (
              <>
                <span className="spinner"></span>
                Translating...
              </>
            ) : modelStatus.isInitializing ? (
              <>
                <span className="spinner"></span>
                Initializing...
              </>
            ) : (
              <>
                <svg width="20" height="20" viewBox="0 0 20 20" fill="none">
                  <path d="M4 5h12M4 5l4 4M4 5l4-4M16 15H4M16 15l-4-4M16 15l-4 4" 
                    stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
                </svg>
                Translate
              </>
            )}
          </button>
        </div>

        {/* Translated Text Panel */}
        <div className="target-panel">
          <div className="panel-header">
            <h3 className="panel-title">{targetLabel}</h3>
            <div className="panel-actions">
              {translatedText && (
                <button 
                  className={`copy-button ${copySuccess ? 'success' : ''}`}
                  onClick={handleCopy}
                  aria-label="Copy translation"
                  title="Copy translation"
                >
                  {copySuccess ? (
                    <>
                      <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                        <path d="M13.5 3L6 10.5 2.5 7" stroke="currentColor" strokeWidth="2" 
                          strokeLinecap="round" strokeLinejoin="round" fill="none"/>
                      </svg>
                      Copied!
                    </>
                  ) : (
                    <>
                      <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                        <path d="M10 2H4a2 2 0 00-2 2v8h2V4h6V2zm2 4H8a2 2 0 00-2 2v6a2 2 0 002 2h4a2 2 0 002-2V8a2 2 0 00-2-2z"/>
                      </svg>
                      Copy
                    </>
                  )}
                </button>
              )}
            </div>
          </div>
          <div className="text-output">
            {isLoading ? (
              <div className="loading-skeleton">
                <div className="skeleton-line"></div>
                <div className="skeleton-line"></div>
                <div className="skeleton-line short"></div>
              </div>
            ) : translatedText ? (
              <div className="translated-text">{translatedText}</div>
            ) : (
              <div className="placeholder-text">
                Translation will appear here...
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Model Status Indicator */}
      {modelStatus.isInitializing && (
        <div className="model-status">
          <span className="status-icon">
            <span className="spinner small"></span>
          </span>
          <span className="status-text">
            {modelStatus.isDownloaded ? 'Loading translation model...' : 'Downloading translation model...'}
          </span>
        </div>
      )}
    </div>
  );
}

export default TranslationPanel;