import { useQuery } from "react-query";
import { getInformation } from "../services/informationApi";
import { Information } from "../models/apiCalls.model";

export const useInformation = () => {
  return useQuery<Information, Error>(["information"], () => getInformation(), {
    refetchInterval: 1000,
    onError: (error) => {
      console.error("Error fetching information:", error);
    },
    keepPreviousData: true,
  });
};
