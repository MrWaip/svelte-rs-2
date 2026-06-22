import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	const $derived = () => $.store_get(derived, "$derived", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let count = $.prop($$props, "count", 8, 0);
	let multiplier = $.prop($$props, "multiplier", 8, 2);
	let doubled = $derived()(count() * multiplier());
	let summary = $derived()("x:" + count());
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `doubled=${doubled ?? ""}, summary=${summary ?? ""}`));
	$.append($$anchor, p);
	$$cleanup();
}
