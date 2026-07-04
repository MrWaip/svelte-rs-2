import "svelte/internal/flags/legacy";
Modal_1[$.FILENAME] = "/x/Modal.svelte";
import * as $ from "svelte/internal/client";
import Outer from "./Outer.svelte";
export default function Modal_1($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, Modal_1);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Outer($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const Modal = $.derived_safe_equal(() => $$slotProps.value);
			$.add_svelte_meta(() => $.get(Modal).default($$anchor, {}), "component", Modal_1, 6, 1, { componentTag: "Modal.default" });
		} }
	}), "component", Modal_1, 5, 0, { componentTag: "Outer" });
	return $.pop($$exports);
}
