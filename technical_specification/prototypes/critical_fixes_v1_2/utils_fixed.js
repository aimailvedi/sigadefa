class StableShuffler {
  static fisherYates(array) {
    const shuffled = [...array];

    for (let i = shuffled.length - 1; i > 0; i -= 1) {
      const randomSource = window.crypto || window.msCrypto;
      let randomIndex;

      if (randomSource && randomSource.getRandomValues) {
        const randomArray = new Uint32Array(1);
        randomSource.getRandomValues(randomArray);
        randomIndex = randomArray[0] % (i + 1);
      } else {
        randomIndex = Math.floor(Math.random() * (i + 1));
      }

      [shuffled[i], shuffled[randomIndex]] = [shuffled[randomIndex], shuffled[i]];
    }

    return shuffled;
  }

  static selectRandomOptions(correctId, count = 4) {
    const otherIds = LANGUAGE_CONTRACT.meaning_ids.filter((id) => id !== correctId);
    const shuffledOthers = this.fisherYates(otherIds).slice(0, count - 1);
    const options = [correctId, ...shuffledOthers];

    return this.fisherYates(options);
  }
}

if (typeof window !== 'undefined') {
  window.StableShuffler = StableShuffler;
}
