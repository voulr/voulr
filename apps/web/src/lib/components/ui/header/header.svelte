<script lang="ts">
	import { Github } from "$lib/components/icons/github"
	import { Logomark } from "$lib/components/icons/logomark"
	import { Menu } from "$lib/components/icons/menu/"
	import { ArrowBtn } from "$lib/components/ui/arrow-btn"
	import { Btn } from "$lib/components/ui/btn"
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu"
</script>

<header class="flex h-[72px] items-center justify-between">
	<a href="/">
		<Logomark />
	</a>
	<div class="hidden items-center gap-1.5 lg:flex">
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
			<DropdownMenu.Content class="w-52">
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
</header>
