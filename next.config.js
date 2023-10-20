/** @type {import('next').NextConfig} */
module.exports = {
  pageExtensions: ["ts", "tsx"],
  images: {
    remotePatterns: [
      {
        protocol: "https",
        hostname: "cdn.hashnode.com",
      },
      {
        protocol: "https",
        hostname: "images.unsplash.com",
      },
      {
        protocol: "https",
        hostname: "raw.githubusercontent.com",
      },
    ],
  },
  webpack: (config) => {
    config.module.rules.push({
      test: /\.graphql/,
      type: "asset/source",
    });
    return config;
  },
};
