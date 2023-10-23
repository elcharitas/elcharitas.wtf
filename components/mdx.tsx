import * as React from "react";
import Link from "next/link";
import ReactMarkdown, { Components } from "react-markdown";
import remarkGfm from "remark-gfm";
import rehypeSlug from "rehype-slug";
import rehypeRaw from "rehype-raw";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import shiki from "shiki";

const highlighter = shiki.getHighlighter({ theme: "nord" });

async function Code({ code }: { code: string }) {
  const html = (await highlighter).codeToHtml(code, { lang: "tsx" });
  return <div dangerouslySetInnerHTML={{ __html: html }} />;
}

function clsx(...args: (string | undefined)[]) {
  return args.filter(Boolean).join(" ");
}
const components: Components = {
  h1: ({ className, node: _n, ...props }) => (
    <h1
      className={clsx(
        "mt-2 scroll-m-20 text-3xl font-bold tracking-tight text-zinc-100",
        className
      )}
      {...props}
    />
  ),
  h2: ({ className, node: _n, ...props }) => (
    <h2
      className={clsx(
        "mt-10 scroll-m-20 border-b  border-b-zinc-50/10 pb-3 text-2xl font-semibold tracking-tight text-zinc-100 first:mt-0",
        className
      )}
      {...props}
    />
  ),
  h3: ({ className, node: _n, ...props }) => (
    <h3
      className={clsx(
        "mt-8 scroll-m-20 text-xl font-semibold tracking-tight text-zinc-100",
        className
      )}
      {...props}
    />
  ),
  h4: ({ className, node: _n, ...props }) => (
    <h4
      className={clsx(
        "mt-8 scroll-m-20 text-lg font-semibold tracking-tight",
        className
      )}
      {...props}
    />
  ),
  h5: ({ className, node: _n, ...props }) => (
    <h5
      className={clsx(
        "mt-8 scroll-m-20 text-sm font-semibold tracking-tight",
        className
      )}
      {...props}
    />
  ),
  h6: ({ className, node: _n, ...props }) => (
    <h6
      className={clsx(
        "mt-8 scroll-m-20 text-base font-semibold tracking-tight",
        className
      )}
      {...props}
    />
  ),
  a: ({ className, href, node: _n, ...props }) => (
    <Link
      className={clsx(
        "font-medium text-zinc-50 underline underline-offset-4",
        className
      )}
      href={href ?? ""}
      {...props}
    />
  ),
  strong: ({ className, node: _n, ...props }) => (
    <strong className={clsx("font-bold text-zinc-50", className)} {...props} />
  ),
  p: ({ className, node: _n, ...props }) => (
    <p
      className={clsx("leading-7 [&:not(:first-child)]:mt-6", className)}
      {...props}
    />
  ),
  ul: ({ className, node: _n, ...props }) => (
    <ul className={clsx("my-6 ml-6 list-disc", className)} {...props} />
  ),
  ol: ({ className, node: _n, ...props }) => (
    <ol className={clsx("my-6 ml-6 list-decimal", className)} {...props} />
  ),
  li: ({ className, node: _n, ...props }) => (
    <li className={clsx("mt-2", className)} {...props} />
  ),
  blockquote: ({ className, node: _n, ...props }) => (
    <blockquote
      className={clsx(
        "mt-6 border-l-2 border-zinc-300 pl-6 italic text-zinc-800 [&>*]:text-zinc-600",
        className
      )}
      {...props}
    />
  ),
  img: ({ className, alt, node: _n, ...props }) => (
    // eslint-disable-next-line @next/next/no-img-element
    <img
      className={clsx("rounded-md inline-block", className)}
      alt={alt}
      {...props}
    />
  ),
  hr: ({ ...props }) => (
    <hr className="my-4 border-zinc-200 md:my-8" {...props} />
  ),
  table: ({ className, node: _n, ...props }) => (
    <div className="w-full my-6 overflow-y-auto">
      <table className={clsx("w-full", className)} {...props} />
    </div>
  ),
  tr: ({ className, node: _n, ...props }) => (
    <tr
      className={clsx(
        "m-0 border-t border-zinc-300 p-0 even:bg-zinc-100",
        className
      )}
      {...props}
    />
  ),
  th: ({ className, node: _n, ...props }) => (
    <th
      className={clsx(
        "border border-zinc-200 px-4 py-2 text-left font-bold [&[align=center]]:text-center [&[align=right]]:text-right",
        className
      )}
      {...props}
    />
  ),
  td: ({ className, node: _n, ...props }) => (
    <td
      className={clsx(
        "border border-zinc-200 px-4 py-2 text-left [&[align=center]]:text-center [&[align=right]]:text-right",
        className
      )}
      {...props}
    />
  ),
  pre: ({ className, node: _n, ...props }) => (
    <pre
      className={clsx(
        "mt-6 mb-4 overflow-x-auto rounded-lg bg-zinc-900 py-4",
        className
      )}
      {...props}
    />
  ),
  code: ({ className, children, node: _n, ...props }) => {
    const isMultiline = children?.toString().includes("\n");
    if (isMultiline) {
      // @ts-expect-error Code is a RSC
      return <Code code={children?.toString() ?? ""} />;
    }
    return (
      <code
        className={clsx(
          `${
            isMultiline
              ? "block p-8 bg-black/30 group md:gap-8"
              : "text-zinc-200 mx-1 py-[0.2rem] px-[0.3rem] align-middle bg-zinc-300 bg-opacity-25"
          } relative rounded font-mono text-sm text-zinc-80`,
          className
        )}
        {...props}
      >
        {children}
      </code>
    );
  },
};

interface MdxProps {
  code: string;
  baseUri?: string;
}

export function Mdx({ code, baseUri }: MdxProps) {
  const transformLink = (href: string) => {
    if (href.startsWith("http")) {
      return href;
    }
    return baseUri ? `${baseUri}${href}` : href;
  };

  return (
    <ReactMarkdown
      components={components}
      remarkPlugins={[remarkGfm]}
      rehypePlugins={[rehypeRaw, rehypeSlug, rehypeAutolinkHeadings]}
      transformLinkUri={transformLink}
      transformImageUri={transformLink}
      className="mdx text-zinc-100"
    >
      {code}
    </ReactMarkdown>
  );
}
