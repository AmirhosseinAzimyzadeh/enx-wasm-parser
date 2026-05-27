export interface ParseOptions {
  outputFormat?: 'json' | 'binary';
  anchor: { lat: number; lon: number };
}

export interface ParseMetadata {
  totalPings: number;
  validPings: number;
  droppedCorruptBins: number;
  /** Pings that had no VmDas GPS block (raw PD0 files). x/z will be null in JSON, 0.0 in binary. */
  pingsWithoutGps: number;
  durationMs: number;
}

export interface ParseResultBinary {
  data: Float32Array;
  metadata: ParseMetadata;
}

export interface ParseResultJson {
  data: string;
  metadata: ParseMetadata;
}

export type ParseResult = ParseResultBinary | ParseResultJson;

export interface WorkerRequest {
  id: number;
  buffer: ArrayBuffer;
  options: ParseOptions;
}

export interface WorkerResponse {
  id: number;
  result?: ParseResult;
  error?: string;
}
