import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
import Inner from "./Inner.svelte";
var root = $.from_html(`<span slot="caption"> </span>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $meta = () => $.store_get(meta(), "$meta", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let meta = $.prop($$props, "meta", 24, () => writable({ hint: "x" }));
	let component = Inner;
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.component(node, () => component, ($$anchor, $$component) => {
		$$component($$anchor, { $$slots: { caption: ($$anchor, $$slotProps) => {
			var span = root();
			var text = $.child(span, true);
			$.reset(span);
			$.template_effect(() => $.set_text(text, ($meta(), $.untrack(() => $meta().hint || ""))));
			$.append($$anchor, span);
		} } });
	});
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
