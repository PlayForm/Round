import { mergeProps } from "solid-js";

import "./../assets/css/corner.css";

export default function Corner(props) {
	const merged = mergeProps({ id: "default" }, props);

	return <div class="Corner" data-id={merged.id}></div>;
}
