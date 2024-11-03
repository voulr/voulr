<script lang="ts">
	const title = "VOULR"
	const desc = "a lambda deployment tool"
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
	<meta content={desc} name="description" />
</svelte:head>

<div class="pt-40 lg:pt-60">
	<section class="flex flex-col items-center gap-6 lg:gap-8">
		<h1
			class="text-center text-5xl font-semibold leading-[1.1] tracking-[-1px] sm:text-6xl sm:leading-none lg:text-7xl"
		>
			Ship Lambda. <br class="hidden min-[450px]:flex" /> Without Headache.
		</h1>
		<h2 class="max-w-sm text-center text-white/75 lg:text-lg">
			Connect your repo and automatically deploy to AWS Lambda.
		</h2>
	</section>
</div>
