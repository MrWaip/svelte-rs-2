Modal_1[$.FILENAME] = "/x/Modal.svelte";
import * as $ from "svelte/internal/server";
$.prevent_snippet_stringification(item);
function item($$renderer, Modal) {
	$.validate_snippet_args($$renderer);
	$$renderer.push(`<p>`);
	$.push_element($$renderer, "p", 6, 1);
	$$renderer.push(`${$.escape(Modal)}</p>`);
	$.pop_element();
}
function Modal_1($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { children } = $$props;
		item($$renderer, "hi");
	}, Modal_1);
}
Modal_1.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default Modal_1;
