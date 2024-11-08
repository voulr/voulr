<script lang="ts">
	import { cx } from "$lib/utils/cva.config"

	import { Btn, type BtnProps } from "../btn/index"

	let { children, class: className, ...restProps }: BtnProps = $props()
</script>

<Btn class={cx("group gap-x-1 duration-300", className)} {...restProps}>
	{@render children?.()}
	<span class="transition-translate duration-300 ease-in-out group-hover:translate-x-1"> → </span>
</Btn>
