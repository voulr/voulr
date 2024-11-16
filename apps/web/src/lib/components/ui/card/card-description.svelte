<script lang="ts">
	import type { WithElementRef } from "bits-ui"
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "$lib/utils/cva.config"

	let {
		children,
		class: className,
		ref = $bindable(null),
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLParagraphElement>> = $props()
</script>

<p bind:this={ref} class={cx("text-sm text-white/80", className)} {...restProps}>
	{@render children?.()}
</p>
