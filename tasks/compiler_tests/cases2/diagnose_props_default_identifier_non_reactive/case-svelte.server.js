import * as $ from "svelte/internal/server";
import { noop } from "./helpers";
export default function App($$renderer, $$props) {
	let { onError = noop } = $$props;
}
