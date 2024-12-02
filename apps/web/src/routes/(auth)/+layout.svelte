<script lang="ts">
	import type { Snippet } from "svelte"

	type $Props = {
		children: Snippet
	}

	let { children }: $Props = $props()
</script>

<div class="container mx-auto px-6">
	{@render children()}
</div>
