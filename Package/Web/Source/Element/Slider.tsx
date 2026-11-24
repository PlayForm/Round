import { createEffect, createSignal } from "solid-js";

/** A range input that reports once, on release, so one drag is one write. */
export default (properties:{ value:number; maximum:number; Commit:(value:number) => void }) => {
	const [value, Write] = createSignal(properties.value);

	createEffect(() => Write(properties.value));

	return (
		<input
			type="range"
			min="0"
			max={properties.maximum}
			step="1"
			value={value()}
			onInput={(event) => Write(Number(event.currentTarget.value))}
			onChange={() => properties.Commit(value())}
		/>
	);
};
