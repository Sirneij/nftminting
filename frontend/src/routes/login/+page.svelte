<script>
	import { GOOGLE_AUTH_URL } from '$lib/utils/constants';

	async function handleGoogleLogin() {
		// Redirect to Google OAuth URL
		window.location.href = GOOGLE_AUTH_URL;
	}
</script>

<div class="login-container">
	<button class="login-button" on:click={handleGoogleLogin}> Login/Signup with Google </button>
</div>
