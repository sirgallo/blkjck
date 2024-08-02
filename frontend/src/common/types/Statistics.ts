export interface ResultsArrayElement {
  numSlots: number;
  numTransactions: number;
  samplePeriodSecs: number;
  slot: number;
}

export interface PerformanceMetricsResponse {
  id: number;
  jsonrpc: string;
  result: ResultsArrayElement[];
}

export interface PerformanceMetrics {
  tps: number;
}

export const PERFORMANCE_METRICS_REQUEST = ((): { method: string, headers: any, body: string } => { 
  const body = JSON.stringify({ jsonrpc: '2.0', id: 1, method: 'getRecentPerformanceSamples', params: [ 4 ] });
  const headers = { 'Content-Type': 'application/json' };
  return { method: 'POST', headers: headers, body };
})();