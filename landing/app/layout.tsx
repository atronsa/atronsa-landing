import type { Metadata } from "next";
import { Poppins } from "next/font/google";
import localFont from "next/font/local";
import Script from "next/script";

import Navbar from "@/layout/Navbar";
import Footer from "@/layout/Footer";

import "./globals.css";

import { DynamicManifest } from "@/components/DynamicManifest";

const poppins = Poppins({
  subsets: ["latin"],
  variable: "--font-poppins",
  weight: ["400", "500", "600"],
  display: "swap",
  preload: true,
});

const danleySans = localFont({
  src: [
    {
      path: "./font/OPTIDanley-Medium.otf",
      weight: "400",
      style: "normal",
    },
  ],
  variable: "--font-title",
  display: "swap",
  preload: true,
});

export const metadata: Metadata = {
  metadataBase: new URL("https://atronsa.com"),

  title: {
    default: "Atronsa | Web Design & Software Agency in Addis Ababa, Ethiopia",
    template: "%s | Atronsa - Web Agency in Addis Ababa",
  },

  applicationName: "Atronsa",
  category: "Technology",
  generator: "Next.js",
  referrer: "origin-when-cross-origin",

  description:
    "Atronsa is a web design and software development agency in Addis Ababa, Ethiopia. We build modern websites, web applications, and digital solutions for growing businesses.",

  keywords: [
    "web agency Addis Ababa",
    "web design Ethiopia",
    "software company Addis Ababa",
    "software company Ethiopia",
    "website development Addis Ababa",
    "web development agency Ethiopia",
    "web agency",
    "web development",
    "custom software development",
    "SEO services Ethiopia",
    "digital agency Addis Ababa",
    "website development company Ethiopia",
  ],

  authors: [{ name: "Atronsa" }],
  creator: "Atronsa",
  publisher: "Atronsa",

  alternates: {
    canonical: "https://atronsa.com",
  },

  icons: {
    icon: "/favicon.ico",
  },

  openGraph: {
    type: "website",
    locale: "en_US",
    url: "https://atronsa.com",
    siteName: "Atronsa",
    title: "Atronsa | Web Design & Software Agency in Addis Ababa",
    description:
      "Web design and software development agency in Addis Ababa, Ethiopia — building modern websites and digital solutions for growing businesses.",
    images: [
      {
        url: "/og-image.png",
        width: 1200,
        height: 630,
        alt: "Atronsa — Web Design & Software Agency in Addis Ababa",
      },
    ],
  },

  twitter: {
    card: "summary_large_image",
    creator: "@atronsa0",
    title: "Atronsa | Web Design & Software Agency in Addis Ababa",
    description:
      "Web design and software development agency in Addis Ababa, Ethiopia — building modern websites and digital solutions for growing businesses.",
    images: ["/og-image.png"],
  },

  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      "max-snippet": -1,
      "max-image-preview": "large",
      "max-video-preview": -1,
    },
  },

  appleWebApp: {
    capable: true,
    title: "Atronsa",
    statusBarStyle: "black-translucent",
  },
};

const organizationSchema = {
  "@context": "https://schema.org",
  "@type": "Organization",

  name: "Atronsa",
  url: "https://atronsa.com",
  logo: "https://atronsa.com/logo.png",
  image: "https://atronsa.com/og-image.png",

  slogan: "Designed for modern businesses.",

  description:
    "Atronsa is a web design and software development agency based in Addis Ababa, Ethiopia, building modern websites and digital solutions for businesses.",

  foundingLocation: {
    "@type": "Country",
    name: "Ethiopia",
  },

  sameAs: [
    "https://github.com/atronsa",
    "https://x.com/atronsa0",
    "https://facebook.com/atronsa",
    "https://linkedin.com/company/atronsa",
  ],
};

const localBusinessSchema = {
  "@context": "https://schema.org",
  "@type": "ProfessionalService",

  name: "Atronsa",
  url: "https://atronsa.com",
  image: "https://atronsa.com/og-image.png",

  description:
    "Web design and software development agency in Addis Ababa, Ethiopia, offering website design, web application development, and website revamps for businesses.",

  address: {
    "@type": "PostalAddress",
    addressLocality: "Addis Ababa",
    addressCountry: "ET",
  },

  areaServed: [
    {
      "@type": "City",
      name: "Addis Ababa",
    },
    {
      "@type": "Country",
      name: "Ethiopia",
    },
  ],

  priceRange: "ETB",

  sameAs: [
    "https://github.com/atronsa",
    "https://x.com/atronsa0",
    "https://facebook.com/atronsa",
    "https://linkedin.com/company/atronsa",
  ],
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html
      lang="en"
      className={`
        ${poppins.variable}
        ${danleySans.variable}
        h-full
        antialiased
      `}
    >
      <body className="flex min-h-full flex-col">
        <DynamicManifest />

        <noscript>
          <iframe
            src="https://www.googletagmanager.com/ns.html?id=GTM-WVHGVWN3"
            height="0"
            width="0"
            style={{
              display: "none",
              visibility: "hidden",
            }}
          />
        </noscript>

        <Script id="gtm-script" strategy="lazyOnload">
          {`
            (function(w,d,s,l,i){
              w[l]=w[l]||[];
              w[l].push({
                'gtm.start': new Date().getTime(),
                event:'gtm.js'
              });

              var f=d.getElementsByTagName(s)[0],
                  j=d.createElement(s),
                  dl=l!='dataLayer'?'&l='+l:'';

              j.async=true;
              j.src='https://www.googletagmanager.com/gtm.js?id='+i+dl;
              f.parentNode.insertBefore(j,f);
            })(window,document,'script','dataLayer','GTM-WVHGVWN3');
          `}
        </Script>

        <Script
          id="organization-schema"
          type="application/ld+json"
          strategy="afterInteractive"
        >
          {JSON.stringify(organizationSchema)}
        </Script>

        <Script
          id="local-business-schema"
          type="application/ld+json"
          strategy="afterInteractive"
        >
          {JSON.stringify(localBusinessSchema)}
        </Script>

        <Navbar />

        <main className="flex-1">{children}</main>

        <Footer />
      </body>
    </html>
  );
}
