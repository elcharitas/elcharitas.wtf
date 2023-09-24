type Props = {
  text: string | undefined;
};

const BOLD_REGEX = /\*\*(.*?)\*\*/g;
const UNDERLINE_REGEX = /__(.*?)__/g;
const URL_REGEX = /(https?:\/\/[a-z\d-]+\.+[a-z\d]{2,}[\w/?&=#%]*)/g;

interface Node {
  index: number;
  text: string;
  pattern?: RegExp;
}

const parseAndSplit = (text: string, pattern: RegExp, nodeIndex: number) => {
  const splits = text.split(pattern);
  return splits.map((text, index) => {
    if (index === 1) {
      return {
        index: nodeIndex,
        text: text,
        pattern: pattern,
      };
    }
    return {
      index: 0,
      text: text,
    };
  });
};

const parseToJsx = (text: string, patterns: RegExp[]) => {
  const matches = patterns.reduce((acc, pattern) => {
    const nodes = pattern.exec(text);

    if (nodes === null || nodes.length === 1) {
      return acc;
    }

    if (nodes.length > acc.length) {
      return parseAndSplit(text, pattern, nodes.index);
    }

    acc.forEach((node, index) => {
      if (node.text.includes(nodes[0])) {
        acc.splice(index, 1, ...parseAndSplit(node.text, pattern, nodes.index));
      }
      return node;
    });
    return acc;
  }, [] as Node[]);

  if (matches.length === 0) {
    return text;
  }

  return matches.map((node) => {
    if (node.pattern === BOLD_REGEX) {
      return <strong key={node.index}>{node.text}</strong>;
    }
    if (node.pattern === UNDERLINE_REGEX) {
      return <u key={node.index}>{node.text}</u>;
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
    return <span key={node.index}>{node.text}</span>;
  });
};

export const Content: React.FC<Props> = ({ text }) => {
  return (
    <>
      {text !== undefined &&
        parseToJsx(text, [BOLD_REGEX, UNDERLINE_REGEX, URL_REGEX])}
    </>
  );
};
