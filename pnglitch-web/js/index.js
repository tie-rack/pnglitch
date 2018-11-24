const createControl = (controls, id, options) => {
  let name = id.replace(/-/g, ' ');
  let prop = id.replace(/-/g, '_');
  let setter = `set_${prop}_chance`;
  let getter = `${prop}_chance`;

  let container = document.createElement('div');
  container.classList.add('glitch-control');

  let input = document.createElement('input');
  input.id = id;
  input.setAttribute('type', 'range');
  input.setAttribute('min', '0.0');
  input.setAttribute('max', '1.0');
  input.setAttribute('step', '0.1');

  let inputDiv = document.createElement('div');
  inputDiv.classList.add('flex-50', 'text-right');
  inputDiv.append(input);

  let label = document.createElement('label');
  label.setAttribute('for', id);
  label.innerText = name;
  label.classList.add('flex-50', 'text-left', 'label');

  input.value = options[getter]();
  input.addEventListener('change', (e) => {
    options[setter](+e.target.value);
  });

  container.append(inputDiv, label);
  controls.append(container);
}

import("../../pnglitch-wasm/pkg").then(module => {
  let imageData;

  let pngSelectorLabel = document.getElementById('png-upload-label');
  let pngSelector = document.getElementById('png-upload');
  let resultImg = document.getElementById('result-img');
  let reglitch = document.getElementById('reglitch');
  let errorDiv = document.getElementById('error');
  let controlsDiv = document.getElementById('glitch-controls');

  const resetError = () => {
    errorDiv.innerText = '';
    errorDiv.classList.add('hidden');
  };

  const setError = (error) => {
    errorDiv.innerText = error;
    errorDiv.classList.remove('hidden');
  };

  let options = module.Options.default();

  ['channel-swap',
   'darken',
   'flip',
   'lighten',
   'line-shift',
   'off-by-one',
   'quantize',
   'reverse',
   'shift-channel',
   'xor'].forEach(id => createControl(controlsDiv, id, options));

  let glitch = () => {
    resetError();
    try {
      let glitchedPNG = module.pnglitch(imageData, options);

      let blob = new Blob([glitchedPNG.buffer], {type: 'image/png'});
      let url = URL.createObjectURL(blob);

      resultImg.src = url;
    } catch(error) {
      console.log(error);
      setError('Uh oh. Something went wrong.');
      resultImg.src = '';
    }
  }

  reglitch.addEventListener('click', glitch);

  pngSelector.addEventListener('change', (e) => {
    reglitch.disabled = false;
    resetError();

    let file = e.target.files[0];

    if (file) {
      let reader = new FileReader();

      reader.addEventListener('load', e => {
        imageData = new Uint8Array(e.target.result);

        glitch();
      });

      reader.readAsArrayBuffer(file);
    }
  });

  pngSelectorLabel.classList.remove('disabled');
});
