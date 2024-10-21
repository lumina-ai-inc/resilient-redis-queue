import { useState } from "react";
import { QueueListProps } from "../models/components.model";
import { deleteQueue } from "../services/queuesApi";
import "../App.css";

export default function QueueList({ queues }: QueueListProps) {
  const [isDeleting, setIsDeleting] = useState(false);

  const onDelete = async (queueName: string) => {
    setIsDeleting(true);
    if (confirm(`Are you sure you want to delete the queue "${queueName}"?`)) {
      await deleteQueue(queueName);
    }
    setIsDeleting(false);
  };

  return (
    <div className="queue-list">
      <h2>Queues</h2>
      <table>
        <thead>
          <tr>
            <th>Name</th>
            <th>Length</th>
            <th>Delete</th>
          </tr>
        </thead>
        <tbody>
          {queues.map((queue) => (
            <tr key={queue.name}>
              <td style={{ width: "45%" }}>{queue.name}</td>
              <td style={{ width: "45%" }}>{queue.length}</td>
              <td style={{ width: "10%" }}>
                <button
                  className="delete-button"
                  onClick={() => onDelete(queue.name)}
                  disabled={isDeleting}
                >
                  {isDeleting ? "Deleting..." : "Delete"}
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
