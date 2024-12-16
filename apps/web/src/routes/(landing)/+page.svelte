<script lang="ts">
	import { ArrowBtn } from "$lib/components/ui/arrow-btn"
	import { Btn } from "$lib/components/ui/btn"

	const title = "Voulr"
	const desc = "A Lambda platform for backend developers"
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
	<meta content={desc} name="description" />
</svelte:head>

<section class="py-40 lg:pt-60">
	<div class="flex flex-col items-center gap-6 lg:gap-8">
		<h1
			class="text-center font-neue text-5xl leading-[1.1] sm:text-6xl sm:leading-none lg:text-7xl"
		>
			Lambda is finally <br class="hidden min-[500px]:flex" /> effortless
		</h1>
		<h2 class="max-w-sm text-center text-white/75 lg:text-lg">
			Connect your repo and automatically deploy to AWS Lambda.
		</h2>
		<div class="flex items-center gap-2">
			<ArrowBtn variant="violet" href="/register">Start now</ArrowBtn>
			<Btn
				class="duration-300"
				variant="minimal"
				href="https://github.com/voulr/voulr"
				rel="noopener noreferrer"
				target="_blank"
			>
				Repo
			</Btn>
		</div>
	</div>
</section>
