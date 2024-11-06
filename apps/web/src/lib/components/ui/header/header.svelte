<script lang="ts">
	import { Lockup } from "$lib/components/icons/lockup"
	import { Logomark } from "$lib/components/icons/logomark"
</script>

<header class="relative flex h-16 items-center justify-between">
	<a href="/">
		<Lockup class="hidden lg:flex" />
		<Logomark class="flex lg:hidden" />
	</a>
	<hr
		style="background-image:linear-gradient(270deg, rgba(255, 255, 255, 0.00) 0%, #FFF 52.07%, rgba(255, 255, 255, 0.00) 100%)"
		class="absolute bottom-0 left-1/2 h-px w-full -translate-x-1/2 border-0 opacity-10"
	/>
</header>
