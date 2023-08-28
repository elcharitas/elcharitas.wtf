/** @type {import('next').NextConfig} */
const nextConfig = {
  pageExtensions: ["js", "jsx", "ts", "tsx"],
  experimental: {
    appDir: true,
    mdxRs: true,
  },
};

export default nextConfig;
