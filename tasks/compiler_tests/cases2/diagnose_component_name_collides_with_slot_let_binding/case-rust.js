import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Outer from "./Outer.svelte";
export default function Modal_1($$anchor) {
	Outer($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const Modal = $.derived_safe_equal(() => $$slotProps.value);
			$.get(Modal).default($$anchor, {});
		} }
	});
}
