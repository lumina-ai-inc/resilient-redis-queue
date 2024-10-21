Resilient Redis Queue

Adds resilience to redis by creating a unique queue for every worker. 
Only consume the items from the queue in case of success and failures. 
It also atomically provides retires and dead letter queues. There is also a analytics dashboard to monitor the queue health.

## Usage

```bash
docker compose up -d
```
