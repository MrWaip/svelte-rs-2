Modal_1[$.FILENAME] = "/x/Modal.svelte";
import * as $ from "svelte/internal/server";
import Outer from "./Outer.svelte";
function Modal_1($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Outer($$renderer, {
			children: $.invalid_default_snippet,
			$$slots: { default: ($$renderer, { value: Modal }) => {
				Modal.default($$renderer, {});
			} }
		});
	}, Modal_1);
}
Modal_1.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default Modal_1;
