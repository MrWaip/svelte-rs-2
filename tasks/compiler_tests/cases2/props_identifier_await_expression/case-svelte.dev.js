App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy"
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const props = $.rest_props($$props, rest_excludes, "props");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.await(node, () => fetch(1, 2, 3, $$props.field1), ($$anchor) => {}), "await", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
