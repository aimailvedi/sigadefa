class PoorModeWithContract {
  constructor() {
    this.contractRequirements = {
      version: LANGUAGE_CONTRACT.version,
      mandatoryFeatures: [
        'field_breath.edges',
        'piece_texture.open',
        'input_normalization',
        'dt_continuity',
        'autonomy_sleep',
        'resonance_detection.basic',
      ],
      configHash: this.calculateConfigHash(),
    };
  }

  verifyOrganismIntegrity() {
    const currentVersion = window.OrganismKernel?.contractVersion;
    if (currentVersion !== this.contractRequirements.version) {
      return false;
    }

    const missingFeatures = [];

    for (const feature of this.contractRequirements.mandatoryFeatures) {
      if (!this.isFeatureAvailable(feature)) {
        missingFeatures.push(feature);
      }
    }

    if (missingFeatures.length > 0) {
      return false;
    }

    if (!this.verifySemanticIntegrity()) {
      return false;
    }

    return true;
  }

  verifySemanticIntegrity() {
    const language = window.OrganismKernel?.language;

    if (!language) {
      return true;
    }

    const checks = [
      {
        expectedMeaning: 'спокойствие/сон',
        check: () => language.field_breath?.edges?.meaning === 'спокойствие/сон',
      },
      {
        expectedMeaning: 'открытость',
        check: () => language.piece_texture?.open?.meaning === 'открытость',
      },
    ];

    return checks.every((item) => item.check());
  }

  calculateConfigHash() {
    const config = {
      dtSafeRange: [0.008, 0.05],
      minStateDuration: 1.5,
      maxHueChange: 30,
    };

    return JSON.stringify(config);
  }

  isFeatureAvailable(feature) {
    if (window.OrganismKernel?.hasFeature) {
      return window.OrganismKernel.hasFeature(feature);
    }

    return this.checkFeatureFallback(feature);
  }

  checkFeatureFallback(feature) {
    switch (feature) {
      case 'field_breath.edges':
        return Boolean(window.Organism?.breathSystem?.edges);
      case 'piece_texture.open':
        return Boolean(window.Organism?.textureSystem?.open);
      default:
        return false;
    }
  }
}

if (typeof window !== 'undefined') {
  window.PoorModeWithContract = PoorModeWithContract;
}
