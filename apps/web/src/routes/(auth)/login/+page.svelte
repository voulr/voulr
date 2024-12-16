<script lang="ts">
	import { Github } from "$lib/components/icons/github"
	import { Google } from "$lib/components/icons/google"
	import { Lockup } from "$lib/components/icons/lockup"
	import { Btn } from "$lib/components/ui/btn"
	import * as Card from "$lib/components/ui/card"
	import { Input } from "$lib/components/ui/input"

	const title = "Login | Voulr"
	const desc = "Sign in to the Voulr Dashboard to manage your Lambda deployments"
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
	<meta content={desc} name="description" />
</svelte:head>

<main class="py-8 sm:pt-16">
	<div class="flex justify-center">
		<div class="w-full max-w-lg">
			<Lockup class="mb-8 ml-6" />
			<Card.Root class="flex justify-center py-2 sm:py-6">
				<div class="w-full max-w-md">
					<Card.Header>
						<Card.Title class="text-xl">Sign in to your account</Card.Title>
						<Card.Description>
							New to Voulr?
							<span class="text-violet-300">
								<a href="/register">Create account</a>
							</span>
						</Card.Description>
					</Card.Header>
					<Card.Content class="grid gap-6">
						<div class="grid gap-2.5">
							<label for="email" class="text-sm">Email</label>
							<Input id="email" type="email" class="h-11" />
						</div>
						<div class="grid gap-2.5">
							<label for="password" class="text-sm">Password</label>
							<Input id="password" type="password" class="h-11" />
						</div>
					</Card.Content>
					<Card.Footer class="grid gap-6 pt-2">
						<Btn size="lg" variant="violet" class="w-full">Sign in</Btn>
						<div class="flex items-center">
							<hr class="h-px w-full border-0 bg-white/15" />
							<span class="text-nowrap px-2 text-xs text-white/75">Or with</span>
							<hr class="h-px w-full border-0 bg-white/15" />
						</div>
						<div class="flex gap-4">
							<Btn size="lg" variant="outline" class="w-full">
								<Github />
								GitHub
							</Btn>
							<Btn size="lg" variant="outline" class="w-full">
								<Google />
								Google
							</Btn>
						</div>
					</Card.Footer>
				</div>
			</Card.Root>
		</div>
	</div>
</main>
