import * as $ from "svelte/internal/server";
import Tooltip from "./Tooltip.svelte";
import Icon from "./Icon.svelte";
export default function App($$renderer) {
	Tooltip($$renderer, { $$slots: { activator: ($$renderer) => {
		$.css_props($$renderer, true, { "--color": "red" }, () => {
			Icon($$renderer, { slot: "activator" });
		});
	} } });
}
