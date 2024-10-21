import axiosInstance from "./axiosConfig";

export const getQueues = async (search: string) => {
  const encodedSearch = encodeURIComponent(search);
  const response = await axiosInstance.get(`/queues/${encodedSearch}`);
  return response.data;
};
