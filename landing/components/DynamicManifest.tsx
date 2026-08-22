"use client";

import { useEffect } from "react";
import { usePathname } from "next/navigation";

export function DynamicManifest() {
  const pathname = usePathname();

  useEffect(() => {
    const isAdmin = pathname?.startsWith("/admin");

    const manifestData = {
      name: "Atronsa Digital Solutions",
      short_name: "Atronsa",
      description: isAdmin
        ? "Admin dashboard for Atronsa Digital Solutions"
        : "Web design and software development agency.",
      start_url: isAdmin ? "/admin" : "/",
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

    document
      .querySelectorAll<HTMLLinkElement>('link[rel="manifest"]')
      .forEach((link) => {
        link.remove();
      });

    const link = document.createElement("link");
    link.rel = "manifest";
    link.href = manifestURL;
    document.head.appendChild(link);

    return () => {
      URL.revokeObjectURL(manifestURL);
      document
        .querySelectorAll<HTMLLinkElement>('link[rel="manifest"]')
        .forEach((link) => {
          if (link.href === manifestURL) {
            link.remove();
          }
        });
    };
  }, [pathname]);

  return null;
}
