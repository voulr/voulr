<script lang="ts">
	import type { WithElementRef } from "bits-ui"
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "$lib/utils/cva.config"

	let {
		children,
		class: className,
		ref = $bindable(null),
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props()
</script>

<div bind:this={ref} class={cx("rounded-lg border bg-black shadow-sm", className)} {...restProps}>
	{@render children?.()}
</div>
