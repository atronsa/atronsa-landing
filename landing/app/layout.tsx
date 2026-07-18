import type { Metadata } from "next";
import { Montserrat, Poppins } from "next/font/google";
import "./globals.css";

const montserrat = Montserrat({
  subsets: ["latin"],
  variable: "--font-montserrat",
  weight: ["400", "500", "600", "700"],
});

const poppins = Poppins({
  subsets: ["latin"],
  variable: "--font-poppins",
  weight: ["400", "500", "600", "700"],
});

export const metadata: Metadata = {
  metadataBase: new URL("https://atronsa.com"),

  title: {
    default: "Atronsa | Offline-First Digital Payment Infrastructure",
    template: "%s | Atronsa",
  },

  applicationName: "Atronsa",
  category: "Finance",
  generator: "Next.js",
  referrer: "origin-when-cross-origin",

  description:
    "Atronsa is building secure, offline-first, and accessible digital payment infrastructure that connects people, organizations, and communities through one trusted wallet ecosystem.",

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
    title: "Atronsa | Offline-First Digital Payment Infrastructure",
    description:
      "Building secure, offline-first, and accessible digital payment infrastructure.",
    images: [
      {
        url: "/og-image.png",
        width: 1200,
        height: 630,
        alt: "Atronsa",
      },
    ],
  },

  twitter: {
    card: "summary_large_image",
    creator: "@atronsa0",
    title: "Atronsa",
    description:
      "Building secure, offline-first, and accessible digital payment infrastructure.",
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
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const structuredData = {
    "@context": "https://schema.org",
    "@type": "Organization",
    name: "Atronsa",
    url: "https://atronsa.com",
    logo: "https://atronsa.com/logo.png",
    image: "https://atronsa.com/og-image.png",
    slogan: "Building secure, offline-first, and accessible digital payments.",
    description:
      "Atronsa is building secure, offline-first, and accessible digital payment infrastructure for individuals and organizations.",
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

  const GTM_ID = "GTM-WVHGVWN3";

  return (
    <html
      lang="en"
      className={`${montserrat.variable} ${poppins.variable} h-full antialiased`}
    >
      <head>
        {/* Google Tag Manager - Head Script */}
        <script
          dangerouslySetInnerHTML={{
            __html: `(function(w,d,s,l,i){w[l]=w[l]||[];w[l].push({'gtm.start':
new Date().getTime(),event:'gtm.js'});var f=d.getElementsByTagName(s)[0],
j=d.createElement(s),dl=l!='dataLayer'?'&l='+l:'';j.async=true;j.src=
'https://www.googletagmanager.com/gtm.js?id='+i+dl;f.parentNode.insertBefore(j,f);
})(window,document,'script','dataLayer','${GTM_ID}');`,
          }}
        />
      </head>
      <body className="min-h-full flex flex-col">
        {/* Google Tag Manager - NoScript Fallback */}
        <noscript>
          <iframe
            src={`https://www.googletagmanager.com/ns.html?id=${GTM_ID}`}
            height="0"
            width="0"
            style={{ display: "none", visibility: "hidden" }}
          />
        </noscript>

        {/* Structured Data - Organization Schema */}
        <script
          type="application/ld+json"
          dangerouslySetInnerHTML={{
            __html: JSON.stringify(structuredData),
          }}
        />

        {children}
      </body>
    </html>
  );
}
