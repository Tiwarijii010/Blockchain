import axios from 'axios';

const BASE_URL = 'http://127.0.0.1:8000/api'; // Adjust the base URL as needed

// API call to get user information
export const getUser = async (id) => {
  try {
    const response = await axios.get(`${BASE_URL}/user/${id}`);
    return response.data;
  } catch (error) {
    console.error("Error fetching user:", error);
    throw error;
  }
};

// API call to get token balance
export const getTokenBalance = async (id) => {
  try {
    const response = await axios.get(`${BASE_URL}/token/balance/${id}`);
    return response.data;
  } catch (error) {
    console.error("Error fetching token balance:", error);
    throw error;
  }
};
