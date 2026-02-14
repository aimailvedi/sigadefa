const OrganismKernel = {
  contractVersion: LANGUAGE_CONTRACT.version,
  features: new Set(),
  language: { ...LANGUAGE_CONTRACT.semantic_blocks },
  state: {
    energy: 0.5,
    chi: 0,
    metabolism: 0.5,
    coherence: 0.5,
  },

  setFeature(name, active) {
    if (active) {
      this.features.add(name);
    } else {
      this.features.delete(name);
    }
  },

  hasFeature(name) {
    return this.features.has(name);
  },

  getActiveFeatures() {
    return Array.from(this.features);
  },

  updateState(newState) {
    Object.assign(this.state, newState);
  },

  verifyContract() {
    if (this.contractVersion !== LANGUAGE_CONTRACT.version) {
      return false;
    }

    const required = [
      'field_breath.edges',
      'piece_texture.open',
      'input_normalization',
      'dt_continuity',
    ];

    return required.every((feature) => this.hasFeature(feature));
  },

  integrateWithModule(moduleName, module) {
    switch (moduleName) {
      case 'PoorModeManager':
        module.organismKernel = this;
        break;
      case 'DoubleTimer':
        module.organismKernel = this;
        break;
      case 'FeatureRegistry':
        window.FeatureRegistry = module;
        break;
      default:
        break;
    }
  },
};

if (typeof window !== 'undefined') {
  window.OrganismKernel = OrganismKernel;
  OrganismKernel.setFeature('organism_kernel', true);
  OrganismKernel.setFeature('language_contract', true);
}
