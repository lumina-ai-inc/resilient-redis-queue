import React, { useState, useEffect } from 'react';
import axios from 'axios';
import APIKeyForm from './components/APIKeyForm';
import SearchForm from './components/SearchForm';
import QueueList from './components/QueueList';
import QueueChart from './components/QueueChart';
import './App.css';

interface QueueInfo {
  name: string;
  length: number;
}

interface Information {
  // Define based on the /information endpoint response
  [key: string]: any;
}

interface DataPoint {
  timestamp: string;
  queueLength: number;
}

const App: React.FC = () => {
  const [apiKey, setApiKey] = useState<string>('');
  const [information, setInformation] = useState<Information | null>(null);
  const [queues, setQueues] = useState<QueueInfo[]>([]);
  const [searchTerm, setSearchTerm] = useState<string>('');
  const [dataPoints, setDataPoints] = useState<{ [key: string]: DataPoint[] }>({});

  const BASE_URL = process.env.REACT_APP_BASE_URL || 'http://localhost:8000';

  const headers = {
    'X-API-Key': apiKey,
  };

  const fetchInformation = async () => {
    try {
      const response = await axios.get(`${BASE_URL}/information`, { headers });
      setInformation(response.data);
      console.log('Information fetched successfully');
    } catch (error) {
      console.error('Error fetching information:', error);
    }
  };

  const fetchQueues = async (search: string) => {
    try {
      const encodedSearch = encodeURIComponent(search);
      const response = await axios.get(`${BASE_URL}/queues/${encodedSearch}`, { headers });
      setQueues(response.data);
      console.log('Queues fetched successfully');
    } catch (error) {
      console.error('Error fetching queues:', error);
    }
  };

  const handleAPIKeySubmit = (key: string) => {
    setApiKey(key);
  };

  const handleSearch = (search: string) => {
    setSearchTerm(search);
  };

  useEffect(() => {
    if (apiKey) {
      fetchInformation();
      fetchQueues(searchTerm);
      setDataPoints({});
    }
  }, [apiKey, searchTerm]);

  useEffect(() => {
    if (apiKey && searchTerm) {
      const interval = setInterval(async () => {
        try {
          const encodedSearch = encodeURIComponent(searchTerm);
          const response = await axios.get(`${BASE_URL}/queues/${encodedSearch}`, { headers });
          const timestamp = new Date().toLocaleTimeString();
          setDataPoints((prevData) => {
            const newData = { ...prevData };
            response.data.forEach((queue: QueueInfo) => {
              if (!newData[queue.name]) {
                newData[queue.name] = [];
              }
              newData[queue.name].push({ timestamp, queueLength: queue.length });
            });
            return newData;
          });
          setQueues(response.data);
        } catch (error) {
          console.error('Error updating queue data:', error);
        }
      }, 1000); // Update every 1 second

      return () => clearInterval(interval);
    }
  }, [apiKey, searchTerm, BASE_URL, headers]);

  return (
    <div className="App">
      <header>
        <h1>Redis Queue Monitor</h1>
      </header>
      {!apiKey ? (
        <APIKeyForm onSubmit={handleAPIKeySubmit} />
      ) : (
        <div className="content">
          <SearchForm onSearch={handleSearch} />
          <QueueList queues={queues} dataPoints={dataPoints} />
          <QueueChart data={dataPoints} />
        </div>
      )}
    </div>
  );
};

export default App;