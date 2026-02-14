class IntegratedProtocolV1_2 {
  constructor() {
    if (!window.OrganismKernel) {
      throw new Error('OrganismKernel не найден');
    }

    this.featureRegistry = new FeatureRegistry();
    OrganismKernel.integrateWithModule('FeatureRegistry', this.featureRegistry);
    this.registerMandatoryFeatures();
  }

  registerMandatoryFeatures() {
    const mandatoryFeatures = [
      {
        name: 'field_breath.edges',
        critical: true,
        healthCheck: () => Boolean(window.Organism?.breathSystem?.edges),
      },
      {
        name: 'piece_texture.open',
        critical: true,
        healthCheck: () => Boolean(window.Organism?.textureSystem?.open),
      },
      {
        name: 'input_normalization',
        critical: true,
        healthCheck: () => Boolean(window.InputNormalizer),
      },
      {
        name: 'dt_continuity',
        critical: true,
        healthCheck: () => Boolean(window.DoubleTimer),
      },
      {
        name: 'autonomy_sleep',
        critical: true,
        healthCheck: () => Boolean(window.AutonomyGuard),
      },
      {
        name: 'resonance_detection.basic',
        critical: false,
        healthCheck: () => Boolean(window.ResonanceDetector),
      },
    ];

    mandatoryFeatures.forEach((feature) => {
      this.featureRegistry.register(feature.name, null, {
        critical: feature.critical,
        healthCheck: feature.healthCheck,
      });
    });
  }

  async runCompleteProtocol() {
    if (!OrganismKernel.verifyContract()) {
      return this.fail('contract_verification_failed');
    }

    const fetus = new FinalFetusPhase();
    const viability = await fetus.run();
    if (!viability.viable) {
      return this.fail('fetus_phase_failed', { viability });
    }

    const languageTest = new FixedBlindTestValidatorV2();
    const languageResult = await languageTest.runBlindTest();
    if (!languageResult.passed) {
      return this.fail('language_test_failed', { languageResult });
    }

    const integrityCheck = new PoorModeWithContract();
    if (!integrityCheck.verifyOrganismIntegrity()) {
      return this.fail('integrity_check_failed');
    }

    const systemsOk = await this.checkAllSystems();
    if (!systemsOk) {
      return this.fail('systems_check_failed');
    }

    return {
      status: 'ready_for_birth',
      version: '1.2',
      timestamp: Date.now(),
      viability,
      language: languageResult,
      integrity: true,
      systems: this.getSystemStatus(),
    };
  }

  async checkAllSystems() {
    const systems = [
      { name: 'DoubleTimer', check: () => Boolean(window.DoubleTimer) },
      { name: 'AutonomyGuard', check: () => Boolean(window.AutonomyGuard) },
      { name: 'StorageTiers', check: () => Boolean(window.StorageTiers) },
      { name: 'HSLColorSystem', check: () => Boolean(window.HSLColorSystem) },
      { name: 'StateSignature', check: () => Boolean(window.StateSignature) },
      { name: 'AdaptiveNormalizer', check: () => Boolean(window.AdaptiveNormalizer) },
    ];

    const failed = [];

    for (const system of systems) {
      try {
        const ok = await system.check();
        if (!ok) {
          failed.push(system.name);
        }
      } catch (_error) {
        failed.push(system.name);
      }
    }

    return failed.length === 0;
  }

  getSystemStatus() {
    return {
      activeFeatures: this.featureRegistry.getCriticalFeatures(),
      kernelVersion: OrganismKernel.contractVersion,
    };
  }

  fail(reason, details = {}) {
    return {
      status: 'aborted',
      reason,
      details,
      timestamp: Date.now(),
      version: '1.2',
    };
  }
}

if (typeof window !== 'undefined') {
  window.IntegratedProtocolV1_2 = IntegratedProtocolV1_2;
}
