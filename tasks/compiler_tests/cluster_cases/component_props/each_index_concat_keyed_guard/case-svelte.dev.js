App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.tag_proxy($.proxy([{ id: 1 }]), "items");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 19, () => items, (badge) => badge.id, ($$anchor, badge, i) => {
		$.add_svelte_meta(() => Badge($$anchor, { get dataTestid() {
			return `badge-${$.get(i) ?? ""}`;
		} }), "component", App, 6, 1, { componentTag: "Badge" });
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
