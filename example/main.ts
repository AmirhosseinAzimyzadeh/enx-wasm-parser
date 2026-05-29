import { ENXParser } from '../ts/index';

const parser = new ENXParser();

const fileInput  = document.getElementById('fileInput')  as HTMLInputElement;
const anchorLat  = document.getElementById('anchorLat')  as HTMLInputElement;
const anchorLon  = document.getElementById('anchorLon')  as HTMLInputElement;
const fmtSelect  = document.getElementById('fmt')        as HTMLSelectElement;
const parseBtn   = document.getElementById('parseBtn')   as HTMLButtonElement;
const output     = document.getElementById('output')     as HTMLPreElement;
const pingNav    = document.getElementById('pingNav')    as HTMLDivElement;
const prevBtn    = document.getElementById('prevBtn')    as HTMLButtonElement;
const nextBtn    = document.getElementById('nextBtn')    as HTMLButtonElement;
const pingCounter = document.getElementById('pingCounter') as HTMLSpanElement;

let jsonPings: unknown[] = [];
let currentPing = 0;

function showPing(index: number) {
  currentPing = index;
  pingCounter.textContent = `ping ${index + 1} / ${jsonPings.length}`;
  prevBtn.disabled = index === 0;
  nextBtn.disabled = index === jsonPings.length - 1;
  output.textContent = JSON.stringify(jsonPings[index], null, 2);
}

prevBtn.addEventListener('click', () => { if (currentPing > 0) showPing(currentPing - 1); });
nextBtn.addEventListener('click', () => { if (currentPing < jsonPings.length - 1) showPing(currentPing + 1); });

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
  pingNav.style.display = 'none';
  jsonPings = [];

  try {
    const result = await parser.parse(buffer, {
      outputFormat: fmt,
      anchor: { lat: parseFloat(anchorLat.value), lon: parseFloat(anchorLon.value) },
    });

    const meta = result.metadata;
    let text = `[metadata]\n`;
    text += `  totalPings:         ${meta.totalPings}\n`;
    text += `  validPings:         ${meta.validPings}\n`;
    text += `  droppedCorruptBins: ${meta.droppedCorruptBins}\n`;
    text += `  durationMs:         ${meta.durationMs.toFixed(1)}\n\n`;

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
      output.textContent = text;
    } else {
      jsonPings = JSON.parse(result.data as string) as unknown[];
      output.textContent = text;
      if (jsonPings.length > 0) {
        pingNav.style.display = 'flex';
        showPing(0);
      }
    }
  } catch (err: unknown) {
    output.innerHTML = `<span class="error">Error: ${err instanceof Error ? err.message : String(err)}</span>`;
    pingNav.style.display = 'none';
  } finally {
    parseBtn.disabled = false;
  }
});
