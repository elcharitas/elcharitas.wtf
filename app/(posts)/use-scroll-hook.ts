import { useCallback, useMemo, useSyncExternalStore } from "react";

const subscribeScroll = (ref: any, listener: any) => {
  ref.addEventListener("scroll", listener);
  return () => {
    ref.removeEventListener("scroll", listener);
  };
};

/**
 * useScroll aids adding functionality to scroll event
 *
 * @param callback
 */
const useScroll = <T extends () => void>(
  callback: T,
  ref: any = null,
  deps: any[] = []
) => {
  const subscribe = useMemo(() => subscribeScroll.bind({}, ref || window), [
    ref,
  ]);
  const getScrollSnapShot = useCallback(callback, deps);
  return useSyncExternalStore(subscribe, getScrollSnapShot, () => {});
};

export { useScroll };
