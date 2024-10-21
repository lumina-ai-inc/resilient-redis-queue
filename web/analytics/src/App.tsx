import "./App.css";
import { useState, useEffect } from "react";
import RedisInfo from "./components/RedisInfo";
import APIKeyForm from "./components/APIKeyForm";
import SearchForm from "./components/SearchForm";
import QueueList from "./components/QueueList";
import QueueChart from "./components/QueueChart";
import { useInformation } from "./hooks/useInformation";
import { useQueues } from "./hooks/useQueues";
import { setApiKey } from "./services/axiosConfig";

export default function App() {
  const [isAuthenticated, setIsAuthenticated] = useState<boolean>(false);
  const [searchTerm, setSearchTerm] = useState<string>("main*");

  useEffect(() => {
    // Retrieve API key from local storage on component mount
    const storedApiKey = localStorage.getItem("apiKey");
    if (storedApiKey) {
      setApiKey(storedApiKey);
      setIsAuthenticated(true);
    }
  }, []);

  const { data: information } = useInformation();
  const { data: queueData } = useQueues(searchTerm);

  const handleAPIKeySubmit = (key: string) => {
    setApiKey(key);
    localStorage.setItem("apiKey", key);
    setIsAuthenticated(true);
  };

  const handleLogout = () => {
    setApiKey("");
    localStorage.removeItem("apiKey");
    setIsAuthenticated(false);
  };

  const handleSearch = (search: string) => {
    setSearchTerm(search);
  };

  return (
    <div className="App">
      <header>
        <h1>Redis Queue Monitor</h1>
        {isAuthenticated && (
          <button onClick={handleLogout} className="logout-button">
            Logout
          </button>
        )}
      </header>
      {!isAuthenticated ? (
        <APIKeyForm onSubmit={handleAPIKeySubmit} />
      ) : (
        <div className="content">
          <SearchForm onSearch={handleSearch} defaultValue="main*" />
          {information && <RedisInfo information={information} />}
          {queueData && (
            <>
              <QueueList
                queues={queueData.queues}
                dataPoints={queueData.dataPoints}
              />
              <QueueChart data={queueData.dataPoints} />
            </>
          )}
        </div>
      )}
    </div>
  );
}
