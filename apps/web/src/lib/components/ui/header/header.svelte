<script lang="ts">
	import lockupRaw from "$lib/assets/svgs/voulr-lockup.svg?raw"
	import logomarkRaw from "$lib/assets/svgs/voulr-logomark.svg?raw"
	import { Download } from "$lib/components/icons/download"
	import { Github } from "$lib/components/icons/github"
	import { Lockup } from "$lib/components/icons/lockup"
	import { Logomark } from "$lib/components/icons/logomark"
	import { Menu } from "$lib/components/icons/menu"
	import { T } from "$lib/components/icons/t"
	import { ArrowBtn } from "$lib/components/ui/arrow-btn"
	import { Btn } from "$lib/components/ui/btn"
	import * as ContextMenu from "$lib/components/ui/context-menu"
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu"
</script>

<header class="fixed left-0 z-50 w-full bg-black/10 backdrop-blur-lg">
	<div class="container mx-auto px-6 lg:px-10">
		<div class="flex h-16 items-center justify-between lg:h-[72px]">
			<ContextMenu.Root>
				<ContextMenu.Trigger>
					<a href="/">
						<Lockup class="hidden lg:flex" />
						<Logomark class="flex lg:hidden" />
					</a>
				</ContextMenu.Trigger>
				<ContextMenu.Content class="w-56">
					<ContextMenu.Item onclick={() => navigator.clipboard.writeText(logomarkRaw)}>
						<Logomark class="size-4" />
						Copy logomark as SVG
					</ContextMenu.Item>
					<ContextMenu.Item onclick={() => navigator.clipboard.writeText(lockupRaw)}>
						<T class="size-4" />
						Copy lockup as SVG
					</ContextMenu.Item>
				</ContextMenu.Content>
			</ContextMenu.Root>
			<div class="hidden items-center gap-2 lg:flex">
				<ArrowBtn>Start now</ArrowBtn>
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
			<div class="flex lg:hidden">
				<DropdownMenu.Root>
					<DropdownMenu.Trigger><Menu /></DropdownMenu.Trigger>
					<DropdownMenu.Content class="w-56">
						<DropdownMenu.Item>
							{#snippet child({ props })}
								<a {...props}>
									<Logomark class="size-4" />
									Start now
								</a>
							{/snippet}
						</DropdownMenu.Item>
						<DropdownMenu.Item>
							{#snippet child({ props })}
								<a
									href="https://github.com/voulr/voulr"
									rel="noopener noreferrer"
									target="_blank"
									{...props}
								>
									<Github />
									Repo
								</a>
							{/snippet}
						</DropdownMenu.Item>
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</div>
		</div>
	</div>
	<hr
		style="background-image:linear-gradient(270deg, rgba(255, 255, 255, 0.00) 0%, #FFF 52.07%, rgba(255, 255, 255, 0.00) 100%)"
		class="absolute bottom-0 left-1/2 h-px w-full -translate-x-1/2 border-0 opacity-10"
	/>
</header>
