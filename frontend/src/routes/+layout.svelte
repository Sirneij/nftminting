<script>
	import { navigating } from '$app/stores';
	import Loader from '$lib/components/Loader.svelte';
	import Nav from '$lib/components/Nav.svelte';
	import { loading } from '$lib/stores/loading.store';
	import '../app.css';

	$: loading.setNavigate(!!$navigating);
	$: loading.setLoading(!!$navigating, 'Loading, please wait...');
</script>

<Loader />

<Nav />

<slot />
