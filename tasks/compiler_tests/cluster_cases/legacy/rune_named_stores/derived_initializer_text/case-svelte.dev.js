import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $derived = () => ($.validate_store(derived, "derived"), $.store_get(derived, "$derived", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let count = $.prop($$props, "count", 8, 0);
	let multiplier = $.prop($$props, "multiplier", 8, 2);
	let doubled = $derived()(count() * multiplier());
	let summary = $derived()("x:" + count());
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `doubled=${doubled ?? ""}, summary=${summary ?? ""}`));
	$.append($$anchor, p);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
