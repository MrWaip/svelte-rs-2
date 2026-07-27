App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let ratings = $.tag_proxy($.proxy([0, 1]), "ratings");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, () => ratings, $.index, ($$anchor, value, index) => {
		$.add_svelte_meta(() => Child($$anchor, { onChange: (v) => ratings[index] = v }), "component", App, 7, 1, { componentTag: "Child" });
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
