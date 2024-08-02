import { asyncExponentialBackoff } from '@app/common/utils/AsyncExponentialBackoff';
import { type PerformanceMetrics, type PerformanceMetricsResponse, PERFORMANCE_METRICS_REQUEST } from '@app/common/types/Statistics';


export const getPerformanceMetrics = async (apiEndpoint: string): Promise<PerformanceMetrics> => {
  const resp = await asyncExponentialBackoff(apiEndpoint, 5, 500, PERFORMANCE_METRICS_REQUEST);
  const perfMetrics: PerformanceMetricsResponse = await resp.json();
  
  const tps = Math.floor(perfMetrics.result[0].numTransactions / perfMetrics.result[0].samplePeriodSecs);
  return { tps };
};