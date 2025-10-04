import { useWindowSize } from "@vueuse/core";

export function calcHeight(top: number, delta: number = 0) {
  const { height: vh } = useWindowSize();
  const FOOTNOTE_HEIGHT = 48;

  let height = Math.max(
    vh.value * 0.5,
    vh.value - window.scrollY - top - delta - FOOTNOTE_HEIGHT,
  );
  return {
    height: "" + height + "px",
  };
}
