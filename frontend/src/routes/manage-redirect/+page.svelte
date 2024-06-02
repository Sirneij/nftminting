<script lang="ts">
	import { loading } from '$lib/stores/loading.store';
	import { onMount } from 'svelte';
	import type { PageData } from './$types';

	$: loading.setLoading(true, 'Logging in, please wait...');

	export let data: PageData;

	$: ({ status, error, status_text } = data);

	onMount(() => {
		if (status_text !== 'success') {
			loading.setLoading(false);
		} else {
			loading.setLoading(true, 'Redirecting...');

			setTimeout(() => {
				window.location.href = '/';
			}, 2000);
		}
	});
</script>

{#if status_text === 'success'}
	<div class="container">
		<h1>Redirecting...</h1>
	</div>
{:else}
	<div class="container">
		<h1>Redirect failed</h1>
		<p>{status} - {error}</p>
	</div>
{/if}
