class FeatureRegistry {
  constructor() {
    this.registry = new Map();
    this.contract = new Map();
  }

  register(feature, implementation, metadata = {}) {
    this.registry.set(feature, {
      implementation,
      metadata,
      active: true,
      registeredAt: Date.now(),
    });

    if (window.OrganismKernel) {
      window.OrganismKernel.setFeature(feature, true);
    }
  }

  unregister(feature) {
    this.registry.delete(feature);

    if (window.OrganismKernel) {
      window.OrganismKernel.setFeature(feature, false);
    }
  }

  isActive(feature) {
    const entry = this.registry.get(feature);
    if (!entry) {
      return false;
    }

    if (entry.metadata.healthCheck) {
      try {
        const healthy = entry.metadata.healthCheck();
        if (!healthy) {
          return false;
        }
      } catch (_error) {
        return false;
      }
    }

    return entry.active;
  }

  degradeFeature(feature) {
    const entry = this.registry.get(feature);
    if (entry) {
      entry.active = false;
    }
  }

  getCriticalFeatures() {
    const critical = [];

    for (const [feature, entry] of this.registry) {
      if (entry.metadata.critical) {
        critical.push(feature);
      }
    }

    return critical;
  }
}

if (typeof window !== 'undefined') {
  window.FeatureRegistry = FeatureRegistry;
}
