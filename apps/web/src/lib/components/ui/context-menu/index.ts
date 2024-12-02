import { ContextMenu as ContextMenuPrimitive } from "bits-ui"

import Content from "./context-menu-content.svelte"
import Item from "./context-menu-item.svelte"

const Root = ContextMenuPrimitive.Root
const Trigger = ContextMenuPrimitive.Trigger

export {
	Content,
	Content as ContextMenuContent,
	Item,
	Item as ContextMenuItem,
	Root,
	//
	Root as ContextMenu,
	Trigger,
	Trigger as ContextMenuTrigger
}
