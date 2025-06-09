import axios from "axios";

let accessToken = "";

export const setAccessToken = (token: string) => {
	accessToken = token;
};

export const getAccessToken = () => accessToken;

const api = axios.create({
	baseURL: process.env.API_BASE_URL,
	withCredentials: true,
});

api.interceptors.request.use((config) => {
	if (accessToken && config.headers) {
		config.headers.Authorization = `Bearer ${accessToken}`;
	}
	return config;
});

api.interceptors.response.use(
	(res) => res,
	async (error) => {
		const originalRequest = error.config;
		if (error.response?.status === 401 && !originalRequest._retry) {
			originalRequest._retry = true;
			try {
				const { data } = await api.post("/auth/refresh");
				setAccessToken(data.accessToken);
				originalRequest.headers["Authorization"] =
					`Bearer ${data.accessToken}`;
				return api(originalRequest);
			} catch (err) {
				return Promise.reject(err);
			}
		}
		return Promise.reject(error);
	},
);

export default api;
