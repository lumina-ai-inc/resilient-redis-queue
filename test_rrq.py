import test_rrq as st
import requests
import dotenv
import os
dotenv.load_dotenv(override=True)

# Set the API endpoint and headers
BASE_URL = "http://localhost:8000"
HEADERS = {"X-API-Key": os.getenv("API_KEY")}

# Function to test API endpoints
def test_endpoints():
    endpoints = [
        "/",
        "/information",
        "/queues/main:*",
    ]
    results = {}
    for endpoint in endpoints:
        try:
            if endpoint in ["/produce", "/consume", "/complete"]:
                response = requests.post(f"{BASE_URL}{endpoint}", headers=HEADERS)
            else:
                response = requests.get(f"{BASE_URL}{endpoint}", headers=HEADERS)
            results[endpoint] = f"Status: {response.status_code}, Response: {response.text[:100]}..."
        except requests.RequestException as e:
            results[endpoint] = f"Error: {str(e)}"
    return results

# Streamlit app
print("Redis Queue Monitor - API Test")

# Test and display results
print("API Endpoint Test Results")
test_results = test_endpoints()

for endpoint, result in test_results.items():
    print(f"Endpoint: {endpoint}")
    print(result)

# Add a refresh button
print("Refresh Data")
