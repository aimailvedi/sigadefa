class FixedBlindTestValidatorV2 {
  generateStateForMeaningId(meaningId) {
    const [block, state] = meaningId.split('.');

    const stateTemplate = {
      field_breath: {
        active: block === 'field_breath' ? state : 'edges',
        edges: { active: false },
        center: { active: false },
        whole: { active: false },
        skip: { active: false },
      },
      piece_texture: {
        active: block === 'piece_texture' ? state : 'open',
        open: { active: false },
        closed: { active: false },
        curious: { active: false },
        creative: { active: false },
      },
    };

    if (block === 'field_breath' && stateTemplate.field_breath[state]) {
      stateTemplate.field_breath[state].active = true;
    } else {
      stateTemplate.field_breath.edges.active = true;
    }

    if (block === 'piece_texture' && stateTemplate.piece_texture[state]) {
      stateTemplate.piece_texture[state].active = true;
    } else {
      stateTemplate.piece_texture.open.active = true;
    }

    this.validateStateAgainstContract(stateTemplate);

    return stateTemplate;
  }

  validateStateAgainstContract(state) {
    const contract = LANGUAGE_CONTRACT.semantic_blocks;

    for (const blockKey of Object.keys(state)) {
      if (!contract[blockKey]) {
        console.error(`Неизвестный блок в состоянии: ${blockKey}`);
        delete state[blockKey];
        continue;
      }

      for (const stateKey of Object.keys(state[blockKey])) {
        if (stateKey === 'active') {
          continue;
        }

        if (!contract[blockKey][stateKey]) {
          console.error(`Неизвестное состояние в блоке ${blockKey}: ${stateKey}`);
          delete state[blockKey][stateKey];
        }
      }
    }
  }
}

if (typeof window !== 'undefined') {
  window.FixedBlindTestValidatorV2 = FixedBlindTestValidatorV2;
}
