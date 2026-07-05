import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
import Inner from "./Inner.svelte";
var root = $.add_locations($.from_html(`<span slot="caption"> </span>`), App[$.FILENAME], [[11, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $meta = () => ($.validate_store(meta(), "meta"), $.store_get(meta(), "$meta", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let meta = $.prop($$props, "meta", 24, () => writable({ hint: "x" }));
	let component = Inner;
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.component(node, () => component, ($$anchor, $$component) => {
		$$component($$anchor, { $$slots: { caption: ($$anchor, $$slotProps) => {
			var span = root();
			var text = $.child(span, true);
			$.reset(span);
			$.template_effect(() => $.set_text(text, ($meta(), $.untrack(() => $meta().hint || ""))));
			$.append($$anchor, span);
		} } });
	}), "component", App, 10, 0, { componentTag: "svelte:component" });
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
