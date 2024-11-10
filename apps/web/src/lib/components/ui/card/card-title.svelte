<script lang="ts">
	import type { WithElementRef } from "bits-ui"
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "$lib/utils/cva.config"

	let {
		children,
		class: className,
		level = 3,
		ref = $bindable(null),
		...restProps
	}: {
		level?: 1 | 2 | 3 | 4 | 5 | 6
	} & WithElementRef<HTMLAttributes<HTMLDivElement>> = $props()
</script>

<div
	role="heading"
	aria-level={level}
	bind:this={ref}
	class={cx("text-lg font-semibold leading-none tracking-tight", className)}
	{...restProps}
>
	{@render children?.()}
</div>
