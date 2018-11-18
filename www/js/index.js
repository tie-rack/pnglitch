import("../../wasm-pnglitch/pkg").then(module => {
  let imageData;

  let pngSelector = document.getElementById('png-upload');
  let resultDiv = document.getElementById('result');
  let reglitch = document.getElementById('reglitch');

  const clearResults = () => {
    while (resultDiv.firstChild) {
      resultDiv.removeChild(resultDiv.firstChild);
    }
  }

  let glitch = () => {
    let glitchedPNG = module.pnglitch(imageData);

    let blob = new Blob([glitchedPNG.buffer], {type: 'image/png'});
    let url = URL.createObjectURL(blob);

    let img = document.createElement('img');
    img.classList.add('result-img');
    img.src = url;

    clearResults();

    resultDiv.append(img);
  }

  reglitch.addEventListener('click', glitch);

  pngSelector.addEventListener('change', (e) => {
    reglitch.disabled = false;

    clearResults();

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
});
