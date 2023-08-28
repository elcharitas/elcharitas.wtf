import "../global.css";
import { Plus_Jakarta_Sans } from "next/font/google";
import LocalFont from "next/font/local";
import { Metadata } from "next";

export const metadata: Metadata = {
  title: {
    default: "elcharitas.wtf",
    template: "%s | elcharitas.wtf",
  },
  description: "Software engineer at alphaday.com",
  openGraph: {
    title: "elcharitas.wtf",
    description: "Software engineer at alphaday.com",
    url: "https://elcharitas.wtf",
    siteName: "elcharitas.wtf",
    images: [
      {
        url: "https://elcharitas.wtf/og.png",
        width: 1920,
        height: 1080,
      },
    ],
    locale: "en-US",
    type: "website",
  },
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      "max-video-preview": -1,
      "max-image-preview": "large",
      "max-snippet": -1,
    },
  },
  twitter: {
    title: "Chronark",
    card: "summary_large_image",
  },
  icons: {
    shortcut: "/favicon.png",
  },
};
const inter = Plus_Jakarta_Sans({
  subsets: ["latin"],
  variable: "--font-inter",
});

const calSans = LocalFont({
  src: "../public/fonts/CalSans-SemiBold.ttf",
  variable: "--font-calsans",
});

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className={[inter.variable, calSans.variable].join(" ")}>
      <body className="bg-black">{children}</body>
    </html>
  );
}
