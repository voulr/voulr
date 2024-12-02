<script lang="ts">
	import type { WithElementRef } from "bits-ui"
	import type { HTMLInputAttributes } from "svelte/elements"

	import { cx } from "$lib/utils/cva.config"

	let {
		class: className,
		ref = $bindable(null),
		value = $bindable(),
		...restProps
	}: WithElementRef<HTMLInputAttributes> = $props()
</script>

<input
	bind:this={ref}
	class={cx(
		"flex h-10 w-full rounded-lg border bg-black px-3 py-2 text-sm ring-offset-black file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-white/40 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/20 disabled:cursor-not-allowed disabled:opacity-50",
		className
	)}
	bind:value
	{...restProps}
/>
