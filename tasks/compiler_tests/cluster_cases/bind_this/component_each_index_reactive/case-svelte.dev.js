App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
var root = $.add_locations($.from_html(`<!> <button>x</button>`, 1), App[$.FILENAME], [[10, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let refs = $.tag_proxy($.proxy([]), "refs");
	let items = $.tag_proxy($.proxy([{ id: 1 }, { id: 2 }]), "items");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 19, () => items, (item) => item.id, ($$anchor, item, i) => {
		var fragment_1 = root();
		var node_1 = $.first_child(fragment_1);
		$.validate_binding("bind:this={refs[i]}", [], () => refs, () => $.get(i), 9, 7);
		$.add_svelte_meta(() => $.bind_this(Comp(node_1, {}), ($$value, i) => refs[i] = $$value, (i) => refs?.[i], () => [$.get(i)]), "component", App, 9, 1, { componentTag: "Comp" });
		var button = $.sibling(node_1, 2);
		$.delegated("click", button, function click() {
			return refs[$.get(i)].foo();
		});
		$.append($$anchor, fragment_1);
	}), "each", App, 8, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
