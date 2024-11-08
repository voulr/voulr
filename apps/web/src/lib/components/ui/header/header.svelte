<script lang="ts">
	import { Lockup } from "$lib/components/icons/lockup"
	import { Logomark } from "$lib/components/icons/logomark"
	import { Menu } from "$lib/components/icons/menu/"
	import { ArrowBtn } from "$lib/components/ui/arrow-btn"
	import { Btn } from "$lib/components/ui/btn"
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu"
</script>

<header class="fixed z-50 w-full backdrop-blur-md">
	<div class="container">
		<div class="relative flex h-16 items-center justify-between">
			<a href="/">
				<Lockup class="hidden lg:flex" />
				<Logomark class="flex lg:hidden" />
			</a>
			<div class="hidden items-center gap-1.5 lg:flex">
				<ArrowBtn>Start now</ArrowBtn>
				<Btn class="duration-300" variant="minimal">Login</Btn>
			</div>
			<div class="flex lg:hidden">
				<DropdownMenu.Root>
					<DropdownMenu.Trigger><Menu /></DropdownMenu.Trigger>
					<DropdownMenu.Content class="w-52">
						<DropdownMenu.Item>Start now</DropdownMenu.Item>
						<DropdownMenu.Item>Login</DropdownMenu.Item>
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</div>
			<hr
				style="background-image:linear-gradient(270deg, rgba(255, 255, 255, 0.00) 0%, #FFF 52.07%, rgba(255, 255, 255, 0.00) 100%)"
				class="absolute bottom-0 left-1/2 h-px w-full -translate-x-1/2 border-0 opacity-10"
			/>
		</div>
	</div>
</header>
