import { ENXParser } from '../ts/index';

const parser = new ENXParser();

const fileInput = document.getElementById('fileInput') as HTMLInputElement;
const anchorLat = document.getElementById('anchorLat') as HTMLInputElement;
const anchorLon = document.getElementById('anchorLon') as HTMLInputElement;
const fmtSelect = document.getElementById('fmt') as HTMLSelectElement;
const parseBtn = document.getElementById('parseBtn') as HTMLButtonElement;
const output = document.getElementById('output') as HTMLPreElement;

parseBtn.addEventListener('click', async () => {
  if (!fileInput.files?.length) {
    output.textContent = 'No file selected.';
    return;
  }

  const file = fileInput.files[0];
  const buffer = await file.arrayBuffer();
  const fmt = fmtSelect.value as 'json' | 'binary';

  output.textContent = 'Parsing…';
  parseBtn.disabled = true;

  try {
    const result = await parser.parse(buffer, {
      outputFormat: fmt,
      anchor: { lat: parseFloat(anchorLat.value), lon: parseFloat(anchorLon.value) },
    });

    const meta = result.metadata;
    let text = `[metadata]\n`;
    text += `  totalPings:        ${meta.totalPings}\n`;
    text += `  validPings:        ${meta.validPings}\n`;
    text += `  droppedCorruptBins: ${meta.droppedCorruptBins}\n`;
    text += `  durationMs:        ${meta.durationMs.toFixed(1)}\n\n`;

    if (fmt === 'binary') {
      const arr = result.data as Float32Array;
      text += `[binary output]\n`;
      text += `  Float32Array length: ${arr.length} (${arr.length / 7} bins × 7 floats)\n\n`;
      text += `  First 3 bins (X Y Z U V W Intensity):\n`;
      for (let i = 0; i < Math.min(3, arr.length / 7); i++) {
        const b = i * 7;
        text += `  [${i}] X=${arr[b].toFixed(2)} Y=${arr[b+1].toFixed(2)} Z=${arr[b+2].toFixed(2)} ` +
                `U=${arr[b+3].toFixed(4)} V=${arr[b+4].toFixed(4)} W=${arr[b+5].toFixed(4)} ` +
                `I=${arr[b+6].toFixed(1)}\n`;
      }
    } else {
      const json = JSON.parse(result.data as string);
      text += `[json output — first ping preview]\n`;
      text += JSON.stringify(json[0], null, 2);
    }

    output.textContent = text;
  } catch (err: unknown) {
    output.innerHTML = `<span class="error">Error: ${err instanceof Error ? err.message : String(err)}</span>`;
  } finally {
    parseBtn.disabled = false;
  }
});
