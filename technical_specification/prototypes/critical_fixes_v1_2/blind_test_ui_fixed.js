class BlindTestUIFixed {
  constructor() {
    this.currentTest = null;
    this.testResults = [];
  }

  async promptGuess(options) {
    return new Promise((resolve) => {
      const modal = this.createModal(options);

      options.forEach((option, index) => {
        const button = modal.querySelector(`[data-option="${index}"]`);
        button.addEventListener('click', () => {
          this.cleanupModal(modal);
          resolve(option);
        });
      });

      document.body.appendChild(modal);
    });
  }

  createModal(options) {
    const modal = document.createElement('div');
    modal.className = 'blind-test-modal';

    const buttonsHTML = options.map((option, index) => {
      const [block, state] = option.split('.');
      const meaning = LANGUAGE_CONTRACT.semantic_blocks[block]?.[state]?.meaning || option;

      return `
        <button class="blind-test-option" data-option="${index}">
          <div class="option-preview" data-meaning-id="${option}"></div>
          <div class="option-text">${meaning}</div>
        </button>
      `;
    }).join('');

    modal.innerHTML = `
      <div class="blind-test-content">
        <h3>Что выражает организм сейчас?</h3>
        <div class="options-grid">${buttonsHTML}</div>
        <p class="hint">Выберите один вариант</p>
      </div>
    `;

    Object.assign(modal.style, {
      position: 'fixed',
      top: '0',
      left: '0',
      width: '100%',
      height: '100%',
      backgroundColor: 'rgba(0, 0, 0, 0.8)',
      zIndex: '10000',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
    });

    return modal;
  }

  cleanupModal(modal) {
    modal.style.opacity = '0';
    modal.style.transition = 'opacity 0.3s';

    setTimeout(() => {
      if (modal.parentNode) {
        modal.parentNode.removeChild(modal);
      }
    }, 300);
  }

  validateTestResult(guess, correctMeaningId) {
    const isCorrect = guess === correctMeaningId;

    this.testResults.push({
      guess,
      correct: correctMeaningId,
      isCorrect,
      timestamp: Date.now(),
    });

    return isCorrect;
  }
}

if (typeof window !== 'undefined') {
  window.BlindTestUIFixed = BlindTestUIFixed;
}
