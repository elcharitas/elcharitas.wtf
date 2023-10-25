import { getHighlighter } from "shiki";

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
 * Builds a class string from a given class name.
 *
 * @param className - The class name to build the string from.
 * @returns
 */
export function buildClassString(className: string): string {
  return className.split(" ").reduce((classString, className) => {
    if (className && !/[:/[\]]/.test(className)) {
      return `${classString}.${className}`;
    }
    return classString;
  }, "");
}

export async function highlight(code: string, lang: string) {
  const highlighter = await getHighlighter({
    theme: "dracula",
  });
  return highlighter.codeToHtml(code, { lang });
}
