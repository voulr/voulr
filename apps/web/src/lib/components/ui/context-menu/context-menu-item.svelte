<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import { ContextMenu as ContextMenuPrimitive } from "bits-ui"

	let {
		class: className,
		inset,
		ref = $bindable(null),
		...restProps
	}: {
		inset?: boolean
	} & ContextMenuPrimitive.ItemProps = $props()
</script>

<ContextMenuPrimitive.Item
	bind:ref
	class={cx(
		"relative flex cursor-default select-none items-center gap-2 rounded-md px-2 py-1.5 text-sm outline-none data-[disabled]:pointer-events-none data-[highlighted]:bg-white/10 data-[disabled]:opacity-50",
		inset && "pl-8",
		className
	)}
	{...restProps}
/>
