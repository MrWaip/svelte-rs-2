import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
import { BRAND } from "./brand";
import { getName } from "./helpers";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Comp($$renderer, { title: `prefix ${$.stringify(BRAND)}${$.stringify(getName())}` });
	});
}
