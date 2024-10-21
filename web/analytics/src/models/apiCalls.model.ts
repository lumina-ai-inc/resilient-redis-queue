export interface QueueInfo {
  name: string;
  length: number;
}

export interface DataPoint {
  timestamp: string;
  queueLength: number;
}

export interface Information {
  memory: {
    [key: string]: string;
  };
  keyspace: {
    [key: string]: string;
  };
  server: {
    [key: string]: string;
  };
  clients: {
    [key: string]: string;
  };
  stats: {
    [key: string]: string;
  };
}
