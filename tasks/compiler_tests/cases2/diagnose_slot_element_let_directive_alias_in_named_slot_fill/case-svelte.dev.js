App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Wrapper($$anchor, { $$slots: { cell: ($$anchor, $$slotProps) => {
		var fragment_1 = $.comment();
		var node = $.first_child(fragment_1);
		const week = $.derived(() => $$slotProps.week);
		$.slot(node, $$props, "cell", { get week() {
			return $.get(week);
		} }, null);
		$.append($$anchor, fragment_1);
	} } }), "component", App, 1, 0, { componentTag: "Wrapper" });
	return $.pop($$exports);
}
