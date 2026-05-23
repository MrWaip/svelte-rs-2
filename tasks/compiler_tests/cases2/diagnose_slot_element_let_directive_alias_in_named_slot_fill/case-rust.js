import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	Wrapper($$anchor, { $$slots: { cell: ($$anchor, $$slotProps) => {
		var fragment_1 = $.comment();
		var node = $.first_child(fragment_1);
		const week = $.derived(() => $$slotProps.week);
		$.slot(node, $$props, "cell", { get week() {
			return $.get(week);
		} }, null);
		$.append($$anchor, fragment_1);
	} } });
}
