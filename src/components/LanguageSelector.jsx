import React from 'react';
import './LanguageSelector.css';

/**
 * LanguageSelector component for switching translation direction
 * Provides a visual and intuitive way to toggle between EN→JA and JA→EN
 */
export function LanguageSelector({ direction, onDirectionChange }) {
  const isEnToJa = direction === 'en_to_ja';
  
  const handleSwap = () => {
    onDirectionChange(isEnToJa ? 'ja_to_en' : 'en_to_ja');
  };

  return (
    <div className="language-selector">
      <div className={`language-item ${isEnToJa ? 'active' : ''}`}>
        <span className="language-code">EN</span>
        <span className="language-name">English</span>
      </div>
      
      <button 
        className="swap-button"
        onClick={handleSwap}
        aria-label="Swap languages"
        title="Swap translation direction"
      >
        <svg 
          width="24" 
          height="24" 
          viewBox="0 0 24 24" 
          fill="none" 
          xmlns="http://www.w3.org/2000/svg"
        >
          <path 
            d="M8 7L4 7L4 3M4 7L9 2M16 17L20 17L20 21M20 17L15 22" 
            stroke="currentColor" 
            strokeWidth="2" 
            strokeLinecap="round" 
            strokeLinejoin="round"
          />
        </svg>
      </button>
      
      <div className={`language-item ${!isEnToJa ? 'active' : ''}`}>
        <span className="language-code">JA</span>
        <span className="language-name">日本語</span>
      </div>
    </div>
  );
}

export default LanguageSelector;