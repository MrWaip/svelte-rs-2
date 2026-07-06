import * as $ from "svelte/internal/server";
import Comp from "./Comp.svelte";
import { getProductName } from "./helpers";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		Comp($$renderer, { title: `${$.stringify(getProductName())} suffix` });
	});
}
