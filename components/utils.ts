export interface TextNode {
  index: number;
  text: string;
  pattern?: RegExp;
}

/**
 * Function to parse a text input into nodes using regex patterns.
 *
 * @param text
 * @param patterns
 * @returns nodes
 */
export const parseTextToNodes = (
  text: string,
  patterns: RegExp[]
): TextNode[] => {
  let nodeIndex = 0;
  let nodes: TextNode[] = [{ index: nodeIndex, text: text }];

  patterns.forEach((pattern) => {
    let newNodes: TextNode[] = [];
    nodes.forEach((node) => {
      if (node.pattern) {
        newNodes.push(node);
      } else {
        const matches = node.text.match(pattern);
        if (matches) {
          let currentIndex = 0;
          matches.forEach((match, index) => {
            const matchedText = match;
            const matchedIndex = node.text.indexOf(matchedText);

            if (matchedIndex > 0) {
              newNodes.push({
                index: nodeIndex++,
                text: node.text.substring(currentIndex, matchedIndex),
              });
              currentIndex = matchedIndex + matchedText.length;
            }

            newNodes.push({
              index: nodeIndex++,
              text: matchedText,
              pattern: pattern,
            });

            if (
              currentIndex < node.text.length && // Not at the end of the node
              index === matches.length - 1 // Last match
            ) {
              // Add the remaining text
              newNodes.push({
                index: nodeIndex++,
                text: node.text.substring(currentIndex),
              });
            }
          });
        } else {
          newNodes.push(node);
        }
      }
    });
    nodes = newNodes;
  });

  return nodes;
};

/**
 * Retrieves the unique selector for a given HTML element.
 *
 * @param element - The HTML element to get the selector for.
 * @returns - The unique selector for the element.
 */
export function getUniqueSelector(element: Element): string {
  const tag = element.tagName.toLowerCase();
  const id = element.id ? `#${element.id}` : "";
  const classNames = element.className?.split(" ").reduce((acc, className) => {
    if (
      className &&
      !className.includes(":") &&
      !className.includes("/") &&
      !className.includes("[") &&
      !className.includes("]")
    ) {
      return `${acc}.${className}`;
    }
    return acc;
  }, "");

  if (
    element.parentElement === null ||
    element.parentElement === document.body
  ) {
    return `${tag}${id}${classNames}`;
  }

  const siblings = Array.from(element.parentElement.children).filter(
    (el) => el.tagName === element.tagName
  );
  const position = siblings.indexOf(element) + 1;

  const prefix = getUniqueSelector(element.parentElement);

  if (!id && !classNames) {
    return `${prefix} > ${tag}:nth-child(${position})`;
  }

  return `${prefix} > ${tag}${id}${classNames}:nth-child(${position})`;
}
