import React from 'react';
import { LineChart, Line, ResponsiveContainer } from 'recharts';

interface QueueInfo {
    name: string;
    length: number;
}

interface DataPoint {
    timestamp: string;
    queueLength: number;
}

interface QueueListProps {
    queues: QueueInfo[];
    dataPoints: { [key: string]: DataPoint[] };
}

const QueueList: React.FC<QueueListProps> = ({ queues, dataPoints }) => {
    return (
        <div className="queue-list">
            <h2>Queues</h2>
            <table>
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Length</th>
                    </tr>
                </thead>
                <tbody>
                    {queues.map((queue) => (
                        <tr key={queue.name}>
                            <td>{queue.name}</td>
                            <td>{queue.length}</td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    );
};

export default QueueList;