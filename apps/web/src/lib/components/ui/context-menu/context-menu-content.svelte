<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import { ContextMenu as ContextMenuPrimitive } from "bits-ui"

	let {
		class: className,
		portalProps,
		ref = $bindable(null),
		...restProps
	}: {
		portalProps?: ContextMenuPrimitive.PortalProps
	} & ContextMenuPrimitive.ContentProps = $props()
</script>

<ContextMenuPrimitive.Portal {...portalProps}>
	<ContextMenuPrimitive.Content
		bind:ref
		class={cx(
			"z-50 min-w-[8rem] rounded-lg border bg-black p-1 shadow-md focus:outline-none",
			className
		)}
		{...restProps}
	/>
</ContextMenuPrimitive.Portal>
