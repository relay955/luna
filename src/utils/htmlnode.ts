/**
 * 특정 노드가 다른 노드에 포함되어 있는지 확인합니다.
 */
export const containsNode = (checkRoot: HTMLElement, target: HTMLElement) => {
  let node:HTMLElement | null = target;
  while (node) {
    if (node === checkRoot) {
      return true;
    }
    node = node.parentElement;
  }
  return false;
}
