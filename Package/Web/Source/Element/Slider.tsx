import { createEffect, createSignal } from "solid-js";

/** A range input that reports once, on release, so one drag is one write. */
export default (properties:{ value:number; maximum:number; onCommit:(value:number) => void }) => {
	const [value, setValue] = createSignal(properties.value);

	createEffect(() => setValue(properties.value));

	return (
		<input
			type="range"
			min="0"
			max={properties.maximum}
			step="1"
			value={value()}
			onInput={(event) => setValue(Number(event.currentTarget.value))}
			onChange={() => properties.onCommit(value())}
		/>
	);
};
