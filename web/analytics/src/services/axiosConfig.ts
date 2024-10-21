import axios from "axios";

const BASE_URL = import.meta.env.VITE_API_URL;

const axiosInstance = axios.create({
  baseURL: BASE_URL,
});

export const setApiKey = (apiKey: string) => {
  axiosInstance.defaults.headers.common["X-API-Key"] = apiKey;
};

export default axiosInstance;
