<script lang="ts">
	import { ArrowBtn } from "$lib/components/ui/arrow-btn"
	import { Btn } from "$lib/components/ui/btn"

	const title = "Voulr | Lambda is finally effortless."
	const desc = "The AWS Lambda platform."
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
					class="text-center font-neue text-5xl leading-[1.1] sm:text-6xl sm:leading-none lg:text-7xl"
				>
					Lambda is finally <br class="hidden min-[500px]:flex" /> effortless.
				</h1>
				<h2 class="max-w-sm text-center text-white/75 lg:text-lg">
					Connect your repo and automatically deploy to AWS Lambda.
				</h2>
				<div class="flex items-center gap-1.5">
					<ArrowBtn href="/waitlist">Join waitlist</ArrowBtn>
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
		</div>
	</div>
</section>
