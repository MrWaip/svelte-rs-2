Modal_1[$.FILENAME] = "/x/Modal.svelte";
import * as $ from "svelte/internal/server";
function Modal_1($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let items = [];
		$$renderer.push(`<!--[-->`);
		const each_array = $.ensure_array_like(items);
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let Modal = each_array[$$index];
			$$renderer.push(`<p>`);
			$.push_element($$renderer, "p", 6, 1);
			$$renderer.push(`${$.escape(Modal)}</p>`);
			$.pop_element();
		}
		$$renderer.push(`<!--]-->`);
	}, Modal_1);
}
Modal_1.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default Modal_1;
