import { useQuery } from "react-query";
import { getQueues } from "../services/queuesApi";
import { QueueInfo, DataPoint } from "../models/apiCalls.model";

export const useQueues = (search: string) => {
  return useQuery<
    { queues: QueueInfo[]; dataPoints: { [key: string]: DataPoint[] } },
    Error
  >(
    ["queues", search],
    async () => {
      const queues = await getQueues(search);
      const timestamp = new Date().toLocaleTimeString();
      const dataPoints: { [key: string]: DataPoint[] } = {};

      queues.forEach((queue: QueueInfo) => {
        if (!dataPoints[queue.name]) {
          dataPoints[queue.name] = [];
        }
        dataPoints[queue.name].push({
          timestamp,
          queueLength: queue.length,
        });
      });

      return { queues, dataPoints };
    },
    {
      enabled: !!search,
      refetchInterval: 1000,
      onError: (error) => {
        console.error("Error fetching queues:", error);
      },
      keepPreviousData: true,
    }
  );
};
