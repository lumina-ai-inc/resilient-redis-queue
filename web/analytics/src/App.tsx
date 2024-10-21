import React, { useState, useEffect } from 'react';
import axios from 'axios';
import './App.css';
import RedisInfo from './components/RedisInfo';
import APIKeyForm from './components/APIKeyForm';
import SearchForm from './components/SearchForm';
import QueueList from './components/QueueList';
import QueueChart from './components/QueueChart';

interface QueueInfo {
  name: string;
  length: number;
}

interface Information {
  memory: {
    [key: string]: string;
  };
  keyspace: {
    [key: string]: string;
  };
  server: {
    [key: string]: string;
  };
  clients: {
    [key: string]: string;
  };
  stats: {
    [key: string]: string;
  };
}

interface DataPoint {
  timestamp: string;
  queueLength: number;
}
const App: React.FC = () => {
  const [apiKey, setApiKey] = useState<string>('');
  const [information, setInformation] = useState<Information | null>(null);
  const [queues, setQueues] = useState<QueueInfo[]>([]);
  const [searchTerm, setSearchTerm] = useState<string>('main*');
  const [dataPoints, setDataPoints] = useState<{ [key: string]: DataPoint[] }>({});

  const BASE_URL = import.meta.env.VITE_API_URL;

  useEffect(() => {
    // Retrieve API key from local storage on component mount
    const storedApiKey = localStorage.getItem('apiKey');
    if (storedApiKey) {
      setApiKey(storedApiKey);
    }
  }, []);

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
    // Save API key to local storage
    localStorage.setItem('apiKey', key);
  };

  const handleLogout = () => {
    setApiKey('');
    setInformation(null);
    setQueues([]);
    setDataPoints({});
    // Remove API key from local storage
    localStorage.removeItem('apiKey');
  };

  const handleSearch = (search: string) => {
    setSearchTerm(search);
  };

  useEffect(() => {
    if (apiKey) {
      fetchInformation();
      fetchQueues(searchTerm);
    }
  }, [apiKey, searchTerm]);

  useEffect(() => {
    if (apiKey && searchTerm) {
      const fetchData = async () => {
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
      };

      fetchData(); // Fetch data immediately when component mounts or searchTerm changes

      const interval = setInterval(fetchData, 1000); // Update every 1 second

      return () => clearInterval(interval);
    }
  }, [apiKey, searchTerm, BASE_URL, headers]);

  return (
    <div className="App">
      <header>
        <h1>Redis Queue Monitor</h1>
        {apiKey && (
          <button onClick={handleLogout} className="logout-button">
            Logout
          </button>
        )}
      </header>
      {!apiKey ? (
        <APIKeyForm onSubmit={handleAPIKeySubmit} />
      ) : (
        <div className="content">
          <SearchForm onSearch={handleSearch} defaultValue="main*" />
          {information && <RedisInfo information={information} />}
          <QueueList queues={queues} dataPoints={dataPoints} />
          <QueueChart data={dataPoints} />
        </div>
      )}
    </div>
  );
};

export default App;