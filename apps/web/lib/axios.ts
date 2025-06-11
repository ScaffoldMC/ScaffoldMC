import axios from "axios";

const api = axios.create({
	baseURL: process.env.NEXT_PUBLIC_API_BASE_URL,
	withCredentials: true,
});

api.interceptors.response.use(
	(res) => res,
	async (error) => {
		const originalRequest = error.config;
		if (error.response?.status === 401 && !originalRequest._retry) {
			originalRequest._retry = true;
			try {
				await api.post("/auth/refresh");
				return api(originalRequest);
			} catch (err) {
				return Promise.reject(err);
			}
		}
		return Promise.reject(error);
	},
);

export default api;
