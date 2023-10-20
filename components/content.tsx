const BOLD_REGEX = /\*\*(.*?)\*\*/g; // Example: **bold**
const UNDERLINE_REGEX = /__(.*?)__/g; // Example: __underline__
const URL_REGEX = /(https?:\/\/[a-z\-\d-]+\.+[a-z\-\d]{2,}[\w/?&=#%]*)/g; // Example: https://google.com
const NAMED_URL_REGEX = /\[(.*?)\]\((https:\/\/[^\)]+)\)/;
const NAMED_GROUP_URL_REGEX = /(\[(.*?)\]\((https:\/\/[^\)]+)\))/g; // Example: [Google](https://google.com)
const EXACT_URL_REGEX = new RegExp(`^${URL_REGEX.source}$`); // Example: https://google.com

interface Node {
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
const parseTextToNodes = (text: string, patterns: RegExp[]): Node[] => {
  let nodeIndex = 0;
  let nodes: Node[] = [{ index: nodeIndex, text: text }];

  patterns.forEach((pattern) => {
    let newNodes: Node[] = [];
    nodes.forEach((node) => {
      if (node.pattern) {
        newNodes.push(node);
      } else {
        const match = node.text.match(pattern);
        if (match) {
          const matchedText = match[0];
          const matchedIndex = node.text.indexOf(matchedText);
          if (matchedIndex > 0) {
            newNodes.push({
              index: nodeIndex++,
              text: node.text.substring(0, matchedIndex),
            });
          }
          newNodes.push({
            index: nodeIndex++,
            text: matchedText,
            pattern: pattern,
          });
          if (matchedIndex + matchedText.length < node.text.length) {
            newNodes.push({
              index: nodeIndex++,
              text: node.text.substring(matchedIndex + matchedText.length),
            });
          }
        } else {
          newNodes.push(node);
        }
      }
    });
    nodes = newNodes;
  });

  return nodes;
};

const parseToJsx = (text: string, patterns: RegExp[]) => {
  const nodes = parseTextToNodes(text, patterns);

  if (nodes.length === 0) {
    return <span key={text}>{text}</span>;
  }

  return nodes.map((node) => {
    if (node.pattern === BOLD_REGEX) {
      const [_, boldText] = BOLD_REGEX.exec(node.text) ?? [];
      return <strong key={node.index}>{boldText}</strong>;
    }
    if (node.pattern === UNDERLINE_REGEX) {
      const [_, underlineText] = UNDERLINE_REGEX.exec(node.text) ?? [];
      return <u key={node.index}>{underlineText}</u>;
    }
    if (node.pattern === URL_REGEX) {
      return (
        <a
          key={node.index}
          href={node.text}
          className="text-zinc-100 hover:text-zinc-300"
        >
          {node.text}
        </a>
      );
    }
    if (node.pattern === NAMED_GROUP_URL_REGEX) {
      const matches = NAMED_URL_REGEX.exec(node.text);
      if (matches === null || matches.length === 1) {
        return node.text;
      }
      return (
        <a
          key={node.index}
          href={matches[2]}
          className="text-zinc-100 hover:text-zinc-300"
        >
          {matches[1]}
        </a>
      );
    }
    if (node.pattern === undefined) {
      return <span key={node.index}>{node.text}</span>;
    }
  });
};

type Props = {
  text: string | undefined;
};

export const Content: React.FC<Props> = ({ text }) => {
  if (text !== undefined) {
    return (
      <>
        {parseToJsx(text, [
          BOLD_REGEX,
          UNDERLINE_REGEX,
          NAMED_GROUP_URL_REGEX,
          URL_REGEX,
        ])}
      </>
    );
  }
  return null;
};
