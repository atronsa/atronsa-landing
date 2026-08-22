"use client";

import { useEffect } from "react";
import { usePathname } from "next/navigation";

export function DynamicManifest() {
  const pathname = usePathname();

  useEffect(() => {
    const isAdmin = pathname?.startsWith("/admin");

    document
      .querySelectorAll<HTMLLinkElement>(
        'link[rel="manifest"][data-dynamic="true"]',
      )
      .forEach((link) => {
        link.remove();
      });

    if (isAdmin) {
      const manifestData = {
        name: "Atronsa Admin",
        short_name: "Atronsa",
        description: "Admin dashboard for Atronsa Digital Solutions",
        start_url: "/admin",
        scope: "/admin",
        display: "standalone",
        background_color: "#000000",
        theme_color: "#000000",
        orientation: "portrait",
        icons: [
          {
            src: "/icons/icon-192.png",
            sizes: "192x192",
            type: "image/png",
          },
          {
            src: "/icons/icon-512.png",
            sizes: "512x512",
            type: "image/png",
          },
        ],
      };

      const blob = new Blob([JSON.stringify(manifestData)], {
        type: "application/json",
      });
      const manifestURL = URL.createObjectURL(blob);

      const link = document.createElement("link");
      link.rel = "manifest";
      link.href = manifestURL;
      link.setAttribute("data-dynamic", "true");
      document.head.appendChild(link);

      return () => {
        URL.revokeObjectURL(manifestURL);
      };
    }
  }, [pathname]);

  return null;
}
