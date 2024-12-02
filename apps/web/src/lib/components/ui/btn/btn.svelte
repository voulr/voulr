<script lang="ts">
	import { cx } from "$lib/utils/cva.config"

	import { type BtnProps, btnVariants } from "./index"

	let {
		children,
		class: className,
		href = undefined,
		size = "md",
		type = "button",
		variant = "default",
		...restProps
	}: BtnProps = $props()
</script>

{#if href}
	<a class={cx(btnVariants({ className, size, variant }))} {href} {...restProps}>
		{@render children?.()}
	</a>
{:else}
	<button class={cx(btnVariants({ className, size, variant }))} {type} {...restProps}>
		{@render children?.()}
	</button>
{/if}
