import redis
import random
import string
import subprocess
import time
import os
queues = ['banana', 'apple', 'orange']
def start_redis_server():
    try:
        # Start Redis server
        redis_process = subprocess.Popen(["redis-server"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        print("Starting Redis server...")
        time.sleep(2)  # Give some time for the server to start
        return redis_process
    except FileNotFoundError:
        print("Redis server not found. Make sure it's installed and in your PATH.")
        return None

# Function to generate random string
def random_string(length=10):
    return ''.join(random.choices(string.ascii_lowercase + string.digits, k=length))

# Start Redis server
redis_process = start_redis_server()

if redis_process:
    try:
        # Connect to Redis
        r = redis.Redis(host='localhost', port=6379, db=0)

        # Create a few queues with random names and add items
        for queue in queues:
            queue_name = f"main:{queue}"
            
            # Add a few items to each queue
            for _ in range(random.randint(3, 7)):
                item = random_string(20)
                r.lpush(queue_name, item)
            
            print(f"Created queue '{queue_name}' with {r.llen(queue_name)} items")

        print("Local Redis server is running with sample queues created.")

    except redis.exceptions.ConnectionError:
        print("Failed to connect to Redis. Make sure the server is running.")

    finally:
        # Keep the script running to maintain the Redis server
        input("Press Enter to stop the Redis server and exit...")
        
        # Stop Redis server
        redis_process.terminate()
        redis_process.wait()
        print("Redis server stopped.")
else:
    print("Failed to start Redis server.")

