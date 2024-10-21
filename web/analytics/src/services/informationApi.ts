import axiosInstance from "./axiosConfig";

export const getInformation = async () => {
  const response = await axiosInstance.get("/information");
  return response.data;
};
