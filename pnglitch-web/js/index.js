import("../../pnglitch-wasm/pkg").then(module => {
  let imageData;

  let pngSelectorLabel = document.getElementById('png-upload-label');
  let pngSelector = document.getElementById('png-upload');
  let resultImg = document.getElementById('result-img');
  let reglitch = document.getElementById('reglitch');
  let errorDiv = document.getElementById('error');

  const resetError = () => {
    errorDiv.innerText = '';
    errorDiv.classList.add('hidden');
  };

  const setError = (error) => {
    errorDiv.innerText = error;
    errorDiv.classList.remove('hidden');
  };

  let glitch = () => {
    resetError();
    try {
      let glitchedPNG = module.pnglitch(imageData);

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
