export const BASE_API_URI = import.meta.env.DEV
	? import.meta.env.VITE_BASE_API_URI_DEV
	: import.meta.env.VITE_BASE_API_URI_PROD;

const GOOGLE_CLIENT_ID = import.meta.env.VITE_GOOGLE_CLIENT_ID;

export const GOOGLE_AUTH_URL = `https://accounts.google.com/o/oauth2/v2/auth?scope=email%20profile&access_type=offline&include_granted_scopes=true&state=state_parameter_passthrough_value&redirect_uri=${BASE_API_URI}/auth/google&response_type=code&client_id=${GOOGLE_CLIENT_ID}`;
