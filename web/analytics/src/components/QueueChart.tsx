import React from 'react';
import {
    LineChart,
    Line,
    XAxis,
    YAxis,
    CartesianGrid,
    Tooltip,
    Legend,
    ResponsiveContainer,
} from 'recharts';

interface DataPoint {
    timestamp: string;
    queueLength: number;
}

interface QueueChartProps {
    data: { [key: string]: DataPoint[] };
}

const QueueChart: React.FC<QueueChartProps> = ({ data }) => {
    const colors = ['#8884d8', '#82ca9d', '#ffc658', '#ff7300', '#0088FE', '#00C49F'];

    const allDataPoints = Object.values(data).flat();
    const sortedDataPoints = allDataPoints.sort((a, b) => new Date(a.timestamp).getTime() - new Date(b.timestamp).getTime());

    return (
        <div className="queue-chart">
            <h2>Queue Length Over Time</h2>
            <ResponsiveContainer width="100%" height={400}>
                <LineChart data={sortedDataPoints}>
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis dataKey="timestamp" />
                    <YAxis scale="log" domain={['auto', 'auto']} />
                    <Tooltip />
                    <Legend />
                    {Object.entries(data).map(([queueName, points], index) => (
                        <Line
                            key={queueName}
                            type="monotone"
                            dataKey="queueLength"
                            data={points}
                            name={queueName}
                            stroke={colors[index % colors.length]}
                            activeDot={{ r: 8 }}
                        />
                    ))}
                </LineChart>
            </ResponsiveContainer>
        </div>
    );
};

export default QueueChart;