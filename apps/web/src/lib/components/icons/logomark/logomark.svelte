<script lang="ts">
	import { cx } from "$lib/utils/cva.config"

	type $Props = {
		class?: string
	}

	let { class: className }: $Props = $props()
</script>

<svg
	class={cx("size-6", className)}
	fill="currentColor"
	viewBox="0 0 600 600"
	xmlns="http://www.w3.org/2000/svg"
>
	<defs id="defs1">
		<clipPath clipPathUnits="userSpaceOnUse" id="clipPath3">
			<path
				id="lpe_path-effect5"
				class="powerclip"
				d="M 145,145 H 455 V 455 H 145 Z m 4,2.5 v 304.19013 l 100.00012,0.80986 -50.39821,-152.09506 z"
			/>
		</clipPath>
	</defs>
	<g id="layer1">
		<path
			id="rect1"
			clip-path="url(#clipPath3)"
			d="m 150.00015,150.00015 v 299.99977 h 199.6891 l -48.79082,-150.40477 -49.5701,-149.595 z m 200.4999,0 v 299.99977 h 99.49987 V 150.00015 Z"
			transform="matrix(2.0000015,0,0,2.0000015,-300.00052,-300.00052)"
		/>
	</g>
</svg>
