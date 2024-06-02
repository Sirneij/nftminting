<script>
	import { loading } from '$lib/stores/loading.store';
	import { onMount } from 'svelte';
	let isPageLoaded = false;

	onMount(() => {
		isPageLoaded = true;
	});
	$: if ($loading.status === 'NAVIGATING') {
		setTimeout(() => {
			if ($loading.status === 'NAVIGATING') {
				$loading.status = 'LOADING';
			}
		}, 500);
	}
</script>

{#if $loading.status === 'LOADING'}
	<div class="loader-container">
		<div class="loader" />
		{#if $loading.message}
			<div class="text-container">
				<p>{$loading.message}</p>
			</div>
		{/if}
	</div>
{/if}

{#if !isPageLoaded}
	<div class="loader-start">
		<div class="loader" />
		<div class="text-container">
			<p>Welcome to www.nftminting.com...</p>
		</div>
	</div>
{/if}

<style>
	/* Source: https://www.freecodecamp.org/news/how-to-create-a-css-only-loader/ */
	.loader-container {
		position: fixed;
		top: 3.5rem;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: hsla(222, 44%, 13%, 1);
		background: linear-gradient(180deg, #011627 3%, hsla(222, 47%, 11%, 1) 41.35%);
		z-index: 99999;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}
	.loader-start {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: hsla(222, 44%, 13%, 1);
		background: linear-gradient(180deg, #011627 3%, hsla(222, 47%, 11%, 1) 41.35%);
		z-index: 99999;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}

	.loader {
		--n: 10; /* control the number of stripes */
		--s: 10px; /* control the width of stripes */
		--g: 5px; /* control the gap between stripes */
		--c: var(--text-carolina-blue); /* the color */
		width: calc(var(--n) * (var(--s) + var(--g)) - var(--g));
		height: 30px;
		padding: var(--g);
		margin: 5px auto;
		border: 1px solid;
		background: repeating-linear-gradient(
				90deg,
				var(--c) 0 var(--s),
				#0000 0 calc(var(--s) + var(--g))
			)
			left / calc((var(--n) + 1) * (var(--s) + var(--g))) 100% no-repeat content-box;
		animation: load 1.5s steps(calc(var(--n) + 1)) infinite;
	}

	.text-container {
		padding-top: 1rem;
		display: flex;
		justify-content: center;
	}
	.text-container p {
		line-height: 1.5;
		font-weight: 700;
		overflow: hidden; /* Ensures the content is not revealed until the animation */
		border-right: 0.15em solid var(--text-carolina-blue); /* The typwriter cursor */
		white-space: nowrap; /* Keeps the content on a single line */
		margin: 0 auto; /* Gives that scrolling effect as the typing happens */
		animation:
			typing 3.5s steps(30, end),
			blink-caret 0.5s step-end infinite;
	}
	@keyframes load {
		0% {
			background-size: 0% 100%;
		}
	}
	/* The typing effect */
	@keyframes typing {
		from {
			width: 0;
		}
		to {
			width: 100%;
		}
	}
	/* The typewriter cursor effect */
	@keyframes blink-caret {
		from,
		to {
			border-color: transparent;
		}
		50% {
			border-color: var(--text-carolina-blue);
		}
	}
</style>
