import * as $ from "svelte/internal/server";
import Outer from "./Outer.svelte";
export default function Modal_1($$renderer) {
	Outer($$renderer, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$renderer, { value: Modal }) => {
			Modal.default($$renderer, {});
		} }
	});
}
