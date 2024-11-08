<script lang="ts">
	import { ArrowBtn } from "$lib/components/ui/arrow-btn"
	import { Btn } from "$lib/components/ui/btn"

	const title = "Voulr | Lambda without the headache."
	const desc = "A Lambda deployment tool"
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
	<meta content={desc} name="description" />
</svelte:head>

<section class="py-16">
	<div class="container">
		<div class="pt-24 xl:pt-44">
			<div class="flex flex-col items-center gap-6 lg:gap-8">
				<h1
					class="text-center font-neue text-5xl leading-[1.1] tracking-[-0.9px] sm:text-6xl sm:leading-none lg:text-7xl"
				>
					Lambda without the <br class="hidden min-[500px]:flex" /> headache.
				</h1>
				<h2 class="max-w-md text-center text-white/75 lg:text-lg">
					Connect your repo and automatically deploy to AWS Lambda.
				</h2>
				<div class="flex items-center gap-1.5">
					<ArrowBtn>Start now</ArrowBtn>
					<Btn class="duration-300" variant="minimal">Login</Btn>
				</div>
			</div>
		</div>
	</div>
</section>
