import type { VariantProps } from "cva"
import type { HTMLAnchorAttributes, HTMLButtonAttributes } from "svelte/elements"

import { cva } from "$lib/utils/cva.config"

import Root from "./btn.svelte"

const btnVariants = cva({
	base: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-lg text-sm font-medium transition-colors focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50",
	defaultVariants: {
		size: "md",
		variant: "default"
	},
	variants: {
		size: {
			lg: "h-11 px-8",
			md: "h-9 px-4 py-2"
		},
		variant: {
			default: "bg-white text-black hover:bg-white/85",
			minimal: "hover:text-white/85",
			outline: "border bg-black hover:bg-white/10",
			violet: "border border-[#8565A0] bg-[#705387] hover:border-[#7E5588] hover:bg-[#714C7A]"
		}
	}
})

type BtnVariant = VariantProps<typeof btnVariants>["variant"]
type BtnSize = VariantProps<typeof btnVariants>["size"]

type BtnProps = {
	size?: BtnSize
	variant?: BtnVariant
} & HTMLAnchorAttributes &
	HTMLButtonAttributes

export {
	type BtnProps as Props,
	type BtnProps,
	type BtnSize,
	type BtnVariant,
	btnVariants,
	Root,
	//
	Root as Btn
}
