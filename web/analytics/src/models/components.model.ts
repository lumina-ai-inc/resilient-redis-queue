import { DataPoint, QueueInfo, Information } from "./apiCalls.model";

export interface QueueChartProps {
  data: { [key: string]: DataPoint[] };
}

export interface APIKeyFormProps {
  onSubmit: (apiKey: string) => void;
}

export interface QueueListProps {
  queues: QueueInfo[];
  dataPoints: { [key: string]: DataPoint[] };
}

export interface SearchFormProps {
  onSearch: (searchTerm: string) => void;
  defaultValue?: string;
}

export interface RedisInfoProps {
  information: Information;
}
