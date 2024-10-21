import { useMemo } from "react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from "recharts";
import { QueueChartProps } from "../models/components.model";

export default function QueueChart({ data }: QueueChartProps) {
  const colors = [
    "#8884d8",
    "#82ca9d",
    "#ffc658",
    "#ff7300",
    "#0088FE",
    "#00C49F",
  ];

  const chartData = useMemo(() => {
    if (Object.keys(data).length === 0) {
      return [];
    }

    const allTimestamps = new Set<string>();
    Object.values(data).forEach((points) => {
      points.forEach((point) => allTimestamps.add(point.timestamp));
    });

    const sortedTimestamps = Array.from(allTimestamps).sort();

    return sortedTimestamps.map((timestamp) => {
      const point: { [key: string]: string | number | null } = { timestamp };
      Object.entries(data).forEach(([queueName, points]) => {
        const matchingPoint = points.find((p) => p.timestamp === timestamp);
        point[queueName] = matchingPoint ? matchingPoint.queueLength : null;
      });
      return point;
    });
  }, [data]);

  if (Object.keys(data).length === 0) {
    return <div>No queues available</div>;
  }

  if (chartData.length === 0) {
    return <div>No queue data available</div>;
  }

  return (
    <div className="queue-chart">
      <h2>Queue Length Over Time</h2>
      <ResponsiveContainer width="100%" height={400}>
        <LineChart data={chartData}>
          <CartesianGrid strokeDasharray="3 3" />
          <XAxis dataKey="timestamp" />
          <YAxis scale="log" domain={["auto", "auto"]} />
          <Tooltip />
          <Legend />
          {Object.keys(data).map((queueName, index) => (
            <Line
              key={queueName}
              type="monotone"
              dataKey={queueName}
              name={queueName}
              stroke={colors[index % colors.length]}
              activeDot={{ r: 8 }}
              connectNulls
            />
          ))}
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
}
