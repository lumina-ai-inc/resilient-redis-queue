import { QueueListProps } from "../models/components.model";

export default function QueueList({ queues }: QueueListProps) {
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
}
