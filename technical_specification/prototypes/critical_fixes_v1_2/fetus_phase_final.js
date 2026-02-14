class FinalFetusPhase {
  constructor() {
    this.minDuration = 30000;
    this.maxDuration = 90000;
    this.startTime = 0;
    this.lastFrameTime = 0;
    this.frameCount = 0;
    this.rafId = null;

    this.metrics = {
      fps: { values: [], stable: false, lastUpdate: 0 },
      dt: { values: [], spikes: 0, stable: false },
      input: { events: [], normalized: false },
      render: { consistent: true, framesRendered: 0 },
    };

    this.stabilityCheckpoints = [];
    this.requiredStableSeconds = 10;

    this.canvas = null;
    this.ctx = null;

    this.viabilityResult = null;
    this.isRunning = false;
    this.handleCanvasEvent = null;
  }

  async run() {
    if (this.isRunning) {
      console.warn('Фаза Плод уже запущена');
      return this.viabilityResult;
    }

    this.isRunning = true;
    this.startTime = performance.now();
    this.lastFrameTime = this.startTime;

    this.setupCanvas();
    this.setupInputMonitoring();

    return new Promise((resolve) => {
      const mainLoop = (currentTime) => {
        if (!this.isRunning) {
          return;
        }

        const dt = currentTime - this.lastFrameTime;
        this.lastFrameTime = currentTime;

        this.updateMetrics(dt, currentTime);
        this.renderOrganism(currentTime);

        const stability = this.checkStability(currentTime);
        const elapsed = currentTime - this.startTime;

        if (this.shouldFinish(elapsed, stability)) {
          this.stop();
          this.viabilityResult = this.assessViability();
          resolve(this.viabilityResult);
          return;
        }

        this.rafId = requestAnimationFrame(mainLoop);
      };

      this.rafId = requestAnimationFrame(mainLoop);
    });
  }

  setupCanvas() {
    this.canvas = document.createElement('canvas');
    this.canvas.width = 800;
    this.canvas.height = 600;

    Object.assign(this.canvas.style, {
      position: 'fixed',
      top: '50%',
      left: '50%',
      transform: 'translate(-50%, -50%)',
      border: '1px solid rgba(255, 255, 255, 0.1)',
      pointerEvents: 'auto',
      cursor: 'default',
      zIndex: '9999',
    });

    document.body.appendChild(this.canvas);
    this.ctx = this.canvas.getContext('2d');
  }

  setupInputMonitoring() {
    this.handleCanvasEvent = (event) => {
      event.preventDefault();

      this.metrics.input.events.push({
        type: event.type,
        timestamp: performance.now(),
        x: event.clientX - this.canvas.offsetLeft,
        y: event.clientY - this.canvas.offsetTop,
        target: 'canvas',
      });

      if (this.metrics.input.events.length > 100) {
        this.metrics.input.events = this.metrics.input.events.slice(-100);
      }
    };

    const events = ['mousedown', 'mousemove', 'touchstart', 'touchmove'];
    events.forEach((eventType) => this.canvas.addEventListener(eventType, this.handleCanvasEvent));
  }

  updateMetrics(dt, currentTime) {
    const dtSec = dt / 1000;
    this.metrics.dt.values.push(dtSec);
    if (this.metrics.dt.values.length > 300) {
      this.metrics.dt.values = this.metrics.dt.values.slice(-300);
    }

    const fps = dt > 0 ? 1000 / dt : 0;
    this.frameCount += 1;

    if (currentTime - this.metrics.fps.lastUpdate > 1000) {
      this.metrics.fps.values.push(fps);
      if (this.metrics.fps.values.length > 10) {
        this.metrics.fps.values = this.metrics.fps.values.slice(-10);
      }
      this.metrics.fps.lastUpdate = currentTime;
    }

    if (dt > 50) {
      this.metrics.dt.spikes += 1;
    }
  }

  renderOrganism(currentTime) {
    if (!this.ctx) {
      return;
    }

    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

    const elapsedSeconds = (currentTime - this.startTime) / 1000;
    const breathPhase = elapsedSeconds * 0.2;
    const breathAmplitude = Math.sin(breathPhase * Math.PI * 2) * 0.3;

    const edgeWidth = 20;
    const pulseWidth = edgeWidth * (1 + breathAmplitude);

    this.ctx.fillStyle = `rgba(100, 150, 255, ${0.3 + breathAmplitude * 0.2})`;
    this.ctx.fillRect(0, 0, this.canvas.width, pulseWidth);
    this.ctx.fillRect(this.canvas.width - pulseWidth, 0, pulseWidth, this.canvas.height);
    this.ctx.fillRect(0, this.canvas.height - pulseWidth, this.canvas.width, pulseWidth);
    this.ctx.fillRect(0, 0, pulseWidth, this.canvas.height);

    const blockSize = 30;
    const centerX = this.canvas.width / 2 - blockSize * 1.5;
    const centerY = this.canvas.height / 2 - blockSize * 1.5;

    this.ctx.fillStyle = 'rgba(255, 255, 255, 0.6)';
    for (let i = 0; i < 2; i += 1) {
      for (let j = 0; j < 2; j += 1) {
        this.ctx.fillRect(centerX + i * blockSize, centerY + j * blockSize, blockSize - 2, blockSize - 2);
      }
    }

    this.ctx.fillStyle = 'rgba(255, 255, 255, 0.7)';
    this.ctx.font = '12px monospace';
    this.ctx.fillText('Фаза Плод', 10, 20);
    this.ctx.fillText(`Время: ${((currentTime - this.startTime) / 1000).toFixed(1)}с`, 10, 40);

    const fps = this.metrics.fps.values.length > 0
      ? this.metrics.fps.values[this.metrics.fps.values.length - 1].toFixed(0)
      : '0';
    this.ctx.fillText(`FPS: ${fps}`, 10, 60);

    this.metrics.render.framesRendered += 1;
  }

  checkStability(currentTime) {
    if (this.metrics.fps.values.length < 5) {
      return { stable: false, stableFor: 0 };
    }

    const recentFps = this.metrics.fps.values.slice(-5);
    const avgFps = recentFps.reduce((a, b) => a + b, 0) / recentFps.length;
    const minFps = Math.min(...recentFps);

    const fpsStable = avgFps >= 30 && minFps >= 20;
    const dtStable = this.metrics.dt.spikes < 5;
    const inputActive = this.metrics.input.events.length > 0;

    if (fpsStable && dtStable && inputActive) {
      this.stabilityCheckpoints.push(currentTime);
      const thirtySecondsAgo = currentTime - 30000;
      this.stabilityCheckpoints = this.stabilityCheckpoints.filter((t) => t > thirtySecondsAgo);

      if (this.stabilityCheckpoints.length > 1) {
        const stabilityDuration = (currentTime - this.stabilityCheckpoints[0]) / 1000;
        return { stable: true, stableFor: stabilityDuration };
      }
    } else {
      this.stabilityCheckpoints = [];
    }

    return { stable: false, stableFor: 0 };
  }

  shouldFinish(elapsed, stability) {
    if (elapsed >= this.maxDuration) {
      return true;
    }

    if (elapsed >= this.minDuration && stability.stableFor >= this.requiredStableSeconds) {
      return true;
    }

    return false;
  }

  stop() {
    if (this.rafId) {
      cancelAnimationFrame(this.rafId);
      this.rafId = null;
    }

    if (this.canvas && this.handleCanvasEvent) {
      const events = ['mousedown', 'mousemove', 'touchstart', 'touchmove'];
      events.forEach((eventType) => this.canvas.removeEventListener(eventType, this.handleCanvasEvent));
    }

    if (this.canvas && this.canvas.parentNode) {
      this.canvas.parentNode.removeChild(this.canvas);
    }

    this.isRunning = false;
  }

  assessViability() {
    return {
      viable: this.calculateViabilityScore() >= 0.7,
      metrics: { ...this.metrics },
      duration: (performance.now() - this.startTime) / 1000,
      stabilityCheckpoints: this.stabilityCheckpoints.length,
    };
  }

  calculateViabilityScore() {
    let score = 0;

    if (this.metrics.fps.values.length >= 3) {
      const avgFps = this.metrics.fps.values.reduce((a, b) => a + b, 0) / this.metrics.fps.values.length;
      score += Math.min(avgFps / 60, 1) * 0.3;
    }

    const dtScore = 1 - Math.min(this.metrics.dt.spikes / 10, 1);
    score += dtScore * 0.3;

    score += this.metrics.input.events.length > 0 ? 0.2 : 0;
    score += this.metrics.render.framesRendered > 10 ? 0.2 : 0;

    return score;
  }
}

if (typeof window !== 'undefined') {
  window.FinalFetusPhase = FinalFetusPhase;
}
