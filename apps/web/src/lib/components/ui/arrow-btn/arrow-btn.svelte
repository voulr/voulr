<script lang="ts">
	import { Arrow } from "$lib/components/icons/arrow"
	import { cx } from "$lib/utils/cva.config"

	import { Btn, type BtnProps } from "../btn/index"

	let { children, class: className, ...restProps }: BtnProps = $props()
</script>

<Btn class={cx("group gap-x-[5px] transition-all duration-200 ease-in", className)} {...restProps}>
	{@render children?.()}
	<Arrow />
</Btn>
