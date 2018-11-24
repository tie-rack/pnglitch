import("../../pnglitch-wasm/pkg").then(module => {
  let imageData;

  let pngSelector = document.getElementById('png-upload');
  let resultImg = document.getElementById('result-img');
  let reglitch = document.getElementById('reglitch');

  let glitch = () => {
    let glitchedPNG = module.pnglitch(imageData);

    let blob = new Blob([glitchedPNG.buffer], {type: 'image/png'});
    let url = URL.createObjectURL(blob);

    resultImg.src = url;
  }

  reglitch.addEventListener('click', glitch);

  pngSelector.addEventListener('change', (e) => {
    reglitch.disabled = false;

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
